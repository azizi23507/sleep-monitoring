-- Add ml_processing_log table for Branch 2B (ML Processing)
-- Tracks ML processing runs - audit trail and error tracking

CREATE TABLE IF NOT EXISTS ml_processing_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Processing details
    device_id VARCHAR(50) NOT NULL,
    sleep_date DATE NOT NULL,
    
    -- Status
    status VARCHAR(20) NOT NULL,               -- "success", "failed", "running"
    error_message TEXT,
    
    -- Timing
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    duration_seconds INTEGER,
    
    -- Metrics
    readings_processed INTEGER,
    records_created INTEGER,
    
    -- Metadata
    ml_model_version VARCHAR(20),
    trigger_type VARCHAR(50)                   -- "scheduled", "manual", "api"
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_ml_log_device_date ON ml_processing_log(device_id, sleep_date DESC);
CREATE INDEX IF NOT EXISTS idx_ml_log_status ON ml_processing_log(status);

-- Add comment
COMMENT ON TABLE ml_processing_log IS 'Audit log for ML processing runs (Branch 2B)';
