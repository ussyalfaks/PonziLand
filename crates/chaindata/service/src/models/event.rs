use super::actions::*;
use super::auth::*;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Uuid;
use sqlx::{Decode, Encode};
use torii_ingester::error::ToriiConversionError;
use torii_ingester::prelude::Struct;
use torii_ingester::RawToriiData;

#[derive(Clone, Copy, PartialEq, Eq, Debug, sqlx::Type, Hash, Serialize, Deserialize)]
#[sqlx(transparent, type_name = "uuid")]
pub struct EventId(pub Uuid);

impl From<Uuid> for EventId {
    fn from(uuid: Uuid) -> Self {
        EventId(uuid)
    }
}

impl From<EventId> for Uuid {
    fn from(id: EventId) -> Self {
        id.0
    }
}
impl AsRef<Uuid> for EventId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "event_type")]
pub enum EventType {
    #[sqlx(rename = "ponzi_land-AuctionFinishedEvent")]
    AuctionFinished,
    #[sqlx(rename = "ponzi_land-LandBoughtEvent")]
    LandBought,
    #[sqlx(rename = "ponzi_land-LandNukedEvent")]
    LandNuked,
    #[sqlx(rename = "ponzi_land-NewAuctionEvent")]
    NewAuction,
    #[sqlx(rename = "ponzi_land-RemainingStakeEvent")]
    RemainingStake,
    #[sqlx(rename = "ponzi_land-AddressAuthorizedEvent")]
    AddressAuthorized,
    #[sqlx(rename = "ponzi_land-AddressRemovedEvent")]
    AddressRemoved,
    #[sqlx(rename = "ponzi_land-VerifierUpdatedEvent")]
    VerifierUpdated,
}

#[derive(FromRow, Clone, Debug)]
pub struct Event {
    pub id: EventId,
    pub at: chrono::NaiveDateTime,
    pub event_type: EventType,
}

#[derive(Clone, Debug)]
pub enum EventData {
    AuctionFinished(AuctionFinishedEvent),
    LandBought(LandBoughtEvent),
    LandNuked(LandNukedEvent),
    NewAuction(NewAuctionEvent),
    RemainingStake(RemainingStakeEvent),
    AddressAuthorized(AddressAuthorizedEvent),
    AddressRemoved(AddressRemovedEvent),
    VerifierUpdated(VerifierUpdatedEvent),
}

#[derive(Clone, Debug)]
pub enum EventModelData {
    AuctionFinished(AuctionFinishedEventModel),
    LandBought(LandBoughtEventModel),
    LandNuked(LandNukedEventModel),
    NewAuction(NewAuctionEventModel),
    RemainingStake(RemainingStakeEventModel),
    AddressAuthorized(AddressAuthorizedEventModel),
    AddressRemoved(AddressRemovedEventModel),
    VerifierUpdated(VerifierUpdatedEventModel),
}

impl From<EventData> for EventType {
    fn from(value: EventData) -> Self {
        match value {
            EventData::AuctionFinished(_) => EventType::AuctionFinished,
            EventData::LandBought(_) => EventType::LandBought,
            EventData::LandNuked(_) => EventType::LandNuked,
            EventData::NewAuction(_) => EventType::NewAuction,
            EventData::RemainingStake(_) => EventType::RemainingStake,
            EventData::AddressAuthorized(_) => EventType::AddressAuthorized,
            EventData::AddressRemoved(_) => EventType::AddressRemoved,
            EventData::VerifierUpdated(_) => EventType::VerifierUpdated,
        }
    }
}

impl From<EventModelData> for EventType {
    fn from(value: EventModelData) -> Self {
        match value {
            EventModelData::AuctionFinished(_) => EventType::AuctionFinished,
            EventModelData::LandBought(_) => EventType::LandBought,
            EventModelData::LandNuked(_) => EventType::LandNuked,
            EventModelData::NewAuction(_) => EventType::NewAuction,
            EventModelData::RemainingStake(_) => EventType::RemainingStake,
            EventModelData::AddressAuthorized(_) => EventType::AddressAuthorized,
            EventModelData::AddressRemoved(_) => EventType::AddressRemoved,
            EventModelData::VerifierUpdated(_) => EventType::VerifierUpdated,
        }
    }
}

impl TryFrom<Struct> for EventData {
    type Error = ToriiConversionError;
    fn try_from(value: Struct) -> Result<Self, Self::Error> {
        Ok(match &*value.name {
            "ponzi_land-AuctionFinishedEvent" => {
                EventData::AuctionFinished(AuctionFinishedEvent::try_from(value)?)
            }
            "ponzi_land-LandBoughtEvent" => {
                EventData::LandBought(LandBoughtEvent::try_from(value)?)
            }
            "ponzi_land-LandNukedEvent" => EventData::LandNuked(LandNukedEvent::try_from(value)?),
            "ponzi_land-NewAuctionEvent" => {
                EventData::NewAuction(NewAuctionEvent::try_from(value)?)
            }
            "ponzi_land-RemainingStakeEvent" => {
                EventData::RemainingStake(RemainingStakeEvent::try_from(value)?)
            }
            "ponzi_land-AddressAuthorizedEvent" => {
                EventData::AddressAuthorized(AddressAuthorizedEvent::try_from(value)?)
            }
            "ponzi_land-AddressRemovedEvent" => {
                EventData::AddressRemoved(AddressRemovedEvent::try_from(value)?)
            }
            "ponzi_land-VerifierUpdatedEvent" => {
                EventData::VerifierUpdated(VerifierUpdatedEvent::try_from(value)?)
            }
            _ => panic!("Unknown event type"),
        })
    }
}

#[derive(Clone, Debug)]
pub struct FilledEvent {
    pub id: EventId,
    pub at: chrono::NaiveDateTime,
    pub data: EventData,
}

#[derive(Clone, Debug)]
pub struct FilledEventModel {
    pub id: EventId,
    pub at: NaiveDateTime,
    pub data: EventModelData,
}

impl From<EventData> for EventModelData {
    fn from(value: EventData) -> Self {
        match value {
            EventData::AuctionFinished(e) => EventModelData::AuctionFinished(e.into()),
            EventData::LandBought(e) => EventModelData::LandBought(e.into()),
            EventData::LandNuked(e) => EventModelData::LandNuked(e.into()),
            EventData::NewAuction(e) => EventModelData::NewAuction(e.into()),
            EventData::RemainingStake(e) => EventModelData::RemainingStake(e.into()),
            EventData::AddressAuthorized(e) => EventModelData::AddressAuthorized(e.into()),
            EventData::AddressRemoved(e) => EventModelData::AddressRemoved(e.into()),
            EventData::VerifierUpdated(e) => EventModelData::VerifierUpdated(e.into()),
        }
    }
}

impl From<FilledEvent> for FilledEventModel {
    fn from(event: FilledEvent) -> Self {
        Self {
            id: event.id,
            at: event.at,
            data: event.data.into(),
        }
    }
}
