# Koral

> CLI framework providing simplicity and extensibility.

Koral represents a modern approach to building Command Line Interfaces in Rust, balancing ease of use for simple scripts with robust extensibility for complex applications.

## **Warning**
**Still in the planning stage.**

## Features

- **Simple Builder API**: Create functional CLIs with a fluent builder pattern.
- **Type-Safe Flags**: Leverage Rust's type system for flag parsing (`i32`, `bool`, `String`, custom types).
- **Context-Aware Execution**: Access parsed flags and arguments efficiently via a `Context` object.
- **Extensible Trait System**: Implement the `App` trait for full control over your application structure.
- **Subcommand Support**: Easily nest commands for complex CLI tools (like git).

## Installation

```bash
cargo add --git https://github.com/ksk001100/koral
```

## Quick Start

### Simple Application

For quick tools, use the `App` builder and a closure:

```rust
use koral::{App, Flag, KoralResult};

struct VerboseFlag;
impl Flag for VerboseFlag {
    type Value = bool;
    fn name() -> &'static str { "verbose" }
    fn short() -> Option<char> { Some('v') }
    fn help() -> &'static str { "Enable verbose output" }
    fn takes_value() -> bool { false }
}

struct NameFlag;
impl Flag for NameFlag {
    type Value = String;
    fn name() -> &'static str { "name" }
    fn default_value() -> Option<Self::Value> { Some("User".to_string()) }
    fn help() -> &'static str { "Name to greet" }
}

fn main() -> KoralResult<()> {
    App::new("my-tool")
        .version("1.0")
        .register::<VerboseFlag>()
        .register::<NameFlag>()
        .action(|ctx| {
            let verbose = ctx.get::<VerboseFlag>().unwrap_or(false);
            // Parser guarantees defaults are applied if available in Flag definition?
            // Actually parsing fills default in context if missing? 
            // In current implementation, default values in definition are checked manualy or need Context update.
            // But let's show the ergonomic retrieval.
            let name = ctx.get::<NameFlag>().ok_or("Missing name").unwrap();
            
            if verbose { println!("Verbose mode on"); }
            println!("Hello, {}!", name);
            Ok(())
        })
        .run(std::env::args().skip(1).collect())
}
```

Run the example:
```bash
cargo run --example simple_app -- --verbose --name Koral
```

## Advanced Usage

### Struct-Based Application

For larger applications, define your state in a struct and implement the `App` trait. This allows you to encapsulate dependencies and configuration.

Check `examples/manual_struct.rs` for a complete example.

### Subcommands

Koral supports nested subcommands. Each subcommand is an `App` itself, allowing for infinite nesting depth.

Check `examples/complex_app.rs` to see a git-like CLI implementation.

## Examples

Run the provided examples to see Koral in action:

| Example | Description | Command |
|---------|-------------|---------|
| `simple_app` | Basic usage with builder pattern | `cargo run --example simple_app -- --count 3` |
| `manual_struct` | Implementing `App` trait for a struct | `cargo run --example manual_struct -- --name Koral` |
| `custom_type` | Parsing custom types (enums) | `cargo run --example custom_type -- --env prod` |
| `complex_app` | Nested subcommands | `cargo run --example complex_app remote add origin https://example.com` |
