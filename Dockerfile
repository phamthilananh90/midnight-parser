# syntax=docker/dockerfile:1

# ---- Build stage ----
FROM rust:1-bookworm AS builder

# System deps some crates may need at build time (TLS, etc.)
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies: build against a stub main, then build the real source.
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build the actual application.
COPY . .
# Bust the stub's cached crate so the real binary is rebuilt.
RUN touch src/main.rs && cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Run as an unprivileged user.
RUN useradd --system --user-group --no-create-home appuser

COPY --from=builder /app/target/release/steam-user-api /usr/local/bin/steam-user-api

USER appuser

ENV PORT=3000 \
    RUST_LOG=info

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/steam-user-api"]
