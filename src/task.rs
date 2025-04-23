use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl TaskStatus {
    pub fn from_str(status: &str) -> Option<Self> {
        match status.to_lowercase().as_str() {
            "pending" => Some(TaskStatus::Pending),
            "inprogress" => Some(TaskStatus::InProgress),
            "completed" => Some(TaskStatus::Completed),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: String,
}
