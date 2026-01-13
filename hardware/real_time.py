import serial, json, time, numpy as np, requests
from collections import deque
from datetime import datetime
from config import BACKEND_URL, DEVICE_ID

# Serial port configuration
SERIAL_PORT = '/dev/ttyACM0'
BAUD_RATE = 9600

ser = serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=1)
sound_buffer = deque(maxlen=10)
jwt_token = None
token_expires_at = 0

def get_jwt_token():
    """Get JWT token from backend for authentication"""
    try:
        response = requests.post(
            f"{BACKEND_URL}/api/auth/token",
            json={"device_id": DEVICE_ID},
            timeout=5
        )
        if response.status_code == 200:
            data = response.json()
            print(f"✓ JWT token obtained, expires in {data.get('expires_in', 86400)}s")
            return data['token'], time.time() + data.get('expires_in', 86400)
        else:
            print(f"✗ Failed to get token: {response.status_code}")
            return None, 0
    except Exception as e:
        print(f"✗ Token request error: {e}")
        return None, 0

def process_sound(raw_adc):
    """Convert raw ADC value to decibels"""
    sound_buffer.append(raw_adc)
    rms = np.sqrt(np.mean(np.square(list(sound_buffer))))
    db = 20 * np.log10(max(rms, 1)) + 50
    return max(0, min(120, db))

print(f"Device ID: {DEVICE_ID}")
print(f"Backend: {BACKEND_URL}")
print("SLEEP MONITOR → Backend Real-time Data Stream")
print("-" * 50)

# Get initial JWT token
jwt_token, token_expires_at = get_jwt_token()

while True:
    try:
        # Refresh token if expired
        if time.time() >= token_expires_at - 300:
            print("Token expiring soon, refreshing...")
            jwt_token, token_expires_at = get_jwt_token()
        
        if not jwt_token:
            print("[ERROR] No valid token, retrying in 5s...")
            time.sleep(5)
            jwt_token, token_expires_at = get_jwt_token()
            continue
        
        line = ser.readline().decode('utf-8').strip()
        if not line:
            continue

        data = json.loads(line)
        
        # Convert to backend expected format
        payload = {
            'deviceid': DEVICE_ID,
            'temp': data['temp'],
            'hum': data['hum'],
            'sound_db': process_sound(data['sound']),
            'motion': data['motion'],
            'timestamp': datetime.utcnow().isoformat() + 'Z'
        }

        print(f"T:{payload['temp']:.1f}°C H:{payload['hum']:.1f}% "
              f"M:{payload['motion']} S:{payload['sound_db']:.1f}dB", end=" ")

        # Send to backend with JWT authentication
        headers = {
            'Authorization': f'Bearer {jwt_token}',
            'Content-Type': 'application/json'
        }
        
        response = requests.post(
            f'{BACKEND_URL}/api/sensor-data',
            json=payload,
            headers=headers,
            timeout=2
        )
        
        if response.status_code == 200:
            print(f"[OK] {response.status_code}")
        else:
            print(f"[ERROR] {response.status_code}: {response.text}")

    except json.JSONDecodeError:
        print(f"[ERROR] Invalid JSON from Arduino: {line}")
    except requests.exceptions.Timeout:
        print("[ERROR] Backend timeout")
    except requests.exceptions.ConnectionError:
        print("[ERROR] Backend connection failed")
    except Exception as e:
        print(f"[ERROR] Error: {repr(e)}")
        time.sleep(1)