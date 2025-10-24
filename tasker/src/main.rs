mod task;
mod repo;
mod storage;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use repo::{Repository, Query};
use task::{Status, Task};

#[cfg(feature = "sqlite")]
use storage::sqlite::SqliteRepo as RepoImpl;

#[derive(Debug, Parser)]
#[command(name="tasker", about="A tiny task manager CLI in Rust")]
struct Cli {
    /// SQLite file path (only used when compiled with `sqlite`)
    #[arg(global = true, long, default_value = "tasker.db")]
    db: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, ValueEnum)]
enum StatusArg { Todo, InProgress, Done }

impl From<StatusArg> for Status {
    fn from(s: StatusArg) -> Self {
        match s { StatusArg::Todo => Status::Todo, StatusArg::InProgress => Status::InProgress, StatusArg::Done => Status::Done }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a task
    Add {
        title: String,
    },
    /// List tasks
    List {
        #[arg(long)] status: Option<StatusArg>,
        #[arg(long)] search: Option<String>,
    },
    /// Show one task
    Get { id: String },
    /// Mark a task as done/in_progress/todo
    SetStatus { id: String, status: StatusArg },
    /// Edit task fields
    Edit {
        id: String,
        #[arg(long)] title: Option<String>,
    },
    /// Delete a task
    Rm { id: String },
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    #[cfg(feature = "sqlite")]
    let repo = {
        let r = RepoImpl::new(&cli.db);
        r.init()?;
        r
    };

    match cli.command {
        Commands::Add { title} => {
            let task = Task::new(title);
            let saved = repo.create(task)?;
            println!("{}", saved.id);
        }
        Commands::List { status, search } => {
            let tasks = repo.list(Query { status: status.map(Into::into), search })?;
            for t in tasks {
                println!("{} | [{}] {}",
                         t.id,
                         match t.status { Status::Todo=>"todo", Status::InProgress=>"in_progress", Status::Done=>"done" },
                         t.title
                );
            }
        }
        Commands::Get { id } => {
            let id = uuid::Uuid::parse_str(&id)?;
            match repo.get(id)? {
                Some(t) => {
                    println!("id: {}", t.id);
                    println!("title: {}", t.title);
                    println!("status: {:?}", t.status);
                    println!("created_at: {}", t.created_at);
                    println!("updated_at: {}", t.updated_at);
                }
                None => println!("Not found"),
            }
        }
        Commands::SetStatus { id, status } => {
            let id = uuid::Uuid::parse_str(&id)?;
            let ok = repo.set_status(id, status.into())?;
            println!("{}", if ok { "ok" } else { "not found" });
        }
        Commands::Edit { id, title } => {
            let id = uuid::Uuid::parse_str(&id)?;
            let mut t = repo.get(id)?.ok_or_else(|| anyhow::anyhow!("Task not found"))?;
            if let Some(v) = title { t.title = v; }
            t.updated_at = chrono::Utc::now();
            repo.update(t)?;
            println!("ok");
        }
        Commands::Rm { id } => {
            let id = uuid::Uuid::parse_str(&id)?;
            let ok = repo.delete(id)?;
            println!("{}", if ok { "ok" } else { "not found" });
        }
    }

    Ok(())
}
