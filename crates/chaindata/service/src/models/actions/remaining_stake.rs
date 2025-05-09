use serde::{Deserialize, Serialize};
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::shared::{EventPrint, Location};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemainingStakeEvent {
    pub land_location: Location,
    pub remaining_stake: U256,
}

impl EventPrint for RemainingStakeEvent {
    fn pretty(&self) -> String {
        format!(
            "💰 Remaining stake for land {} is {}",
            self.land_location, self.remaining_stake
        )
    }
}

impl TryFrom<Struct> for RemainingStakeEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            land_location: get!(entity, "land_location", Location)?,
            remaining_stake: get!(entity, "remaining_stake", U256)?,
        })
    }
}
