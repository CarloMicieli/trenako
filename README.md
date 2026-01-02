# trenako

![license](https://img.shields.io/github/license/CarloMicieli/trenako)
![GitHub last commit](https://img.shields.io/github/last-commit/CarloMicieli/trenako)
[![CI (Rust)](https://github.com/CarloMicieli/trenako/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/CarloMicieli/trenako/actions/workflows/rust-ci.yml)
[![codecov](https://codecov.io/gh/CarloMicieli/trenako/branch/dev/graph/badge.svg?token=i8xoC46ZYN)](https://codecov.io/gh/CarloMicieli/trenako)

A website for model railway collectors.

## Features

tbd

## Tech Stack

* 🦀 `Rust`
* `Cargo` and `cargo-make`
* `Docker` / `Docker compose`

## How to run

```bash
  git clone https://github.com/CarloMicieli/trenako
  cd trenako

  docker compose up
```

- The database admin webpage is available at http://localhost:9000/
- The open api documentation is available at http://localhost:9001/

### Local development

All the most important tasks are defined in the `Makefile.toml` for `cargo-make`.

In order to install `cargo-make` just run the following command:

```bash
  cargo install --force cargo-make
```

To run the `postgres` database as detached docker container:

```bash
  cargo make docker-postgres-run
```

to execute the database migrations:

```bash
  cargo make db-migrate
```

The sqlx `query!` macro is checking the query commands against a live database, to avoid to fail the build when a database is not available the offline mode is handled saving the query information into a json file. To update the file run the following command:

```bash
  cargo make db-update-offline
```

### Start the server

```bash
  cargo make run

    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/trenako`

 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/

Starting the server (127.0.0.1:5000)...
```

#### Environment variables

| Variable                    | Description                                                                    |
|-----------------------------|--------------------------------------------------------------------------------|
| `SERVER__HOST`              | the server host name                                                           |
| `SERVER__PORT`              | the server port number                                                         |
| `DATABASE__NAME`            | the database name                                                              |
| `DATABASE__USERNAME`        | the database username                                                          |
| `DATABASE__PASSWORD`        | the database password                                                          |
| `DATABASE__HOST`            | the database hostname                                                          |
| `DATABASE__PORT`            | the database port number                                                       |
| `DATABASE__REQUIRE_SSL`     | the database connection requires SSL                                           |
| `DATABASE__MIN_CONNECTIONS` | the database min number of connections                                         |
| `DATABASE__MAX_CONNECTIONS` | the database max number of connections                                         |
| `LOGGING__FORMAT`           | the logging format (allowed values: `json`, `compact` and `pretty`)            |
| `LOGGING__LEVEL`            | the logging format (allowed values: `error`, `warn`, `info`, `debug`, `trace`) |

### Checks

Run all tests with the following command:

```bash
  cargo make test
```

The rust code is following the rust formatting standard (via `rustfmt`), to check if the formatting is correct:

```bash
  cargo make fmt-check
```

To run the rust linter (`clippy`):

```bash
  cargo make clippy
```

## Contribution

Contributions are always welcome!

See [CONTRIBUTING.md](CONTRIBUTING.md) for ways to get started.

Please adhere to this project's [code of conduct](CODE_OF_CONDUCT.md).

## License

[Apache 2.0 License](https://choosealicense.com/licenses/apache-2.0/)

```
   Copyright 2022 Carlo Micieli

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```
