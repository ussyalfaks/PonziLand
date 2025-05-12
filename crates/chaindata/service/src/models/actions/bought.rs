use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::QueryBuilder;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::model_repository::EventModelRepository;
use crate::models::shared::{EventPrint, Location};
use crate::models::types::U256;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LandBoughtEventModel {
    pub id: EventId,
    pub location: Location,

    pub buyer: String,
    pub seller: String,

    pub price: U256,
    pub token_used: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandBoughtEvent {
    buyer: ContractAddress,
    land_location: Location,

    sold_price: U256,
    seller: ContractAddress,
    token_used: ContractAddress,
}

impl EventPrint for LandBoughtEvent {
    fn pretty(&self) -> String {
        format!(
            "🔨 Land {} bought for {} by {}",
            self.land_location, self.sold_price, self.buyer
        )
    }
}

impl TryFrom<Struct> for LandBoughtEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            buyer: get!(entity, "buyer", ContractAddress)?,
            land_location: get!(entity, "land_location", Location)?,

            sold_price: get!(entity, "sold_price", U256)?,
            seller: get!(entity, "seller", ContractAddress)?,
            token_used: get!(entity, "token_used", ContractAddress)?,
        })
    }
}

impl From<LandBoughtEvent> for LandBoughtEventModel {
    fn from(event: LandBoughtEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            location: event.land_location,
            buyer: format!("{:#x}", event.buyer),
            seller: format!("{:#x}", event.seller),
            price: event.sold_price,
            token_used: format!("{:#x}", event.token_used),
        }
    }
}

impl EventModelRepository for LandBoughtEventModel {
    const TABLE_NAME: &'static str = "event_land_bought";

    fn push_parameters(query: &mut QueryBuilder<'_, sqlx::Postgres>) {
        query.push("id, location, buyer, seller, price, token_used");
    }

    fn push_tuple(
        mut args: sqlx::query_builder::Separated<'_, '_, sqlx::Postgres, &'static str>,
        model: &Self,
    ) {
        args.push_bind(model.id)
            .push_bind(model.location)
            .push_bind(model.buyer.clone())
            .push_bind(model.seller.clone())
            .push_bind(model.price)
            .push_bind(model.token_used.clone());
    }
}
