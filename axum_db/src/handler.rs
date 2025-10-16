use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use sqlx::PgPool;
use crate::model::Todo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

/// POST /todos
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let id = Uuid::new_v4();
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (id, title, completed) VALUES ($1, $2, $3) RETURNING id, title, completed",
        id,
        payload.title,
        false
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

/// GET /todos
pub async fn list_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

/// PUT /todos/:id
pub async fn mark_complete(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        "UPDATE todos SET completed = true WHERE id = $1",
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
