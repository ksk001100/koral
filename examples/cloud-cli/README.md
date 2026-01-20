# Cloud CLI Example

A comprehensive simulation of a Cloud Provider CLI, demonstrating:
- **Deeply Nested Subcommands**: `instance launch`, `s3 ls`, etc.
- **Typed Flags**: using `#[derive(FlagValue)]` for Enums.
- **Authentication Middleware**: protecting commands with token checks.
- **Complex State**: managing instances and buckets in memory.

## Usage

```bash
# Login (gets a token)
cargo run -- login --user admin

# List instances (requires token)
cargo run -- instance list --token <token>

# Launch an instance
cargo run -- instance launch --type m5.large --token <token>
```
