# Koral

> [!WARNING]
> This is an experimental project.

A macro-centric CLI framework for Rust emphasizing separation of definition and state.

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

## Documentation

Full documentation is available on docs.rs (once published) or can be generated locally:

```bash
cargo doc --open
```

## Installation

```bash
cargo add --git https://github.com/ksk001100/koral koral
```

## Examples

| Example | Description | Command |
|---------|-------------|---------|
| [`hello-world`](examples/hello-world) | Minimal "Hello World" example | `cd examples/hello-world && cargo run` |
| [`todo-app`](examples/todo-app) | Full application with subcommands and state | `cd examples/todo-app && cargo run` |
| [`sys-monitor`](examples/sys-monitor) | Features showcase (DI, Middleware, etc.) | `cd examples/sys-monitor && cargo run` |
| [`kv-store`](examples/kv-store) | Key-Value store with persistence | `cd examples/kv-store && cargo run` |
| [`cloud-cli`](examples/cloud-cli) | Comprehensive Cloud CLI Simulation | `cd examples/cloud-cli && cargo run` |
