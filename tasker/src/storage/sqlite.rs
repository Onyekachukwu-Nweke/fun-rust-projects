#[cfg(feature = "sqlite")]
use anyhow::{bail, Result};
#[cfg(feature = "sqlite")]
use rusqlite::{params, Connection, OptionalExtension};
#[cfg(feature = "sqlite")]
use uuid::Uuid;

#[cfg(feature = "sqlite")]
use crate::task::{Status, Task};
#[cfg(feature = "sqlite")]
use crate::repo::{Query, Repository};

#[cfg(feature = "sqlite")]
#[derive(Clone)]
pub struct SqliteRepo {
    path: String,
}

#[cfg(feature = "sqlite")]
impl SqliteRepo {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    fn conn(&self) -> Result<Connection> {
        Ok(Connection::open(&self.path)?)
    }
}

#[cfg(feature = "sqlite")]
impl Repository for SqliteRepo {
    fn init(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS tasks(
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
            "#
        )?;
        Ok(())
    }

    fn create(&self, task: Task) -> Result<Task> {
        let conn = self.conn()?;
        conn.execute(
            "INSERT INTO tasks(id,title,status,created_at,updated_at)
             VALUES(?,?,?,?,?)",
            params![
                task.id.to_string(),
                task.title,
                match task.status { Status::Todo=>"todo", Status::InProgress=>"in_progress", Status::Done=>"done" },
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339()
            ],
        )?;
        Ok(task)
    }

    fn get(&self, id: Uuid) -> Result<Option<Task>> {
        let conn = self.conn()?;
        conn.query_row(
            "SELECT id,title,status,created_at,updated_at FROM tasks WHERE id=?1",
            [id.to_string()],
            |r| {
                let status_str: String = r.get(2)?;
                let status = match status_str.as_str() {
                    "todo" => Status::Todo,
                    "in_progress" => Status::InProgress,
                    _ => Status::Done,
                };
                Ok(Task{
                    id: Uuid::parse_str(r.get::<_, String>(0)?.as_str()).unwrap(),
                    title: r.get(1)?,
                    status,
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<_, String>(3)?).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<_, String>(4)?).unwrap().with_timezone(&chrono::Utc),
                })
            }
        ).optional().map_err(Into::into)
    }

    fn list(&self, q: Query) -> Result<Vec<Task>> {
        let conn = self.conn()?;

        let mut sql = "SELECT id,title,status,created_at,updated_at FROM tasks".to_string();
        let mut conditions = vec![];

        if q.status.is_some() {
            conditions.push("status = ?");
        }
        if q.search.is_some() {
            conditions.push("LOWER(title) LIKE ?");
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }
        sql.push_str(" ORDER BY created_at DESC");

        let mut stmt = conn.prepare(&sql)?;
        let mut param_index = 1;

        if let Some(status) = q.status {
            let status_str = match status { Status::Todo=>"todo", Status::InProgress=>"in_progress", Status::Done=>"done" };
            stmt.raw_bind_parameter(param_index, status_str)?;
            param_index += 1;
        }
        if let Some(search) = q.search {
            let search_pattern = format!("%{}%", search.to_lowercase());
            stmt.raw_bind_parameter(param_index, search_pattern)?;
        }

        let mut rows = stmt.raw_query();
        let mut out = vec![];

        while let Some(row) = rows.next()? {
            let status_str: String = row.get(2)?;
            let status = match status_str.as_str() {
                "todo" => Status::Todo,
                "in_progress" => Status::InProgress,
                _ => Status::Done,
            };
            out.push(Task{
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                title: row.get(1)?,
                status,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?).unwrap().with_timezone(&chrono::Utc),
            });
        }

        Ok(out)
    }

    fn update(&self, task: Task) -> Result<Task> {
        let conn = self.conn()?;
        let affected = conn.execute(
            "UPDATE tasks SET title=?, status=?, updated_at=? WHERE id=?",
            params![
                task.title,
                match task.status { Status::Todo=>"todo", Status::InProgress=>"in_progress", Status::Done=>"done" },
                task.updated_at.to_rfc3339(),
                task.id.to_string()
            ],
        )?;
        if affected == 0 { bail!("Task not found"); }
        Ok(task)
    }

    fn delete(&self, id: Uuid) -> Result<bool> {
        let conn = self.conn()?;
        Ok(conn.execute("DELETE FROM tasks WHERE id=?", [id.to_string()])? > 0)
    }

    fn set_status(&self, id: Uuid, status: Status) -> Result<bool> {
        let conn = self.conn()?;
        Ok(conn.execute(
            "UPDATE tasks SET status=?, updated_at=? WHERE id=?",
            params![
                match status { Status::Todo=>"todo", Status::InProgress=>"in_progress", Status::Done=>"done" },
                chrono::Utc::now().to_rfc3339(),
                id.to_string()
            ],
        )? > 0)
    }
}
