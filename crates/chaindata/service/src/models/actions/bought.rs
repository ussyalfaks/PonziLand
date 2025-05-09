use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::shared::{EventPrint, Location};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LandBoughtEventModel {
    pub id: EventId,
    pub location: i16,

    pub buyer: String,
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
            location: event.land_location.into(),
            buyer: format!("{:#x}", event.buyer),
            price: event.sold_price,
            token_used: format!("{:#x}", event.token_used),
        }
    }
}
