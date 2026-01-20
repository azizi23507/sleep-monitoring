#!/bin/bash
# Send random sensor data to test the frontend graphs
# Mimics Raspberry Pi sending data

BASE_URL="http://localhost:3000/api"
DEVICE_ID="pi-001"

echo "========================================="
echo "Sensor Data Generator (Pi Simulator)"
echo "========================================="
echo ""

# Get JWT token
echo "Getting authentication token..."
TOKEN_RESPONSE=$(curl -s -X POST $BASE_URL/auth/token \
  -H "Content-Type: application/json" \
  -d "{\"device_id\": \"$DEVICE_ID\"}")

TOKEN=$(echo "$TOKEN_RESPONSE" | grep -oP '"token":"\K[^"]+' || echo "$TOKEN_RESPONSE" | sed -n 's/.*"token":"\([^"]*\)".*/\1/p')

if [ -z "$TOKEN" ]; then
    echo "ERROR: Failed to get token"
    echo "Response: $TOKEN_RESPONSE"
    exit 1
fi

echo "✓ Token received"
echo ""

# Send random data every 2 seconds
echo "Starting data stream (Press Ctrl+C to stop)..."
echo "Sending data for YESTERDAY's sleep window..."
echo ""

COUNT=1
while true; do
    # Generate BAD sensor data (poor sleep)
TEMP=$(awk -v min=26 -v max=30 'BEGIN{srand(); print min+rand()*(max-min)}')
HUMIDITY=$(awk -v min=70 -v max=85 'BEGIN{srand(); print min+rand()*(max-min)}')
SOUND=$(awk -v min=60 -v max=80 'BEGIN{srand(); print min+rand()*(max-min)}')
MOTION=1
    
    # Hardcoded yesterday date for testing (adjust if needed)
    YESTERDAY_DATE="2026-01-06"
    HOUR=$((20 + RANDOM % 13))
    if [ $HOUR -ge 24 ]; then
        HOUR=$((HOUR - 24))
        YESTERDAY_DATE="2026-01-07"
    fi
    TIMESTAMP="${YESTERDAY_DATE}T$(printf %02d $HOUR):$(date +%M:%S)Z"
    
    # Send to backend
    RESPONSE=$(curl -s -X POST $BASE_URL/sensor-data \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $TOKEN" \
      -d "{
        \"temp\": $TEMP,
        \"hum\": $HUMIDITY,
        \"motion\": $([ $MOTION -eq 1 ] && echo "true" || echo "false"),
        \"sound_db\": $SOUND,
        \"deviceid\": \"$DEVICE_ID\",
        \"timestamp\": \"$TIMESTAMP\"
      }")
    
    # Check response
    if echo "$RESPONSE" | grep -q "ok\|success"; then
        echo "[$COUNT] $TIMESTAMP | ${TEMP}°C, ${HUMIDITY}%, ${SOUND}dB, Motion: $MOTION ✓"
    else
        echo "[$COUNT] ERROR: $RESPONSE"
    fi
    
    COUNT=$((COUNT + 1))
    sleep 2
done
