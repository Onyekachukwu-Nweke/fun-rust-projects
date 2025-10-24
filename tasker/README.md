# Tasker

A lightweight task manager CLI built in Rust. Perfect for keeping track of your todos right from the terminal.

## What is this?

Tasker is a command-line tool that helps you manage tasks without leaving your terminal. It's built with Rust for speed and reliability, and uses SQLite to store your tasks locally. Think of it as a minimal, no-nonsense todo list that lives in your command line.

## Features

- **Add tasks** - Create new tasks with a title
- **List tasks** - View all your tasks, or filter by status/search term
- **Update tasks** - Change task status or edit the title
- **Delete tasks** - Remove completed or unwanted tasks
- **Persistent storage** - Uses SQLite to save your tasks locally

## Installation

Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone <your-repo-url>
cd tasker

# Build the project
cargo build --release

# The binary will be in target/release/tasker
# Optionally, add it to your PATH
```

## Quick Start

```bash
# Add a task
tasker add "Write documentation"

# List all tasks
tasker list

# List only tasks that are in progress
tasker list --status in-progress

# Search for tasks containing "doc"
tasker list --search doc

# Mark a task as done (you'll need the task ID from list)
tasker set-status <task-id> done

# View a specific task
tasker get <task-id>

# Edit a task's title
tasker edit <task-id> --title "Updated title"

# Delete a task
tasker rm <task-id>
```

## How it Works

### Architecture Overview

The project follows a clean, layered architecture:

```
┌─────────────────┐
│   CLI Layer     │  (main.rs - handles user commands)
├─────────────────┤
│  Domain Layer   │  (task.rs - defines what a task is)
├─────────────────┤
│  Repository     │  (repo.rs - defines how to store/retrieve)
├─────────────────┤
│  Storage Layer  │  (sqlite.rs - actual database operations)
└─────────────────┘
```

### Core Components

**1. Task (task.rs)**

This is your basic task entity. Every task has:
- A unique ID (UUID)
- A title
- A status (Todo, InProgress, or Done)
- Creation and update timestamps

**2. Repository Trait (repo.rs)**

This defines the contract for how tasks should be stored and retrieved. It's a trait, which means we can swap out the implementation (maybe you want to use PostgreSQL later, or save to a file).

The repository provides these operations:
- `init()` - Set up the storage
- `create()` - Save a new task
- `get()` - Retrieve a specific task by ID
- `list()` - Get multiple tasks (with optional filters)
- `update()` - Modify an existing task
- `delete()` - Remove a task
- `set_status()` - Quick status update

**3. SQLite Implementation (storage/sqlite.rs)**

This is the actual implementation that stores tasks in a SQLite database. It implements the Repository trait, so it has to provide all those operations.

**4. CLI (main.rs)**

This handles all the command-line interactions. It parses your commands, calls the appropriate repository methods, and displays the results.

## Pseudocode: How Commands Work

Let's break down what happens when you run common commands:

### Adding a Task

```
User runs: tasker add "Buy groceries"

1. Parse command line arguments
2. Create new Task object
   - Generate unique ID
   - Set title to "Buy groceries"
   - Set status to Todo
   - Record current time as created_at and updated_at
3. Call repo.create(task)
   - Open database connection
   - Insert task into 'tasks' table
   - Return the saved task
4. Print the task ID to terminal
```

### Listing Tasks

```
User runs: tasker list --status todo --search "groceries"

1. Parse command line arguments
   - status filter = Todo
   - search term = "groceries"
2. Build Query object with filters
3. Call repo.list(query)
   - Open database connection
   - Build SQL: SELECT * FROM tasks WHERE status = 'todo' AND LOWER(title) LIKE '%groceries%'
   - Bind parameters dynamically
   - Execute query
   - Parse each row into Task object
   - Return list of tasks
4. For each task in results:
   - Print: "ID | [status] title"
```

### Getting a Specific Task

```
User runs: tasker get abc123...

1. Parse command line arguments
2. Convert ID string to UUID
3. Call repo.get(id)
   - Open database connection
   - Run: SELECT * FROM tasks WHERE id = ?
   - If found, parse row into Task object
   - Return Some(task) or None
4. If task exists:
   - Print all task details (id, title, status, timestamps)
   Else:
   - Print "Not found"
```

### Updating Task Status

```
User runs: tasker set-status abc123... done

1. Parse command line arguments
2. Convert ID string to UUID
3. Convert status string to Status enum
4. Call repo.set_status(id, status)
   - Open database connection
   - Run: UPDATE tasks SET status = ?, updated_at = ? WHERE id = ?
   - Return true if a row was updated, false otherwise
5. Print "ok" or "not found"
```

### Editing a Task

```
User runs: tasker edit abc123... --title "New title"

1. Parse command line arguments
2. Convert ID string to UUID
3. Call repo.get(id) to fetch current task
   - If not found, return error
4. Update the task fields
   - If --title provided, update task.title
   - Update task.updated_at to current time
5. Call repo.update(task)
   - Open database connection
   - Run: UPDATE tasks SET title = ?, status = ?, updated_at = ? WHERE id = ?
   - Return updated task
6. Print "ok"
```

### Deleting a Task

```
User runs: tasker rm abc123...

1. Parse command line arguments
2. Convert ID string to UUID
3. Call repo.delete(id)
   - Open database connection
   - Run: DELETE FROM tasks WHERE id = ?
   - Return true if a row was deleted
4. Print "ok" or "not found"
```

## Database Schema

The SQLite database has a simple schema:

```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,           -- UUID as string
    title TEXT NOT NULL,           -- Task description
    status TEXT NOT NULL,          -- "todo", "in_progress", or "done"
    created_at TEXT NOT NULL,      -- ISO 8601 timestamp
    updated_at TEXT NOT NULL       -- ISO 8601 timestamp
);

CREATE INDEX idx_tasks_status ON tasks(status);
```

## Project Structure

```
tasker/
├── Cargo.toml              # Rust dependencies and project config
├── README.md               # This file
└── src/
    ├── main.rs             # CLI interface and command handling
    ├── task.rs             # Task data structure and Status enum
    ├── repo.rs             # Repository trait and Query struct
    └── storage/
        ├── mod.rs          # Storage module declaration
        └── sqlite.rs       # SQLite implementation of Repository
```

## Technical Details

### Why Rust?

Rust gives us:
- **Speed** - Compiled binary runs fast
- **Safety** - Compiler catches bugs before runtime
- **Zero-cost abstractions** - Traits don't add overhead
- **Great ecosystem** - Excellent libraries (clap, rusqlite, uuid, chrono)

### Key Dependencies

- **clap** - Command-line argument parsing with derive macros
- **rusqlite** - SQLite database interface
- **uuid** - Unique identifier generation
- **chrono** - Date and time handling
- **serde** - Serialization (for future JSON export, etc.)
- **anyhow** - Error handling made easy

### Design Patterns

**Repository Pattern**: We separate the "what" (Repository trait) from the "how" (SqliteRepo implementation). This makes it easy to add new storage backends without changing the rest of the code.

**Feature Flags**: The SQLite implementation is behind a feature flag (`#[cfg(feature = "sqlite")]`). This means you could theoretically compile the app with different storage backends.

**Type Safety**: Using enums for Status ensures you can't have invalid states like "in-between" or "maybe-done". The compiler enforces correctness.

## Configuration

By default, tasker creates a database file called `tasker.db` in your current directory. You can specify a different location:

```bash
tasker --db /path/to/my-tasks.db add "Custom location"
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

### Code Quality

```bash
# Check for issues
cargo clippy

# Format code
cargo fmt
```

## Future Ideas

Some things that could be added:
- Due dates for tasks
- Priority levels
- Tags or categories
- Task descriptions (notes)
- Export to JSON/CSV
- Task dependencies
- Recurring tasks
- Multiple task lists/projects

## Troubleshooting

**"Command not found"**: Make sure the binary is in your PATH or use the full path to the executable.

**"Database is locked"**: Another process might be using the database. Close other tasker instances.

**UUID parse errors**: Make sure you're copying the full UUID when using task IDs.

## Contributing

This is a learning project, but contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## License

[Add your license here]

## Author

Built with ❤️ while learning Rust
