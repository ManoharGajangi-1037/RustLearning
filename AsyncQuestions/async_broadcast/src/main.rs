use tokio::sync::broadcast;

async fn broad_cast() {
    let (tx, mut rx) = broadcast::channel::<i32>(10);

    let mut subscriber_1 = tx.subscribe();
    let mut subscriber_2 = tx.subscribe();
    let mut subscriber_3 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(subscriber_1.recv().await, Ok(10));
        assert_eq!(subscriber_2.recv().await, Ok(10));
        assert_eq!(subscriber_3.recv().await, Ok(10));
    });
    tx.send(1);
}

#[tokio::main]
async fn main() {
    broad_cast().await;
    println!("Hello, world!");
}
