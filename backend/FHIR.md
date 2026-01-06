# FHIR Implementation Guide (Branch 2A)

This document explains the FHIR R4 implementation for the Sleep Monitoring System.

## What is FHIR?

**FHIR** (Fast Healthcare Interoperability Resources) is a standard for exchanging healthcare information electronically.

**Purpose in our system:**
- Enable interoperability with external hospital systems
- Standardized data format for healthcare integration
- Compliant with healthcare IT requirements

**Specification:** FHIR R4 (https://www.hl7.org/fhir/)

---

## Architecture

### Data Flow

```
Pi → Backend → PostgreSQL (sensor_readings)
                      ↓
              FHIR Converter
                      ↓
              PostgreSQL (fhir_observations)
                      ↓
              FHIR API → External Systems
```

### Conversion Process

**For each sensor reading, 4 FHIR Observations are created:**

1. **Temperature** (ambient-temperature)
2. **Humidity** (relative-humidity)
3. **Sound Level** (sound-level-db)
4. **Motion Detection** (motion-detected)

---

## FHIR Resources

### Observation Resource Structure

```json
{
  "resourceType": "Observation",
  "id": "obs-abc123-temp",
  "status": "final",
  "category": [{
    "coding": [{
      "system": "http://terminology.hl7.org/CodeSystem/observation-category",
      "code": "vital-signs",
      "display": "Vital Signs"
    }],
    "text": "Vital Signs"
  }],
  "code": {
    "coding": [{
      "system": "http://loinc.org",
      "code": "CUSTOM-TEMP-001",
      "display": "Ambient Temperature"
    }],
    "text": "Temperature"
  },
  "subject": {
    "reference": "Device/pi-001",
    "display": "Sleep Monitor Device"
  },
  "effectiveDateTime": "2024-12-26T15:30:00Z",
  "valueQuantity": {
    "value": 22.5,
    "unit": "°C",
    "system": "http://unitsofmeasure.org",
    "code": "Cel"
  },
  "device": {
    "reference": "Device/pi-001",
    "display": "Arduino/Raspberry Pi Sensor"
  }
}
```

---

## LOINC Codes

**LOINC** (Logical Observation Identifiers Names and Codes) provides standardized codes for observations.

### Codes Used

| Observation | LOINC Code | Category | Value Type |
|-------------|------------|----------|------------|
| Temperature | CUSTOM-TEMP-001 | vital-signs | Quantity (°C) |
| Humidity | CUSTOM-HUM-001 | environment | Quantity (%) |
| Sound Level | CUSTOM-SOUND-001 | environment | Quantity (dB) |
| Motion | CUSTOM-MOTION-001 | activity | Boolean |

**Note:** We use custom LOINC codes (CUSTOM-*) because standard LOINC doesn't have codes for environmental sensors. In production, you could:
1. Apply for official LOINC codes
2. Use system-specific codes
3. Use the closest standard LOINC equivalents

---

## API Endpoints

### 1. Get Observation by ID

**GET /api/fhir/Observation/:id**

Retrieve a single FHIR Observation by its logical ID.

**Example:**
```bash
curl http://localhost:3000/api/fhir/Observation/obs-123e4567-temp
```

**Response (200 OK):**
```json
{
  "resourceType": "Observation",
  "id": "obs-123e4567-temp",
  "status": "final",
  "code": { ... },
  "valueQuantity": { ... }
}
```

**Response (404 Not Found):**
```json
{
  "error": "Observation not found: obs-invalid-id",
  "status": 404
}
```

---

### 2. Search Observations

**GET /api/fhir/Observation**

Search for observations with optional filters.

**Query Parameters:**

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| patient | string | Device/Patient reference | `Device/pi-001` |
| code | string | LOINC code filter | `CUSTOM-TEMP-001` |
| date | string | ISO 8601 date | `2024-12-26` |
| _count | integer | Results limit (max: 100) | `50` |

**Examples:**

```bash
# Get all observations for a device
curl "http://localhost:3000/api/fhir/Observation?patient=Device/pi-001"

# Get only temperature observations
curl "http://localhost:3000/api/fhir/Observation?code=CUSTOM-TEMP-001"

# Combine filters
curl "http://localhost:3000/api/fhir/Observation?patient=Device/pi-001&code=CUSTOM-TEMP-001&_count=10"

# Get recent observations
curl "http://localhost:3000/api/fhir/Observation?_count=20"
```

**Response (200 OK):**
```json
{
  "resourceType": "Bundle",
  "type": "searchset",
  "total": 40,
  "entry": [
    {
      "resource": {
        "resourceType": "Observation",
        "id": "obs-123-temp",
        ...
      }
    },
    {
      "resource": {
        "resourceType": "Observation",
        "id": "obs-123-hum",
        ...
      }
    }
  ]
}
```

---

## Database Schema

### fhir_observations Table

```sql
CREATE TABLE fhir_observations (
    id UUID PRIMARY KEY,
    sensor_reading_id UUID REFERENCES sensor_readings(id),
    resource_type VARCHAR(50) DEFAULT 'Observation',
    resource_data JSONB NOT NULL,           -- Full FHIR resource
    fhir_id VARCHAR(100) UNIQUE NOT NULL,   -- Searchable ID
    patient_id VARCHAR(100),                -- Device reference
    loinc_code VARCHAR(20),                 -- For filtering
    observation_category VARCHAR(50),       -- vital-signs, environment, etc.
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Indexes

- `idx_fhir_obs_fhir_id` - Fast lookup by FHIR ID
- `idx_fhir_obs_patient` - Filter by patient/device
- `idx_fhir_obs_loinc` - Filter by LOINC code
- `idx_fhir_obs_sensor_reading` - Link back to original reading

---

## Testing

### 1. Send Sensor Data

```bash
curl -X POST http://localhost:3000/api/sensor-data \
  -H "Content-Type: application/json" \
  -d '{
    "temp": 22.5,
    "hum": 45.0,
    "motion": false,
    "sound_db": 35.2,
    "deviceid": "pi-001",
    "timestamp": "2024-12-26T15:30:00Z"
  }'
```

This automatically:
1. Stores in Redis (Branch 1)
2. Stores in PostgreSQL sensor_readings
3. Converts to 4 FHIR Observations
4. Stores in fhir_observations table

### 2. Verify Database

```sql
-- Check sensor reading
SELECT * FROM sensor_readings ORDER BY created_at DESC LIMIT 1;

-- Check FHIR observations (should be 4 for each sensor reading)
SELECT 
    fhir_id,
    loinc_code,
    observation_category,
    resource_data->'valueQuantity'->>'value' as value
FROM fhir_observations
ORDER BY created_at DESC
LIMIT 4;
```

### 3. Test FHIR API

```bash
# Get all recent observations
curl http://localhost:3000/api/fhir/Observation?_count=10 | jq '.'

# Get specific observation (use ID from database)
curl http://localhost:3000/api/fhir/Observation/obs-<UUID>-temp | jq '.'

# Filter by device
curl "http://localhost:3000/api/fhir/Observation?patient=Device/pi-001" | jq '.total'
```

---

## Integration with External Systems

### How External Systems Access Data

External hospital systems can:

1. **Query by Patient/Device**
   ```
   GET /api/fhir/Observation?patient=Device/pi-001
   ```

2. **Query by Type**
   ```
   GET /api/fhir/Observation?code=CUSTOM-TEMP-001
   ```

3. **Get Specific Observation**
   ```
   GET /api/fhir/Observation/obs-123-temp
   ```

### Authentication (TODO - Production)

For production deployment, add:
- OAuth 2.0 authentication
- API keys for external systems
- Rate limiting per client
- Audit logging of access

---

## FHIR Compliance

### What We Implement

✅ **Resource Structure:** FHIR R4 Observation  
✅ **Data Types:** Quantity, Boolean, CodeableConcept  
✅ **Categories:** vital-signs, environment, activity  
✅ **References:** Device references  
✅ **Timestamps:** ISO 8601 effectiveDateTime  
✅ **Bundle:** Searchset bundles for queries  

### What's Simplified

⚠️ **LOINC Codes:** Using custom codes (production should apply for official)  
⚠️ **Patient References:** Using Device instead of Patient  
⚠️ **Authentication:** Not implemented (required for production)  
⚠️ **CapabilityStatement:** Not provided (describes server capabilities)  
⚠️ **Validation:** Basic validation (production should use FHIR validators)

---

## Performance Considerations

### Query Optimization

```sql
-- Indexes speed up common queries
CREATE INDEX idx_fhir_obs_fhir_id ON fhir_observations(fhir_id);          -- Single observation lookup
CREATE INDEX idx_fhir_obs_patient ON fhir_observations(patient_id);       -- Filter by device
CREATE INDEX idx_fhir_obs_loinc ON fhir_observations(loinc_code);         -- Filter by type
```

### Storage Impact

**For each sensor reading:**
- sensor_readings: ~100 bytes
- fhir_observations: ~2 KB (4 observations × ~500 bytes each)

**Daily storage (1 reading/second):**
- 86,400 readings/day
- ~8.6 MB sensor data
- ~170 MB FHIR observations

**Mitigation:**
- Archive old data after 90 days
- Compress historical FHIR resources
- Use JSONB in PostgreSQL (efficient storage)

---

## Troubleshooting

### FHIR Observations Not Created

**Check logs:**
```bash
RUST_LOG=debug cargo run
# Look for "Converting to FHIR observations..."
```

**Check database:**
```sql
-- Count observations per sensor reading
SELECT 
    sensor_reading_id,
    COUNT(*) as obs_count
FROM fhir_observations
GROUP BY sensor_reading_id
ORDER BY sensor_reading_id DESC
LIMIT 10;

-- Should be 4 observations per reading
```

### API Returns Empty Bundle

**Check filters:**
```bash
# Remove filters to get all observations
curl "http://localhost:3000/api/fhir/Observation?_count=100"
```

**Check data exists:**
```sql
SELECT COUNT(*) FROM fhir_observations;
```

---

## Next Steps (Production)

1. **Apply for Official LOINC Codes**
   - https://loinc.org
   - Submit custom observations

2. **Add Patient Resources**
   - Create Patient resources (not just Device)
   - Link observations to actual patients

3. **Implement Authentication**
   - OAuth 2.0 / JWT
   - API keys for systems
   - Rate limiting

4. **Add CapabilityStatement**
   - Describe server capabilities
   - Document supported resources
   - FHIR conformance

5. **Validation**
   - Use FHIR validator
   - Ensure compliance
   - Test with FHIR servers

---

## References

- [FHIR R4 Specification](https://www.hl7.org/fhir/)
- [FHIR Observation Resource](https://www.hl7.org/fhir/observation.html)
- [LOINC Database](https://loinc.org/)
- [UCUM Units](https://ucum.org/)
- [FHIR RESTful API](https://www.hl7.org/fhir/http.html)

---

**Branch 2A Status:** ✅ COMPLETE - FHIR conversion and API implemented!
