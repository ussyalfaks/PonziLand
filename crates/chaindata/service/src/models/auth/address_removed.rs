use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::shared::EventPrint;

#[derive(Debug, Clone, FromRow)]
pub struct AddressRemovedEventModel {
    pub id: EventId,
    pub at: NaiveDateTime,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressRemovedEvent {
    address: ContractAddress,
    authorized_at: u64,
}

impl EventPrint for AddressRemovedEvent {
    fn pretty(&self) -> String {
        format!("🔒 Address unauthorized: {}", self.address)
    }
}

impl TryFrom<Struct> for AddressRemovedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            address: get!(entity, "address", ContractAddress)?,
            authorized_at: get!(entity, "authorized_at", u64)?,
        })
    }
}

impl From<AddressRemovedEvent> for AddressRemovedEventModel {
    fn from(event: AddressRemovedEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            at: DateTime::from_timestamp(event.authorized_at as i64, 0)
                .unwrap()
                .naive_utc(),
            address: format!("{:#x}", event.address),
        }
    }
}
