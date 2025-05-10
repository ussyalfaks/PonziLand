use crate::models::event::EventId;
use crate::models::model_repository::EventModelRepository;
use crate::models::shared::EventPrint;
use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::prelude::FromRow;
use sqlx::QueryBuilder;
use torii_ingester::{error::ToriiConversionError, prelude::*};

#[derive(Debug, Clone, FromRow)]
pub struct AddressRemovedEventModel {
    pub id: EventId,
    pub at: NaiveDateTime,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressRemovedEvent {
    address: ContractAddress,
    #[serde(deserialize_with = "deserialize_number_from_string")]
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

impl EventModelRepository for AddressRemovedEventModel {
    const TABLE_NAME: &'static str = "event_address_removed";

    fn push_parameters(query: &mut QueryBuilder<'_, sqlx::Postgres>) {
        query.push("id, at, address");
    }

    fn push_tuple(
        mut args: sqlx::query_builder::Separated<'_, '_, sqlx::Postgres, &'static str>,
        model: &Self,
    ) {
        args.push_bind(model.id)
            .push_bind(model.at)
            .push_bind(model.address.clone());
    }
}
