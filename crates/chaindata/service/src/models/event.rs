use super::actions::*;
use super::auth::*;
use super::model_repository::EventModelRepository;
use crate::define_id;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use torii_ingester::error::ToriiConversionError;
use torii_ingester::prelude::Struct;

define_id!(EventId, Uuid);

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
    AddressAuthorized(AddressAuthorizedEvent),
    AddressRemoved(AddressRemovedEvent),
    VerifierUpdated(VerifierUpdatedEvent),
}

impl From<&EventData> for EventType {
    fn from(value: &EventData) -> Self {
        match value {
            EventData::AuctionFinished(_) => EventType::AuctionFinished,
            EventData::LandBought(_) => EventType::LandBought,
            EventData::LandNuked(_) => EventType::LandNuked,
            EventData::NewAuction(_) => EventType::NewAuction,
            EventData::AddressAuthorized(_) => EventType::AddressAuthorized,
            EventData::AddressRemoved(_) => EventType::AddressRemoved,
            EventData::VerifierUpdated(_) => EventType::VerifierUpdated,
        }
    }
}

impl EventData {
    pub async fn save<'e, Conn>(&self, db: Conn) -> Result<(), sqlx::Error>
    where
        Conn: 'e + sqlx::Executor<'e, Database = sqlx::Postgres>,
    {
        match self.clone() {
            EventData::AuctionFinished(event) => AuctionFinishedEventModel::save(db, event.into()),
            EventData::LandBought(event) => LandBoughtEventModel::save(db, event.into()),
            EventData::LandNuked(event) => LandNukedEventModel::save(db, event.into()),
            EventData::NewAuction(event) => NewAuctionEventModel::save(db, event.into()),
            EventData::AddressAuthorized(event) => {
                AddressAuthorizedEventModel::save(db, event.into())
            }
            EventData::AddressRemoved(event) => AddressRemovedEventModel::save(db, event.into()),
            EventData::VerifierUpdated(event) => VerifierUpdatedEventModel::save(db, event.into()),
        }
        .await
    }
}

impl EventData {
    pub fn from_json(name: &str, json: Value) -> Result<Self, ToriiConversionError> {
        Ok(match name {
            "ponzi_land-AuctionFinishedEvent" => {
                EventData::AuctionFinished(serde_json::from_value(json)?)
            }
            "ponzi_land-LandBoughtEvent" => EventData::LandBought(serde_json::from_value(json)?),
            "ponzi_land-LandNukedEvent" => EventData::LandNuked(serde_json::from_value(json)?),
            "ponzi_land-NewAuctionEvent" => EventData::NewAuction(serde_json::from_value(json)?),
            "ponzi_land-AddressAuthorizedEvent" => {
                EventData::AddressAuthorized(serde_json::from_value(json)?)
            }
            "ponzi_land-AddressRemovedEvent" => {
                EventData::AddressRemoved(serde_json::from_value(json)?)
            }
            "ponzi_land-VerifierUpdatedEvent" => {
                EventData::VerifierUpdated(serde_json::from_value(json)?)
            }
            name => Err(ToriiConversionError::UnknownVariant(name.to_string()))?,
        })
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
            "ponzi_land-AddressAuthorizedEvent" => {
                EventData::AddressAuthorized(AddressAuthorizedEvent::try_from(value)?)
            }
            "ponzi_land-AddressRemovedEvent" => {
                EventData::AddressRemoved(AddressRemovedEvent::try_from(value)?)
            }
            "ponzi_land-VerifierUpdatedEvent" => {
                EventData::VerifierUpdated(VerifierUpdatedEvent::try_from(value)?)
            }
            name => Err(ToriiConversionError::UnknownVariant(name.to_string()))?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct FilledEvent {
    pub id: EventId,
    pub at: chrono::NaiveDateTime,
    pub data: EventData,
}
