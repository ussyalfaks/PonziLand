use ponziland_models::events::auth::VerifierUpdatedEvent;
use sqlx::prelude::FromRow;

use crate::events::EventId;

#[derive(Debug, Clone, FromRow)]
pub struct VerifierUpdatedEventModel {
    pub id: EventId,
    pub new_verifier: String,
    pub old_verifier: String,
}

impl From<VerifierUpdatedEvent> for VerifierUpdatedEventModel {
    fn from(event: VerifierUpdatedEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            new_verifier: format!("{:#x}", event.new_verifier),
            old_verifier: format!("{:#x}", event.old_verifier),
        }
    }
}
