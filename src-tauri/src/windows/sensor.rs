use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSensorStatus {
    pub is_supported: bool,
    pub current_lux: Option<f32>,
    pub error_message: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn get_ambient_light_reading() -> LightSensorStatus {
    match windows::Devices::Sensors::LightSensor::GetDefault() {
        Ok(sensor) => match sensor.GetCurrentReading() {
            Ok(reading) => match reading.IlluminanceInLux() {
                Ok(lux) => {
                    debug!("Ambient light sensor reading: {} lux", lux);
                    LightSensorStatus {
                        is_supported: true,
                        current_lux: Some(lux),
                        error_message: None,
                    }
                }
                Err(e) => {
                    warn!("Failed to read lux from LightSensorReading: {}", e);
                    LightSensorStatus {
                        is_supported: true,
                        current_lux: None,
                        error_message: Some(format!("Failed to read illuminance: {}", e)),
                    }
                }
            },
            Err(e) => {
                warn!("Failed to get current reading from LightSensor: {}", e);
                LightSensorStatus {
                    is_supported: true,
                    current_lux: None,
                    error_message: Some(format!("Failed to retrieve reading: {}", e)),
                }
            }
        },
        Err(e) => LightSensorStatus {
            is_supported: false,
            current_lux: None,
            error_message: Some(format!("No ambient light sensor found: {}", e)),
        },
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_ambient_light_reading() -> LightSensorStatus {
    LightSensorStatus {
        is_supported: false,
        current_lux: None,
        error_message: Some("Sensors are only supported on Windows".to_string()),
    }
}
