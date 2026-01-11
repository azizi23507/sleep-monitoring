# Hardware Setup - Raspberry Pi Sleep Monitor

## Overview

This directory contains code for the Raspberry Pi sensor hardware that collects environmental data and sends it to the backend server.

## Hardware Components

### Required Sensors
- **DHT11** - Temperature & Humidity sensor
- **PIR Motion Sensor** - Detects movement
- **Sound Sensor Module** - Measures ambient noise (analog)
- **Arduino/ESP32** - Reads sensors and sends via serial to Pi

### Wiring Diagram

```
DHT11:
  - VCC → 5V
  - GND → GND
  - DATA → Pin 2

PIR Sensor:
  - VCC → 5V
  - GND → GND
  - OUT → Pin 3

Sound Sensor:
  - VCC → 5V
  - GND → GND
  - OUT → A0
```

## Files

### `temps_reel.ino`
Arduino sketch that reads sensors and outputs JSON via serial.

**Upload to Arduino:**
```bash
# Install Arduino IDE first
# Open temps_reel.ino
# Select board: Arduino Uno (or your board)
# Select port: /dev/ttyACM0 (or your port)
# Click Upload
```

### `real_time.py`
Python script for Raspberry Pi that:
1. Reads sensor data from Arduino via serial
2. Processes sound levels (converts ADC to decibels)
3. Authenticates with backend (JWT)
4. Sends data to backend API every second

## Setup Instructions

### 1. Install Dependencies

```bash
# On Raspberry Pi
sudo apt-get update
sudo apt-get install python3-pip python3-serial

# Install Python packages
pip3 install pyserial numpy requests
```

### 2. Configure Backend URL

Edit `real_time.py` and update line 6 with your computer's IP:

```python
BACKEND_URL = "http://YOUR_COMPUTER_IP:3000"
```

**Find your computer's IP:**
- Windows: `ipconfig` (look for IPv4 Address)
- Mac/Linux: `ifconfig` or `ip addr`

### 3. Check Serial Port

```bash
# List available serial ports
ls /dev/tty*

# Common ports:
# - /dev/ttyACM0 (Arduino Uno)
# - /dev/ttyUSB0 (ESP32, other USB serial)

# Update SERIAL_PORT in real_time.py if different
```

### 4. Run the Monitor

```bash
# Give permission to serial port (first time only)
sudo chmod 666 /dev/ttyACM0

# Run the script
python3 real_time.py

# Or run in background
nohup python3 real_time.py > sensor.log 2>&1 &
```

## Expected Output

```
Device ID: pi-001
Backend: http://192.168.137.1:3000
SLEEP MONITOR → Backend Real-time Data Stream
--------------------------------------------------
✓ JWT token obtained, expires in 86400s
T:22.3°C H:55.2% M:False S:42.5dB ✓ 200
T:22.4°C H:55.1% M:True S:45.8dB ✓ 200
T:22.3°C H:55.3% M:True S:48.2dB ✓ 200
```

## Troubleshooting

### "Permission denied: /dev/ttyACM0"
```bash
sudo chmod 666 /dev/ttyACM0
# Or add user to dialout group:
sudo usermod -a -G dialout $USER
# Then logout and login again
```

### "No module named 'serial'"
```bash
pip3 install pyserial
```

### "Connection refused"
- Check backend is running: `curl http://YOUR_IP:3000/health`
- Verify both devices on same network
- Check firewall allows port 3000
- Ping computer from Pi: `ping YOUR_COMPUTER_IP`

### "401 Unauthorized"
- JWT token expired or invalid
- Backend JWT_SECRET might have changed
- Script will auto-retry token acquisition

### "No data from Arduino"
- Check Arduino is powered and uploaded sketch
- Verify serial port: `cat /dev/ttyACM0` (should see JSON output)
- Check baud rate matches (9600)

## Data Format

### Arduino → Pi (Serial JSON)
```json
{
  "temp": 22.5,
  "hum": 55.0,
  "motion": true,
  "sound": 512
}
```

### Pi → Backend (HTTP POST)
```json
{
  "device_id": "pi-001",
  "temperature": 22.5,
  "humidity": 55.0,
  "sound_level": 45.2,
  "motion_detected": true
}
```

## Network Configuration

### Same WiFi Network
1. Connect Pi and computer to same WiFi
2. Find computer IP: `ipconfig` / `ifconfig`
3. Update `BACKEND_URL` in Python script
4. Ensure backend allows connections (0.0.0.0:3000)

### Hotspot Mode
1. Computer creates WiFi hotspot
2. Pi connects to hotspot
3. Use computer's hotspot IP (usually 192.168.137.1)

## Production Tips

- Use `systemd` service to auto-start on boot
- Log to file: `python3 script.py > /var/log/sleep-monitor.log 2>&1`
- Set static IP for Pi for consistent connection
- Consider USB power bank for portable setup

## Support

For issues:
1. Check backend health: `curl http://BACKEND_IP:3000/health`
2. Test authentication: `curl -X POST http://BACKEND_IP:3000/api/auth/token -H "Content-Type: application/json" -d '{"device_id":"pi-001"}'`
3. Check serial output: `cat /dev/ttyACM0`
4. Review logs: `tail -f sensor.log`

---

**Last Updated:** January 11, 2026
