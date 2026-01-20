# Todo App Example

A full-featured Todo application demonstrating:
- **Subcommands**: `add`, `list`, `complete` commands.
- **State Management**: Managing an in-memory list of tasks.

## Usage

```bash
# Add a task
cargo run -p todo-app -- add "Buy milk"

# List tasks
cargo run -p todo-app -- list

# Complete a task
cargo run -p todo-app -- complete 1
```
