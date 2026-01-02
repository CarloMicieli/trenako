# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.92.0
ARG RUST_IMAGE_SHA256=3d0d1a335e1d1220d416a1f38f29925d40ec9929d3c83e07a263adf30a7e4aa3
ARG APP_NAME=trenako-server
ARG RUNTIME_IMAGE_SHA256=45287d89d96414e57c7705aa30cb8f9836ef30ae8897440dd8f06c4cff801eec

################################################################################
# Create a stage for building the application.
FROM rust:${RUST_VERSION}-slim-bookworm@sha256:${RUST_IMAGE_SHA256} AS build
ARG TARGETPLATFORM
ARG APP_NAME
WORKDIR /app

ENV SQLX_OFFLINE=true

COPY Cargo.toml .
COPY Cargo.lock .
COPY .sqlx/ ./.sqlx
COPY crates/ ./crates/
COPY config/ ./config/

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release --bin trenako-server --target-dir ./target
cp ./target/release/$APP_NAME /bin/server
EOF


################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the alpine image as the foundation for running the app.
# By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# reproducability is important, consider using a digest
# (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
FROM debian:bookworm-slim@sha256:${RUNTIME_IMAGE_SHA256} AS runtime
LABEL maintainer="Carlo Micieli <mail@trenako.com>"
LABEL description="The trenako web server"

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN mkdir -p ${APP}/config
RUN chown -R $APP_USER:$APP_USER ${APP}

USER appuser

HEALTHCHECK --interval=5m --timeout=3s \
  CMD curl -f http://localhost/health-check || exit 1

ENV TZ=Etc/UTC \
    APP_USER=appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
COPY --from=build /app/config/application-docker.yml ${APP}/config/application.yml

# Expose the port that the application listens on.
EXPOSE 5000

# What the container should run when it is started.
CMD ["/bin/server", "/usr/src/app/config/application"]
