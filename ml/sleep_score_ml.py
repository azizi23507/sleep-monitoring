#!/usr/bin/env python3
"""
Sleep Quality Analyzer - Machine Learning Based
================================================

This script uses a pre-trained Random Forest model to predict sleep quality scores
based on environmental sensor data. The classification thresholds are derived from
established sleep research and clinical guidelines.

Scientific Foundation:
- Sleep Quality Classification Standards (2024)
- Pittsburgh Sleep Quality Index (PSQI) methodology
- WHO Guidelines for Community Noise
- Thermal environment effects on sleep research

Setup:
1. Place your trained model file 'random_forest_sleep_score.pkl' in the ml/ directory
2. Ensure PostgreSQL database is running with sensor data
3. The backend scheduler will run this script daily at 8:00 AM

Author: Sleep Monitoring Team
Date: January 2026
"""

import pandas as pd
import psycopg2
import joblib
import os
from datetime import datetime, date, timedelta

# ====================================================================================
# CONFIGURATION
# ====================================================================================

# Database connection parameters
# In Docker: connects to 'postgres' service
# In development: connects to localhost
import os
DB_CONFIG = {
    'host': os.getenv('DB_HOST', 'localhost'),
    'port': 5432,
    'database': 'sleep_monitor',
    'user': 'postgres',
    'password': 'password'
}

# Device identifier for this analysis
# Each Raspberry Pi device should have a unique ID
DEVICE_ID = 'pi-001'

# Path to the pre-trained machine learning model
# Place the model file in the ml/ directory
MODEL_PATH = "random_forest_sleep_score.pkl"

# Initialize status tracking variables
status = "success"
error_message = ""
nb_records = 0
predicted_score = None
classification = None

try:
    # ====================================================================================
    # STEP 1: LOAD PRE-TRAINED MODEL
    # ====================================================================================
    
    # Load the trained Random Forest model from disk
    # The model was trained using scikit-learn and saved with joblib
    # This model contains learned patterns from historical sleep data
    print("Loading trained model...")
    
    # Get absolute path to model file (works in both Docker and dev environments)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    model_path = os.path.join(script_dir, MODEL_PATH)
    
    model = joblib.load(model_path)
    print(f"Model loaded successfully from {model_path}")

    # ====================================================================================
    # STEP 2: ESTABLISH DATABASE CONNECTION
    # ====================================================================================
    
    # Connect to PostgreSQL database where sensor readings are stored
    # The backend continuously writes sensor data to the sensor_readings table
    conn = psycopg2.connect(**DB_CONFIG)
    cursor = conn.cursor()
    print("Database connection established")

    # ====================================================================================
    # STEP 3: READ SENSOR DATA FROM DATABASE
    # ====================================================================================
    
    # Analyze yesterday's sleep data
    # We analyze the previous day's complete data to ensure all night readings are captured
    analysis_date = date.today() - timedelta(days=1)
    
    print(f"Analyzing sleep data for {DEVICE_ID} on {analysis_date}")

    # Query to retrieve all sensor readings for the target date
    # Data is ordered chronologically to preserve time-series patterns
    cursor.execute("""
        SELECT 
            temperature,
            humidity,
            sound_level,
            motion_detected
        FROM sensor_readings
        WHERE device_id = %s
        AND reading_timestamp::date = %s
        ORDER BY reading_timestamp
    """, (DEVICE_ID, analysis_date))

    rows = cursor.fetchall()
    nb_records = len(rows)

    # Validate that we have sensor data for the specified date
    if nb_records == 0:
        raise ValueError(f"No sensor data found for device {DEVICE_ID} on {analysis_date}")

    print(f"Retrieved {nb_records} sensor readings from database")

    # Convert query results to pandas DataFrame for easier feature engineering
    # Column names match the model's expected input features
    data = pd.DataFrame(rows, columns=[
        "temp_c",
        "humidity",
        "sound_db",
        "motion"
    ])

    # ====================================================================================
    # STEP 4: FEATURE ENGINEERING
    # ====================================================================================
    
    # Calculate statistical features from raw sensor readings
    # These features capture both average conditions and variability patterns
    # that are important indicators of sleep quality
    
    # Temperature features
    # Research shows optimal sleep temperature is 15-19C with minimal variance
    avg_temp = data["temp_c"].mean()
    temp_variance = data["temp_c"].max() - data["temp_c"].min()
    
    # Sound features
    # WHO guidelines indicate <30 dB is optimal for sleep
    # Peak sounds >70 dB can cause sleep disruption
    avg_sound = data["sound_db"].mean()
    sound_peaks = (data["sound_db"] > 70).sum()
    
    # Motion features
    # Actigraphy research shows <5 movements per hour indicates good sleep
    # Excessive motion suggests sleep fragmentation
    total_motion = data["motion"].sum()
    
    # Humidity features
    # Optimal humidity range is 30-50% for respiratory comfort
    avg_humidity = data["humidity"].mean()

    # Create feature DataFrame matching the model's training format
    # Feature order and naming must match exactly what the model expects
    features = pd.DataFrame([{
        "avg_temp": avg_temp,
        "temp_variance": temp_variance,
        "avg_sound": avg_sound,
        "sound_peaks": sound_peaks,
        "total_motion": total_motion,
        "avg_humidity": avg_humidity
    }])

    print("\nFeature Summary:")
    print(f"  Average Temperature: {avg_temp:.1f} C (optimal: 15-19 C)")
    print(f"  Temperature Variance: {temp_variance:.1f} C (optimal: <2 C)")
    print(f"  Average Sound: {avg_sound:.1f} dB (optimal: <30 dB)")
    print(f"  Sound Peaks >70dB: {sound_peaks} (optimal: 0)")
    print(f"  Total Motion Events: {total_motion} (optimal: <40 per night)")
    print(f"  Average Humidity: {avg_humidity:.1f}% (optimal: 30-50%)")

    # ====================================================================================
    # STEP 5: PREDICT SLEEP QUALITY SCORE
    # ====================================================================================
    
    # Use the trained Random Forest model to predict sleep quality
    # The model outputs a continuous score based on learned patterns
    predicted_score = model.predict(features)[0]
    
    # Ensure the score is within valid range [0, 100]
    # Some models may output values slightly outside this range
    predicted_score = max(0, min(100, float(predicted_score)))

    print(f"\nPredicted Sleep Quality Score: {predicted_score:.1f} / 100")

    # ====================================================================================
    # STEP 6: CLASSIFY SLEEP QUALITY
    # ====================================================================================
    
    # Apply classification thresholds based on Sleep Quality Classification Standards
    # 
    # These thresholds are derived from:
    # - Pittsburgh Sleep Quality Index (PSQI): score <5 indicates good sleep
    # - Clinical research on sleep environment optimization
    # - The 60-point threshold represents meeting 60% of optimal conditions
    #
    # Classification Rationale:
    # - Score >= 60: "Good Sleep"
    #   Environmental conditions meet research-based standards for quality sleep.
    #   This threshold aligns with PSQI clinical standards where scores below 5
    #   (normalized to our 0-100 scale) indicate good sleep quality.
    #
    # - Score < 60: "Poor Sleep"
    #   Environmental conditions fall below acceptable thresholds.
    #   Indicates need for environment optimization or further clinical evaluation.
    #
    # Note: This is a screening tool based on environmental factors only.
    # It does not replace polysomnography or clinical sleep assessment.
    # Results should inform environmental optimization discussions with healthcare providers.
    
    if predicted_score >= 60:
        classification = "Good"
        interpretation = "Environmental conditions support quality sleep"
    else:
        classification = "Poor"
        interpretation = "Environmental factors may be disrupting sleep quality"

    print(f"Classification: {classification}")
    print(f"Interpretation: {interpretation}")

    # ====================================================================================
    # STEP 7: WRITE RESULTS TO DATABASE
    # ====================================================================================
    
    # Store the analysis results in the sleep_records table
    # This table maintains the historical record of sleep quality assessments
    # The backend API can query this table to display results to users
    cursor.execute("""
        INSERT INTO sleep_records
        (device_id, sleep_date, quality_score, classification, 
         avg_temperature, avg_humidity, avg_sound_level, motion_events_count,
         analysis_start, analysis_end, analyzed_at)
        VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, NOW())
    """, (DEVICE_ID, analysis_date, int(predicted_score), classification,
          float(avg_temp), float(avg_humidity), float(avg_sound), int(total_motion),
          analysis_date, analysis_date))  # Using same date for start/end for simplicity

    print("Results written to sleep_records table")

    # ====================================================================================
    # STEP 8: LOG PROCESSING STATUS
    # ====================================================================================
    
    # Record processing metadata in ml_processing_log table
    # This provides an audit trail of ML operations for monitoring and debugging
    cursor.execute("""
        INSERT INTO ml_processing_log
        (device_id, sleep_date, status, readings_processed, 
         started_at, completed_at)
        VALUES (%s, %s, %s, %s, NOW(), NOW())
    """, (DEVICE_ID, analysis_date, status, nb_records))

    print("Processing status logged to ml_processing_log table")

    # Commit all database changes
    # This ensures data integrity by making all inserts atomic
    conn.commit()
    print("\nAnalysis completed successfully")
    print(f"Processed {nb_records} sensor readings")
    print(f"Final score: {predicted_score:.1f}/100 ({classification})")

except FileNotFoundError as e:
    # Handle missing model file error specifically
    status = "failure"
    error_message = f"Model file not found: {MODEL_PATH}. Please ensure the trained model exists."
    print(f"\nERROR: {error_message}")
    
except ValueError as e:
    # Handle data validation errors (e.g., no sensor data for date)
    status = "failure"
    error_message = str(e)
    print(f"\nERROR: {error_message}")
    
except Exception as e:
    # Catch any other unexpected errors
    status = "failure"
    error_message = f"Unexpected error during processing: {str(e)}"
    print(f"\nERROR: {error_message}")

    # Attempt to log the error to the database for troubleshooting
    # This helps diagnose issues even when the main analysis fails
    try:
        cursor.execute("""
            INSERT INTO ml_processing_log
            (device_id, sleep_date, status, readings_processed, 
             error_message, started_at, completed_at)
            VALUES (%s, %s, %s, %s, %s, NOW(), NOW())
        """, (DEVICE_ID, analysis_date, "failure", nb_records, error_message))
        conn.commit()
        print("Error logged to database for review")
    except:
        print("Unable to log error to database")

finally:
    # ====================================================================================
    # STEP 9: CLEANUP
    # ====================================================================================
    
    # Always close database connections to prevent resource leaks
    # This ensures the connection pool remains healthy for other processes
    try:
        cursor.close()
        conn.close()
        print("\nDatabase connection closed")
    except:
        pass

# ====================================================================================
# MAIN EXECUTION
# ====================================================================================

# This script is designed to be run as a scheduled job (e.g., via cron)
# Recommended schedule: Daily at 8:00 AM to analyze previous night's sleep
# Example cron entry: 0 8 * * * /usr/bin/python3 /path/to/sleep_score_ml.py
