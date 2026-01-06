# Backend Testing - Quick Reference Card

## 🚀 Quick Start (3 Commands)

```bash
# 1. Setup environment variables
export DATABASE_URL="postgres://postgres:password@localhost/sleep_monitor"
export JWT_SECRET="dev-secret-key"

# 2. Start backend
RUST_LOG=info cargo run

# 3. Run tests
./test_branch_2a.sh
```

---

## 🔐 Authentication (Pi Only)

### Get Token (for Pi devices only)
```bash
curl -X POST http://localhost:3000/api/auth/token \
  -H "Content-Type: application/json" \
  -d '{"device_id":"pi-001"}'

# Save token for Pi
TOKEN="eyJhbGci..."
```

**Note:** Only Pi devices need authentication for sending data!

---

## 📋 Manual Test Commands

### Health Check (No Auth)
```bash
curl http://localhost:3000/health
```

### Send Test Data (Auth Required - Pi Only)
```bash
curl -X POST http://localhost:3000/api/sensor-data \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"temp":22.5,"hum":45.0,"motion":false,"sound_db":35.2,"deviceid":"pi-001","timestamp":"2024-12-30T10:00:00Z"}'
```

### Check Database (No Auth)
```bash
# Sensor readings count
psql -U postgres -d sleep_monitor -c "SELECT COUNT(*) FROM sensor_readings;"

# FHIR observations count
psql -U postgres -d sleep_monitor -c "SELECT COUNT(*) FROM fhir_observations;"
```

### Test FHIR API (No Auth - Public)
```bash
# Get all observations
curl "http://localhost:3000/api/fhir/Observation?_count=10" | jq '.total'

# Filter by device
curl "http://localhost:3000/api/fhir/Observation?patient=Device/pi-001" | jq '.total'

# Filter by type
curl "http://localhost:3000/api/fhir/Observation?code=CUSTOM-TEMP-001" | jq '.total'
```

### Test ML Results API (No Auth - Public)
```bash
# Get all sleep records
curl http://localhost:3000/api/sleep-records

# Get latest sleep quality
curl http://localhost:3000/api/sleep-quality/latest
```

### Test WebSocket (No Auth - Public)
Open browser console at http://localhost:3000 and run:
```javascript
const ws = new WebSocket('ws://localhost:3000/ws');
ws.onmessage = (e) => console.log(JSON.parse(e.data));
```

---

## ✅ Expected Results

| Action | Expected Result |
|--------|----------------|
| Get token (Pi) | Returns JWT token valid 24 hours |
| Send data (no token) | 401 Unauthorized |
| Send data (with token) | 200 OK, data stored |
| Health check | Public, no auth needed |
| WebSocket | Public, works without auth |
| FHIR API | Public, no auth needed |
| ML API | Public, no auth needed |

---

## 🔧 Security Model

**Protected (Auth Required):**
- POST /api/sensor-data (Pi sending data)
- POST /api/auth/token (public endpoint for getting tokens)

**Public (No Auth):**
- GET /health
- WS /ws (WebSocket)
- GET /api/fhir/* (all FHIR endpoints)
- GET /api/sleep-records (all ML endpoints)
- GET / (frontend)
- Static files (/js, /css)

---

## 📊 Success Criteria

- ✅ Pi can get token and send data
- ✅ Frontend works without authentication
- ✅ WebSocket connects without token
- ✅ All read APIs work without auth
- ✅ Only sensor data ingestion requires auth

---

**Run automated tests:** `./test_branch_2a.sh` 🧪
