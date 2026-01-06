/// FHIR R4 Observation Resource Converter
/// 
/// This module converts sensor data to FHIR R4 Observation resources
/// for healthcare interoperability (Branch 2A).
/// 
/// FHIR (Fast Healthcare Interoperability Resources) is a standard
/// for exchanging healthcare information electronically.
/// 
/// Resources created:
/// - Temperature observations (LOINC: 8310-5)
/// - Humidity observations (custom code)
/// - Sound level observations (custom code)
/// - Motion detection observations (custom code)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::sensor_data::SensorData;
use crate::error::{ApiError, ApiResult};

/// FHIR R4 Observation Resource
/// 
/// Represents a single measurement or assertion about a patient.
/// 
/// Spec: https://www.hl7.org/fhir/observation.html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirObservation {
    /// Resource type (always "Observation")
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    
    /// Logical ID of the resource
    pub id: String,
    
    /// Status: registered | preliminary | final | amended
    pub status: String,
    
    /// Classification of observation type
    pub category: Vec<CodeableConcept>,
    
    /// Type of observation (LOINC code)
    pub code: CodeableConcept,
    
    /// Who/what observation is about (patient reference)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Reference>,
    
    /// Clinically relevant time/period
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: String,
    
    /// Actual result value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_quantity: Option<Quantity>,
    
    /// Boolean result (for motion detection)
    #[serde(skip_serializing_if = "Option::is_none", rename = "valueBoolean")]
    pub value_boolean: Option<bool>,
    
    /// Device that produced the observation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Reference>,
}

/// CodeableConcept: Concept defined by a coding system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableConcept {
    pub coding: Vec<Coding>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Coding: Reference to a code defined by a terminology system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coding {
    /// Identity of the terminology system
    pub system: String,
    
    /// Symbol in syntax defined by the system
    pub code: String,
    
    /// Representation defined by the system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

/// Reference: A reference from one resource to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Literal reference (e.g., "Patient/123")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    
    /// Text alternative for the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

/// Quantity: A measured amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    /// Numerical value
    pub value: f64,
    
    /// Unit representation (e.g., "°C", "dB")
    pub unit: String,
    
    /// System that defines the coded unit form
    pub system: String,
    
    /// Coded form of the unit (e.g., "Cel" for Celsius)
    pub code: String,
}

/// LOINC codes for common observations
pub mod loinc {
    /// Ambient temperature (custom - not standard LOINC)
    pub const AMBIENT_TEMPERATURE: &str = "CUSTOM-TEMP-001";
    
    /// Relative humidity (custom)
    pub const RELATIVE_HUMIDITY: &str = "CUSTOM-HUM-001";
    
    /// Sound level (custom)
    pub const SOUND_LEVEL: &str = "CUSTOM-SOUND-001";
    
    /// Motion detected (custom)
    pub const MOTION_DETECTED: &str = "CUSTOM-MOTION-001";
}

/// Convert sensor data to FHIR Observations
/// 
/// Creates 4 separate FHIR Observation resources:
/// 1. Temperature
/// 2. Humidity
/// 3. Sound level
/// 4. Motion detection
/// 
/// Each observation is a separate resource following FHIR R4 specification.
pub fn convert_to_fhir_observations(
    data: &SensorData,
    sensor_reading_id: Uuid,
) -> Vec<FhirObservation> {
    let mut observations = Vec::new();
    
    // Generate base FHIR ID prefix
    let base_id = format!("obs-{}", sensor_reading_id);
    
    // 1. Temperature Observation
    observations.push(create_temperature_observation(data, &base_id));
    
    // 2. Humidity Observation
    observations.push(create_humidity_observation(data, &base_id));
    
    // 3. Sound Level Observation
    observations.push(create_sound_observation(data, &base_id));
    
    // 4. Motion Detection Observation
    observations.push(create_motion_observation(data, &base_id));
    
    observations
}

/// Create temperature FHIR Observation
fn create_temperature_observation(data: &SensorData, base_id: &str) -> FhirObservation {
    FhirObservation {
        resource_type: "Observation".to_string(),
        id: format!("{}-temp", base_id),
        status: "final".to_string(),
        category: vec![CodeableConcept {
            coding: vec![Coding {
                system: "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
                code: "vital-signs".to_string(),
                display: Some("Vital Signs".to_string()),
            }],
            text: Some("Vital Signs".to_string()),
        }],
        code: CodeableConcept {
            coding: vec![Coding {
                system: "http://loinc.org".to_string(),
                code: loinc::AMBIENT_TEMPERATURE.to_string(),
                display: Some("Ambient Temperature".to_string()),
            }],
            text: Some("Temperature".to_string()),
        },
        subject: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Sleep Monitor Device".to_string()),
        }),
        effective_date_time: data.timestamp.clone(),
        value_quantity: Some(Quantity {
            value: data.temp as f64,
            unit: "°C".to_string(),
            system: "http://unitsofmeasure.org".to_string(),
            code: "Cel".to_string(),
        }),
        value_boolean: None,
        device: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Arduino/Raspberry Pi Sensor".to_string()),
        }),
    }
}

/// Create humidity FHIR Observation
fn create_humidity_observation(data: &SensorData, base_id: &str) -> FhirObservation {
    FhirObservation {
        resource_type: "Observation".to_string(),
        id: format!("{}-hum", base_id),
        status: "final".to_string(),
        category: vec![CodeableConcept {
            coding: vec![Coding {
                system: "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
                code: "environment".to_string(),
                display: Some("Environment".to_string()),
            }],
            text: Some("Environmental Observation".to_string()),
        }],
        code: CodeableConcept {
            coding: vec![Coding {
                system: "http://loinc.org".to_string(),
                code: loinc::RELATIVE_HUMIDITY.to_string(),
                display: Some("Relative Humidity".to_string()),
            }],
            text: Some("Humidity".to_string()),
        },
        subject: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Sleep Monitor Device".to_string()),
        }),
        effective_date_time: data.timestamp.clone(),
        value_quantity: Some(Quantity {
            value: data.hum as f64,
            unit: "%".to_string(),
            system: "http://unitsofmeasure.org".to_string(),
            code: "%".to_string(),
        }),
        value_boolean: None,
        device: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Arduino/Raspberry Pi Sensor".to_string()),
        }),
    }
}

/// Create sound level FHIR Observation
fn create_sound_observation(data: &SensorData, base_id: &str) -> FhirObservation {
    FhirObservation {
        resource_type: "Observation".to_string(),
        id: format!("{}-sound", base_id),
        status: "final".to_string(),
        category: vec![CodeableConcept {
            coding: vec![Coding {
                system: "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
                code: "environment".to_string(),
                display: Some("Environment".to_string()),
            }],
            text: Some("Environmental Observation".to_string()),
        }],
        code: CodeableConcept {
            coding: vec![Coding {
                system: "http://loinc.org".to_string(),
                code: loinc::SOUND_LEVEL.to_string(),
                display: Some("Sound Level".to_string()),
            }],
            text: Some("Sound Level".to_string()),
        },
        subject: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Sleep Monitor Device".to_string()),
        }),
        effective_date_time: data.timestamp.clone(),
        value_quantity: Some(Quantity {
            value: data.sound_db as f64,
            unit: "dB".to_string(),
            system: "http://unitsofmeasure.org".to_string(),
            code: "dB".to_string(),
        }),
        value_boolean: None,
        device: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Arduino/Raspberry Pi Sensor".to_string()),
        }),
    }
}

/// Create motion detection FHIR Observation
fn create_motion_observation(data: &SensorData, base_id: &str) -> FhirObservation {
    FhirObservation {
        resource_type: "Observation".to_string(),
        id: format!("{}-motion", base_id),
        status: "final".to_string(),
        category: vec![CodeableConcept {
            coding: vec![Coding {
                system: "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
                code: "activity".to_string(),
                display: Some("Activity".to_string()),
            }],
            text: Some("Activity Observation".to_string()),
        }],
        code: CodeableConcept {
            coding: vec![Coding {
                system: "http://loinc.org".to_string(),
                code: loinc::MOTION_DETECTED.to_string(),
                display: Some("Motion Detected".to_string()),
            }],
            text: Some("Motion Detection".to_string()),
        },
        subject: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Sleep Monitor Device".to_string()),
        }),
        effective_date_time: data.timestamp.clone(),
        value_quantity: None,
        value_boolean: Some(data.motion),
        device: Some(Reference {
            reference: Some(format!("Device/{}", data.deviceid)),
            display: Some("Arduino/Raspberry Pi Sensor - PIR Motion Sensor".to_string()),
        }),
    }
}

/// Store FHIR Observations in database
/// 
/// Inserts FHIR observation resources into fhir_observations table.
/// Each sensor reading generates 4 FHIR observations.
#[tracing::instrument(skip(pool, observations))]
pub async fn store_fhir_observations(
    pool: &sqlx::PgPool,
    sensor_reading_id: Uuid,
    observations: &[FhirObservation],
) -> ApiResult<()> {
    tracing::debug!("Storing {} FHIR observations", observations.len());
    
    for obs in observations {
        // Serialize observation to JSON
        let resource_data = serde_json::to_value(obs)
            .map_err(|e| ApiError::Serialization(e))?;
        
        // Extract LOINC code for indexing
        let loinc_code = obs.code.coding.first()
            .map(|c| c.code.clone());
        
        // Extract patient ID if present
        let patient_id = obs.subject.as_ref()
            .and_then(|s| s.reference.as_ref())
            .map(|r| r.clone());
        
        // Insert into database
        sqlx::query!(
            r#"
            INSERT INTO fhir_observations 
                (sensor_reading_id, resource_type, resource_data, fhir_id, patient_id, loinc_code, observation_category)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            sensor_reading_id,
            obs.resource_type,
            resource_data,
            obs.id,
            patient_id,
            loinc_code,
            obs.category.first().and_then(|c| c.text.clone())
        )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to store FHIR observation: {}", e);
            ApiError::Internal(format!("Database error: {}", e))
        })?;
    }
    
    tracing::info!("Stored {} FHIR observations for reading {}", observations.len(), sensor_reading_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_sensor_data() -> SensorData {
        SensorData {
            temp: 22.5,
            hum: 45.0,
            motion: false,
            sound_db: 35.2,
            deviceid: "pi-test-001".to_string(),
            timestamp: "2024-12-26T15:30:00Z".to_string(),
        }
    }
    
    #[test]
    fn test_convert_to_fhir_observations() {
        let data = create_test_sensor_data();
        let sensor_id = Uuid::new_v4();
        
        let observations = convert_to_fhir_observations(&data, sensor_id);
        
        // Should create 4 observations
        assert_eq!(observations.len(), 4);
        
        // Check resource types
        for obs in &observations {
            assert_eq!(obs.resource_type, "Observation");
            assert_eq!(obs.status, "final");
        }
    }
    
    #[test]
    fn test_temperature_observation() {
        let data = create_test_sensor_data();
        let base_id = "test-obs";
        
        let obs = create_temperature_observation(&data, base_id);
        
        assert_eq!(obs.id, "test-obs-temp");
        let value_qty = obs.value_quantity.as_ref().unwrap();
        assert_eq!(value_qty.value, 22.5);
        assert_eq!(value_qty.unit, "°C");
    }
    
    #[test]
    fn test_motion_observation() {
        let data = create_test_sensor_data();
        let base_id = "test-obs";
        
        let obs = create_motion_observation(&data, base_id);
        
        assert_eq!(obs.id, "test-obs-motion");
        assert_eq!(obs.value_boolean, Some(false));
        assert!(obs.value_quantity.is_none());
    }
}
