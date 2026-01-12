# Simplified single-stage build for Rust backend with Python
FROM rust:latest

WORKDIR /app

# Install Python and runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates python3 python3-pip python3-venv && \
    rm -rf /var/lib/apt/lists/*

# Copy and build backend
COPY backend ./backend
WORKDIR /app/backend
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Move binary to app root
RUN mv target/release/sleep-backend /app/

# Set up app directory
WORKDIR /app

# Copy frontend
COPY frontend ./frontend

# Copy migrations
COPY backend/migrations ./migrations

# Copy ML script and install Python dependencies
COPY ml ./ml
RUN pip3 install --no-cache-dir pandas psycopg2-binary scikit-learn joblib numpy --break-system-packages

EXPOSE 3000

CMD ["./sleep-backend"]
