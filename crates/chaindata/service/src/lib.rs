use std::sync::Arc;

pub mod models;
pub mod repositories;

pub mod tasks;

pub type Database = PgPool;

use repositories::{event::EventRepository, land::LandRepository, land_stake::LandStakeRepository};
use serde::*;
use sqlx::PgPool;
use starknet::core::types::Felt;
use tasks::{
    event_listener::EventListenerTask, model_listener::ModelListenerTask, Task, TaskWrapper,
};
use tokio::join;
use torii_ingester::{ToriiClient, ToriiConfiguration};

/// ChainDataService is a service that handles the importation and syncing of new events and data
/// to the database for further processing.
pub struct ChainDataService {
    event_listener_task: TaskWrapper<EventListenerTask>,
    model_listener_task: TaskWrapper<ModelListenerTask>,
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

        let client = Arc::new(
            ToriiClient::new(&config)
                .await
                .expect("Error while setting up torii client"),
        );

        let event_repository = Arc::new(EventRepository::new(database.clone()));
        let land_repository = Arc::new(LandRepository::new(database.clone()));
        let land_stake_repository = Arc::new(LandStakeRepository::new(database.clone()));

        Arc::new(Self {
            event_listener_task: EventListenerTask::new(client.clone(), event_repository).wrap(),
            model_listener_task: ModelListenerTask::new(
                client.clone(),
                land_repository,
                land_stake_repository,
            )
            .wrap(),
        })
    }

    pub async fn stop(self: &Arc<Self>) {
        self.event_listener_task.stop();
        self.model_listener_task.stop();
    }

    pub async fn start(self: &Arc<Self>) {
        // Start all in parallel
        join!(
            self.event_listener_task.start(),
            self.model_listener_task.start()
        );
    }
}
