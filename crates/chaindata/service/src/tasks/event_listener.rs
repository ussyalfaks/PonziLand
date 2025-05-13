use std::sync::Arc;

use chrono::Utc;
use tokio::select;
use tokio_stream::StreamExt;
use torii_ingester::{RawToriiData, ToriiClient};
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    models::event::{EventData, EventId, FilledEvent},
    repositories::event::EventRepository,
};

use super::Task;

/// `EventListenerTask` is a task that subscribes to the events of the on-chain indexer (torii),
/// and pushes them to the local database.
pub struct EventListenerTask {
    client: Arc<ToriiClient>,
    event_repository: Arc<EventRepository>,
}

impl EventListenerTask {
    pub fn new(client: Arc<ToriiClient>, event_repository: Arc<EventRepository>) -> Self {
        Self {
            client,
            event_repository,
        }
    }

    async fn process_event(&self, event: RawToriiData) {
        // Parse and save the event
        let event = match event {
            RawToriiData::Grpc(data) => {
                debug!("Processing GRPC event");

                FilledEvent {
                    id: EventId::new(),
                    at: Utc::now().naive_utc(),
                    data: EventData::try_from(data)
                        .expect("An error occurred while deserializing model"),
                }
            }
            RawToriiData::Json { name, data, at } => FilledEvent {
                id: EventId(Uuid::new_v4()),
                at: at.naive_utc(),
                data: EventData::from_json(&name, data.clone()).unwrap_or_else(|_| {
                    panic!(
                        "An error occurred while deserializing model for event {}: {:#?}",
                        name, data
                    )
                }),
            },
        };

        self.event_repository
            .save_event(event)
            .await
            .expect("An error occurred while saving event");
    }
}

#[async_trait::async_trait]
impl Task for EventListenerTask {
    const NAME: &'static str = "EventListenerTask";

    async fn do_task(self: std::sync::Arc<Self>, mut rx: tokio::sync::oneshot::Receiver<()>) {
        // Start both a sql catch up and a torii event listener
        let last_check = self
            .event_repository
            .get_last_event_date()
            .await
            .expect("Too bad...");

        let events_catchup = self
            .client
            .get_all_events_after(last_check)
            .await
            .expect("Error while fetching entities");

        let events_listener = self
            .client
            .subscribe_events()
            .await
            .expect("Error while subscribing for events");

        // Join the two streams (on the heap to not anger the borrow checker)
        let mut events = Box::pin(events_catchup.merge(events_listener));

        // Process events
        loop {
            select! {
                maybe_event = events.next() => {
                    match maybe_event {
                        Some(event) => {
                            info!("Processing new event");
                            self.process_event(event).await;
                        }
                        None => {
                            info!("Event stream completed, exiting event processing loop");
                            break;
                        }
                    }
                },
                stop_result = &mut rx => {
                    match stop_result {
                        Ok(_) => info!("Received stop signal, shutting down event processing"),
                        Err(e) => info!("Stop channel closed unexpectedly: {}", e),
                    }
                    break;
                }
            }
        }
    }
}
