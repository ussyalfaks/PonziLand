use std::{cell::OnceCell, sync::Arc};

mod models;
mod repositories;

pub type Database = Arc<PgPool>;

use serde::*;
use sqlx::PgPool;
use starknet::core::types::Felt;
use tokio::sync::oneshot;
use torii_ingester::{ToriiClient, ToriiConfiguration};

/// ChainDataService is a service that handles the importation and syncing of new events and data
/// to the database for further processing.
pub struct ChainDataService {
    client: ToriiClient,
    stop_handle: OnceCell<oneshot::Sender<()>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChainDataServiceConfiguration {
    torii_url: String,
    world_address: String,
}

impl ChainDataService {
    pub async fn new(config: &ChainDataServiceConfiguration) -> Self {
        let config = ToriiConfiguration {
            base_url: config.torii_url.clone(),
            world_address: Felt::from_hex(&config.world_address)
                .expect("Unexpected world address."),
        };

        let client = ToriiClient::new(&config)
            .await
            .expect("Error while setting up torii client");

        Self {
            client,
            stop_handle: OnceCell::new(),
        }
    }

    async fn process_events(&self, rx: oneshot::Sender<()>) {}
}
