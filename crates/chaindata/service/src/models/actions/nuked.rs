use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::QueryBuilder;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::model_repository::EventModelRepository;
use crate::models::shared::{EventPrint, Location};

#[derive(Debug, Clone, FromRow)]
pub struct LandNukedEventModel {
    pub id: EventId,
    pub location: Location,
    pub owner: String,
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

impl EventModelRepository for LandNukedEventModel {
    const TABLE_NAME: &'static str = "event_land_nuked";

    fn push_parameters(query: &mut QueryBuilder<'_, sqlx::Postgres>) {
        query.push("id, location, owner");
    }

    fn push_tuple(
        mut args: sqlx::query_builder::Separated<'_, '_, sqlx::Postgres, &'static str>,
        model: &Self,
    ) {
        args.push_bind(model.id)
            .push_bind(model.location)
            .push_bind(model.owner.clone());
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
            location: event.land_location,
            owner: format!("{:#x}", event.owner_nuked),
        }
    }
}
