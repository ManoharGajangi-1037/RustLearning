use std::net::SocketAddr;

use crate::{db::get_db_pool, routes::create_routes};
use axum::{Router, routing::get};
mod db;
mod handler;
mod model;
mod routes;
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let pool = get_db_pool().await;
    let app = create_routes().with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
