# ML Sleep Quality Analyzer

Machine Learning service for analyzing sleep quality based on environmental sensor data.

---

## Status

**Infrastructure:** Complete  
**Implementation:** FULLY OPERATIONAL  
**Model:** Trained Random Forest (`random_forest_sleep_score.pkl`)  
**Script:** Production-ready Python pipeline (319 lines)

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

### Phase 1: Data Access (Complete)
- Database tables created
- API endpoints implemented
- Direct database access available

### Phase 2: ML Model (Complete)
- Feature engineering (6 features)
- Model training (Random Forest)
- Model evaluation and tuning
- Model deployment (`random_forest_sleep_score.pkl`)

### Phase 3: Analysis Pipeline (Complete)
- Scheduled analysis (nightly at 8 AM via backend scheduler)
- Batch processing (daily historical analysis)
- Database integration (reads/writes PostgreSQL)
- Error handling and logging
- Real-time scoring (optional - not implemented)

---

## Requirements

### Installed Dependencies

**Python Libraries:**
```bash
# Already installed in ml/venv
pip install psycopg2-binary numpy pandas scikit-learn joblib
```

**Model Details:**
- Algorithm: Random Forest Classifier
- Features: 6 engineered environmental factors
- Output: Sleep quality score (0-100) + classification
- File: `random_forest_sleep_score.pkl` (joblib format)

**Analysis Factors:**
- Temperature (optimal: 15-19°C, currently measures variance)
- Humidity (optimal: 30-50%)
- Sound level (optimal: <30 dB, tracks peaks >70dB)
- Motion events (optimal: <40 per night)

**Output:**
- Sleep quality score (0-100)
- Classification: "Good" (≥60) or "Poor" (<60)
- Based on Pittsburgh Sleep Quality Index (PSQI) methodology
- Environmental statistics (avg temp, humidity, sound, motion count)

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

## Current Implementation

The ML service is **fully operational** with the following components:

### File: `sleep_score_ml.py`
- 319 lines of production-ready Python code
- Loads trained Random Forest model from `random_forest_sleep_score.pkl`
- Connects to PostgreSQL database
- Reads sensor data for specified date
- Performs feature engineering (6 features)
- Predicts sleep quality score
- Classifies as "Good" or "Poor"
- Writes results to `sleep_records` table
- Logs processing status to `ml_processing_log`
- Comprehensive error handling
- Scientific documentation

### Scheduling
The backend Rust service automatically runs this script daily at 8:00 AM using the `scheduler` module.

### Usage
```bash
# Manual execution (analyzes yesterday's data)
cd ml
source venv/bin/activate  # or .\venv\Scripts\activate on Windows
python sleep_score_ml.py
```

**Output Example:**
```
Loading trained model...
Model loaded successfully
Database connection established
Analyzing sleep data for pi-001 on 2026-01-11
Retrieved 8640 sensor readings from database

Feature Summary:
  Average Temperature: 22.3 C (optimal: 15-19 C)
  Temperature Variance: 3.2 C (optimal: <2 C)
  Average Sound: 35.4 dB (optimal: <30 dB)
  Sound Peaks >70dB: 2 (optimal: 0)
  Total Motion Events: 45 (optimal: <40 per night)
  Average Humidity: 48.2% (optimal: 30-50%)

Predicted Sleep Quality Score: 58.7 / 100
Classification: Poor
Interpretation: Environmental factors may be disrupting sleep quality

Results written to sleep_records table
Processing status logged to ml_processing_log table
Analysis completed successfully
```

---

**Status:** Fully Operational - All 3 branches complete  
**Last Updated:** January 12, 2026
