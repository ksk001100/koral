# System Monitor Example

This example showcases the advanced features of `koral`, specifically:
- **Dependency Injection**: Injecting dependencies directly into command handlers.
- **Middleware**: Using middleware for logging or setup tasks.
- **Required Flags**: Demonstrating validation of required flags (e.g., `--user`).

## Usage

```bash
# Run the monitor with required user flag
cargo run -- --user Admin

# Run with verbose logging
cargo run -- --user Admin --verbose

# Run a subcommand (status)
cargo run -- --user Admin status
```
