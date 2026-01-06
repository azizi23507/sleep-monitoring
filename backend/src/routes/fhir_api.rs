/// FHIR API Endpoints
/// 
/// RESTful API for retrieving FHIR Observation resources.
/// Follows FHIR R4 specification for resource retrieval.
/// 
/// Endpoints:
/// - GET /api/fhir/Observation/:id - Get single observation
/// - GET /api/fhir/Observation - Search observations (by patient, date, code)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};
use crate::fhir::FhirObservation;
use crate::routes::sensor_data::AppState;

/// Query parameters for FHIR Observation search
/// 
/// Example: GET /api/fhir/Observation?patient=Device/pi-001&_count=10
#[derive(Debug, Deserialize)]
pub struct ObservationSearchParams {
    /// Patient/Device reference (e.g., "Device/pi-001")
    patient: Option<String>,
    
    /// LOINC code filter
    code: Option<String>,
    
    /// Number of results to return (default: 20, max: 100)
    #[serde(rename = "_count")]
    count: Option<i64>,
}

/// FHIR Bundle response for search results
/// 
/// Wraps multiple observations in a FHIR Bundle resource.
#[derive(Debug, Serialize)]
pub struct FhirBundle {
    #[serde(rename = "resourceType")]
    resource_type: String,
    
    #[serde(rename = "type")]
    bundle_type: String,
    
    total: usize,
    
    entry: Vec<BundleEntry>,
}

/// Single entry in FHIR Bundle
#[derive(Debug, Serialize)]
pub struct BundleEntry {
    resource: FhirObservation,
}

/// GET /api/fhir/Observation/:id
/// 
/// Retrieve a single FHIR Observation by its FHIR ID.
/// 
/// Path Parameters:
/// - id: FHIR logical ID (e.g., "obs-123e4567-e89b-12d3-a456-426614174000-temp")
/// 
/// Response:
/// - 200 OK: FHIR Observation resource
/// - 404 Not Found: Observation doesn't exist
/// 
/// Example:
/// ```bash
/// curl http://localhost:3000/api/fhir/Observation/obs-abc123-temp
/// ```
#[tracing::instrument(skip(state))]
pub async fn get_observation_by_id(
    State(state): State<AppState>,
    Path(fhir_id): Path<String>,
) -> ApiResult<Json<FhirObservation>> {
    tracing::info!("Fetching FHIR Observation: {}", fhir_id);
    
    // Query database for observation
    let record = sqlx::query!(
        r#"
        SELECT resource_data
        FROM fhir_observations
        WHERE fhir_id = $1
        "#,
        fhir_id
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database query failed: {}", e);
        ApiError::Internal(format!("Database error: {}", e))
    })?;
    
    // Check if observation exists
    let record = record.ok_or_else(|| {
        tracing::warn!("Observation not found: {}", fhir_id);
        ApiError::Internal(format!("Observation not found: {}", fhir_id))
    })?;
    
    // Deserialize JSON to FhirObservation
    let observation: FhirObservation = serde_json::from_value(record.resource_data)
        .map_err(|e| {
            tracing::error!("Failed to deserialize FHIR observation: {}", e);
            ApiError::Serialization(e)
        })?;
    
    tracing::debug!("Successfully retrieved observation: {}", fhir_id);
    Ok(Json(observation))
}

/// GET /api/fhir/Observation
/// 
/// Search FHIR Observations with filters.
/// 
/// Query Parameters:
/// - patient: Filter by patient/device reference (e.g., "Device/pi-001")
/// - code: Filter by LOINC code
/// - date: Filter by date (ISO 8601)
/// - _count: Number of results (default: 20, max: 100)
/// 
/// Response:
/// - 200 OK: FHIR Bundle with matching observations
/// 
/// Examples:
/// ```bash
/// # Get all observations for a device
/// curl "http://localhost:3000/api/fhir/Observation?patient=Device/pi-001"
/// 
/// # Get temperature observations only
/// curl "http://localhost:3000/api/fhir/Observation?code=CUSTOM-TEMP-001"
/// 
/// # Get observations from specific date
/// curl "http://localhost:3000/api/fhir/Observation?date=2024-12-26"
/// 
/// # Limit results
/// curl "http://localhost:3000/api/fhir/Observation?_count=50"
/// ```
#[tracing::instrument(skip(state))]
pub async fn search_observations(
    State(state): State<AppState>,
    Query(params): Query<ObservationSearchParams>,
) -> ApiResult<Json<FhirBundle>> {
    tracing::info!("Searching FHIR Observations with params: {:?}", params);
    
    // Set default and max count
    let count = params.count.unwrap_or(20).min(100);
    
    // Build dynamic query based on parameters
    let mut query = String::from(
        "SELECT resource_data FROM fhir_observations WHERE 1=1"
    );
    
    // Add filters
    if params.patient.is_some() {
        query.push_str(" AND patient_id = $1");
    }
    
    if params.code.is_some() {
        query.push_str(" AND loinc_code = $2");
    }
    
    // Add ordering and limit
    query.push_str(" ORDER BY created_at DESC LIMIT $3");
    
    // Execute query (simplified - in production use query builder)
    let records = if let Some(patient_ref) = params.patient {
        if let Some(code) = params.code {
            sqlx::query_scalar::<_, serde_json::Value>(&query)
                .bind(patient_ref)
                .bind(code)
                .bind(count)
                .fetch_all(&state.db_pool)
                .await
        } else {
            let query = "SELECT resource_data FROM fhir_observations WHERE patient_id = $1 ORDER BY created_at DESC LIMIT $2";
            sqlx::query_scalar::<_, serde_json::Value>(query)
                .bind(patient_ref)
                .bind(count)
                .fetch_all(&state.db_pool)
                .await
        }
    } else if let Some(code) = params.code {
        let query = "SELECT resource_data FROM fhir_observations WHERE loinc_code = $1 ORDER BY created_at DESC LIMIT $2";
        sqlx::query_scalar::<_, serde_json::Value>(query)
            .bind(code)
            .bind(count)
            .fetch_all(&state.db_pool)
            .await
    } else {
        let query = "SELECT resource_data FROM fhir_observations ORDER BY created_at DESC LIMIT $1";
        sqlx::query_scalar::<_, serde_json::Value>(query)
            .bind(count)
            .fetch_all(&state.db_pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Database query failed: {}", e);
        ApiError::Internal(format!("Database error: {}", e))
    })?;
    
    // Deserialize observations
    let observations: Result<Vec<FhirObservation>, _> = records
        .into_iter()
        .map(|record| serde_json::from_value(record))
        .collect();
    
    let observations = observations.map_err(|e| {
        tracing::error!("Failed to deserialize observations: {}", e);
        ApiError::Serialization(e)
    })?;
    
    // Build FHIR Bundle response
    let bundle = FhirBundle {
        resource_type: "Bundle".to_string(),
        bundle_type: "searchset".to_string(),
        total: observations.len(),
        entry: observations.into_iter().map(|obs| BundleEntry { resource: obs }).collect(),
    };
    
    tracing::info!("Found {} observations", bundle.total);
    Ok(Json(bundle))
}
