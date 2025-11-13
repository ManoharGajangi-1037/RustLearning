use std::{sync::Arc, time::Duration};

use axum::{Router, extract::State, routing::get};
use reqwest::Client;
use tokio::{net::TcpListener, sync::Semaphore};
#[derive(Clone)]
struct AppState {
    client: Client,
    limit: Arc<Semaphore>,
    urls: Vec<&'static str>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        client: Client::new(),
        // .pool_idle_timeout(Some(Duration::from_secs(60)))
        // .build()
        // .unwrap(),
        limit: Arc::new(Semaphore::new(3)),
        urls: vec![
            "https://jsonplaceholder.typicode.com/users/1",
            "https://jsonplaceholder.typicode.com/users/2",
            "https://jsonplaceholder.typicode.com/users/3",
            "https://jsonplaceholder.typicode.com/users/4",
            "https://jsonplaceholder.typicode.com/users/5",
            "https://jsonplaceholder.typicode.com/users/6",
            "https://jsonplaceholder.typicode.com/users/7",
            "https://jsonplaceholder.typicode.com/users/8",
        ],
    };
    let router = Router::new().route("/", get(method)).with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn method(State(state): State<AppState>) -> &'static str {
    let mut handles = vec![];
    let urls = state.urls;
    for url in urls {
        let client = state.client.clone();
        let limit = state.limit.clone();
        let handle = tokio::spawn(async move {
            println!("Waiting --->{}", url);
            let permit = limit.acquire_owned().await.unwrap();

            println!("Starting --->{}", url);
            let result = client.get(url).send().await.unwrap();
            println!("url {}->{}", url, result.status());
            println!("Done --->{}", url);
            // drop(permit);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
    "hello"
}
