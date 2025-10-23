use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub status: Status,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            status: Status::Todo,
            title,
            created_at: now,
            updated_at: now,
        }
    }
}