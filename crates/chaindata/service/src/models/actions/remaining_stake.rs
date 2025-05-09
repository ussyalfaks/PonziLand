use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::shared::{EventPrint, Location};

#[derive(Debug, Clone, FromRow)]
pub struct RemainingStakeEventModel {
    pub id: EventId,
    pub location: i16,
    pub remaining_stake: U256,
}

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

impl From<RemainingStakeEvent> for RemainingStakeEventModel {
    fn from(event: RemainingStakeEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            location: event.land_location.into(),
            remaining_stake: event.remaining_stake,
        }
    }
}
