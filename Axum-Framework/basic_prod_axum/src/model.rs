use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
}

#[derive(Serialize)]
pub struct TaskList {
    pub list: Vec<Task>,
}
