# Koral

> [!WARNING]
> This is an experimental project.

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
- **Custom Types**: Easily parse Enums or Structs from flags using `#[derive(FlagValue)]`.
- **Flexible Handlers**: Action handlers can be `fn(Context)` or `fn(&mut App, Context)`.

## Installation

```bash
cargo add --git https://github.com/ksk001100/koral
```

## Quick Start

### Simple Application

Define your app and flags using macros. You can access the application instance state via `ctx.app`!

```rust
use koral::prelude::*;

#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
struct VerboseFlag;

#[derive(Flag, Debug)]
#[flag(name = "name", default = "World", help = "Name to greet")]
struct NameFlag(String);

#[derive(App)]
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

### Custom Flag Types

You can use standard Rust Enums or Structs as flag values by deriving `FlagValue`.

#### Enums (Choice)
```rust
#[derive(FlagValue, Clone, Debug, PartialEq)]
enum Format {
    Json,
    Text,
}

#[derive(Flag, Debug)]
#[flag(name = "format", default = "text")]
struct FormatFlag(Format);
```

#### Structs (NewType)
```rust
#[derive(FlagValue, Clone, Debug, PartialEq)]
struct RetryCount(u32);

#[derive(Flag, Debug)]
#[flag(name = "retry", default = "3")]
struct RetryFlag(RetryCount);
```

#### Complex Structs (Manual Parsing)
For structs with multiple fields, you must implement `FromStr` manually to define how the string should be parsed (e.g., CSV, JSON).

```rust
struct Person { name: String, age: u32 }

impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse "name,age" string...
    }
}
// Once FromStr is implemented, you can use it in a flag:
#[derive(Flag)]
struct PersonFlag(Person);
```

## Advanced Usage: Todo App

Check `examples/todo.rs` for a complete Todo application demonstrating:

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
cargo run --example todo -- add "Buy milk"

# List tasks
cargo run --example todo -- list --all
```

## Examples

| Example | Description | Command |
|---------|-------------|---------|
| `simple` | Basic usage with `derive(App)` | `cargo run --example simple` |
| `custom_types` | Usage of Enums and Structs as Flags | `cargo run --example custom_types` |
| `complex_flag_manual` | Manual parsing for multi-field structs | `cargo run --example complex_flag_manual -- --person "Bob,25"` |
| `todo` | Todo App with subcommands and complex logic | `cargo run --example todo -- --help` |
