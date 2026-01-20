/// FHIR R4 Observation Resource Converter
/// 
/// This module converts sleep duration data to FHIR R4 Observation resources
/// for healthcare interoperability (Branch 2A).
/// 
/// FHIR (Fast Healthcare Interoperability Resources) is a standard
/// for exchanging healthcare information electronically.
/// 
/// Resources created:
/// - Sleep duration observations (LOINC: 93832-4)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

/// LOINC codes for sleep observations
pub mod loinc {
    /// Sleep duration in hours (standard LOINC code)
    #[allow(dead_code)]
    pub const SLEEP_DURATION: &str = "93832-4";
}

/// Sleep record data for FHIR conversion
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SleepRecordData {
    pub device_id: String,
    pub patient_id: Option<String>,
    pub sleep_date: String,
    pub sleep_duration_hours: f64,
    pub quality_score: i32,
    pub classification: String,
}

/// Convert sleep record to FHIR Observation
/// 
/// Creates a FHIR R4 Observation resource for sleep duration
/// using the standard LOINC code 93832-4.
#[allow(dead_code)]
pub fn convert_sleep_to_fhir_observation(
    data: &SleepRecordData,
    sleep_record_id: Uuid,
) -> FhirObservation {
    let fhir_id = format!("sleep-{}", sleep_record_id);
    
    FhirObservation {
        resource_type: "Observation".to_string(),
        id: fhir_id,
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
                code: loinc::SLEEP_DURATION.to_string(),
                display: Some("Sleep duration".to_string()),
            }],
            text: Some("Sleep Duration".to_string()),
        },
        subject: Some(Reference {
            reference: data.patient_id.as_ref()
                .map(|pid| format!("Patient/{}", pid))
                .or_else(|| Some(format!("Device/{}", data.device_id))),
            display: Some("Sleep Monitor Subject".to_string()),
        }),
        effective_date_time: data.sleep_date.clone(),
        value_quantity: Some(Quantity {
            value: data.sleep_duration_hours,
            unit: "h".to_string(),
            system: "http://unitsofmeasure.org".to_string(),
            code: "h".to_string(),
        }),
        value_boolean: None,
        device: Some(Reference {
            reference: Some(format!("Device/{}", data.device_id)),
            display: Some("Sleep Monitoring System".to_string()),
        }),
    }
}

/// Store FHIR Observation in database
/// 
/// Inserts FHIR observation resource into fhir_observations table.
/// Each sleep record generates one FHIR observation for sleep duration.
#[allow(dead_code)]
#[tracing::instrument(skip(pool, observation))]
pub async fn store_fhir_observation(
    pool: &sqlx::PgPool,
    sleep_record_id: Uuid,
    observation: &FhirObservation,
) -> ApiResult<()> {
    tracing::debug!("Storing FHIR observation for sleep record");
    
    // Serialize observation to JSON
    let resource_data = serde_json::to_value(observation)
        .map_err(|e| ApiError::Serialization(e))?;
    
    // Extract LOINC code for indexing
    let loinc_code = observation.code.coding.first()
        .map(|c| c.code.clone());
    
    // Extract patient ID if present
    let patient_id = observation.subject.as_ref()
        .and_then(|s| s.reference.as_ref())
        .and_then(|r| {
            if r.starts_with("Patient/") {
                Some(r.trim_start_matches("Patient/").to_string())
            } else {
                None
            }
        });
    
    // Insert into database
    sqlx::query!(
        r#"
        INSERT INTO fhir_observations 
            (sleep_record_id, resource_type, resource_data, fhir_id, patient_id, loinc_code, observation_category)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        sleep_record_id,
        observation.resource_type,
        resource_data,
        observation.id,
        patient_id,
        loinc_code,
        observation.category.first().and_then(|c| c.text.clone())
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to store FHIR observation: {}", e);
        ApiError::Internal(format!("Database error: {}", e))
    })?;
    
    tracing::info!("Stored FHIR observation for sleep record {}", sleep_record_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_sleep_record() -> SleepRecordData {
        SleepRecordData {
            device_id: "pi-test-001".to_string(),
            patient_id: Some("patient-123".to_string()),
            sleep_date: "2024-12-26".to_string(),
            sleep_duration_hours: 7.5,
            quality_score: 75,
            classification: "Good".to_string(),
        }
    }
    
    #[test]
    fn test_convert_sleep_to_fhir_observation() {
        let data = create_test_sleep_record();
        let record_id = Uuid::new_v4();
        
        let observation = convert_sleep_to_fhir_observation(&data, record_id);
        
        // Check resource type
        assert_eq!(observation.resource_type, "Observation");
        assert_eq!(observation.status, "final");
        
        // Check LOINC code
        let loinc = &observation.code.coding[0];
        assert_eq!(loinc.code, "93832-4");
        assert_eq!(loinc.system, "http://loinc.org");
        
        // Check value
        let value = observation.value_quantity.as_ref().unwrap();
        assert_eq!(value.value, 7.5);
        assert_eq!(value.unit, "h");
    }
    
    #[test]
    fn test_sleep_observation_category() {
        let data = create_test_sleep_record();
        let record_id = Uuid::new_v4();
        
        let obs = convert_sleep_to_fhir_observation(&data, record_id);
        
        let category = &obs.category[0];
        assert_eq!(category.coding[0].code, "vital-signs");
    }
}
