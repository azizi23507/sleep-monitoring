#!/bin/bash
# Complete setup script for Sleep Monitoring System

set -e

echo "========================================"
echo "Sleep Monitoring System - Setup"
echo "========================================"
echo ""

# Check if .env exists
if [ ! -f ".env" ]; then
    echo "ERROR: .env file not found!"
    echo "Please run: cp .env.example .env"
    echo "Then edit .env with your credentials"
    exit 1
fi

# Load environment variables
export $(cat .env | grep -v '^#' | xargs)

echo "✓ Environment variables loaded"
echo ""

# Check PostgreSQL
echo "Checking PostgreSQL..."
if ! command -v psql &> /dev/null; then
    echo "ERROR: PostgreSQL not installed"
    echo "Install with: sudo apt install postgresql postgresql-contrib"
    exit 1
fi

# Check if PostgreSQL is running
if ! sudo service postgresql status &> /dev/null; then
    echo "Starting PostgreSQL..."
    sudo service postgresql start
fi

echo "✓ PostgreSQL is running"
echo ""

# Create database
echo "Setting up database..."
sudo -u postgres psql -c "DROP DATABASE IF EXISTS sleep_monitor;" 2>/dev/null || true
sudo -u postgres psql -c "CREATE DATABASE sleep_monitor;"

echo "✓ Database created"
echo ""

# Check Redis
echo "Checking Redis..."
if ! command -v redis-server &> /dev/null; then
    echo "WARNING: Redis not installed"
    echo "Install with: sudo apt install redis-server"
    echo "Continuing anyway..."
else
    if ! pgrep redis-server > /dev/null; then
        echo "Starting Redis..."
        redis-server --daemonize yes
    fi
    echo "✓ Redis is running"
fi

echo ""
echo "========================================"
echo "Setup Complete!"
echo "========================================"
echo ""
echo "Now run:"
echo "  cd backend"
echo "  cargo run"
echo ""
echo "Backend will:"
echo "  1. Run migrations automatically"
echo "  2. Start on port 3000"
echo "  3. Serve frontend at http://localhost:3000/"
echo ""
