#!/bin/bash
# Database Setup Script - Run this BEFORE cargo build

echo "Setting up database..."

# Create database (using postgres user)
sudo -u postgres psql -c "DROP DATABASE IF EXISTS sleep_monitor;"
sudo -u postgres psql -c "CREATE DATABASE sleep_monitor;"

echo "Database created!"
echo ""
echo "Now run migrations:"
echo "  cd backend"
echo "  sqlx migrate run"
echo ""
echo "Then build:"
echo "  cargo build"
