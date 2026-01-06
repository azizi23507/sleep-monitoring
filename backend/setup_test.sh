#!/bin/bash
# Quick Setup Script for Testing Branch 2A

echo "========================================="
echo "Branch 2A Testing Setup"
echo "========================================="
echo ""

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "[ERROR] PostgreSQL not installed!"
    echo ""
    echo "Install with:"
    echo "  Ubuntu/Debian: sudo apt-get install postgresql postgresql-contrib"
    echo "  macOS: brew install postgresql"
    echo "  Docker: docker run -d --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 postgres:15"
    exit 1
fi

echo "[OK] PostgreSQL is installed"
echo ""

# Check if PostgreSQL is running
if ! pg_isready -q; then
    echo "[ERROR] PostgreSQL is not running!"
    echo ""
    echo "Start with:"
    echo "  Ubuntu/Debian: sudo systemctl start postgresql"
    echo "  macOS: brew services start postgresql"
    echo "  Docker: docker start postgres"
    exit 1
fi

echo "[OK] PostgreSQL is running"
echo ""

# Create database
echo "Creating database 'sleep_monitor'..."
sudo -u postgres psql -c "CREATE DATABASE sleep_monitor;" 2>/dev/null || echo "  (Database may already exist - that's okay)"
echo ""

# Apply schema
echo "Applying database schema..."
if [ -f "schema.sql" ]; then
    sudo -u postgres psql -d sleep_monitor -f schema.sql > /dev/null 2>&1
    echo "[OK] Schema applied"
else
    echo "[WARNING] schema.sql not found - will use migrations instead"
fi
echo ""

# Set DATABASE_URL environment variable
export DATABASE_URL="postgres://postgres:password@localhost/sleep_monitor"
echo "[OK] DATABASE_URL set"
echo ""

# Check if Redis is running (needed for Branch 1)
if ! redis-cli ping &> /dev/null; then
    echo "[WARNING] Redis is not running (needed for Branch 1)"
    echo "   Start with: redis-server"
    echo ""
else
    echo "[OK] Redis is running"
    echo ""
fi

echo "========================================="
echo "Setup Complete!"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. export DATABASE_URL=\"postgres://postgres:password@localhost/sleep_monitor\""
echo "2. RUST_LOG=info cargo run"
echo "3. Run test commands from TEST_COMMANDS.sh"
echo ""
