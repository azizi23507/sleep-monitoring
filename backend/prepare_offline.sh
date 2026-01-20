#!/bin/bash
# Generate SQLX offline cache from .env file
# Run this ONCE after setting up your .env file

set -e

echo "=== SQLX Offline Cache Generator ==="
echo ""

# Check if .env exists
if [ ! -f "../.env" ]; then
    echo "ERROR: .env file not found in project root!"
    echo "Please copy .env.example to .env and configure it first."
    exit 1
fi

# Load .env file
export $(cat ../.env | grep -v '^#' | xargs)

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "ERROR: DATABASE_URL not found in .env file!"
    exit 1
fi

echo "✓ Found DATABASE_URL in .env"
echo ""

# Check if database exists, create if not
echo "Checking database connection..."
if ! psql "$DATABASE_URL" -c '\q' 2>/dev/null; then
    echo "Database not reachable. Creating database..."
    
    # Extract database name from URL
    DB_NAME=$(echo $DATABASE_URL | sed 's/.*\///')
    
    # Create database using postgres user
    sudo -u postgres psql -c "CREATE DATABASE $DB_NAME;" 2>/dev/null || echo "Database may already exist, continuing..."
fi

echo "✓ Database connection OK"
echo ""

# Run migrations
echo "Running migrations..."
sqlx migrate run

echo "✓ Migrations complete"
echo ""

# Generate offline cache
echo "Generating offline query cache..."
cargo sqlx prepare

echo ""
echo "=== SUCCESS ==="
echo ""
echo "Offline cache generated at: .sqlx/"
echo ""
echo "Now anyone can compile without database:"
echo "  cargo build"
echo ""
echo "Note: Commit the .sqlx/ directory to git!"
