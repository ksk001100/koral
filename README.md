# Koral

> A macro-centric CLI framework for Rust emphasizing separation of definition and state.

Koral provides a declarative way to build Command Line Interfaces. By leveraging Rust's type system and procedural macros, it allows you to define your application's structure (flags, subcommands, metadata) separately from its runtime state.

## Philosophy: Definition vs. State

Unlike other CLI libraries where the parsed result *is* your struct, Koral separates them:

1.  **Definition**: You define **Flags** and **Apps** as structs/enums with attributes. These represent *what* your CLI accepts.
2.  **State**: At runtime, Koral parses arguments into a `Context`. Your application logic receives this context and retrieves values using your Flag types as keys.

This approach keeps your application logic clean and decoupling it from the parsing mechanics.

## Features

- **Declarative Macros**: Use `#[derive(App)]`, `#[derive(Subcommand)]`, and `#[derive(Flag)]` to define your CLI with minimal boilerplate.
- **Type-Safe Flags**: Flags are types. Retrieve them safely from the context (e.g., `ctx.get::<VerboseFlag>()`).
- **Reuse**: Define a `VerboseFlag` once and reuse it across multiple subcommands or applications.
- **Flexible Handlers**: Action handlers can be `fn(Context)` or `fn(&mut App, Context)`.

## Installation

```bash
cargo add --git https://github.com/ksk001100/koral
```

## Quick Start

### Simple Application

Define your app and flags using macros. You can access the application instance state via `ctx.app`!

```rust
use koral::traits::App;
use koral::{Context, Flag, KoralResult};

#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
struct VerboseFlag;

#[derive(Flag, Debug)]
#[flag(name = "name", default = "World", help = "Name to greet")]
struct NameFlag(String);

#[derive(koral::App)]
#[app(name = "greet", version = "1.0", action = run)]
#[app(flags(VerboseFlag, NameFlag))]
struct GreetApp {
    greet_count: u32,
}

// Handler receives Context<GreetApp> to access the app instance
fn run(mut ctx: Context<GreetApp>) -> KoralResult<()> {
    if let Some(app) = &mut ctx.app {
        app.greet_count += 1;
    }

    let verbose = ctx.get::<VerboseFlag>().unwrap_or(false);
    let name = ctx.get::<NameFlag>().expect("Default value guaranteed");

    if verbose {
        println!("Debug mode: ON");
    }
    println!("Hello, {}!", name);
    Ok(())
}

fn main() -> KoralResult<()> {
    let mut app = GreetApp { greet_count: 0 };
    app.run(std::env::args().skip(1).collect())
}
```

Run it:
```bash
cargo run --example simple -- --name Koral --verbose
```

## Advanced Usage: Todo App

Check `examples/full.rs` for a complete Todo application demonstrating:

*   **Subcommands**: `add`, `list`, `done`
*   **Positional Arguments**: `todo add "Buy milk"`
*   **Global & Local Flags**: `--all`, `--verbose`
*   **Shared State**: `Arc<Mutex<TodoState>>`

```rust
// Action handler for a subcommand
fn add_task(ctx: Context) -> KoralResult<()> {
    if ctx.args.is_empty() {
        println!("Error: Task description required.");
        return Ok(());
    }
    let task = ctx.args.join(" ");
    
    // Access shared state
    let state = ctx
        .state::<Arc<Mutex<TodoState>>>()
        .expect("State mismatch");
    let mut guard = state.lock().unwrap();
    guard.tasks.push(task.clone());

    println!("Added task: '{}'", task);
    Ok(())
}
```

```bash
# Add a task
cargo run --example full -- add "Buy milk"

# List tasks
cargo run --example full -- list --all
```

## Examples

| Example | Description | Command |
|---------|-------------|---------|
| `simple` | Basic usage with `derive(App)` | `cargo run --example simple` |
| `full` | Todo App with subcommands and complex logic | `cargo run --example full -- --help` |
