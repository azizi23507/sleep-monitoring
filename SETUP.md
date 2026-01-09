# Sleep Monitoring System - Setup Instructions

## ⚠️ Important Note

Due to Docker logging issues with the Rust backend binary, the system currently runs with:
- **PostgreSQL and Redis**: In Docker containers
- **Rust Backend**: Run locally (WSL/Linux/Mac) or natively on Windows

This hybrid approach works perfectly and avoids Docker containerization issues.

---

## Quick Start

### Option 1: Run Backend Locally (Recommended)

**On Linux/Mac/WSL:**

```bash
# 1. Start database and Redis
docker-compose up -d postgres redis

# 2. Wait for services
sleep 10

# 3. Run backend
cd backend
export DATABASE_URL="postgres://postgres:password@localhost:5432/sleep_monitor"
export REDIS_URL="redis://127.0.0.1:6379"
export JWT_SECRET="dev-secret-key-for-docker-CHANGE-IN-PRODUCTION"
export RUST_LOG="info"
cargo run --release
```

**Or use the provided script:**
```bash
chmod +x run-local.sh
./run-local.sh
```

### Option 2: Full Docker (Has logging issues but functional)

```bash
docker-compose up -d
```

**Note:** Backend logs won't be visible in Docker, but the system will work once migrations complete.

---

## System Access

Once running:
- 🌐 **Frontend**: http://localhost:3000
- 🔌 **API**: http://localhost:3000/api  
- 🗄️ **Database**: localhost:5432 (postgres/password)
- 🔴 **Redis**: localhost:6379

---

## Testing

### 1. Check Health
```bash
curl http://localhost:3000/health
```

**Expected:**
```json
{"status":"healthy","redis":"connected","uptime_seconds":10}
```

### 2. Get Device Token
```bash
curl -X POST http://localhost:3000/api/auth/token \
  -H "Content-Type: application/json" \
  -d '{"device_id":"pi-001"}'
```

### 3. Send Test Data
```bash
TOKEN="your-token-here"

curl -X POST http://localhost:3000/api/sensor-data \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "pi-001",
    "temperature": 22.5,
    "humidity": 45.0,
    "motion": false,
    "heart_rate": 72,
    "respiratory_rate": 16,
    "timestamp": "2026-01-09T20:00:00Z"
  }'
```

---

## Troubleshooting

### Migration Error

If you see:
```
ERROR: migration 20241226 was previously applied but has been modified
```

**Fix:**
```bash
# Clean database
docker-compose down -v
docker-compose up -d postgres redis
sleep 10

# Restart backend
cd backend && cargo run --release
```

### Port Already in Use

```bash
# Check what's using port 3000
lsof -i :3000  # Mac/Linux
netstat -ano | findstr :3000  # Windows

# Kill the process or change port
export SERVER_PORT=8080
```

### Database Connection Failed

```bash
# Check if containers are running
docker-compose ps

# Check PostgreSQL logs
docker-compose logs postgres

# Restart database
docker-compose restart postgres
```

---

## Project Structure

```
sleep-monitoring/
├── backend/              # Rust backend (run locally)
│   ├── src/             # Source code
│   ├── migrations/      # Database migrations  
│   └── Cargo.toml       # Dependencies
├── frontend/            # Web dashboard
│   ├── index.html       # Main page
│   ├── css/             # Styles
│   └── js/              # Client code
├── docker-compose.yml   # PostgreSQL + Redis
├── Dockerfile           # Backend image (has issues)
└── run-local.sh         # Helper script
```

---

## Security

### Current Credentials (Development)

- **PostgreSQL**: `postgres` / `password`
- **Redis**: No password
- **JWT Secret**: `dev-secret-key-for-docker-CHANGE-IN-PRODUCTION`

### For Production

1. Change PostgreSQL password:
```bash
docker-compose exec postgres psql -U postgres -c "ALTER USER postgres WITH PASSWORD 'new-secure-password';"
```

2. Update `DATABASE_URL` in your environment
3. Generate secure JWT secret: `openssl rand -base64 32`

---

## Known Issues

1. **Docker Backend Logging**: The Rust backend in Docker exits silently due to stdout/stderr not being captured properly. Run locally instead.

2. **Migration Checksums**: If migration files are modified after being run, you'll need to clean the database: `docker-compose down -v`

---

## API Documentation

See `backend/README.md` for full API documentation.

**Key Endpoints:**

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/health` | GET | No | Health check |
| `/api/auth/token` | POST | No | Get JWT token |
| `/api/sensor-data` | POST | Yes | Submit readings |
| `/api/sensor-data/:device_id` | GET | No | Get readings |
| `/api/fhir/observation/:device_id` | GET | No | FHIR format |
| `/ws/stream/:device_id` | WS | No | Real-time stream |

---

## Support

- 📚 Documentation: See `backend/README.md`
- 🐛 Issues: GitHub repository
- 📧 Contact: Project maintainers

---

**Last Updated:** January 9, 2026  
**Version:** 1.0.0 (Hybrid Docker + Local)
