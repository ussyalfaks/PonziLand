use std::sync::{Arc, Mutex};

mod models;
mod repositories;

pub mod tasks;

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
        })
    }

    pub async fn stop(self: &Arc<Self>) {}

    pub async fn start(self: &Arc<Self>) {}

    async fn process_events(self: &Arc<Self>, mut rx: oneshot::Receiver<()>) {
        let this = self.clone();

        tokio::spawn(async move {});
    }
}
