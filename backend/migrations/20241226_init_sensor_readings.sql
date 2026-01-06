-- Initial migration: Create sensor_readings table
-- This is the foundation for both Branch 2A (FHIR) and Branch 2B (ML)

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create sensor_readings table
CREATE TABLE IF NOT EXISTS sensor_readings (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Device information
    device_id VARCHAR(50) NOT NULL,
    
    -- Sensor measurements
    temperature DECIMAL(5, 2) NOT NULL,
    humidity DECIMAL(5, 2) NOT NULL,
    sound_level DECIMAL(5, 2) NOT NULL,
    motion_detected BOOLEAN NOT NULL,
    
    -- Timing
    reading_timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT valid_temperature CHECK (temperature BETWEEN -50 AND 50),
    CONSTRAINT valid_humidity CHECK (humidity BETWEEN 0 AND 100),
    CONSTRAINT valid_sound CHECK (sound_level BETWEEN 0 AND 120)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_sensor_readings_device ON sensor_readings(device_id);
CREATE INDEX IF NOT EXISTS idx_sensor_readings_timestamp ON sensor_readings(reading_timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_sensor_readings_device_time ON sensor_readings(device_id, reading_timestamp DESC);

-- Add comment
COMMENT ON TABLE sensor_readings IS 'Raw sensor data from Arduino/Pi devices - foundation for FHIR and ML processing';
