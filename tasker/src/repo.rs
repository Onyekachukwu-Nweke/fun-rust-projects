use crate::task::{Status, Task};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Query {
    pub status: Option<Status>,
    pub search: Option<String>,
}

pub trait Repository: Send + Sync {
    fn init(&self) -> Result<()>;
    fn create(&self, task: Task) -> Result<Task>;
    fn get(&self, id: Uuid) -> Result<Option<Task>>;
    fn list(&self, q: Query) -> Result<Vec<Task>>;
    fn update(&self, task: Task) -> Result<Task>;
    fn delete(&self, id: Uuid) -> Result<bool>;
    fn set_status(&self, id: Uuid, status: Status) -> Result<bool>;
}
