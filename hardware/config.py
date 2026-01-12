"""
Hardware Configuration
======================

Edit these settings to match your setup:
1. Set BACKEND_URL to your computer's IP address
2. Set DEVICE_ID to identify this Raspberry Pi
3. Update credentials if you changed them

To find your computer's IP:
- Windows: ipconfig
- Linux/Mac: ifconfig or ip addr
"""

# Backend API Configuration
# Change localhost to your computer's IP address (e.g., "192.168.1.100")
BACKEND_URL = "http://localhost:3000"

# Device Configuration
DEVICE_ID = "pi-001"

# Authentication (matches docker-compose.yml defaults)
USERNAME = "pi_device"
PASSWORD = "secure_pi_password_2024"
