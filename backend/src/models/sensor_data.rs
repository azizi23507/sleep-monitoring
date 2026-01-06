use serde::{Deserialize, Serialize};

/// Sensor data structure received from Raspberry Pi
/// 
/// This structure represents a single sensor reading collected by the
/// Arduino/Raspberry Pi system and sent to the backend via HTTP POST.
/// 
/// Fields:
/// - temp: Temperature in Celsius (-50 to 50°C valid range)
/// - hum: Humidity percentage (0-100% valid range)
/// - motion: Boolean indicating if motion was detected by PIR sensor
/// - sound_db: Sound level in decibels (0-120 dB valid range)
/// - deviceid: Unique identifier for the Pi/Arduino device
/// - timestamp: ISO 8601 timestamp (e.g., "2024-12-26T15:30:00Z")
/// 
/// Example JSON payload:
/// ```json
/// {
///   "temp": 22.5,
///   "hum": 45.0,
///   "motion": false,
///   "sound_db": 35.2,
///   "deviceid": "pi-001",
///   "timestamp": "2024-12-26T15:30:00Z"
/// }
/// ```
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SensorData {
    /// Temperature in Celsius
    pub temp: f32,
    /// Humidity percentage (0-100)
    pub hum: f32,
    /// Motion detected (true/false)
    pub motion: bool,
    /// Sound level in decibels
    pub sound_db: f32,
    /// Device identifier
    pub deviceid: String,
    /// ISO 8601 timestamp
    pub timestamp: String,
}
