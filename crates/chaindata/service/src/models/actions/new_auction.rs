use torii_ingester::{error::ToriiConversionError, prelude::*};

use serde::{Deserialize, Serialize};

use crate::models::shared::{EventPrint, Location};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAuctionEvent {
    land_location: Location,
    start_time: u64,
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
            start_time: get!(entity, "start_time", u64)?,
            start_price: get!(entity, "start_price", U256)?,
            floor_price: get!(entity, "floor_price", U256)?,
        })
    }
}
