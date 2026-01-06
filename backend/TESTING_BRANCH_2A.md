# Branch 2A Testing Guide

Complete guide to test FHIR conversion functionality.

---

## Prerequisites

### Required Software

1. **PostgreSQL** (database)
   ```bash
   # Check if installed
   psql --version
   
   # Install if needed
   sudo apt-get install postgresql postgresql-contrib  # Ubuntu/Debian
   brew install postgresql                              # macOS
   ```

2. **Redis** (for Branch 1)
   ```bash
   # Check if installed
   redis-cli --version
   
   # Install if needed
   sudo apt-get install redis-server  # Ubuntu/Debian
   brew install redis                  # macOS
   ```

3. **Rust** (backend)
   ```bash
   rustc --version
   cargo --version
   ```

4. **jq** (JSON formatting - optional but helpful)
   ```bash
   sudo apt-get install jq  # Ubuntu/Debian
   brew install jq          # macOS
   ```

---

## Quick Start (Automated Testing)

### Option 1: Run Automated Test Script

```bash
# 1. Setup environment variables
export DATABASE_URL="postgres://postgres:password@localhost/sleep_monitor"
export JWT_SECRET="dev-secret-key"

# 2. Start backend (in separate terminal)
RUST_LOG=info cargo run

# 3. Run tests (in another terminal)
./test_branch_2a.sh
```

**The test script automatically:**
- ✅ Checks health endpoint (public)
- ✅ Gets JWT authentication token
- ✅ Sends test sensor data (with token)
- ✅ Verifies PostgreSQL storage
- ✅ Checks FHIR conversion (4 observations per reading)
- ✅ Tests all FHIR API endpoints (with token)
- ✅ Validates search filters
- ✅ Sends multiple test readings (authenticated)

---

## Manual Testing (Step by Step)

### Step 1: Setup PostgreSQL

```bash
# Start PostgreSQL
sudo systemctl start postgresql  # Linux
brew services start postgresql   # macOS

# Create database
sudo -u postgres psql
CREATE DATABASE sleep_monitor;
\q

# Apply schema
sudo -u postgres psql -d sleep_monitor -f schema.sql

# Verify tables exist
sudo -u postgres psql -d sleep_monitor -c "\dt"
# Expected: sensor_readings, fhir_observations
```

---

### Step 2: Start Backend

```bash
# Terminal 1: Start Redis
redis-server

# Terminal 2: Start Backend
export DATABASE_URL="postgres://postgres:password@localhost/sleep_monitor"
export REDIS_URL="redis://127.0.0.1:6379"
RUST_LOG=info cargo run

# Expected output:
# 🚀 Starting Sleep Monitoring Backend
# 🔧 Connecting to PostgreSQL...
# ✅ PostgreSQL connected successfully
# 🔧 Running database migrations...
# ✅ Database migrations complete
# 🔧 Connecting to Redis at: redis://127.0.0.1:6379
# ✅ Redis connected successfully
# 🚀 Sleep Monitoring Backend - READY
#    Server: http://0.0.0.0:3000
#    WebSocket: ws://0.0.0.0:3000/ws
#    API: POST /api/sensor-data
#    Health: GET /health
#    PostgreSQL: Connected ✅
#    Redis: Connected ✅
```

---

### Step 3: Test Health Check (No Auth Required)

```bash
curl http://localhost:3000/health | jq '.'
```

**Expected response:**
```json
{
  "status": "healthy",
  "redis": "connected",
  "uptime_seconds": 5,
  "timestamp": "2024-12-30T10:00:00Z"
}
```

✅ **Success:** Status is "healthy"  
❌ **Failure:** Check PostgreSQL and Redis are running

---

### Step 4: Get Authentication Token

```bash
curl -X POST http://localhost:3000/api/auth/token \
  -H "Content-Type: application/json" \
  -d '{"device_id":"pi-001"}' | jq '.'
```

**Expected response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

**Save the token:**
```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

✅ **Success:** Token received  
❌ **Failure:** Check backend is running

---

### Step 5: Send Test Sensor Data (With Token)

```bash
curl -X POST http://localhost:3000/api/sensor-data \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "temp": 22.5,
    "hum": 45.0,
    "motion": false,
    "sound_db": 35.2,
    "deviceid": "pi-001",
    "timestamp": "2024-12-30T10:00:00Z"
  }'
```

**Expected response:**
```json
{
  "status": "ok",
  "message": "Data received and stored successfully"
}
```

**Backend logs should show:**
```
INFO ingest_sensor_data{device_id="pi-001"}: Validated: temp=22.5°C, hum=45.0%, sound=35.2dB, motion=false
INFO ingest_sensor_data{device_id="pi-001"}: Stored in Redis: 22.5°C, 45.0% hum, 35.2dB, motion: false
DEBUG: Storing in PostgreSQL...
DEBUG: Stored in PostgreSQL successfully with ID: <UUID>
DEBUG: Converting to FHIR observations...
INFO: FHIR conversion complete: 4 observations created
```

✅ **Success:** All log messages appear  
❌ **Failure:** Check DATABASE_URL and REDIS_URL

---

### Step 5: Verify PostgreSQL Storage

```bash
# Check sensor_readings table
sudo -u postgres psql -d sleep_monitor -c "
  SELECT 
    device_id, 
    temperature, 
    humidity, 
    sound_level, 
    motion_detected, 
    reading_timestamp 
  FROM sensor_readings 
  ORDER BY created_at DESC 
  LIMIT 1;
"
```

**Expected output:**
```
 device_id | temperature | humidity | sound_level | motion_detected |   reading_timestamp    
-----------+-------------+----------+-------------+-----------------+------------------------
 pi-001    |       22.50 |    45.00 |       35.20 | f               | 2024-12-26 15:30:00+00
```

✅ **Success:** One row with your test data  
❌ **Failure:** Check PostgreSQL connection and logs

---

### Step 6: Verify FHIR Conversion

```bash
# Count FHIR observations (should be 4 per sensor reading)
sudo -u postgres psql -d sleep_monitor -c "
  SELECT COUNT(*) as total_observations 
  FROM fhir_observations;
"
```

**Expected output:**
```
 total_observations 
--------------------
                  4
```

**View FHIR observation details:**
```bash
sudo -u postgres psql -d sleep_monitor -c "
  SELECT 
    fhir_id, 
    loinc_code, 
    observation_category,
    resource_data->'valueQuantity'->>'value' as value,
    resource_data->'valueBoolean' as boolean_value
  FROM fhir_observations 
  ORDER BY created_at DESC 
  LIMIT 4;
"
```

**Expected output:**
```
              fhir_id              |    loinc_code     | observation_category | value | boolean_value 
-----------------------------------+-------------------+----------------------+-------+---------------
 obs-<UUID>-temp                   | CUSTOM-TEMP-001   | Vital Signs          | 22.5  | 
 obs-<UUID>-hum                    | CUSTOM-HUM-001    | Environmental...     | 45    | 
 obs-<UUID>-sound                  | CUSTOM-SOUND-001  | Environmental...     | 35.2  | 
 obs-<UUID>-motion                 | CUSTOM-MOTION-001 | Activity...          |       | false
```

✅ **Success:** 4 observations with different LOINC codes  
❌ **Failure:** Check FHIR conversion logs

---

### Step 7: Test FHIR API - Get All Observations

```bash
curl "http://localhost:3000/api/fhir/Observation?_count=10" | jq '.'
```

**Expected response:**
```json
{
  "resourceType": "Bundle",
  "type": "searchset",
  "total": 4,
  "entry": [
    {
      "resource": {
        "resourceType": "Observation",
        "id": "obs-<UUID>-temp",
        "status": "final",
        "category": [...],
        "code": {
          "coding": [{
            "system": "http://loinc.org",
            "code": "CUSTOM-TEMP-001",
            "display": "Ambient Temperature"
          }],
          "text": "Temperature"
        },
        "effectiveDateTime": "2024-12-26T15:30:00Z",
        "valueQuantity": {
          "value": 22.5,
          "unit": "°C",
          "system": "http://unitsofmeasure.org",
          "code": "Cel"
        }
      }
    }
    // ... 3 more observations
  ]
}
```

✅ **Success:** Bundle with 4 observations  
❌ **Failure:** Check FHIR API routes

---

### Step 8: Test FHIR API - Filter by Device

```bash
curl "http://localhost:3000/api/fhir/Observation?patient=Device/pi-001" | jq '.total'
```

**Expected response:**
```json
4
```

✅ **Success:** Returns 4 (all observations for pi-001)  
❌ **Failure:** Check patient_id in database

---

### Step 9: Test FHIR API - Filter by Type

```bash
# Get only temperature observations
curl "http://localhost:3000/api/fhir/Observation?code=CUSTOM-TEMP-001" | jq '.total'
```

**Expected response:**
```json
1
```

```bash
# Get only humidity observations
curl "http://localhost:3000/api/fhir/Observation?code=CUSTOM-HUM-001" | jq '.total'
```

**Expected response:**
```json
1
```

✅ **Success:** Returns 1 for each observation type  
❌ **Failure:** Check LOINC codes in database

---

### Step 10: Test FHIR API - Get Single Observation

```bash
# First, get an observation ID from database
FHIR_ID=$(sudo -u postgres psql -t -d sleep_monitor -c "SELECT fhir_id FROM fhir_observations WHERE loinc_code = 'CUSTOM-TEMP-001' LIMIT 1;" | tr -d ' ')

# Then fetch it via API
curl "http://localhost:3000/api/fhir/Observation/$FHIR_ID" | jq '.code.text, .valueQuantity.value'
```

**Expected response:**
```json
"Temperature"
22.5
```

✅ **Success:** Returns specific observation  
❌ **Failure:** Check observation ID exists

---

### Step 11: Send Multiple Test Data Points

```bash
# Send 5 readings with different values
for i in {1..5}; do
  TEMP=$(echo "scale=1; 20 + $i * 0.5" | bc)
  HUM=$(echo "scale=1; 40 + $i * 2" | bc)
  SOUND=$(echo "scale=1; 30 + $i * 3" | bc)
  
  curl -s -X POST http://localhost:3000/api/sensor-data \
    -H "Content-Type: application/json" \
    -d "{
      \"temp\": $TEMP,
      \"hum\": $HUM,
      \"motion\": false,
      \"sound_db\": $SOUND,
      \"deviceid\": \"pi-001\",
      \"timestamp\": \"2024-12-26T15:3${i}:00Z\"
    }"
  
  echo "Sent reading $i (temp: ${TEMP}°C)"
  sleep 0.5
done
```

**Verify counts:**
```bash
# Should have 6 sensor readings total (1 + 5)
sudo -u postgres psql -t -d sleep_monitor -c "SELECT COUNT(*) FROM sensor_readings;"
# Expected: 6

# Should have 24 FHIR observations (6 * 4)
sudo -u postgres psql -t -d sleep_monitor -c "SELECT COUNT(*) FROM fhir_observations;"
# Expected: 24
```

✅ **Success:** Correct counts (1:4 ratio)  
❌ **Failure:** Check FHIR conversion is running

---

### Step 12: Verify Data Consistency

```bash
# Check if every sensor reading has exactly 4 FHIR observations
sudo -u postgres psql -d sleep_monitor -c "
  SELECT 
    sr.id as sensor_reading_id,
    COUNT(fo.id) as fhir_obs_count
  FROM sensor_readings sr
  LEFT JOIN fhir_observations fo ON sr.id = fo.sensor_reading_id
  GROUP BY sr.id
  ORDER BY sr.created_at DESC;
"
```

**Expected output:**
```
           sensor_reading_id            | fhir_obs_count 
----------------------------------------+----------------
 123e4567-e89b-12d3-a456-426614174000  |              4
 234e5678-e89b-12d3-a456-426614174001  |              4
 345e6789-e89b-12d3-a456-426614174002  |              4
 ...
```

✅ **Success:** Every reading has exactly 4 observations  
❌ **Failure:** FHIR conversion may have failed for some readings

---

## Troubleshooting

### Backend Won't Start

**Problem:** Database connection error

**Solution:**
```bash
# Check PostgreSQL is running
sudo systemctl status postgresql

# Check DATABASE_URL is set
echo $DATABASE_URL

# Test connection manually
psql -U postgres -d sleep_monitor -c "SELECT 1;"
```

---

### No FHIR Observations Created

**Problem:** sensor_readings has data, but fhir_observations is empty

**Check logs:**
```bash
RUST_LOG=debug cargo run
# Look for "Converting to FHIR observations..."
```

**Check database constraint:**
```bash
sudo -u postgres psql -d sleep_monitor -c "\d fhir_observations"
# Verify foreign key to sensor_readings exists
```

---

### API Returns 404

**Problem:** FHIR API endpoint not found

**Check routes:**
```bash
# In src/routes/mod.rs, verify:
.route("/api/fhir/Observation/:id", get(get_observation_by_id))
.route("/api/fhir/Observation", get(search_observations))
```

**Test with curl verbose:**
```bash
curl -v "http://localhost:3000/api/fhir/Observation?_count=1"
```

---

### Empty Bundle Response

**Problem:** API returns `{"total": 0, "entry": []}`

**Check data exists:**
```bash
sudo -u postgres psql -d sleep_monitor -c "SELECT COUNT(*) FROM fhir_observations;"
```

**Check filters:**
```bash
# Try without filters
curl "http://localhost:3000/api/fhir/Observation?_count=100"
```

---

## Success Criteria

**Branch 2A is working correctly if:**

✅ Backend starts without errors  
✅ Health check returns "healthy"  
✅ Sensor data is stored in sensor_readings table  
✅ **4 FHIR observations are created per sensor reading**  
✅ FHIR observations are stored in fhir_observations table  
✅ GET /api/fhir/Observation returns Bundle  
✅ Search filters work (patient, code, _count)  
✅ GET /api/fhir/Observation/:id returns single observation  
✅ Data ratio is 1:4 (readings:observations)  

---

## Next Steps After Testing

Once Branch 2A is verified:

1. **Move to Branch 2B (ML Processing)**
   - Implement sleep quality scoring
   - Add ML results API endpoints
   
2. **Update Frontend**
   - (Optional) Display FHIR data
   - Add ML results visualization

3. **Production Deployment**
   - Add authentication
   - Performance optimization
   - Monitoring setup

---

**Ready to test? Run `./test_branch_2a.sh`** 🧪
