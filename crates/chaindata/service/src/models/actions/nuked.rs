use torii_ingester::{error::ToriiConversionError, prelude::*};

use serde::{Deserialize, Serialize};

use crate::models::shared::{EventPrint, Location};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandNukedEvent {
    pub owner_nuked: ContractAddress,
    pub land_location: Location,
}

impl EventPrint for LandNukedEvent {
    fn pretty(&self) -> String {
        format!(
            "💥 {} owned by {:x} was nuked!",
            self.land_location, self.owner_nuked
        )
    }
}

impl TryFrom<Struct> for LandNukedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            owner_nuked: get!(entity, "owner_nuked", ContractAddress)?,
            land_location: get!(entity, "land_location", Location)?,
        })
    }
}
