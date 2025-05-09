use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::shared::{EventPrint, Location};

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
