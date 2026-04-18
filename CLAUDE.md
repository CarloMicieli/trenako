# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**trenako** is a website for model railway collectors, built as a Rust monorepo using a workspace structure with multiple crates for different concerns (libraries, services, and tools).

## Architecture

The project uses a **modular workspace** with the following structure:

- **`crates/libs/`**: Core libraries
  - `common`: Shared domain types, entities, validation, and utilities (addresses, contacts, measure units, etc.)
  - `catalog`: Domain models and entities specific to the railway catalog (manufacturers, railways, scales, catalog items)
  - `configuration`: Application configuration loading from files and environment variables
  - `data`: Data access layer (database repositories and queries using sqlx)

- **`crates/services/`**: Application services
  - `server`: HTTP API server using Axum framework with REST endpoints, middleware, HATEOAS support, and health checks

- **`crates/tools/`**: Development and utility tools
  - `cli`: Command-line tooling

### Key Dependencies

- **Web Framework**: Axum (async Rust web framework)
- **Database**: PostgreSQL with sqlx (compile-time checked SQL queries)
- **Configuration**: config crate with YAML support + environment variables
- **Serialization**: Serde (JSON, URL-encoded data)
- **Validation**: validator crate with derive macros
- **Testing**: rstest (parametrized tests), testcontainers (integration tests with real database)
- **Tracing**: tracing and tracing-subscriber for logging

## Common Development Commands

All commands are defined in `Makefile.toml` for cargo-make. Install with: `cargo install --force cargo-make`

### Building and Checking

```bash
cargo make build       # Compile the workspace (uses SQLX_OFFLINE=true)
cargo make check       # Analyze code without building (fast)
cargo make clean       # Clean build artifacts
```

### Testing

```bash
cargo make test        # Run all tests (integration + unit, 2 threads for I/O stability)
cargo make unit-test   # Run unit tests only (lib code, 4 threads)
cargo make e2e-test    # Run integration tests only (2 threads)
```

### Linting and Formatting

```bash
cargo make fmt-check   # Check code formatting (rustfmt)
cargo make clippy      # Run clippy linter with deny-warnings
cargo make all-checks  # Run fmt-check, clippy, and all tests
```

### Running the Server

```bash
cargo make run         # Start the server (runs trenako-server binary)
```

Server binds to configured address (default: 127.0.0.1:5000) and logs startup info.

### Database Management

```bash
cargo make docker-postgres-run   # Start PostgreSQL in Docker for development
cargo make docker-postgres-stop  # Stop the development database container
cargo make db-migrate            # Run all pending migrations
cargo make db-prepare            # Update .sqlx query cache for offline mode
cargo make db-update-offline     # Full cycle: docker-up, migrate, prepare, docker-down
cargo make db-dev                # Full dev setup: start docker, migrate, prepare cache
```

**Important**: The sqlx `query!` macro validates SQL at compile-time against a live database. To build without a database, use `SQLX_OFFLINE=true` (already set in Makefile.toml tasks). Query metadata is cached in `.sqlx/` JSON files—run `db-prepare` when adding/changing queries.

### Documentation

```bash
cargo make docs        # Generate rustdoc (uses SQLX_OFFLINE=true)
```

## Configuration

Application settings are loaded from:
1. `config/application.yaml` (if present)
2. Environment variables (with `__` separator, e.g., `DATABASE__HOST`)

### Key Environment Variables

| Variable | Description |
|----------|-------------|
| `SERVER__HOST` | Server bind address (default: 127.0.0.1) |
| `SERVER__PORT` | Server port (default: 5000) |
| `DATABASE__HOST` | PostgreSQL host |
| `DATABASE__PORT` | PostgreSQL port |
| `DATABASE__NAME` | Database name |
| `DATABASE__USERNAME` | Database user |
| `DATABASE__PASSWORD` | Database password |
| `DATABASE__MIN_CONNECTIONS` | Connection pool minimum size |
| `DATABASE__MAX_CONNECTIONS` | Connection pool maximum size |
| `LOGGING__FORMAT` | Log format: `json`, `compact`, `pretty`, `full` |
| `LOGGING__LEVEL` | Log level: `error`, `warn`, `info`, `debug`, `trace` |

Settings are typed via `configuration::Settings` struct with subconfigs for server, database, and logging.

## Code Organization Patterns

### Domain Models and Entities

Models are in `catalog` and `common` crates. Key patterns:
- Use strong types (avoid primitives for domain concepts)
- Validation via `validator::Validate` derive macro
- Serialization via `serde::Serialize/Deserialize`
- Entity IDs as `uuid::Uuid`

### Database Access (sqlx)

- Queries in `data` crate using sqlx macros (`sqlx::query!`, `sqlx::query_as!`)
- Compile-time checking against live database or cached query metadata
- Transaction handling via `sqlx::PgPool` connection pool
- Repositories/use-case handlers in `data::catalog` module organize database operations

### HTTP API (Axum)

- Endpoints defined in `server::catalog` and `server::web`
- Middleware for tracing, CORS, compression in `server::middlewares`
- HATEOAS links in `server::hateoas`
- Request/response validation via serde + validator crate
- State management through Axum's shared state mechanism

### Testing

- **Unit tests**: In same crate as code, use `#[cfg(test)]` modules
- **Integration tests**: In `tests/` directory with testcontainers for real database
- Use `rstest` for parametrized tests
- Test data generation with `fake` crate and custom builders

## Conventions

### Commits

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `style:` Code formatting (run `cargo fmt`)
- `refactor:` Code restructuring
- `test:` Test additions/changes
- `chore:` Dependencies, tooling, etc.

Example: `feat: add manufacturer search endpoint`

### Code Style

- Run `cargo fmt --all` before committing
- Run `cargo clippy --all-targets --all-features` to catch lint warnings (deny-warnings enabled)
- Follow Rust API guidelines and idiomatic patterns
- Prefer explicit error handling with `thiserror` and `anyhow`

### Module Structure

- `lib.rs` re-exports public API; internal implementation in submodules
- Domain logic in `common` and `catalog` (business rules, entities)
- Data access in `data` crate (queries, repositories)
- HTTP layer in `server` (handlers, middleware, serialization)

## Important Notes

- **Workspace**: Disables default workspace support (`default_to_workspace = false` in Makefile.toml) to allow task-scoped execution
- **Offline Mode**: Most tasks set `SQLX_OFFLINE=true` to work without a live database; only database tasks require a running PostgreSQL
- **Test Threads**: Reduced to 2-4 threads in some tasks due to database I/O contention in integration tests
- **OpenAPI**: Available at `http://localhost:9001/` when server is running
- **PgAdmin**: Database admin interface at `http://localhost:9000/` (when using docker-compose)

## Running Locally

Full development setup:

```bash
cargo make db-dev      # Start database and run migrations
cargo make run         # In another terminal, start the server
# Server runs on http://127.0.0.1:5000
# OpenAPI docs at http://127.0.0.1:9001/
```

For quick iteration during development, use individual tasks:

```bash
cargo make unit-test           # Fast feedback on business logic
cargo make db-prepare          # After changing queries
cargo make clippy && cargo fmt # Before committing
```
