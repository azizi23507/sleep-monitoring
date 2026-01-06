-- Add fhir_observations table for Branch 2A
-- Stores FHIR R4 Observation resources converted from sensor readings

CREATE TABLE IF NOT EXISTS fhir_observations (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Link to original sensor reading
    sensor_reading_id UUID NOT NULL REFERENCES sensor_readings(id) ON DELETE CASCADE,
    
    -- FHIR resource data (JSON format)
    resource_type VARCHAR(50) NOT NULL DEFAULT 'Observation',
    resource_data JSONB NOT NULL,
    
    -- FHIR identifiers
    fhir_id VARCHAR(100) UNIQUE NOT NULL,      -- FHIR logical ID
    patient_id VARCHAR(100),                   -- Patient reference
    
    -- LOINC codes for categorization
    loinc_code VARCHAR(20),                    -- e.g., "8310-5" for body temp
    observation_category VARCHAR(50),          -- e.g., "vital-signs"
    
    -- Metadata
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_fhir_obs_fhir_id ON fhir_observations(fhir_id);
CREATE INDEX IF NOT EXISTS idx_fhir_obs_patient ON fhir_observations(patient_id);
CREATE INDEX IF NOT EXISTS idx_fhir_obs_loinc ON fhir_observations(loinc_code);
CREATE INDEX IF NOT EXISTS idx_fhir_obs_sensor_reading ON fhir_observations(sensor_reading_id);

-- Add comment
COMMENT ON TABLE fhir_observations IS 'FHIR R4 Observation resources for healthcare interoperability (Branch 2A)';
