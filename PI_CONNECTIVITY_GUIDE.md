# Raspberry Pi - Backend Connection Guide

Quick guide for connecting your Raspberry Pi sensor client to the Sleep Monitoring Backend.

---

## Network Setup

### 1. Connect Pi and Computer to Same WiFi

**Options:**
- Use phone WiFi hotspot (recommended)
- Use home WiFi router
- Direct connection via Ethernet

**Both devices MUST be on the same network!**

---

## Find Backend Server IP Address

### On Your Computer (where backend runs):

**Windows:**
```cmd
ipconfig
```
Look for: `IPv4 Address` → Example: `192.168.43.100`

**Linux/WSL:**
```bash
ip addr show
```
Look for: `inet` under `wlan0` → Example: `192.168.43.100`

**macOS:**
```bash
ifconfig en0 | grep inet
```

**Save this IP address!**

---

## Backend Connection Code (Pi Side)

### Configuration
```python
# ==============================================
# BACKEND CONNECTION CONFIGURATION
# ==============================================

# Step 1: Set backend URL (CHANGE THIS to your computer's IP!)
BACKEND_URL = "http://192.168.43.100:3000"  # Replace with YOUR computer IP

# Step 2: Set device identifier
DEVICE_ID = "pi-001"  # Unique ID for this Pi device
```

---

## Authentication (Required for Pi)

### Step 1: Get JWT Token (Once at Startup)

```python
import requests

# Get authentication token from backend
def get_auth_token():
    """Request JWT token from backend"""
    try:
        response = requests.post(
            f"{BACKEND_URL}/api/auth/token",
            json={"device_id": DEVICE_ID},
            timeout=10
        )
        
        if response.status_code == 200:
            data = response.json()
            token = data["token"]
            print(f"✅ Token received")
            return token
        else:
            print(f"❌ Authentication failed: {response.status_code}")
            return None
            
    except Exception as e:
        print(f"❌ Connection error: {e}")
        return None

# Get token at startup
TOKEN = get_auth_token()
```

---

### Step 2: Send Data with Token

```python
def send_sensor_data(sensor_data):
    """Send sensor readings to backend with authentication"""
    try:
        # Prepare headers with authentication token
        headers = {
            "Content-Type": "application/json",
            "Authorization": f"Bearer {TOKEN}"  # Include JWT token
        }
        
        # Send POST request to backend
        response = requests.post(
            f"{BACKEND_URL}/api/sensor-data",
            json=sensor_data,
            headers=headers,
            timeout=5
        )
        
        if response.status_code == 200:
            print(f"✅ Data sent successfully")
            return True
        elif response.status_code == 401:
            print(f"⚠️ Token expired - need to refresh")
            return False
        else:
            print(f"❌ Failed: {response.status_code}")
            return True
            
    except Exception as e:
        print(f"❌ Connection error: {e}")
        return True

# Example: Send data every second
import time

while True:
    # Prepare sensor data (your sensor reading code here)
    data = {
        "temp": 22.5,           # Temperature in Celsius
        "hum": 45.0,            # Humidity percentage
        "motion": False,        # Motion detected (True/False)
        "sound_db": 35.2,       # Sound level in decibels
        "deviceid": DEVICE_ID,  # Device identifier
        "timestamp": "2024-12-30T10:00:00Z"  # ISO 8601 format
    }
    
    # Send to backend
    send_sensor_data(data)
    
    # Wait 1 second before next reading
    time.sleep(1)
```

---

## Data Format Requirements

### Request Body (JSON)
```json
{
  "temp": 22.5,          // Float: -50 to 50°C
  "hum": 45.0,           // Float: 0 to 100%
  "motion": false,       // Boolean: true/false
  "sound_db": 35.2,      // Float: 0 to 120 dB
  "deviceid": "pi-001",  // String: your device ID
  "timestamp": "2024-12-30T10:00:00Z"  // ISO 8601 format
}
```

### Response Codes
- `200 OK` - Data accepted
- `400 Bad Request` - Validation failed (check data ranges)
- `401 Unauthorized` - Invalid/expired token (refresh token)

---

## Testing Connection

### Test 1: Check Backend Health (No Auth)
```python
response = requests.get(f"{BACKEND_URL}/health")
print(response.json())
# Expected: {"status": "healthy", ...}
```

### Test 2: Get Token
```python
response = requests.post(
    f"{BACKEND_URL}/api/auth/token",
    json={"device_id": "pi-001"}
)
print(response.json())
# Expected: {"token": "eyJhbGci...", "expires_in": 86400}
```

### Test 3: Send Data
```python
headers = {"Authorization": f"Bearer {TOKEN}"}
data = {
    "temp": 22.5,
    "hum": 45.0,
    "motion": False,
    "sound_db": 35.2,
    "deviceid": "pi-001",
    "timestamp": "2024-12-30T10:00:00Z"
}
response = requests.post(
    f"{BACKEND_URL}/api/sensor-data",
    json=data,
    headers=headers
)
print(response.status_code)
# Expected: 200
```

---

## Troubleshooting

### Problem: Connection Refused
**Solution:**
- ✅ Backend is running on computer
- ✅ Pi and computer on same WiFi
- ✅ Use computer's IP, not `localhost`
- ✅ Port 3000 is not blocked by firewall

### Problem: 401 Unauthorized
**Solution:**
- Get new token (tokens expire after 24 hours)
- Check `JWT_SECRET` is set on backend

### Problem: 400 Bad Request
**Solution:**
- Check data ranges:
  - Temperature: -50 to 50°C
  - Humidity: 0 to 100%
  - Sound: 0 to 120 dB

---

## Summary

```
Pi Configuration:
├── Backend URL: http://YOUR_COMPUTER_IP:3000
├── Device ID: pi-001
├── Authentication: JWT token (get once, use for 24h)
├── Endpoint: POST /api/sensor-data
└── Frequency: Every 1 second
```

**Important:** Replace `192.168.43.100` with YOUR actual computer IP!

---

## Complete Minimal Example

```python
import requests
import time

# Configuration
BACKEND_URL = "http://192.168.43.100:3000"  # CHANGE THIS!
DEVICE_ID = "pi-001"

# Get token
response = requests.post(f"{BACKEND_URL}/api/auth/token", 
                        json={"device_id": DEVICE_ID})
TOKEN = response.json()["token"]

# Send data loop
while True:
    data = {
        "temp": 22.5,
        "hum": 45.0,
        "motion": False,
        "sound_db": 35.2,
        "deviceid": DEVICE_ID,
        "timestamp": "2024-12-30T10:00:00Z"
    }
    
    requests.post(f"{BACKEND_URL}/api/sensor-data",
                 json=data,
                 headers={"Authorization": f"Bearer {TOKEN}"})
    
    time.sleep(1)
```

---

**That's it! Focus on:**
1. ✅ Get computer's IP address
2. ✅ Update `BACKEND_URL` in code
3. ✅ Get authentication token
4. ✅ Send data with token in headers
