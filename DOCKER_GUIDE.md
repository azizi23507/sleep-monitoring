# Sleep Monitoring System - Docker Setup

## Quick Start

### Prerequisites
- Docker Desktop installed
- Docker Compose installed (included with Docker Desktop)

### Run Everything with One Command

```bash
# Start all services (backend, PostgreSQL, Redis)
docker-compose up

# Or run in background
docker-compose up -d
```

### Access the Application

- **Frontend:** http://localhost:3000
- **Backend API:** http://localhost:3000/api
- **Health Check:** http://localhost:3000/health

### Stop Services

```bash
# Stop and remove containers
docker-compose down

# Stop and remove containers + volumes (deletes database data)
docker-compose down -v
```

## What Gets Started

1. **PostgreSQL Database** (port 5432)
 - Database: `sleep_monitor`
 - User: `postgres`
 - Password: `password`
 - Data persisted in Docker volume

2. **Redis Cache** (port 6379)
 - Used for WebSocket real-time data
 - Used for caching last 100 sensor readings
 - **Note:** Data resets on container restart (no persistence configured)
 - To enable persistence, see "Redis Persistence" section below

3. **Rust Backend + Frontend** (port 3000)
 - Rust API backend
 - Serves frontend HTML/CSS/JS
 - Auto-runs database migrations on startup

## Development Workflow

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f postgres
docker-compose logs -f redis
```

### Rebuild After Code Changes

```bash
# Rebuild backend image
docker-compose build backend

# Restart with new image
docker-compose up -d backend
```

### Execute Commands in Containers

```bash
# PostgreSQL shell
docker-compose exec postgres psql -U postgres -d sleep_monitor

# Backend shell
docker-compose exec backend /bin/bash

# Redis CLI
docker-compose exec redis redis-cli
```

## Environment Variables

### Default Values (docker-compose.yml)
- `DATABASE_URL`: postgres://postgres:password@postgres:5432/sleep_monitor
- `REDIS_URL`: redis://redis:6379
- `JWT_SECRET`: dev-secret-key-for-docker-CHANGE-IN-PRODUCTION (Warning: Change in production)
- `RUST_LOG`: info

### Custom Configuration

Create `.env` file in project root:

```bash
JWT_SECRET=your-custom-secure-secret-key
RUST_LOG=debug
```

Docker Compose will automatically load it.

### Security Warning

**Before deploying to production:**
1. Change `JWT_SECRET` to a strong random value
2. Generate with: `openssl rand -base64 32`
3. Set in `.env` file or docker-compose.yml
4. Never use the default development secret!

## Testing with Docker

### 1. Send Test Data

```bash
# Get authentication token
TOKEN=$(curl -s -X POST http://localhost:3000/api/auth/token \
 -H "Content-Type: application/json" \
 -d '{"device_id": "test-device"}' | grep -oP '"token":"\K[^"]+')

# Send sensor data
curl -X POST http://localhost:3000/api/sensor-data \
 -H "Content-Type: application/json" \
 -H "Authorization: Bearer $TOKEN" \
 -d '{
 "temp": 22.5,
 "hum": 45.0,
 "motion": false,
 "sound_db": 35.2,
 "deviceid": "test-device",
 "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
 }'
```

### 2. Check Database

```bash
docker-compose exec postgres psql -U postgres -d sleep_monitor -c "SELECT COUNT(*) FROM sensor_readings;"
```

### 3. Check Redis

```bash
docker-compose exec redis redis-cli LLEN sensor:latest
```

## Troubleshooting

### Port Already in Use

```bash
# Check what's using port 3000
lsof -i :3000 # macOS/Linux
netstat -ano | findstr :3000 # Windows

# Change port in docker-compose.yml
ports:
 - "8080:3000" # Use 8080 instead
```

### Database Connection Failed

```bash
# Check PostgreSQL is running
docker-compose ps postgres

# Check logs
docker-compose logs postgres

# Restart PostgreSQL
docker-compose restart postgres
```

### Backend Won't Start

```bash
# Check if dependencies are healthy
docker-compose ps

# Rebuild without cache
docker-compose build --no-cache backend

# Check backend logs
docker-compose logs backend
```

### Clear Everything and Start Fresh

```bash
# Stop and remove everything including volumes
docker-compose down -v

# Rebuild from scratch
docker-compose build --no-cache

# Start again
docker-compose up
```

## Production Deployment

### Security Checklist

- [ ] Change `JWT_SECRET` to strong random value
- [ ] Change PostgreSQL password
- [ ] Set `RUST_LOG=warn` or `RUST_LOG=error`
- [ ] Configure CORS to specific domains
- [ ] Use HTTPS/TLS (add reverse proxy like nginx)
- [ ] Set up database backups
- [ ] Use Docker secrets instead of environment variables

### Example Production Override

Create `docker-compose.prod.yml`:

```yaml
version: '3.8'

services:
 postgres:
 environment:
 POSTGRES_PASSWORD: ${DB_PASSWORD}
 restart: always

 backend:
 environment:
 JWT_SECRET: ${JWT_SECRET}
 RUST_LOG: warn
 restart: always
```

Run with:
```bash
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Build Information

- **Base Image:** rust:1.75 (builder), debian:bookworm-slim (runtime)
- **Build Type:** Multi-stage (optimized for size)
- **Image Size:** ~150MB (runtime)
- **Build Time:** ~5-10 minutes (first build)
- **Subsequent Builds:** ~1-2 minutes (cached layers)

## Volumes

- `postgres_data`: Persists database data across container restarts
 - Data survives `docker-compose down`
 - Deleted with `docker-compose down -v`
 
- **Redis:** No volume configured (cache resets on restart)
 - Intentional for real-time streaming cache
 - To enable persistence, see "Redis Persistence" section below

## Redis Persistence (Optional)

By default, Redis cache resets when the container restarts. To enable persistence:

1. **Edit docker-compose.yml:**
```yaml
redis:
 volumes:
 - redis_data:/data
 command: redis-server --appendonly yes
```

2. **Add to volumes section:**
```yaml
volumes:
 postgres_data:
 driver: local
 redis_data:
 driver: local
```

3. **Restart services:**
```bash
docker-compose down
docker-compose up -d
```

**Note:** For a real-time streaming cache, persistence may not be necessary as data is meant to be temporary.

## Networks

- `sleep-network`: Internal bridge network
- Services communicate by container name (postgres, redis, backend)
- Only exposed ports accessible from host

## Last Updated
January 6, 2026

