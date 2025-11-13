use crate::{
    middlewares::auth::{auth_middle_ware, auth_rate_limitng},
    routes::tasks::task_router,
    state::AppState,
};
use axum::{
    Router,
    middleware::{self, from_fn},
};
use tokio::net::TcpListener;
use tower::{Layer, ServiceBuilder};
mod error;
mod middlewares;
mod model;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let middleware_stack = ServiceBuilder::new()
        .layer(middleware::from_fn(auth_middle_ware))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_rate_limitng,
        ));
    let app = create_router()
        .route_layer(middleware_stack)
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_router() -> Router<AppState> {
    let router = Router::new().nest("/task", task_router());
    router
}
