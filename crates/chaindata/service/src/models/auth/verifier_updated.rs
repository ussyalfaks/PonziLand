use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use starknet::core::types::Felt;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::event::EventId;
use crate::models::shared::EventPrint;

// Note: There's no corresponding table in the SQL schema for this event yet
#[derive(Debug, Clone, FromRow)]
pub struct VerifierUpdatedEventModel {
    pub id: EventId,
    pub at: NaiveDateTime,
    pub new_verifier: String,
    pub old_verifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierUpdatedEvent {
    new_verifier: Felt,
    old_verifier: Felt,
}

impl EventPrint for VerifierUpdatedEvent {
    fn pretty(&self) -> String {
        format!("🔒 Updated verifier to {:#x}", self.new_verifier)
    }
}

impl TryFrom<Struct> for VerifierUpdatedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            new_verifier: get!(entity, "new_verifier", Felt)?,
            old_verifier: get!(entity, "old_verifier", Felt)?,
        })
    }
}

impl From<VerifierUpdatedEvent> for VerifierUpdatedEventModel {
    fn from(event: VerifierUpdatedEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            at: chrono::Utc::now().naive_utc(),
            new_verifier: format!("{:#x}", event.new_verifier),
            old_verifier: format!("{:#x}", event.old_verifier),
        }
    }
}
