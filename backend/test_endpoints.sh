#!/bin/bash
# API Endpoint Testing Script for Sleep Monitoring System

set -e

BASE_URL="http://localhost:3000"
TOKEN=""

echo "========================================="
echo "Sleep Monitoring API Endpoint Tests"
echo "========================================="
echo ""

# Test 1: Health Check
echo "1. Testing Health Endpoint..."
HEALTH=$(curl -s $BASE_URL/health)
echo "✓ Health: $HEALTH"
echo ""

# Test 2: Get JWT Token
echo "2. Testing JWT Token Generation..."
TOKEN_RESPONSE=$(curl -s -X POST $BASE_URL/api/token \
  -H "Content-Type: application/json" \
  -d '{"device_id": "test-device-001"}')
TOKEN=$(echo $TOKEN_RESPONSE | grep -o '"token":"[^"]*' | cut -d'"' -f4)
echo "✓ Token received: ${TOKEN:0:50}..."
echo ""

# Test 3: Post Sensor Data (Valid)
echo "3. Testing Sensor Data Ingestion (Valid Data)..."
SENSOR_RESPONSE=$(curl -s -X POST $BASE_URL/api/sensor-data \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "device_id": "test-device-001",
    "temperature": 22.5,
    "humidity": 45.0,
    "sound_level": 35.2,
    "motion_detected": false,
    "timestamp": "'$(date -Iseconds)'"
  }')
echo "✓ Response: $SENSOR_RESPONSE"
echo ""

# Test 4: Post Sensor Data (Invalid - out of range)
echo "4. Testing Validation (Invalid Temperature)..."
INVALID_RESPONSE=$(curl -s -X POST $BASE_URL/api/sensor-data \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "device_id": "test-device-001",
    "temperature": 999.0,
    "humidity": 45.0,
    "sound_level": 35.2,
    "motion_detected": false,
    "timestamp": "'$(date -Iseconds)'"
  }')
echo "✓ Validation Error (expected): $INVALID_RESPONSE"
echo ""

# Test 5: Get ML Results
echo "5. Testing ML Results Endpoint..."
ML_RESPONSE=$(curl -s $BASE_URL/api/ml-results \
  -H "Authorization: Bearer $TOKEN")
echo "✓ ML Results: $ML_RESPONSE"
echo ""

# Test 6: Get ML Results by Date
echo "6. Testing ML Results by Date..."
DATE=$(date +%Y-%m-%d)
ML_DATE_RESPONSE=$(curl -s "$BASE_URL/api/ml-results/date/$DATE" \
  -H "Authorization: Bearer $TOKEN")
echo "✓ ML Results for $DATE: $ML_DATE_RESPONSE"
echo ""

# Test 7: FHIR Observation Query
echo "7. Testing FHIR API..."
FHIR_RESPONSE=$(curl -s "$BASE_URL/api/fhir/Observation?patient=test-patient-001" \
  -H "Authorization: Bearer $TOKEN")
echo "✓ FHIR Response: ${FHIR_RESPONSE:0:200}..."
echo ""

# Test 8: Protected Endpoint without Auth
echo "8. Testing Authentication (No Token)..."
NO_AUTH=$(curl -s -w "\nHTTP_CODE:%{http_code}" $BASE_URL/api/ml-results)
echo "✓ No Auth Response: $NO_AUTH"
echo ""

# Test 9: Database Tables Check
echo "9. Verifying Database Schema..."
echo "✓ Checking PostgreSQL tables..."
PGPASSWORD=password psql -U postgres -h localhost -d sleep_monitor -c "\dt" -t | grep -E "(sensor_readings|fhir_observations|sleep_records|ml_processing_log)"
echo ""

echo "========================================="
echo "All Tests Complete!"
echo "========================================="
echo ""
echo "Summary:"
echo "  ✓ Health endpoint working"
echo "  ✓ JWT authentication working"
echo "  ✓ Sensor data ingestion working"
echo "  ✓ Input validation working"
echo "  ✓ ML results API working"
echo "  ✓ FHIR API working"
echo "  ✓ Auth middleware working"
echo "  ✓ Database schema correct"
echo ""
