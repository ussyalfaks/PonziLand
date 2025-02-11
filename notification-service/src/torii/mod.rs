use models::Event;
use starknet_crypto::Felt;
use tokio::sync::mpsc;
use tokio::{spawn, sync::mpsc::Sender};
use tokio_stream::StreamExt;
use torii_client::client::Client;
use torii_grpc::types::MemberClause;
use torii_grpc::{
    client::EntityUpdateStreaming,
    types::{Clause, CompositeClause, LogicalOperator, Query as ToriiQuery},
};
use tracing::{debug, info, warn};

mod models;

pub fn setup_torii_client() {
    let torii_url: String = "http://localhost:8080/".into();
    let rpc_url = "https://api.cartridge.gg/x/starknet/sepolia".into();
    let relay_url = "".into();
    let world = Felt::from_hex_unchecked(
        "0x027a82b1641d3a6b4b0b049afeb6c9f0196fe6b440fcb636e87be0243b23736f",
    );

    let (tx, mut rx) = mpsc::channel::<Vec<Event>>(32);

    spawn(async move {
        info!("Starting Torii client...");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();

        info!("Reading existing entities...");

        catchup_paging(tx.clone(), &client).await;

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
        let mut count = 0;

        while let Some(events) = rx.recv().await {
            for _ in events {
                count += 1;
            }
        }
    });

    info!("Background Torii client setup complete.");
}

async fn catchup_paging(tx: Sender<Vec<Event>>, client: &Client) {
    let mut current_offset = 0;

    loop {
        let query = ToriiQuery {
            clause: None,
            offset: current_offset,
            dont_include_hashed_keys: true,
            order_by: vec![],
            entity_models: vec![],
            entity_updated_after: 1,
            limit: 100,
        };

        let all_events = client.event_messages(query, true).await.unwrap();

        warn!(
            "Got {} new events with offset {current_offset}",
            all_events.len()
        );

        if all_events.is_empty() {
            return;
        }

        current_offset += 100;

        for entity in all_events {
            debug!("Existing Event: {:?}", entity);

            let events = entity.models.into_iter().map(|e| Event::from(e)).collect();

            tx.send(events).await.unwrap();
        }
    }
}
