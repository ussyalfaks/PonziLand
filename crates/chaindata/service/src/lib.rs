use std::sync::{Arc, Mutex};

mod models;
mod repositories;

pub type Database = PgPool;

use chrono::Utc;
use models::event::{EventData, EventId, FilledEvent};
use repositories::event::EventRepository;
use serde::*;
use sqlx::PgPool;
use starknet::core::types::Felt;
use tokio::{select, sync::oneshot};
use tokio_stream::StreamExt;
use torii_ingester::{RawToriiData, ToriiClient, ToriiConfiguration};
use tracing::{debug, info};
use uuid::Uuid;

/// ChainDataService is a service that handles the importation and syncing of new events and data
/// to the database for further processing.
pub struct ChainDataService {
    client: ToriiClient,
    event_repository: EventRepository,
    stop_handle: Mutex<Option<oneshot::Sender<()>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChainDataServiceConfiguration {
    pub torii_url: String,
    pub world_address: String,
}

impl ChainDataService {
    pub async fn new(database: Database, config: ChainDataServiceConfiguration) -> Arc<Self> {
        let config = ToriiConfiguration {
            base_url: config.torii_url.clone(),
            world_address: Felt::from_hex(&config.world_address)
                .expect("Unexpected world address."),
        };

        let client = ToriiClient::new(&config)
            .await
            .expect("Error while setting up torii client");

        Arc::new(Self {
            client,
            event_repository: EventRepository::new(database),
            stop_handle: Mutex::new(None),
        })
    }

    pub async fn stop(self: &Arc<Self>) {
        // Acquire the lock on the stop handle
        if let Ok(mut guard) = self.stop_handle.lock() {
            // Take the sender out of the Option (replacing it with None)
            if let Some(sender) = guard.take() {
                // Send the stop signal, ignoring errors if the receiver was dropped
                let _ = sender.send(());
                info!("Stop signal sent to ChainDataService");
            } else {
                info!("ChainDataService already stopped");
            }
        } else {
            info!("Failed to acquire lock for stopping ChainDataService");
        }
    }

    pub async fn start(self: &Arc<Self>) {
        let (tx, rx) = oneshot::channel();

        // Store the sender in the mutex
        if let Ok(mut guard) = self.stop_handle.lock() {
            if guard.is_some() {
                info!("ChainDataService already started");
                return;
            }
            *guard = Some(tx);
        } else {
            info!("Failed to acquire lock for starting ChainDataService");
            return;
        }

        self.process_events(rx).await;
    }

    async fn process_events(self: &Arc<Self>, mut rx: oneshot::Receiver<()>) {
        let this = self.clone();

        tokio::spawn(async move {
            // Start both a sql catch up and a torii event listener
            let last_check = this
                .event_repository
                .get_last_event_date()
                .await
                .expect("Too bad...");

            let events_catchup = this
                .client
                .get_all_events_after(last_check)
                .await
                .expect("Error while fetching entities");

            let events_listener = this
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
                                this.process_event(event).await;
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
        });
    }

    async fn process_event(&self, event: RawToriiData) {
        // Parse and save the event
        let event = match event {
            RawToriiData::Grpc(data) => {
                debug!("Processing GRPC event");

                FilledEvent {
                    id: EventId(Uuid::new_v4()),
                    at: Utc::now().naive_utc(),
                    data: EventData::try_from(data)
                        .expect("An error occurred while deserializing model"),
                }
            }
            RawToriiData::Json { name, data, at } => FilledEvent {
                id: EventId(Uuid::new_v4()),
                at: at.naive_utc(),
                data: EventData::from_json(&*name, data.clone()).expect(&*format!(
                    "An error occurred while deserializing model for event {}: {:#?}",
                    name, data
                )),
            },
        };

        self.event_repository
            .save_event(event)
            .await
            .expect("An error occurred while saving event");
    }
}
