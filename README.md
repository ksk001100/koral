# Koral

> CLI framework providing simplicity and extensibility

## **Warning**
**Still in the planning stage.**

## Install
```bash
cargo add --git https://github.com/ksk001100/koral
```

## Run example
### Hello world
```bash
cargo run --example hello
```

### Nest app
```bash
cargo run --example nest_app
cargo run --example nest_app nest_app1
cargo run --example nest_app nest_app1 nest_app2
```

### Custom app
```bash
cargo run --example custom_app -- --help
cargo run --example custom_app add 8 4
cargo run --example custom_app sub 8 4
cargo run --example custom_app mul 8 4
cargo run --example custom_app div 8 4
```
