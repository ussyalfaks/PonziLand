use tokio::sync::broadcast;

#[tokio::main]
pub async fn main() {
    // Create a bounded channel
    let (tx, mut rx1) = broadcast::channel();

    // Clone receivers for multiple consumers
    let mut rx2 = tx.subscribe();

    // Each consumer gets its own receiver
    let handle1 = tokio::spawn(async move {
        while let event = rx1.recv().await.unwrap() {
            println!("Got event! {}", event)
            // Process event
        }

        println!("Finished!")
    });

    let handle2 = tokio::spawn(async move {
        while let event = rx2.recv().await.unwrap() {
            println!("Got event! {}", event)
            // Process event
        }

        println!("Finished!")
    });

    // Send 100 events
    for i in 0..100 {
        tx.send(i).unwrap();
    }

    // Wait for all tasks to complete
    let (_, _) = tokio::join!(handle1, handle2);
}
