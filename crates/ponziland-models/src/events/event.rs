use serde_json::Value;
use torii_ingester::error::ToriiConversionError;
use torii_ingester::prelude::Struct;

use super::actions::{AuctionFinishedEvent, LandBoughtEvent, LandNukedEvent, NewAuctionEvent};
use super::auth::{AddressAuthorizedEvent, AddressRemovedEvent, VerifierUpdatedEvent};

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

#[derive(Clone, Debug)]
pub struct Event {
    pub at: chrono::NaiveDateTime,
    pub data: EventData,
}

impl EventData {
    /// Create an event data from a JSON value.
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON value cannot be deserialized into the corresponding event data.
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
