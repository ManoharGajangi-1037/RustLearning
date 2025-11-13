pub mod tests;
use std::{sync::Arc};
use axum::{body::Body, extract::State, http::{Request, StatusCode, header}, middleware::{self, Next}, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower::{ServiceBuilder};


#[derive(Clone)]
pub struct AppState {
    pub count: Arc<Mutex<i32>>,
}

// Function to create and return the app router, ready for testing.
pub fn create_app(state: AppState) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(middleware::from_fn(auth_middleware))
        .layer(middleware::from_fn_with_state(state, rate_limit_middleware));

    Router::new()
        .route("/", get(ping).route_layer(middleware_stack))
}

async fn rate_limit_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let mut count = state.count.lock().await;

    *count += 1;
    if *count > 5 {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(req).await)
}

async fn auth_middleware(req: Request<Body>, next: Next) -> Result<impl IntoResponse, StatusCode> {
    let auth = req.headers().get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let allowed = auth == "Bearer Secret-token";

    if !allowed {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    Ok(next.run(req).await)
}

#[derive(Serialize, Deserialize,PartialEq,Debug)]
struct Thing {
    name: String,
}

async fn ping() -> Result<Json<Thing>, StatusCode> {
    // This example logic will always return an error for demonstration.
    let name_is_valid = true;
    if !name_is_valid {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(Json(Thing { name: "Manohar".to_string() }))
}
