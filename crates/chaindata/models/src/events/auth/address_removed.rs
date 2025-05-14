use chrono::NaiveDateTime;
use ponziland_models::events::auth::AddressRemovedEvent;
use sqlx::prelude::FromRow;

use crate::{events::EventId, utils::date::naive_from_u64};

#[derive(Debug, Clone, FromRow)]
pub struct AddressRemovedEventModel {
    pub id: EventId,
    pub at: NaiveDateTime,
    pub address: String,
}

impl From<AddressRemovedEvent> for AddressRemovedEventModel {
    fn from(event: AddressRemovedEvent) -> Self {
        Self {
            id: EventId(sqlx::types::Uuid::new_v4()),
            at: naive_from_u64(event.authorized_at),
            address: format!("{:#x}", event.address),
        }
    }
}
