# Runtime compilation Dockerfile with migrations
FROM rust:latest

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates python3 python3-pip postgresql-client && \
    rm -rf /var/lib/apt/lists/*

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy everything
COPY backend ./backend
COPY frontend ./frontend
COPY backend/migrations ./migrations
COPY ml ./ml

# Install Python dependencies
RUN pip3 install --no-cache-dir pandas psycopg2-binary scikit-learn joblib numpy --break-system-packages

EXPOSE 3000

# Startup script: wait → migrate → build → run
RUN echo '#!/bin/bash\n\
set -e\n\
\n\
cd /app/backend\n\
\n\
echo "Waiting for database..."\n\
until PGPASSWORD=$POSTGRES_PASSWORD psql -h "$DB_HOST" -U "$POSTGRES_USER" -d "$POSTGRES_DB" -c "\\q" 2>/dev/null; do\n\
  sleep 2\n\
done\n\
\n\
echo "Running migrations..."\n\
sqlx migrate run\n\
\n\
if [ ! -f "target/release/sleep-backend" ]; then\n\
  echo "Building backend..."\n\
  cargo build --release\n\
fi\n\
\n\
cd /app\n\
exec ./backend/target/release/sleep-backend' > /start.sh && chmod +x /start.sh

CMD ["/start.sh"]
