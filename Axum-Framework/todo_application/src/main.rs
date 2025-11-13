use axum::Json;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::header::AUTHORIZATION;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;

use axum::{Router, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    name: String,
}

#[derive(serde::Serialize)]
struct TaskList {
    list: Vec<Task>,
}
#[derive(Clone, Debug)]
struct AppState {
    list: Arc<Mutex<Vec<Task>>>,
}
#[tokio::main]
async fn main() {
    let state = AppState {
        list: Arc::new(Mutex::new(Vec::new())),
    };
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("info,tower_http=trace")
        .init();

    let middle_ware_stack = ServiceBuilder::new();
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/todo", get(list_all).post(add_task))
        .route(
            "/todo/{id}",
            get(get_task)
                .layer(axum::middleware::from_fn(authorize))
                .layer(axum::middleware::from_fn(rate_limit)),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn rate_limit(req: Request<Body>, next: Next) -> Result<impl IntoResponse, StatusCode> {
    println!("rate limit ");
    let count = 5;
    if count <= 5 {
        return Ok((next.run(req).await));
    }
    Err(StatusCode::UNAUTHORIZED)
}
async fn authorize(req: Request<Body>, next: Next) -> Result<impl IntoResponse, StatusCode> {
    println!("authorize");
    let header = req.headers().get(AUTHORIZATION).unwrap();
    let authenticate = header == "Bearer Secret-token";
    if authenticate {
        return Ok((next.run(req).await));
    }
    Err(StatusCode::UNAUTHORIZED)
}
async fn hello_world() -> &'static str {
    "hello world"
}

async fn health() -> impl IntoResponse {}
async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<i32>,
) -> Result<Json<Task>, StatusCode> {
    let list = state.list.lock().await;
    for task in list.iter() {
        if task.id == task_id {
            return Ok(Json(Task {
                id: task.id,
                name: task.name.clone(),
            }));
        }
    }
    Err(StatusCode::NOT_FOUND)
}
#[axum::debug_handler]
#[instrument]
async fn add_task(State(state): State<AppState>, Json(task): Json<Task>) -> impl IntoResponse {
    info!("Task Creation");
    let mut list = state.list.lock().await;
    list.push(task);
    StatusCode::CREATED
}

async fn list_all(State(state): State<AppState>) -> Json<TaskList> {
    info!("list_all");
    let list = state.list.lock().await;
    let value = list.clone();

    let list = TaskList { list: value };
    Json(list)
}
