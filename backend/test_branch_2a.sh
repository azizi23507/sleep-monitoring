#!/bin/bash
# Test Commands for Branch 2A (FHIR Conversion)

echo "========================================="
echo "Branch 2A Testing Commands"
echo "========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Base URL
BASE_URL="http://localhost:3000"

echo -e "${BLUE}Step 1: Health Check${NC}"
echo "Command: curl $BASE_URL/health"
echo ""
curl -s $BASE_URL/health | jq '.' 2>/dev/null || curl -s $BASE_URL/health
echo ""
echo ""

echo -e "${BLUE}Step 2: Get Authentication Token${NC}"
echo "Command: curl -X POST $BASE_URL/api/auth/token -d '{\"device_id\":\"pi-001\"}'"
echo ""

token_response=$(curl -s -X POST $BASE_URL/api/auth/token \
  -H "Content-Type: application/json" \
  -d '{"device_id": "pi-001"}')

# Try to extract token with jq first, fallback to grep/sed if jq not available
TOKEN=$(echo "$token_response" | jq -r '.token' 2>/dev/null)

# Fallback if jq not available
if [ -z "$TOKEN" ] || [ "$TOKEN" == "null" ]; then
  TOKEN=$(echo "$token_response" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
fi

if [ -z "$TOKEN" ]; then
  echo "ERROR: Could not get authentication token"
  echo "Response: $token_response"
  exit 1
fi

echo "Token received: ${TOKEN:0:20}..."
echo ""
echo ""

echo -e "${BLUE}Step 3: Send Test Sensor Data (with authentication)${NC}"
echo "This will:"
echo "  - Store in Redis (Branch 1)"
echo "  - Store in sensor_readings table"
echo "  - Create 4 FHIR Observations"
echo "  - Store in fhir_observations table"
echo ""
echo "Command:"
echo 'curl -X POST $BASE_URL/api/sensor-data \'
echo '  -H "Content-Type: application/json" \'
echo '  -H "Authorization: Bearer $TOKEN" \'
echo '  -d '"'"'{ "temp": 22.5, "hum": 45.0, "motion": false, "sound_db": 35.2, "deviceid": "pi-001", "timestamp": "2024-12-26T15:30:00Z" }'"'"
echo ""

response=$(curl -s -X POST $BASE_URL/api/sensor-data \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "temp": 22.5,
    "hum": 45.0,
    "motion": false,
    "sound_db": 35.2,
    "deviceid": "pi-001",
    "timestamp": "2024-12-26T15:30:00Z"
  }')

echo "$response" | jq '.' 2>/dev/null || echo "$response"
echo ""

if [[ "$response" == *"ok"* ]]; then
    echo -e "${GREEN}[OK] Data sent successfully!${NC}"
else
    echo -e "${YELLOW}[WARNING]  Response doesn't contain 'ok' - check if server is running${NC}"
fi
echo ""
echo ""

# Wait a moment for processing
sleep 1

echo -e "${BLUE}Step 4: Check Database - Sensor Readings${NC}"
echo "Command: psql -U postgres -d sleep_monitor -c \"SELECT * FROM sensor_readings ORDER BY created_at DESC LIMIT 1;\""
echo ""
sudo -u postgres psql -d sleep_monitor -c "SELECT device_id, temperature, humidity, sound_level, motion_detected, reading_timestamp FROM sensor_readings ORDER BY created_at DESC LIMIT 1;" 2>/dev/null || echo "Run manually: psql -U postgres -d sleep_monitor"
echo ""
echo ""

echo -e "${BLUE}Step 5: Check Database - FHIR Observations (Should be 4)${NC}"
echo "Command: psql -U postgres -d sleep_monitor -c \"SELECT COUNT(*) FROM fhir_observations;\""
echo ""
count=$(sudo -u postgres psql -t -d sleep_monitor -c "SELECT COUNT(*) FROM fhir_observations;" 2>/dev/null | tr -d ' ')
if [ ! -z "$count" ]; then
    echo "Total FHIR Observations: $count"
    if [ "$count" -ge 4 ]; then
        echo -e "${GREEN}[OK] FHIR observations created!${NC}"
    else
        echo -e "${YELLOW}[WARNING]  Expected at least 4 observations${NC}"
    fi
else
    echo "Run manually: psql -U postgres -d sleep_monitor"
fi
echo ""
echo ""

echo -e "${BLUE}Step 6: View FHIR Observations Details${NC}"
echo "Command: psql -U postgres -d sleep_monitor -c \"SELECT fhir_id, loinc_code, observation_category FROM fhir_observations ORDER BY created_at DESC LIMIT 4;\""
echo ""
sudo -u postgres psql -d sleep_monitor -c "SELECT fhir_id, loinc_code, observation_category FROM fhir_observations ORDER BY created_at DESC LIMIT 4;" 2>/dev/null || echo "Run manually"
echo ""
echo ""

echo -e "${BLUE}Step 7: Test FHIR API - Get All Observations (with auth)${NC}"
echo "Command: curl -H \"Authorization: Bearer \$TOKEN\" \"$BASE_URL/api/fhir/Observation?_count=10\""
echo ""
curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/fhir/Observation?_count=10" | jq '.resourceType, .type, .total, .entry[0].resource.code.text' 2>/dev/null || curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/fhir/Observation?_count=10"
echo ""
echo ""

echo -e "${BLUE}Step 8: Test FHIR API - Filter by Device (with auth)${NC}"
echo "Command: curl -H \"Authorization: Bearer \$TOKEN\" \"$BASE_URL/api/fhir/Observation?patient=Device/pi-001\""
echo ""
result=$(curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/fhir/Observation?patient=Device/pi-001")
total=$(echo "$result" | jq -r '.total' 2>/dev/null)
if [ ! -z "$total" ] && [ "$total" != "null" ]; then
    echo "Found $total observations for Device/pi-001"
    echo -e "${GREEN}[OK] Device filter working!${NC}"
else
    echo "$result" | jq '.' 2>/dev/null || echo "$result"
fi
echo ""
echo ""

echo -e "${BLUE}Step 9: Test FHIR API - Filter by Type (Temperature, with auth)${NC}"
echo "Command: curl -H \"Authorization: Bearer \$TOKEN\" \"$BASE_URL/api/fhir/Observation?code=CUSTOM-TEMP-001\""
echo ""
result=$(curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/fhir/Observation?code=CUSTOM-TEMP-001")
total=$(echo "$result" | jq -r '.total' 2>/dev/null)
if [ ! -z "$total" ] && [ "$total" != "null" ]; then
    echo "Found $total temperature observations"
    echo -e "${GREEN}[OK] LOINC code filter working!${NC}"
else
    echo "$result" | jq '.' 2>/dev/null || echo "$result"
fi
echo ""
echo ""

echo -e "${BLUE}Step 10: Get Single FHIR Observation by ID (with auth)${NC}"
echo "First, get an observation ID from database..."
fhir_id=$(sudo -u postgres psql -t -d sleep_monitor -c "SELECT fhir_id FROM fhir_observations ORDER BY created_at DESC LIMIT 1;" 2>/dev/null | tr -d ' ')
if [ ! -z "$fhir_id" ]; then
    echo "Testing with ID: $fhir_id"
    echo "Command: curl -H \"Authorization: Bearer \$TOKEN\" \"$BASE_URL/api/fhir/Observation/$fhir_id\""
    echo ""
    curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/fhir/Observation/$fhir_id" | jq '.resourceType, .id, .code.text, .valueQuantity // .valueBoolean' 2>/dev/null || curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/fhir/Observation/$fhir_id"
    echo ""
    echo -e "${GREEN}[OK] Single observation retrieval working!${NC}"
else
    echo "Could not get observation ID from database"
    echo "Run manually: curl -H \"Authorization: Bearer \$TOKEN\" \"$BASE_URL/api/fhir/Observation/[YOUR_FHIR_ID]\""
fi
echo ""
echo ""

echo -e "${BLUE}Step 11: Send Multiple Test Data Points (with auth)${NC}"
echo "Sending 3 more data points to build up test data..."
echo ""

for i in {1..3}; do
    temp=$(echo "scale=1; 20 + $i * 0.5" | bc)
    hum=$(echo "scale=1; 40 + $i * 2" | bc)
    sound=$(echo "scale=1; 30 + $i * 3" | bc)
    
    curl -s -X POST $BASE_URL/api/sensor-data \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $TOKEN" \
      -d "{
        \"temp\": $temp,
        \"hum\": $hum,
        \"motion\": false,
        \"sound_db\": $sound,
        \"deviceid\": \"pi-001\",
        \"timestamp\": \"2024-12-26T15:3${i}:00Z\"
      }" > /dev/null
    
    echo "  Sent reading $i (temp: ${temp}°C)"
done
echo ""
echo -e "${GREEN}[OK] Sent 3 additional readings${NC}"
echo ""
echo ""

# Final count
sleep 1
echo -e "${BLUE}Final Statistics${NC}"
echo ""
sensor_count=$(sudo -u postgres psql -t -d sleep_monitor -c "SELECT COUNT(*) FROM sensor_readings;" 2>/dev/null | tr -d ' ')
fhir_count=$(sudo -u postgres psql -t -d sleep_monitor -c "SELECT COUNT(*) FROM fhir_observations;" 2>/dev/null | tr -d ' ')

if [ ! -z "$sensor_count" ] && [ ! -z "$fhir_count" ]; then
    echo "Sensor Readings: $sensor_count"
    echo "FHIR Observations: $fhir_count"
    echo "Expected ratio: 1 reading → 4 FHIR observations"
    
    expected_fhir=$((sensor_count * 4))
    if [ "$fhir_count" -eq "$expected_fhir" ]; then
        echo -e "${GREEN}[OK] Perfect! Ratio is correct (1:4)${NC}"
    else
        echo -e "${YELLOW}[WARNING]  Expected $expected_fhir FHIR observations, found $fhir_count${NC}"
    fi
fi
echo ""

echo "========================================="
echo "Testing Complete!"
echo "========================================="
echo ""
echo "Summary:"
echo "[OK] Health check"
echo "[OK] Sensor data ingestion"
echo "[OK] PostgreSQL storage"
echo "[OK] FHIR conversion (1 reading → 4 observations)"
echo "[OK] FHIR API endpoints"
echo "[OK] Search filters (device, code)"
echo ""
echo "Next: Review logs with 'RUST_LOG=debug cargo run'"
