# API Endpoint Testing Script

## Overview
`test_endpoints.sh` is an automated testing script that verifies all backend API endpoints are working correctly.

## What It Tests

### 1. Health Check
- **Endpoint:** `GET /health`
- **Purpose:** Verify server is running
- **Expected:** JSON with status, uptime, database, and Redis connection

### 2. JWT Authentication
- **Endpoint:** `POST /api/auth/token`
- **Purpose:** Generate JWT token for authentication
- **Expected:** Token with 24-hour expiration

### 3. Sensor Data Ingestion (Valid)
- **Endpoint:** `POST /api/sensor-data`
- **Purpose:** Test data ingestion with valid sensor readings
- **Auth:** Required (Bearer token)
- **Expected:** Success response

### 4. Input Validation (Invalid)
- **Endpoint:** `POST /api/sensor-data`
- **Purpose:** Test validation rejects out-of-range values
- **Expected:** 400 Bad Request with validation error

### 5. ML Results Query
- **Endpoint:** `GET /api/sleep-records`
- **Purpose:** Retrieve sleep analysis results
- **Auth:** Required
- **Expected:** List of sleep records

### 6. ML Results by Date
- **Endpoint:** `GET /api/sleep-records/{date}`
- **Purpose:** Get specific date's sleep analysis
- **Auth:** Required
- **Expected:** Single record or empty

### 7. FHIR API Query
- **Endpoint:** `GET /api/fhir/Observation?patient=...`
- **Purpose:** Test FHIR compliance
- **Expected:** FHIR Bundle with observations

### 8. Authentication Required
- **Endpoint:** `GET /api/sleep-records` (no token)
- **Purpose:** Verify auth middleware works
- **Expected:** 401 Unauthorized

### 9. Database Schema
- **Purpose:** Verify all PostgreSQL tables exist
- **Expected:** 4 tables (sensor_readings, fhir_observations, sleep_records, ml_processing_log)

## Prerequisites

- Backend server running: `cargo run`
- PostgreSQL running with `sleep_monitor` database
- Redis running (for WebSocket/caching)

## Usage

```bash
cd backend
./test_endpoints.sh
```

## Expected Output

```
=========================================
Sleep Monitoring API Endpoint Tests
=========================================

1. Testing Health Endpoint...
✓ Health: {"status":"healthy",...}

2. Testing JWT Token Generation...
✓ Token received: eyJ0eXAiOiJKV1QiLCJhbGc...

3. Testing Sensor Data Ingestion (Valid Data)...
✓ Response: {"status":"ok","message":"Data received..."}

4. Testing Validation (Invalid Temperature)...
✓ Validation Error (expected): Temperature out of range...

5. Testing ML Results Endpoint...
✓ ML Results: {"total":0,"records":[]}

6. Testing ML Results by Date...
✓ ML Results for 2026-01-06: {...}

7. Testing FHIR API...
✓ FHIR Response: {"resourceType":"Bundle",...}

8. Testing Authentication (No Token)...
✓ No Auth Response: Unauthorized...

9. Verifying Database Schema...
✓ Checking PostgreSQL tables...
  fhir_observations
  ml_processing_log
  sensor_readings
  sleep_records

=========================================
All Tests Complete!
=========================================
```

## What Success Looks Like

- ✅ All 9 tests pass
- ✅ Health endpoint returns "healthy"
- ✅ JWT token generated successfully
- ✅ Valid data accepted
- ✅ Invalid data rejected with proper error
- ✅ Authentication enforced on protected endpoints
- ✅ Database tables exist and are accessible

## Troubleshooting

### Token Extraction Fails
**Issue:** Token not parsed from JSON response  
**Fix:** Ensure `grep -oP` or `sed` is available in your shell

### Authentication Errors
**Issue:** "Unauthorized: Invalid Authorization header format"  
**Cause:** Token variable empty  
**Fix:** Check token extraction logic

### Database Errors
**Issue:** "Database error" or "Connection refused"  
**Cause:** PostgreSQL not running or wrong credentials  
**Fix:** Start PostgreSQL and verify DATABASE_URL

### Redis Errors
**Issue:** "Redis connection failed"  
**Cause:** Redis not running  
**Fix:** Start Redis: `redis-server`

## Related Scripts

- **generate_test_data.sh** - Continuously sends random sensor data (Pi simulator)
- **cargo test** - Runs Rust unit tests
- **.env.example** - Environment configuration template

## Notes

- Tests use device_id: `test-device-001`
- Default base URL: `http://localhost:3000/api`
- Tests are non-destructive (safe to run anytime)
- Run with backend active to see real-time logs

## Last Updated
January 6, 2026
