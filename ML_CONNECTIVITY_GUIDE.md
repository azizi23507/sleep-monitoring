# ML Model - Database Connection Guide

Quick guide for connecting your ML analysis script to the backend's PostgreSQL database.

---

## Network Setup

**ML script runs on the SAME computer as the backend!**

No network configuration needed - uses `localhost`.

---

## Database Connection

### Connection Information

```python
# ==============================================
# DATABASE CONNECTION CONFIGURATION
# ==============================================

import psycopg2

# Database connection parameters
DB_CONFIG = {
    'host': 'localhost',        # Same machine as backend
    'port': 5432,               # PostgreSQL default port
    'database': 'sleep_monitor',  # Database name
    'user': 'postgres',         # PostgreSQL username
    'password': 'password'      # CHANGE THIS to your PostgreSQL password!
}
```

---

## Connecting to Database

### Connect Function

```python
def connect_to_database():
    """Connect to PostgreSQL database (no authentication needed - direct access)"""
    try:
        # Establish connection
        conn = psycopg2.connect(**DB_CONFIG)
        print("✅ Connected to database")
        return conn
        
    except Exception as e:
        print(f"❌ Connection failed: {e}")
        return None

# Example usage
conn = connect_to_database()
cursor = conn.cursor()
```

**Note:** No JWT token needed - ML uses direct database access!

---

## Reading Data from Backend's Tables

### Read Sensor Data (from backend's table)

```python
from datetime import date

def get_sensor_data(device_id, analysis_date):
    """Read sensor readings from backend's sensor_readings table"""
    
    conn = connect_to_database()
    cursor = conn.cursor()
    
    # Query backend's sensor_readings table
    cursor.execute("""
        SELECT 
            temperature,
            humidity,
            sound_level,
            motion_detected,
            reading_timestamp
        FROM sensor_readings
        WHERE device_id = %s
        AND reading_timestamp::date = %s
        ORDER BY reading_timestamp
    """, (device_id, analysis_date))
    
    # Fetch all rows
    rows = cursor.fetchall()
    
    cursor.close()
    conn.close()
    
    print(f"✅ Retrieved {len(rows)} sensor readings")
    return rows

# Example: Get yesterday's data
from datetime import timedelta
yesterday = date.today() - timedelta(days=1)
data = get_sensor_data('pi-001', yesterday)
```

---

## Writing Results to Backend's Tables

### Write Sleep Analysis Results

```python
def save_sleep_quality(device_id, analysis_date, score, classification):
    """Write sleep quality results to backend's sleep_records table"""
    
    conn = connect_to_database()
    cursor = conn.cursor()
    
    # Insert into backend's sleep_records table
    cursor.execute("""
        INSERT INTO sleep_records
        (device_id, date, sleep_quality_score, classification, created_at)
        VALUES (%s, %s, %s, %s, NOW())
    """, (device_id, analysis_date, score, classification))
    
    # Commit changes
    conn.commit()
    
    cursor.close()
    conn.close()
    
    print(f"✅ Sleep quality saved: {score}/100 ({classification})")

# Example: Save results
save_sleep_quality('pi-001', yesterday, 85.5, 'Good')
```

### Log Processing Status

```python
def log_processing(device_id, processing_date, status, records_count):
    """Log ML processing status to ml_processing_log table"""
    
    conn = connect_to_database()
    cursor = conn.cursor()
    
    # Insert into ml_processing_log table
    cursor.execute("""
        INSERT INTO ml_processing_log
        (device_id, processing_date, status, records_processed, 
         started_at, completed_at)
        VALUES (%s, %s, %s, %s, NOW(), NOW())
    """, (device_id, processing_date, status, records_count))
    
    conn.commit()
    
    cursor.close()
    conn.close()
    
    print(f"✅ Processing logged: {status}")

# Example: Log success
log_processing('pi-001', yesterday, 'success', 8640)
```

---

## Complete ML Analysis Flow

```python
import psycopg2
from datetime import date, timedelta

# Configuration
DB_CONFIG = {
    'host': 'localhost',
    'database': 'sleep_monitor',
    'user': 'postgres',
    'password': 'password'  # CHANGE THIS!
}
DEVICE_ID = 'pi-001'

def analyze_sleep():
    """Complete ML analysis flow"""
    
    # Step 1: Connect to database
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()
    print("✅ Connected to database")
    
    # Step 2: Read sensor data from backend's table
    yesterday = date.today() - timedelta(days=1)
    
    cursor.execute("""
        SELECT temperature, humidity, sound_level, motion_detected
        FROM sensor_readings
        WHERE device_id = %s AND reading_timestamp::date = %s
    """, (DEVICE_ID, yesterday))
    
    readings = cursor.fetchall()
    print(f"✅ Retrieved {len(readings)} readings")
    
    # Step 3: Analyze sleep quality (your ML algorithm here)
    sleep_score = 85.5  # Your calculation
    classification = 'Good'  # Your classification
    print(f"✅ Analysis complete: {sleep_score}/100")
    
    # Step 4: Write results to backend's tables
    cursor.execute("""
        INSERT INTO sleep_records
        (device_id, date, sleep_quality_score, classification)
        VALUES (%s, %s, %s, %s)
    """, (DEVICE_ID, yesterday, sleep_score, classification))
    
    cursor.execute("""
        INSERT INTO ml_processing_log
        (device_id, processing_date, status, records_processed, 
         started_at, completed_at)
        VALUES (%s, %s, %s, %s, NOW(), NOW())
    """, (DEVICE_ID, yesterday, 'success', len(readings)))
    
    # Step 5: Commit and close
    conn.commit()
    cursor.close()
    conn.close()
    print("✅ Results saved to database")

# Run analysis
analyze_sleep()
```

---

## Database Tables Used

### Tables You READ FROM:
```sql
-- sensor_readings (created by backend when Pi sends data)
SELECT * FROM sensor_readings WHERE device_id = 'pi-001';
```

### Tables You WRITE TO:
```sql
-- sleep_records (your ML results)
INSERT INTO sleep_records (device_id, date, sleep_quality_score, classification) 
VALUES ('pi-001', '2024-12-30', 85.5, 'Good');

-- ml_processing_log (your processing history)
INSERT INTO ml_processing_log (device_id, processing_date, status, records_processed)
VALUES ('pi-001', '2024-12-30', 'success', 8640);
```

---

## Testing Database Connection

### Test 1: Check Connection
```python
import psycopg2

try:
    conn = psycopg2.connect(
        host='localhost',
        database='sleep_monitor',
        user='postgres',
        password='password'
    )
    print("✅ Connection successful")
    conn.close()
except Exception as e:
    print(f"❌ Connection failed: {e}")
```

### Test 2: Check Data Exists
```python
conn = psycopg2.connect(**DB_CONFIG)
cursor = conn.cursor()

# Count sensor readings
cursor.execute("SELECT COUNT(*) FROM sensor_readings WHERE device_id = 'pi-001'")
count = cursor.fetchone()[0]
print(f"Sensor readings: {count}")

cursor.close()
conn.close()
```

### Test 3: Write Test Record
```python
conn = psycopg2.connect(**DB_CONFIG)
cursor = conn.cursor()

# Insert test record
cursor.execute("""
    INSERT INTO sleep_records
    (device_id, date, sleep_quality_score, classification)
    VALUES ('pi-001', '2024-12-30', 85.0, 'Good')
""")

conn.commit()
cursor.close()
conn.close()
print("✅ Test record inserted")
```

---

## Scheduling (Run Nightly at 8 AM)

### Using Cron (Linux/macOS)

```bash
# Edit crontab
crontab -e

# Add this line (runs at 8:00 AM every day)
0 8 * * * /usr/bin/python3 /path/to/your/ml_script.py
```

### Using Windows Task Scheduler

1. Open Task Scheduler
2. Create Task
3. Trigger: Daily at 8:00 AM
4. Action: Run Python script

---

## Troubleshooting

### Problem: Connection Refused
**Solution:**
- ✅ PostgreSQL is running: `sudo systemctl status postgresql`
- ✅ Database exists: `psql -U postgres -l`

### Problem: Authentication Failed
**Solution:**
- ✅ Correct password in `DB_CONFIG`
- ✅ User `postgres` has access to database

### Problem: Table Does Not Exist
**Solution:**
- ✅ Backend migrations ran successfully
- ✅ Check tables: `psql -U postgres -d sleep_monitor -c "\dt"`

### Problem: No Sensor Data Found
**Solution:**
- ✅ Pi is sending data to backend
- ✅ Check: `SELECT COUNT(*) FROM sensor_readings;`

---

## Summary

```
ML Script Configuration:
├── Connection: Direct to PostgreSQL (localhost)
├── Authentication: Database user/password (NO JWT token)
├── Reads from: sensor_readings table (backend's table)
├── Writes to: sleep_records, ml_processing_log tables
└── Runs: Daily at 8:00 AM (scheduled)
```

**Key Difference from Pi:**
- Pi → Backend API (HTTP with JWT)
- ML → Database (Direct with PostgreSQL password)

---

## Complete Minimal Example

```python
import psycopg2
from datetime import date, timedelta

# Database connection
conn = psycopg2.connect(
    host='localhost',
    database='sleep_monitor',
    user='postgres',
    password='password'  # CHANGE THIS!
)
cursor = conn.cursor()

# Read data
yesterday = date.today() - timedelta(days=1)
cursor.execute("""
    SELECT temperature, humidity, sound_level, motion_detected
    FROM sensor_readings
    WHERE device_id = 'pi-001' AND reading_timestamp::date = %s
""", (yesterday,))
readings = cursor.fetchall()

# Analyze (your algorithm)
sleep_score = 85.0
classification = 'Good'

# Write results
cursor.execute("""
    INSERT INTO sleep_records
    (device_id, date, sleep_quality_score, classification)
    VALUES ('pi-001', %s, %s, %s)
""", (yesterday, sleep_score, classification))

cursor.execute("""
    INSERT INTO ml_processing_log
    (device_id, processing_date, status, records_processed, started_at, completed_at)
    VALUES ('pi-001', %s, 'success', %s, NOW(), NOW())
""", (yesterday, len(readings)))

# Save and close
conn.commit()
cursor.close()
conn.close()
```

---

**That's it! Focus on:**
1. ✅ Set correct PostgreSQL password
2. ✅ Read from `sensor_readings` table
3. ✅ Write to `sleep_records` and `ml_processing_log` tables
4. ✅ No JWT token needed (direct database access)
