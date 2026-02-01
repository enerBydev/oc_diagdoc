# ═══════════════════════════════════════════════════════════════
# oc_diagdoc Dockerfile - Multi-stage build
# ═══════════════════════════════════════════════════════════════

# Stage 1: Build
FROM rust:1.75-slim as builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build actual application
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/oc_diagdoc /usr/local/bin/

# Default data directory
VOLUME /data

ENTRYPOINT ["oc_diagdoc"]
CMD ["--help"]
