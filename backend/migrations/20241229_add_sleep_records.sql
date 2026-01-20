-- Add sleep_records table for Branch 2B (ML Processing)
-- Stores ML-analyzed sleep quality records with scores and classifications

CREATE TABLE IF NOT EXISTS sleep_records (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Device and patient info
    device_id VARCHAR(50) NOT NULL,
    patient_id VARCHAR(100),
    
    -- Time period analyzed
    sleep_date DATE NOT NULL,                  -- Night of sleep (YYYY-MM-DD)
    analysis_start TIMESTAMPTZ NOT NULL,       -- Start of analysis period
    analysis_end TIMESTAMPTZ NOT NULL,         -- End of analysis period
    
    -- Sleep quality metrics
    sleep_duration_hours DECIMAL(4, 2) NOT NULL,  -- Total sleep hours (e.g., 7.50)
    quality_score INTEGER NOT NULL,            -- 0-100 score
    classification VARCHAR(20) NOT NULL,       -- "Good" or "Poor"
    
    -- Environmental statistics
    avg_temperature DECIMAL(5, 2),
    temp_variance DECIMAL(5, 2),
    avg_humidity DECIMAL(5, 2),
    avg_sound_level DECIMAL(5, 2),
    sound_peaks_count INTEGER,                 -- Count of sounds >70dB
    motion_events_count INTEGER,               -- Total motion detections
    
    -- Detailed analysis (optional JSON)
    analysis_details JSONB,                    -- Additional ML insights
    
    -- Metadata
    analyzed_at TIMESTAMPTZ DEFAULT NOW(),     -- When ML ran
    ml_model_version VARCHAR(20),              -- Model version used
    
    -- Constraints
    CONSTRAINT valid_score CHECK (quality_score BETWEEN 0 AND 100),
    CONSTRAINT valid_classification CHECK (classification IN ('Good', 'Poor')),
    CONSTRAINT unique_device_date UNIQUE (device_id, sleep_date)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_sleep_records_device ON sleep_records(device_id);
CREATE INDEX IF NOT EXISTS idx_sleep_records_date ON sleep_records(sleep_date DESC);
CREATE INDEX IF NOT EXISTS idx_sleep_records_patient ON sleep_records(patient_id);

-- Add comment
COMMENT ON TABLE sleep_records IS 'ML-analyzed sleep quality records with scores and classifications (Branch 2B)';
