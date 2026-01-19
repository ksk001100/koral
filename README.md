# Koral

> [!WARNING]
> This is an experimental project.

> A macro-centric CLI framework for Rust emphasizing separation of definition and state.

Koral provides a declarative way to build Command Line Interfaces. By leveraging Rust's type system and procedural macros, it allows you to define your application's structure (flags, subcommands, metadata) separately from its runtime state.

## Philosophy: Definition vs. State

Unlike other CLI libraries where the parsed result *is* your struct, Koral separates them:

1.  **Definition**: You define **Flags** and **Apps** as structs/enums with attributes. These represent *what* your CLI accepts.
2.  **State**: At runtime, Koral parses arguments into a `Context`. Your application logic receives this context (or extracts values from it) and processes data.

## Key Features

- **Declarative Macros**: Use `#[derive(App)]`, `#[derive(Subcommand)]`, and `#[derive(Flag)]`.
- **Dependency Injection**: Defines handlers that extract States and Flags directly (`fn run(state: State<S>, verbose: FlagVal<V>)`).
- **Middleware**: Hook into lifecycle execution (`before`/`after`) for logging, auth, etc. Supports both static registration and dynamic injection.
- **Type-Safe**: Flags are strongly typed. Custom types (Enums/Structs) supported via `#[derive(FlagValue)]`.
- **Extensible**: Share state easily across subcommands.
- **Validation**: strict mode, required flags, and custom validators.
- **Shell Completion**: Generate scripts for Bash, Zsh, and Fish.

## Installation

```bash
cargo add --git https://github.com/ksk001100/koral
```

## Quick Start (Dependency Injection Style)

```rust
use koral::prelude::*;

// 1. Define State
#[derive(Default, Clone)]
struct AppState {
    count: u32,
}

// 2. Define Flags
#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
struct VerboseFlag(bool);

#[derive(Flag, Debug)]
#[flag(name = "name", default = "World", help = "Target name")]
struct NameFlag(String);

// 3. Define App
#[derive(App)]
#[app(name = "greet", version = "1.0", action = run)]
#[app(flags(VerboseFlag, NameFlag))]
struct GreetApp;

// 4. Define Handler with DI
// Koral automatically injects State, Flags, and Args!
fn run(
    state: State<AppState>, 
    verbose: FlagArg<VerboseFlag>, 
    name: FlagArg<NameFlag>
) -> KoralResult<()> {
    if *verbose {
        println!("Debug: State count is {}", state.count);
    }
    println!("Hello, {}!", *name);
    Ok(())
}

fn main() -> KoralResult<()> {
    // Run with state
    let mut state = AppState { count: 42 };
    let mut app = GreetApp;
    app.run_with_state(&mut state, std::env::args().collect())
}
```

## Advanced Features

### Middleware (Hooks)

You can define logic to run before and after your command.

**Static Registration** (Simple):
```rust
#[derive(Default)]
struct LoggerMiddleware;
impl Middleware for LoggerMiddleware {
    fn before(&self, _: &mut Context) -> KoralResult<()> {
        println!("Starting...");
        Ok(())
    }
}

#[derive(App)]
#[app(middleware(LoggerMiddleware))]
struct MyApp;
```

**Dynamic Injection** (Configurable):
```rust
#[derive(Clone)]
struct AuthMiddleware { api_key: String }
impl Middleware for AuthMiddleware { ... }

#[derive(App)]
struct MyApp {
    #[app(middleware)] // Injects this field as middleware
    auth: AuthMiddleware
}

fn main() {
    let app = MyApp { 
        auth: AuthMiddleware { api_key: "secret".into() } 
    };
    app.run(args);
}
```

### Flag Configuration

Koral supports various attributes to customize flag behavior:

- **required**: Marks the flag as mandatory (`required = true`).
- **env**: Sets an environment variable to read from if the flag is missing (`env = "MY_ENV_VAR"`).
- **value_name**: Customizes the placeholder name in help/completion (e.g. `value_name = "FILE"` -> `--config <FILE>`).
- **help_heading**: Groups the flag under a custom heading in the help message.
- **Strict Mode**: Add `#[app(strict)]` to treat unknown flags as errors instead of positional args.

```rust
#[derive(Flag)]
#[flag(
    name = "token", 
    required = true, 
    env = "API_TOKEN", 
    help_heading = "Authentication"
)]
struct TokenFlag(String);

#[derive(Flag)]
#[flag(
    name = "output", 
    short = 'o', 
    value_name = "PATH",
    help = "Output file path"
)]
struct OutputFlag(String);

#[derive(App)]
#[app(name = "secure-app", strict)]
#[app(flags(TokenFlag, OutputFlag))]
struct SecureApp;
```

### Custom Flag Types

Easily parse Enums or Structs.

```rust
#[derive(FlagValue, Clone, Debug, PartialEq)] // Auto-implements FromStr/ToString
enum Format {
    Json,
    Text,
}

#[derive(Flag)]
#[flag(name = "format", default = "text")]
struct FormatFlag(Format);
```

### Shell Completion

Generate completion scripts.

```rust
use koral::completion::{Shell, generate_to};
// ... inside handler ...
generate_to(&app, Shell::Bash, &mut std::io::stdout())?;
```

## Examples

| Example | Description | Command |
|---------|-------------|---------|
| `hello-world` | Minimal "Hello World" example | `cargo run -p hello-world` |
| `todo-app` | Full application with subcommands and state | `cargo run -p todo-app` |
| `sys-monitor` | Features showcase (DI, Middleware, etc.) | `cargo run -p sys-monitor` |
| `kv-store` | Key-Value store with persistence | `cargo run -p kv-store` |
