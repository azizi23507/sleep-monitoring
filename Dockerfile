# Multi-stage build for optimized Rust backend
# Using nightly for edition2024 support
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Copy dependency files first for better caching
COPY backend/Cargo.toml backend/Cargo.lock* ./

# Create dummy main to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY backend/src ./src
COPY backend/migrations ./migrations

# Set DATABASE_URL for sqlx offline mode
ENV DATABASE_URL=postgres://postgres:password@localhost/sleep_monitor
ENV SQLX_OFFLINE=true

# Build the real application
RUN cargo build --release

# Runtime stage - smaller final image
FROM debian:bookworm-slim

# Install PostgreSQL client and CA certificates
RUN apt-get update && \
    apt-get install -y postgresql-client ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/sleep-backend /app/sleep-backend

# Copy migrations
COPY backend/migrations ./migrations

# Copy frontend files
COPY frontend ./frontend

# Expose port
EXPOSE 3000

# Set environment variables (will be overridden by docker-compose)
ENV RUST_LOG=info
ENV DATABASE_URL=postgres://postgres:password@postgres:5432/sleep_monitor
ENV REDIS_URL=redis://redis:6379
ENV JWT_SECRET=docker-default-secret-change-in-production

# Make binary executable
RUN chmod +x /app/sleep-backend

# Run the application
CMD ["/app/sleep-backend"]
