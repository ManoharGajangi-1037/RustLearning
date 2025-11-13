use std::sync::Arc;
use axum_middleware::{AppState, create_app};
use tokio::{net::TcpListener, sync::Mutex};
#[tokio::main]
async fn main(){
    let state = AppState{
        count:Arc::new(Mutex::new(0))
    };
    let app = create_app(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
