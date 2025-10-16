use axum::{Router, routing::{post, get, put}};
use crate::handler::{create_todo, list_todos, mark_complete};

pub fn create_routes() -> Router {
    Router::new()
        .route("/todos", post(create_todo).get(list_todos))
        .route("/todos/:id", put(mark_complete))
}
