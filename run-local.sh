# Run backend locally (not in Docker)
# This avoids Docker logging issues

echo "Starting database and Redis..."
docker-compose up -d postgres redis

echo "Waiting for services to be healthy..."
sleep 10

echo "Starting Rust backend locally..."
cd backend

# Set environment variables
export DATABASE_URL="postgres://postgres:password@localhost:5432/sleep_monitor"
export REDIS_URL="redis://127.0.0.1:6379"
export JWT_SECRET="dev-secret-key-for-docker-CHANGE-IN-PRODUCTION"
export RUST_LOG="info"
export SERVER_HOST="0.0.0.0"
export SERVER_PORT="3000"

# Run backend
cargo run --release
