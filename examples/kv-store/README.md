# KV Store Example

A simple Key-Value store application with file persistence.
Demonstrates:
- **Persistence**: Loading and saving state to a file.
- **Command Arguments**: Handling key-value pairs as arguments.

## Usage

```bash
# Set a value
cargo run -p kv-store -- set mykey myvalue

# Get a value
cargo run -p kv-store -- get mykey
```
