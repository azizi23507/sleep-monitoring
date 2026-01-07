# Quick Start Guide - Sleep Monitoring System

**Last Updated:** January 7, 2026

---

## 🚀 Fastest Way to Run

```bash
# One command to start everything with Docker
docker-compose up -d

# Access the system
# Frontend: http://localhost:3000
# API: http://localhost:3000/api
# Health: http://localhost:3000/health
```

That's it! System is running with PostgreSQL, Redis, and backend.

---

## 📋 What You Need

### For Docker (Recommended)
- Docker Desktop installed
- 2 GB free disk space
- That's all!

### For Local Development
- Rust (nightly)
- PostgreSQL 15+
- Redis 7+
- Git

---

## 🔑 Quick Commands

### Docker Management
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f backend

# Stop all services
docker-compose down

# Reset everything (including data)
docker-compose down -v
```

### Test the System
```bash
# Get authentication token
curl -X POST http://localhost:3000/api/auth/token \
  -H "Content-Type: application/json" \
  -d '{"device_id":"test-pi"}'

# Use token to send data (replace YOUR_TOKEN)
curl -X POST http://localhost:3000/api/sensor-data \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "temp": 22.5,
    "hum": 45.0,
    "motion": false,
    "sound_db": 35.2,
    "deviceid": "test-pi",
    "timestamp": "2026-01-07T12:00:00Z"
  }'

# Check health
curl http://localhost:3000/health
```

---

## 📚 Documentation Guide

| Need to... | Read this |
|------------|-----------|
| **Get started quickly** | This file (QUICK_START.md) |
| **Understand the project** | README.md |
| **Deploy with Docker** | DOCKER_GUIDE.md |
| **Security & production** | SECURITY.md |
| **Backend API details** | backend/README.md |
| **Frontend details** | frontend/README.md |
| **See what changed** | UPDATE_SUMMARY.md |
| **Check requirements** | FINAL_REQUIREMENTS_DOCUMENTATION.md |

---

## ⚠️ Important Security Notes

### Before Production Deployment

**You MUST change these defaults:**

1. **JWT Secret** (Critical!)
```bash
# Generate secure secret
openssl rand -base64 32

# Set in docker-compose.yml or .env
JWT_SECRET=<your-generated-secret>
```

2. **Database Password**
```bash
# In docker-compose.yml, change:
POSTGRES_PASSWORD=password  # <- Change this!
```

3. **CORS Configuration**
- Edit `backend/src/main.rs`
- Restrict to your frontend domain

See `SECURITY.md` for complete checklist.

---

## 🧪 Quick Test

```bash
# 1. Start system
docker-compose up -d

# 2. Wait 30 seconds for initialization

# 3. Test health
curl http://localhost:3000/health
# Should return: {"status":"healthy",...}

# 4. Open dashboard
# Navigate to: http://localhost:3000
```

---

## 🐛 Troubleshooting

### Docker won't start
```bash
# Check Docker is running
docker --version

# Check ports are free
netstat -ano | findstr :3000  # Windows
lsof -i :3000                 # macOS/Linux
```

### Backend shows migration warning
- **Status:** Benign, system works fine
- **Fix:** Can ignore or drop/recreate database
- **Details:** See UPDATE_SUMMARY.md

### Frontend not loading
- Check backend is running: `docker-compose ps`
- Check logs: `docker-compose logs backend`
- Verify: `curl http://localhost:3000/health`

---

## 📊 System Architecture

```
┌─────────────────┐
│  Raspberry Pi   │ (Sensors)
└────────┬────────┘
         │ HTTP POST with JWT
         ↓
┌─────────────────┐
│  Rust Backend   │ (Port 3000)
│  + Frontend     │
└────────┬────────┘
         │
    ┌────┴────┐
    ↓         ↓
┌─────────┐ ┌──────┐
│PostgreSQL│ │ Redis│
│(Persist) │ │(Cache)│
└─────────┘ └──────┘
```

**Data Flow:**
1. Pi sends sensor data → Backend (with JWT)
2. Backend stores in PostgreSQL (permanent)
3. Backend caches in Redis (last 100 readings)
4. Frontend connects via WebSocket (real-time)
5. Frontend fetches ML results via REST API

---

## 🎯 Project Status

| Component | Status |
|-----------|--------|
| Backend | ✅ Production Ready |
| Frontend | ✅ Production Ready |
| Database | ✅ Working |
| Redis Cache | ✅ Working |
| Authentication | ✅ Implemented |
| FHIR API | ✅ Implemented |
| Docker Setup | ✅ Working |
| ML Service | ⏳ Pending Delivery |

**Overall:** 95% Complete (awaiting ML service)

---

## 🔗 Quick Links

- Backend API: http://localhost:3000/api
- Health Check: http://localhost:3000/health
- WebSocket: ws://localhost:3000/ws
- Frontend: http://localhost:3000/

---

## 💡 Pro Tips

1. **First time?** Use Docker - it's easier
2. **Developing?** Use local setup for faster iteration
3. **Testing?** Backend has 30+ automated tests
4. **Need help?** Check backend/README.md for detailed docs
5. **Production?** Read SECURITY.md first!

---

## 📞 Need Help?

1. Check relevant README in folder
2. Review troubleshooting sections
3. Check logs: `docker-compose logs -f`
4. Review UPDATE_SUMMARY.md for recent changes

---

**System Ready!** 🎉  
Open http://localhost:3000 to get started.
