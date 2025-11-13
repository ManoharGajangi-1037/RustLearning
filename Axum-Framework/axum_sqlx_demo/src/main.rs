use axum::{
    Json, Router, extract::{Path, State}, http::StatusCode, response::Html, routing::{get, post}
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// Data models
#[derive(Serialize)]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
    email: String,
}

#[tokio::main]
async fn main(){

    dotenv().ok();

    // Initialize tracing subscriber (logging)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Connect to DB
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://@localhost:5432/postgres".to_string());

    let pool = PgPool::connect(&database_url).await.unwrap();

    println!("{:?}",pool);
    info!("✅ Connected to Postgres!");

    let state = AppState { db: pool.clone() };

    // Build router
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/users", get(get_users).post(create_user)).without_v07_checks()
        .route("/users/:id", get(get_user_by_id))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server running on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn hello_handler()-> Html<String>{
    Html(format!("hello"))
}
//GET /users
async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, (axum::http::StatusCode, String)> {
    let all= sqlx::query!("SELECT * from user").fetch_all(&state.db).await.map_err(|_| (StatusCode::NOT_FOUND,"Error Occured while quering"));
    println!("everythinggg {:?}",all);
    println!("get usersss");
    let rows = sqlx::query!("SELECT id, username, email FROM users ORDER BY id")
        .fetch_all(&state.db)
        .await
        .map_err(internal_error)?;

    let users = rows
        .into_iter()
        .map(|r| User { id: r.id, username: r.username, email: r.email })
        .collect();

    Ok(Json(users))
}

// GET /users/:id
async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let row = sqlx::query!("SELECT id, username, email FROM users WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (axum::http::StatusCode::NOT_FOUND, "User not found".into()))?;

    Ok(Json(User {
        id: row.id,
        username: row.username,
        email: row.email,
    }))
}

// POST /users
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let row = sqlx::query!(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        payload.username,
        payload.email
    )
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

    Ok(Json(User {
        id: row.id,
        username: row.username,
        email: row.email,
    }))
}

fn internal_error<E: std::fmt::Display>(err: E) -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Server Error: {}", err),
    )
}
