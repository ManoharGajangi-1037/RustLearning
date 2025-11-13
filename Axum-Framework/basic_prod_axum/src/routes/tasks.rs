use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    error::AppError,
    model::{Task, TaskList},
    state::AppState,
};

pub fn task_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(get_tasks).post(add_task))
        .route("/:id", get(get_task));
    router
}
async fn get_tasks(State(state): State<AppState>) -> Result<Json<TaskList>, AppError> {
    let task_list = state.task_list.lock().await;
    let tasks: &Vec<Task> = &task_list;
    if tasks.is_empty() {
        return Err(AppError::NotFound);
    }
    let result = TaskList {
        list: tasks.to_vec(),
    };
    Ok(Json(result))
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Result<Json<Task>, AppError> {
    let task_list = state.task_list.lock().await;
    let task = task_list.get(id).ok_or(AppError::NotFound)?;
    let result = Task {
        name: task.name.clone(),
    };
    Ok(Json(result))
}
async fn add_task(
    State(state): State<AppState>,
    Json(new_task): Json<Task>,
) -> Result<impl IntoResponse, AppError> {
    let mut tasks = state.task_list.lock().await;
    tasks.push(new_task);
    Ok(StatusCode::CREATED)
}
