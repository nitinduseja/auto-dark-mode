use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle};
use tracing::info;

use super::OperationResult;
use crate::config::{ConfigManager, ScheduleConfig};
use crate::scheduler::trigger_immediate_evaluation;
use crate::state::{calculate_solar_times, format_minutes_as_12h};

#[derive(Debug, Clone, Deserialize)]
pub struct SetScheduleConfigRequest {
    pub schedule: ScheduleConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SolarPreviewRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub sunrise_offset_minutes: i32,
    pub sunset_offset_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarPreviewResult {
    pub sunrise_minute: u32,
    pub sunset_minute: u32,
    pub sunrise_time_formatted: String,
    pub sunset_time_formatted: String,
    pub daylight_window_formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedLocationResult {
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[command]
pub async fn detect_user_location() -> OperationResult<DetectedLocationResult> {
    info!("Handling detect_user_location command based on local system timezone");
    let now = Local::now();
    let tz_offset_hours = now.offset().local_minus_utc() / 3600;

    let (name, lat, lon) = match tz_offset_hours {
        -10 => ("Honolulu, Hawaii", 21.3069, -157.8583),
        -8 | -7 => ("Pacific Time (US)", 37.7749, -122.4194),
        -6 => ("Central Time (US)", 41.8781, -87.6298),
        -5 | -4 => ("Eastern Time (US)", 40.7128, -74.0060),
        0 => ("London, UK (GMT)", 51.5074, -0.1278),
        1 => ("Berlin / Paris (CET)", 52.5200, 13.4050),
        2 => ("Eastern Europe (EET)", 50.4501, 30.5234),
        3 => ("Moscow / Dubai", 55.7558, 37.6173),
        5 | 6 => ("India / South Asia (IST)", 28.6139, 77.2090),
        8 => ("Singapore / Beijing (SGT)", 1.3521, 103.8198),
        9 => ("Tokyo, Japan (JST)", 35.6762, 139.6503),
        10 | 11 => ("Sydney, Australia (AEST)", -33.8688, 151.2093),
        12 | 13 => ("Auckland, New Zealand", -36.8485, 174.7633),
        _ => ("Local Timezone Location", 40.7128, -74.0060),
    };

    OperationResult::ok(DetectedLocationResult {
        location_name: name.to_string(),
        latitude: lat,
        longitude: lon,
    })
}

#[command]
pub async fn get_schedule_config() -> OperationResult<ScheduleConfig> {
    info!("Handling get_schedule_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => OperationResult::ok(config.schedule),
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_schedule_config(
    app: AppHandle,
    request: SetScheduleConfigRequest,
) -> OperationResult<ScheduleConfig> {
    info!(
        "Handling set_schedule_config command with mode: {}",
        request.schedule.schedule_type
    );

    if let Err(e) = request.schedule.validate() {
        return OperationResult::err(format!("Schedule validation error: {}", e));
    }

    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.schedule = request.schedule.clone();
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(request.schedule)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn calculate_solar_preview(
    request: SolarPreviewRequest,
) -> OperationResult<SolarPreviewResult> {
    let now = Local::now();
    let day_of_year = now.date_naive().ordinal();
    let tz_offset_minutes = now.offset().local_minus_utc() / 60;

    let (raw_sunrise, raw_sunset) = calculate_solar_times(
        day_of_year,
        request.latitude,
        request.longitude,
        tz_offset_minutes,
    );

    let sunrise_minute =
        ((raw_sunrise as i32 + request.sunrise_offset_minutes).rem_euclid(1440)) as u32;
    let sunset_minute =
        ((raw_sunset as i32 + request.sunset_offset_minutes).rem_euclid(1440)) as u32;

    let daylight_duration = sunset_minute.saturating_sub(sunrise_minute);
    let hours = daylight_duration / 60;
    let mins = daylight_duration % 60;

    OperationResult::ok(SolarPreviewResult {
        sunrise_minute,
        sunset_minute,
        sunrise_time_formatted: format_minutes_as_12h(sunrise_minute),
        sunset_time_formatted: format_minutes_as_12h(sunset_minute),
        daylight_window_formatted: format!("{}h {:02}m", hours, mins),
    })
}
