use crate::models::event::EventId;
use crate::models::model_repository::EventModelRepository;
use crate::models::shared::{EventPrint, Location};
use crate::models::types::U256;
use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::QueryBuilder;
use torii_ingester::{error::ToriiConversionError, prelude::*};

#[derive(Debug, Clone, FromRow)]
pub struct AuctionFinishedEventModel {
    pub id: EventId,
    pub location: i16,
    pub buyer: String,
    pub price: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionFinishedEvent {
    pub land_location: Location,
    pub buyer: ContractAddress,
    pub final_price: U256,
}

impl EventPrint for AuctionFinishedEvent {
    fn pretty(&self) -> String {
        format!(
            "🏁 Auction for land {} ended with a final price of {}, bought by {}",
            self.land_location, self.final_price, self.buyer
        )
    }
}

impl TryFrom<Struct> for AuctionFinishedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            land_location: get!(entity, "land_location", Location)?,
            buyer: get!(entity, "buyer", ContractAddress)?,
            final_price: get!(entity, "final_price", U256)?,
        })
    }
}

impl From<AuctionFinishedEvent> for AuctionFinishedEventModel {
    fn from(event: AuctionFinishedEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            location: event.land_location.into(),
            buyer: format!("{:#x}", event.buyer),
            price: event.final_price,
        }
    }
}

impl EventModelRepository for AuctionFinishedEventModel {
    const TABLE_NAME: &'static str = "event_auction_finished";

    fn push_parameters(query: &mut QueryBuilder<'_, sqlx::Postgres>) {
        query.push("id, location, buyer, price");
    }

    fn push_tuple(
        mut args: sqlx::query_builder::Separated<'_, '_, sqlx::Postgres, &'static str>,
        model: &Self,
    ) {
        args.push_bind(model.id)
            .push_bind(model.location)
            .push_bind(model.buyer.clone())
            .push_bind(model.price);
    }
}
