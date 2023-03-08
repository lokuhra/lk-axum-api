use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: i64,
    pub task: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct TaskError {
    pub error: String,
}
