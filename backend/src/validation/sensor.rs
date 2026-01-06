use crate::models::sensor_data::SensorData;

/// Validates sensor data against acceptable ranges
/// 
/// This function ensures all sensor readings are within physically
/// meaningful and safe ranges before storing or processing them.
/// 
/// Validation ranges are based on:
/// - Temperature: -50 to 50°C (Arctic to Desert extremes)
/// - Humidity: 0 to 100% (physical limits of relative humidity)
/// - Sound: 0 to 120 dB (silence to pain threshold)
/// 
/// Returns:
/// - Ok(()) if all values are within valid ranges
/// - Err(String) with descriptive error message if validation fails
/// 
/// Example usage:
/// ```rust
/// if let Err(e) = validate_sensor_data(&data) {
///     eprintln!("Validation failed: {}", e);
///     return BadRequest;
/// }
/// ```
pub fn validate_sensor_data(data: &SensorData) -> Result<(), String> {
    // Validate temperature range: -50 to 50°C
    // Covers extreme climates (Arctic -50°C to Sahara 50°C)
    if !(-50.0..=50.0).contains(&data.temp) {
        return Err(format!(
            "Temperature out of range: {} (valid: -50 to 50°C)", 
            data.temp
        ));
    }
    
    // Validate humidity range: 0 to 100%
    // Physical limit of relative humidity
    if !(0.0..=100.0).contains(&data.hum) {
        return Err(format!(
            "Humidity out of range: {} (valid: 0 to 100%)", 
            data.hum
        ));
    }
    
    // Validate sound level range: 0 to 120 dB
    // 0 dB = silence, 120 dB = pain threshold
    if !(0.0..=120.0).contains(&data.sound_db) {
        return Err(format!(
            "Sound level out of range: {} (valid: 0 to 120 dB)", 
            data.sound_db
        ));
    }
    
    // All validations passed
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create test sensor data
    fn create_test_data(temp: f32, hum: f32, sound: f32) -> SensorData {
        SensorData {
            temp,
            hum,
            motion: false,
            sound_db: sound,
            deviceid: "test-device".to_string(),
            timestamp: "2024-12-26T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_valid_data() {
        let data = create_test_data(22.0, 50.0, 40.0);
        assert!(validate_sensor_data(&data).is_ok());
    }

    #[test]
    fn test_temperature_too_low() {
        let data = create_test_data(-51.0, 50.0, 40.0);
        assert!(validate_sensor_data(&data).is_err());
    }

    #[test]
    fn test_temperature_too_high() {
        let data = create_test_data(51.0, 50.0, 40.0);
        assert!(validate_sensor_data(&data).is_err());
    }

    #[test]
    fn test_humidity_too_low() {
        let data = create_test_data(22.0, -1.0, 40.0);
        assert!(validate_sensor_data(&data).is_err());
    }

    #[test]
    fn test_humidity_too_high() {
        let data = create_test_data(22.0, 101.0, 40.0);
        assert!(validate_sensor_data(&data).is_err());
    }

    #[test]
    fn test_sound_too_high() {
        let data = create_test_data(22.0, 50.0, 121.0);
        assert!(validate_sensor_data(&data).is_err());
    }

    #[test]
    fn test_sound_negative() {
        let data = create_test_data(22.0, 50.0, -1.0);
        assert!(validate_sensor_data(&data).is_err());
    }

    #[test]
    fn test_boundary_values_valid() {
        // Test exact boundary values (should be valid)
        let data_min = create_test_data(-50.0, 0.0, 0.0);
        assert!(validate_sensor_data(&data_min).is_ok());
        
        let data_max = create_test_data(50.0, 100.0, 120.0);
        assert!(validate_sensor_data(&data_max).is_ok());
    }

    #[test]
    fn test_boundary_values_invalid() {
        // Test just outside boundary values (should be invalid)
        let data_temp_low = create_test_data(-50.1, 50.0, 40.0);
        assert!(validate_sensor_data(&data_temp_low).is_err());
        
        let data_temp_high = create_test_data(50.1, 50.0, 40.0);
        assert!(validate_sensor_data(&data_temp_high).is_err());
        
        let data_hum_low = create_test_data(22.0, -0.1, 40.0);
        assert!(validate_sensor_data(&data_hum_low).is_err());
        
        let data_hum_high = create_test_data(22.0, 100.1, 40.0);
        assert!(validate_sensor_data(&data_hum_high).is_err());
        
        let data_sound_high = create_test_data(22.0, 50.0, 120.1);
        assert!(validate_sensor_data(&data_sound_high).is_err());
    }

    #[test]
    fn test_typical_room_conditions() {
        // Test typical indoor room conditions
        let data = create_test_data(21.0, 45.0, 35.0);
        assert!(validate_sensor_data(&data).is_ok());
    }

    #[test]
    fn test_extreme_valid_conditions() {
        // Test extreme but valid conditions
        let arctic = create_test_data(-40.0, 10.0, 20.0);
        assert!(validate_sensor_data(&arctic).is_ok());
        
        let desert = create_test_data(45.0, 5.0, 50.0);
        assert!(validate_sensor_data(&desert).is_ok());
    }

    #[test]
    fn test_error_message_content() {
        // Test that error messages are descriptive
        let data = create_test_data(-60.0, 50.0, 40.0);
        let err = validate_sensor_data(&data).unwrap_err();
        assert!(err.contains("Temperature"));
        assert!(err.contains("-60"));
    }
}
