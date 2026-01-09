#!/bin/sh
set -e

echo "=== Sleep Monitor Backend Startup ==="
echo "Checking binary..."
ls -la /app/sleep-backend

echo "Checking dependencies..."
ldd /app/sleep-backend

echo "Environment variables:"
echo "DATABASE_URL: $DATABASE_URL"
echo "REDIS_URL: $REDIS_URL"
echo "JWT_SECRET: ${JWT_SECRET:0:10}..."
echo "RUST_LOG: $RUST_LOG"

echo "Starting backend..."
exec /app/sleep-backend
