use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::QueryBuilder;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::model_repository::EventModelRepository;
use crate::models::shared::{EventPrint, Location};
use crate::models::types::U256;

#[derive(Debug, Clone, FromRow)]
pub struct NewAuctionEventModel {
    pub id: EventId,
    pub location: i16,
    pub starting_price: U256,
    pub floor_price: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAuctionEvent {
    land_location: Location,
    start_price: U256,
    floor_price: U256,
}

impl EventPrint for NewAuctionEvent {
    fn pretty(&self) -> String {
        format!(
            "🔨 New auction for land {} starting at {} with a floor price of {}",
            self.land_location, self.start_price, self.floor_price
        )
    }
}

impl TryFrom<Struct> for NewAuctionEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            land_location: get!(entity, "land_location", Location)?,
            start_price: get!(entity, "start_price", U256)?,
            floor_price: get!(entity, "floor_price", U256)?,
        })
    }
}

impl From<NewAuctionEvent> for NewAuctionEventModel {
    fn from(event: NewAuctionEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            location: event.land_location.into(),
            starting_price: event.start_price,
            floor_price: event.floor_price,
        }
    }
}

impl EventModelRepository for NewAuctionEventModel {
    const TABLE_NAME: &'static str = "event_new_auction";

    fn push_parameters(query: &mut QueryBuilder<'_, sqlx::Postgres>) {
        query.push("id, location, starting_price, floor_price");
    }

    fn push_tuple(
        mut args: sqlx::query_builder::Separated<'_, '_, sqlx::Postgres, &'static str>,
        model: &Self,
    ) {
        args.push_bind(model.id)
            .push_bind(model.location)
            .push_bind(model.starting_price)
            .push_bind(model.floor_price);
    }
}
