/// ML Results API Endpoints (Branch 2B)
/// 
/// RESTful API for retrieving ML-analyzed sleep quality results.
/// Python ML service writes to sleep_records table, this API reads from it.
/// 
/// Endpoints:
/// - GET /api/sleep-records - Get all sleep records (with filters)
/// - GET /api/sleep-records/:date - Get record for specific date
/// - GET /api/sleep-quality/latest - Get latest sleep quality score

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::error::{ApiError, ApiResult};
use crate::routes::sensor_data::AppState;

/// Sleep quality record from ML analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct SleepRecord {
    pub id: String,
    pub device_id: String,
    pub sleep_date: String,
    pub quality_score: i32,
    pub classification: String,
    pub avg_temperature: Option<f64>,
    pub avg_humidity: Option<f64>,
    pub avg_sound_level: Option<f64>,
    pub motion_events_count: Option<i32>,
    pub analyzed_at: Option<String>,
}

/// Query parameters for sleep records search
#[derive(Debug, Deserialize)]
pub struct SleepRecordsQuery {
    /// Device ID filter
    device_id: Option<String>,
    
    /// Start date (YYYY-MM-DD)
    #[allow(dead_code)]
    start_date: Option<String>,
    
    /// End date (YYYY-MM-DD)
    #[allow(dead_code)]
    end_date: Option<String>,
    
    /// Number of results (default: 30, max: 100)
    limit: Option<i64>,
}

/// GET /api/sleep-records
/// 
/// Get all sleep records with optional filters
/// 
/// Query Parameters:
/// - device_id: Filter by device
/// - start_date: Start date (YYYY-MM-DD)
/// - end_date: End date (YYYY-MM-DD)
/// - limit: Max results (default: 30, max: 100)
/// 
/// Response:
/// ```json
/// {
///   "total": 10,
///   "records": [
///     {
///       "id": "...",
///       "device_id": "pi-001",
///       "sleep_date": "2024-12-28",
///       "quality_score": 75,
///       "classification": "Good",
///       ...
///     }
///   ]
/// }
/// ```
#[tracing::instrument(skip(state))]
pub async fn get_sleep_records(
    State(state): State<AppState>,
    Query(params): Query<SleepRecordsQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    tracing::info!("Fetching sleep records with filters: {:?}", params);
    
    let limit = params.limit.unwrap_or(30).min(100);
    
    // Build query based on filters
    let records = if let Some(device_id) = params.device_id {
        // Filter by device
        sqlx::query_as!(
            SleepRecord,
            r#"
            SELECT 
                id::text as "id!",
                device_id as "device_id!",
                sleep_date::text as "sleep_date!",
                quality_score as "quality_score!",
                classification as "classification!",
                avg_temperature::double precision as "avg_temperature?",
                avg_humidity::double precision as "avg_humidity?",
                avg_sound_level::double precision as "avg_sound_level?",
                motion_events_count as "motion_events_count?",
                analyzed_at::text as "analyzed_at?"
            FROM sleep_records
            WHERE device_id = $1
            ORDER BY sleep_date DESC
            LIMIT $2
            "#,
            device_id,
            limit
        )
        .fetch_all(&state.db_pool)
        .await
    } else {
        // Get all records
        sqlx::query_as!(
            SleepRecord,
            r#"
            SELECT 
                id::text as "id!",
                device_id as "device_id!",
                sleep_date::text as "sleep_date!",
                quality_score as "quality_score!",
                classification as "classification!",
                avg_temperature::double precision as "avg_temperature?",
                avg_humidity::double precision as "avg_humidity?",
                avg_sound_level::double precision as "avg_sound_level?",
                motion_events_count as "motion_events_count?",
                analyzed_at::text as "analyzed_at?"
            FROM sleep_records
            ORDER BY sleep_date DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&state.db_pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Database query failed: {}", e);
        ApiError::Internal(format!("Database error: {}", e))
    })?;
    
    tracing::info!("Found {} sleep records", records.len());
    
    Ok(Json(serde_json::json!({
        "total": records.len(),
        "records": records
    })))
}

/// GET /api/sleep-records/:date
/// 
/// Get sleep record for a specific date
/// 
/// Path Parameters:
/// - date: Sleep date in YYYY-MM-DD format
/// 
/// Response:
/// ```json
/// {
///   "id": "...",
///   "device_id": "pi-001",
///   "sleep_date": "2024-12-28",
///   "quality_score": 75,
///   "classification": "Good",
///   ...
/// }
/// ```
#[tracing::instrument(skip(state))]
pub async fn get_sleep_record_by_date(
    State(state): State<AppState>,
    Path(date_str): Path<String>,
) -> ApiResult<Json<SleepRecord>> {
    tracing::info!("Fetching sleep record for date: {}", date_str);
    
    // Validate date format
    let _date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| ApiError::Validation("Invalid date format. Use YYYY-MM-DD".to_string()))?;
    
    let record = sqlx::query_as!(
        SleepRecord,
        r#"
        SELECT 
            id::text as "id!",
            device_id as "device_id!",
            sleep_date::text as "sleep_date!",
            quality_score as "quality_score!",
            classification as "classification!",
            avg_temperature::double precision as "avg_temperature?",
            avg_humidity::double precision as "avg_humidity?",
            avg_sound_level::double precision as "avg_sound_level?",
            motion_events_count as "motion_events_count?",
            analyzed_at::text as "analyzed_at?"
        FROM sleep_records
        WHERE sleep_date::text = $1
        LIMIT 1
        "#,
        date_str
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database query failed: {}", e);
        ApiError::Internal(format!("Database error: {}", e))
    })?
    .ok_or_else(|| {
        tracing::warn!("No sleep record found for date: {}", date_str);
        ApiError::Internal(format!("No sleep record found for date: {}", date_str))
    })?;
    
    tracing::info!("Found sleep record for {}: score={}", date_str, record.quality_score);
    
    Ok(Json(record))
}

/// GET /api/sleep-quality/latest
/// 
/// Get the most recent sleep quality score
/// 
/// Response:
/// ```json
/// {
///   "date": "2024-12-28",
///   "score": 75,
///   "classification": "Good"
/// }
/// ```
#[tracing::instrument(skip(state))]
pub async fn get_latest_sleep_quality(
    State(state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    tracing::info!("Fetching latest sleep quality");
    
    let record = sqlx::query!(
        r#"
        SELECT 
            sleep_date,
            quality_score,
            classification
        FROM sleep_records
        ORDER BY sleep_date DESC
        LIMIT 1
        "#
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database query failed: {}", e);
        ApiError::Internal(format!("Database error: {}", e))
    })?
    .ok_or_else(|| {
        tracing::warn!("No sleep records found");
        ApiError::Internal("No sleep records found".to_string())
    })?;
    
    tracing::info!("Latest sleep quality: {} ({})", record.quality_score, record.classification);
    
    Ok(Json(serde_json::json!({
        "date": record.sleep_date.to_string(),
        "score": record.quality_score,
        "classification": record.classification
    })))
}
