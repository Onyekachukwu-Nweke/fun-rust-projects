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
            "INSERT INTO tasks(id,title,notes,status,priority,created_at,updated_at)
             VALUES(?,?,?,?,?,?,?)",
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
            "SELECT id,title,notes,status,priority,created_at,updated_at FROM tasks WHERE id=?1",
            [id.to_string()],
            |r| {
                let status_str: String = r.get(3)?;
                let status = match status_str.as_str() {
                    "todo" => Status::Todo,
                    "in_progress" => Status::InProgress,
                    _ => Status::Done,
                };
                Ok(Task{
                    id: Uuid::parse_str(r.get::<_, String>(0)?.as_str()).unwrap(),
                    title: r.get(1)?,
                    status,
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<_, String>(5)?).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<_, String>(6)?).unwrap().with_timezone(&chrono::Utc),
                })
            }
        ).optional().map_err(Into::into)
    }

    fn list(&self, q: Query) -> Result<Vec<Task>> {
        let conn = self.conn()?;
        let mut sql = "SELECT id,title,notes,status,priority,created_at,updated_at FROM tasks".to_string();
        let mut clauses: Vec<&str> = vec![];
        let mut params: Vec<(String, String)> = vec![];

        if let Some(status) = q.status {
            clauses.push("status = :status");
            params.push(("status".into(), match status { Status::Todo=>"todo".into(), Status::InProgress=>"in_progress".into(), Status::Done=>"done".into() }));
        }
        if let Some(s) = q.search {
            clauses.push("(LOWER(title) LIKE :s OR LOWER(notes) LIKE :s)");
            params.push(("s".into(), format!("%{}%", s.to_lowercase())));
        }
        if !clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clauses.join(" AND "));
        }
        sql.push_str(" ORDER BY COALESCE(priority,0) DESC, created_at DESC");

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.iter().map(|(k,v)| (k.as_str(), v as &dyn rusqlite::ToSql)), |r| {
            let status_str: String = r.get(3)?;
            let status = match status_str.as_str() {
                "todo" => Status::Todo,
                "in_progress" => Status::InProgress,
                _ => Status::Done,
            };
            Ok(Task{
                id: Uuid::parse_str(r.get::<_, String>(0)?.as_str()).unwrap(),
                title: r.get(1)?,
                status,
                created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<_, String>(5)?).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<_, String>(6)?).unwrap().with_timezone(&chrono::Utc),
            })
        })?;

        let mut out = vec![];
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    fn update(&self, task: Task) -> Result<Task> {
        let conn = self.conn()?;
        let affected = conn.execute(
            "UPDATE tasks SET title=?, notes=?, status=?, priority=?, updated_at=? WHERE id=?",
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
