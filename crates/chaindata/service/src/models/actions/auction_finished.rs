use crate::models::event::EventId;
use crate::models::model_repository::EventModelRepository;
use crate::models::shared::{EventPrint, Location};
use crate::models::types::U256;
use chrono::{DateTime, NaiveDateTime};
use sqlx::prelude::FromRow;
use sqlx::QueryBuilder;
use torii_ingester::{error::ToriiConversionError, prelude::*};

#[derive(Debug, Clone, FromRow)]
pub struct AuctionFinishedEventModel {
    pub id: EventId,
    pub location: i16,
    pub start_time: NaiveDateTime,
    pub buyer: String,
    pub at: NaiveDateTime,
    pub price: U256,
}

#[derive(Debug, Clone)]
pub struct AuctionFinishedEvent {
    pub land_location: Location,
    pub buyer: ContractAddress,
    pub start_time: u64,
    pub final_time: u64,
    pub final_price: U256,
}

impl EventPrint for AuctionFinishedEvent {
    fn pretty(&self) -> String {
        format!(
            "🏁 Auction for land {} started at {} and ended at {} with a final price of {}, bought by {}",
            self.land_location, self.start_time, self.final_time, self.final_price, self.buyer
        )
    }
}

impl TryFrom<Struct> for AuctionFinishedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            land_location: get!(entity, "land_location", Location)?,
            buyer: get!(entity, "buyer", ContractAddress)?,
            start_time: get!(entity, "start_time", u64)?,
            final_time: get!(entity, "final_time", u64)?,
            final_price: get!(entity, "final_price", U256)?,
        })
    }
}

impl From<AuctionFinishedEvent> for AuctionFinishedEventModel {
    fn from(event: AuctionFinishedEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            location: event.land_location.into(),
            start_time: DateTime::from_timestamp(event.start_time as i64, 0)
                .unwrap()
                .naive_utc(),
            buyer: format!("{:#x}", event.buyer),
            at: DateTime::from_timestamp(event.final_time as i64, 0)
                .unwrap()
                .naive_utc(),
            price: event.final_price,
        }
    }
}

impl EventModelRepository for AuctionFinishedEventModel {
    const TABLE_NAME: &'static str = "event_auction_finished";

    fn push_parameters(query: &mut QueryBuilder<'_, sqlx::Postgres>) {
        query.push("id, location, start_time, buyer, at, price");
    }

    fn push_tuple(
        mut args: sqlx::query_builder::Separated<'_, '_, sqlx::Postgres, &'static str>,
        model: &Self,
    ) {
        args.push_bind(model.id)
            .push_bind(model.location)
            .push_bind(model.start_time)
            .push_bind(model.buyer.clone())
            .push_bind(model.at)
            .push_bind(model.price);
    }
}
