# ML Sleep Quality Analyzer

Machine Learning service for analyzing sleep quality based on environmental sensor data.

---

## Status

**Infrastructure:** ✅ Ready  
**Implementation:** 🚧 Pending External Delivery

---

## Overview

The ML service analyzes sensor data to generate sleep quality scores and classifications.

### Data Flow
```
sensor_readings (PostgreSQL)
    ↓
ML Analyzer (Python/TensorFlow)
    ↓
sleep_records (PostgreSQL)
ml_processing_log (PostgreSQL)
```

---

## Backend Support (Already Implemented)

### Database Tables
✅ **sensor_readings** - Raw sensor data from Pi devices  
✅ **sleep_records** - ML analysis results storage  
✅ **ml_processing_log** - Processing history and logs

### API Endpoints
✅ **GET /api/sleep-records** - Retrieve all sleep quality records  
✅ **GET /api/sleep-records/:date** - Get specific date analysis  
✅ **GET /api/sleep-quality/latest** - Most recent sleep quality score

### Database Schema

**sleep_records:**
```sql
- id (serial)
- device_id (varchar)
- date (date)
- sleep_quality_score (numeric) -- 0-100 score
- classification (varchar) -- 'Good', 'Fair', 'Poor'
- avg_temperature (numeric)
- avg_humidity (numeric)
- avg_sound_level (numeric)
- motion_events (integer)
- created_at (timestamp)
```

**ml_processing_log:**
```sql
- id (serial)
- device_id (varchar)
- processing_date (date)
- status (varchar) -- 'success', 'failed', 'running'
- records_processed (integer)
- error_message (text)
- started_at (timestamp)
- completed_at (timestamp)
```

---

## Integration Instructions

### For ML Team

See **ML_CONNECTIVITY_GUIDE.md** in root directory for complete integration instructions.

**Quick Summary:**

1. **Connect to PostgreSQL:**
   ```python
   import psycopg2
   
   conn = psycopg2.connect(
       host='localhost',
       database='sleep_monitor',
       user='postgres',
       password='password'
   )
   ```

2. **Read Sensor Data:**
   ```python
   cursor.execute("""
       SELECT temperature, humidity, sound_level, motion_detected
       FROM sensor_readings
       WHERE device_id = %s AND reading_timestamp::date = %s
   """, (device_id, analysis_date))
   ```

3. **Write Results:**
   ```python
   cursor.execute("""
       INSERT INTO sleep_records
       (device_id, date, sleep_quality_score, classification, ...)
       VALUES (%s, %s, %s, %s, ...)
   """, (device_id, date, score, classification, ...))
   ```

---

## Implementation Plan

### Phase 1: Data Access (Complete ✅)
- ✅ Database tables created
- ✅ API endpoints implemented
- ✅ Direct database access available

### Phase 2: ML Model (Pending)
- [ ] Feature engineering
- [ ] Model training
- [ ] Model evaluation
- [ ] Model deployment

### Phase 3: Analysis Pipeline (Pending)
- [ ] Scheduled analysis (nightly at 8 AM)
- [ ] Batch processing
- [ ] Real-time scoring (optional)

---

## Requirements

### When Implementing

**Python Libraries:**
```bash
pip install psycopg2-binary numpy pandas scikit-learn
```

**Analysis Factors:**
- Temperature (optimal: 18-22°C)
- Humidity (optimal: 40-60%)
- Sound level (optimal: <40 dB)
- Motion events (low = better sleep)

**Output:**
- Sleep quality score (0-100)
- Classification (Good/Fair/Poor)
- Environmental statistics

---

## Docker Integration

### Future Dockerfile (Template)

```dockerfile
FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY analyzer.py .

CMD ["python", "analyzer.py"]
```

### Add to docker-compose.yml

```yaml
ml:
  build: ./ml
  container_name: sleep-ml
  environment:
    DATABASE_URL: postgres://postgres:password@postgres:5432/sleep_monitor
  depends_on:
    - postgres
```

---

## Testing

### Test Data Available

The backend continuously collects sensor data. Sample queries:

```sql
-- Check available data
SELECT COUNT(*), MIN(reading_timestamp), MAX(reading_timestamp)
FROM sensor_readings;

-- Get one day's data
SELECT *
FROM sensor_readings
WHERE device_id = 'pi-001'
AND reading_timestamp::date = '2024-12-30'
ORDER BY reading_timestamp;
```

---

## Documentation

- **ML_CONNECTIVITY_GUIDE.md** - Complete integration guide
- **Backend README.md** - API documentation
- **Sleep_Quality_Classification_Standards.pdf** - Sleep quality criteria

---

## Example ML Script Structure

```python
#!/usr/bin/env python3
"""
Sleep Quality Analyzer
"""

import psycopg2
from datetime import date, timedelta

DB_CONFIG = {
    'host': 'localhost',
    'database': 'sleep_monitor',
    'user': 'postgres',
    'password': 'password'
}

def analyze_sleep(device_id, analysis_date):
    """Analyze sleep quality for given date"""
    
    # Connect to database
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()
    
    # Read sensor data
    cursor.execute("""
        SELECT temperature, humidity, sound_level, motion_detected
        FROM sensor_readings
        WHERE device_id = %s AND reading_timestamp::date = %s
    """, (device_id, analysis_date))
    
    readings = cursor.fetchall()
    
    # Analyze (implement your ML model here)
    sleep_score = calculate_score(readings)
    classification = classify_sleep(sleep_score)
    
    # Write results
    cursor.execute("""
        INSERT INTO sleep_records
        (device_id, date, sleep_quality_score, classification)
        VALUES (%s, %s, %s, %s)
    """, (device_id, analysis_date, sleep_score, classification))
    
    # Log processing
    cursor.execute("""
        INSERT INTO ml_processing_log
        (device_id, processing_date, status, records_processed,
         started_at, completed_at)
        VALUES (%s, %s, 'success', %s, NOW(), NOW())
    """, (device_id, analysis_date, len(readings)))
    
    conn.commit()
    conn.close()

def calculate_score(readings):
    """Your ML model implementation"""
    # TODO: Implement sleep quality scoring algorithm
    return 85.0

def classify_sleep(score):
    """Classify sleep quality"""
    if score >= 80:
        return 'Good'
    elif score >= 60:
        return 'Fair'
    else:
        return 'Poor'

if __name__ == "__main__":
    # Analyze yesterday's sleep
    yesterday = date.today() - timedelta(days=1)
    analyze_sleep('pi-001', yesterday)
```

---

## Contact & Support

For questions about ML integration:
1. Review **ML_CONNECTIVITY_GUIDE.md**
2. Check backend API documentation
3. Test with sample queries above

---

**Status:** Infrastructure ready, awaiting ML implementation  
**Last Updated:** January 10, 2026
