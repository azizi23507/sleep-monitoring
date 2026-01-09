# Multi-stage Docker build for Rust backend
FROM rust:latest as builder

WORKDIR /usr/src/app

# Copy manifests
COPY backend/Cargo.toml backend/Cargo.lock ./

# Create dummy src to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY backend/src ./src
COPY backend/migrations ./migrations
COPY backend/.sqlx ./.sqlx

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/sleep-backend .

# Copy frontend
COPY frontend ./frontend

# Copy migrations
COPY backend/migrations ./migrations

EXPOSE 3000

CMD ["./sleep-backend"]
