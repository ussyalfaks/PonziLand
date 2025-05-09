use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::shared::{EventPrint, Location};

#[derive(Debug, Clone, FromRow)]
pub struct LandNukedEventModel {
    pub id: EventId,
    pub location: i16,
    pub owner: String,
    pub at: NaiveDateTime,
}

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

impl From<LandNukedEvent> for LandNukedEventModel {
    fn from(event: LandNukedEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            location: event.land_location.into(),
            owner: format!("{:#x}", event.owner_nuked),
            at: chrono::Utc::now().naive_utc(),
        }
    }
}
