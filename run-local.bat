@echo off
REM Run backend locally on Windows
REM Requires WSL2 with Docker Desktop integration

echo Starting database and Redis...
docker-compose up -d postgres redis

echo Waiting for services...
timeout /t 10 /nobreak

echo.
echo Starting Rust backend in WSL...
echo.
echo Run these commands in WSL/Ubuntu:
echo.
echo cd /mnt/c/Users/shabi/Desktop/sleep\ monitoring\ project/backend
echo export DATABASE_URL="postgres://postgres:password@localhost:5432/sleep_monitor"
echo export REDIS_URL="redis://127.0.0.1:6379"
echo export JWT_SECRET="dev-secret-key-for-docker-CHANGE-IN-PRODUCTION"
echo export RUST_LOG="info"
echo cargo run --release
echo.
pause
