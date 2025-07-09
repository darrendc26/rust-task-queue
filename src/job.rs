use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    SendEmail,
    ResizeImage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: Uuid,
    pub task_type: TaskType,
    pub status: JobStatus,
    pub payload: serde_json::Value,
    pub attempts: u32,
}