mod task;
mod repo;
mod storage;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use repo::Repository;
use task::Status;

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
    let _repo = {
        let r = RepoImpl::new(&cli.db);
        r.init()?;
        r
    };

    Ok(())
}
