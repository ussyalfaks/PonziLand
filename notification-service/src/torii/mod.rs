use std::u32;

use models::Event;
use starknet_crypto::Felt;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use torii_client::client::Client;
use torii_grpc::{client::EntityUpdateStreaming, types::Query as ToriiQuery};
use tracing::{debug, info};

mod models;

pub fn setup_torii_client() {
    let torii_url: String = "https://api.cartridge.gg/x/ponziland-sepolia/torii".into();
    let rpc_url = "https://api.cartridge.gg/x/starknet/sepolia".into();
    let relay_url = "".into();
    let world = Felt::from_hex_unchecked(
        "0x27a82b1641d3a6b4b0b049afeb6c9f0196fe6b440fcb636e87be0243b23736f",
    );

    let (tx, mut rx) = mpsc::channel::<Vec<Event>>(32);

    spawn(async move {
        info!("Starting Torii client...");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();

        info!("Reading existing entities...");
        let query = ToriiQuery {
            clause: None,
            offset: 0,
            dont_include_hashed_keys: false,
            order_by: vec![],
            entity_models: vec![],
            entity_updated_after: 0,
            limit: 100,
        };

        let all_events = client.event_messages(query, true).await.unwrap();
        println!("Got {} events", all_events.len());

        for entity in all_events {
            debug!("Existing Event: {:?}", entity);

            let events = entity.models.into_iter().map(|e| Event::from(e)).collect();

            tx.send(events).await.unwrap();
        }

        info!("Subscribing to new events...");

        let mut subscription: EntityUpdateStreaming =
            client.on_event_message_updated(vec![], true).await.unwrap();

        while let Some(Ok((_, entity))) = subscription.next().await {
            debug!("Received new event: {:?}", entity);

            let events = entity.models.into_iter().map(|e| Event::from(e)).collect();

            tx.send(events).await.unwrap();
        }
    });

    // For testing, spawn a task to display those events:
    spawn(async move {
        while let Some(events) = rx.recv().await {
            for event in events {
                println!("{}", event);
            }
        }
    });

    info!("Background Torii client setup complete.");
}
