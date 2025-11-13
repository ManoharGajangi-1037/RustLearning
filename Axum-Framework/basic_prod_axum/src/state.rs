use std::sync::Arc;

use tokio::sync::Mutex;

use crate::model::Task;

#[derive(Clone)]
pub struct AppState {
    pub counter: Arc<Mutex<i32>>,
    pub task_list: Arc<Mutex<Vec<Task>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            task_list: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
