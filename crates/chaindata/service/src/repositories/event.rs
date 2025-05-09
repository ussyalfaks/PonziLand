
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    models::event::{Event, EventId, EventType, FilledEvent},
    Database,
};

pub struct EventRepository {
    db: Database,
}

impl EventRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_event_by_id(&self, id: EventId) -> Result<Event, sqlx::Error> {
        query_as!(
            Event,
            r#"
            SELECT
                id as "id: _",
                at,
                event_type as "event_type: _"
            FROM event
            WHERE id = $1
        "#,
            id as EventId
        )
        .fetch_one(&*self.db)
        .await
    }

    pub async fn save_event(&self, event: FilledEvent) -> Result<EventId, sqlx::Error> {
        // Start a TX
        let mut tx = self.db.begin().await?;

        // Generate a new id
        let id = EventId(Uuid::new_v4());

        // Insert the event
        let id: EventId = query!(
            r#"
            INSERT INTO event (id, at, event_type)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
            id as EventId,
            event.at,
            EventType::from(event.data) as EventType
        )
        .fetch_one(&mut *tx)
        .await?
        .id
        .into();

        // Commit the TX
        tx.commit().await?;

        Ok(id)
    }
}
