#!/bin/bash
# Run this once to generate .sqlx cache

# Load .env
export $(cat ../.env | grep -v '^#' | xargs)

# Ensure database exists
sudo -u postgres psql -c "SELECT 1 FROM pg_database WHERE datname='sleep_monitor'" | grep -q 1 || \
  sudo -u postgres psql -c "CREATE DATABASE sleep_monitor"

# Run migrations
sqlx migrate run

# Generate cache
cargo sqlx prepare

echo "Done! .sqlx cache generated."
echo "Now commit .sqlx/ folder to git."
