# Enterprise Platform CLI

A comprehensive CLI tool for managing enterprise infrastructure, demonstrating the power of the `koral` library with nested subcommands, middleware, and state management.

## Features

- **Nested Subcommands**: Organized modules for K8s, DB, CI/CD, Monitor, IAM, and Network.
- **Global Flags**: Support for global configurations like `--verbose`, `--dry-run`, `--output`, and `--profile`.
- **Middleware**: Example authentication middleware simulation.
- **State Management**: Shared application state across subcommands.

## Installation

Run the example directly using `cargo`:

```bash
cargo run -p enterprise-ops -- --help
```

## Usage

### Kubernetes

Manage clusters and resources.

```bash
# List clusters
cargo run -p enterprise-ops -- k8s clusters list

# Get pod logs
cargo run -p enterprise-ops -- k8s workloads logs --pod my-app-pod -n default
```

### Database

Manage managed databases.

```bash
# List Postgres instances
cargo run -p enterprise-ops -- db postgres list

# Create a backup
cargo run -p enterprise-ops -- db postgres backups create --name main-db
```

### CI/CD

Orchestrate pipelines.

```bash
# Run a pipeline
cargo run -p enterprise-ops -- cicd pipelines run --id build-backend --branch develop
```

### Monitoring

Check metrics and logs.

```bash
# Query metrics using PromQL
cargo run -p enterprise-ops -- monitor metrics query --query "node_cpu_seconds_total"
```

### IAM & Network

Manage users and network resources.

```bash
# List users
cargo run -p enterprise-ops -- iam users list

# List VPCs
cargo run -p enterprise-ops -- network vpc list
```
