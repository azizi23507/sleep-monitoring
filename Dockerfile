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

# Build application with offline mode for sqlx
ENV SQLX_OFFLINE=true
# Force rebuild of main binary (not just dependencies)
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies including Python
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates libssl3 python3 python3-pip python3-venv && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/sleep-backend .

# Copy frontend
COPY frontend ./frontend

# Copy migrations
COPY backend/migrations ./migrations

# Copy ML script and install Python dependencies
COPY ml ./ml
RUN pip3 install --no-cache-dir pandas psycopg2-binary scikit-learn joblib numpy --break-system-packages

EXPOSE 3000

CMD ["./sleep-backend"]
