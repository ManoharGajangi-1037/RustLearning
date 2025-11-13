use std::{sync::Arc, time::Duration};

use axum::{Router, extract::State, routing::get};
use tokio::{
    net::TcpListener,
    sync::mpsc::{self, Receiver, Sender},
    time::sleep,
};

async fn random_job(mut reciever: Receiver<i32>) {
    loop {
        println!("This is the random job handler");
        // sleep(Duration::from_secs(10)).await

        while let Some(value) = reciever.recv().await {
            println!("the value is {}", value);
            return;
        }
    }
}
#[derive(Clone)]
struct AppState {
    tx: Sender<i32>,
}
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    let state = AppState { tx: tx.clone() };
    let app = Router::new().route("/", get(helloworld).with_state(state));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let task = tokio::spawn(random_job(rx));
    axum::serve(listener, app).await.unwrap();

    task.await;
    println!("Hello, world!");
}

async fn helloworld(State(state): State<AppState>) -> String {
    state.tx.send(5).await;
    "Hello World".to_string()
}
