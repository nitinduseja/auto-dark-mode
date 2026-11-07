use chrono::{Datelike, Local, TimeZone, Timelike};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::config::AppConfig;
use crate::windows::{detect_windows_theme, DetectedTheme, ThemeMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchedulerStatus {
    Real,
    Demonstration,
    Unavailable,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NextTransition {
    pub target_theme: ThemeMode,
    pub event_name: String,
    pub time_formatted: String,
    pub countdown_formatted: String,
    pub location_note: String,
    pub status: SchedulerStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleTimeline {
    pub sunrise_time_formatted: String,
    pub sunset_time_formatted: String,
    pub night1_percentage: f32,
    pub day_percentage: f32,
    pub night2_percentage: f32,
    pub current_needle_percentage: f32,
    pub current_time_formatted: String,
    pub daylight_window_formatted: String,
    pub status: SchedulerStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlState {
    pub id: String,
    pub title: String,
    pub description: String,
    pub enabled: bool,
    pub supported: bool,
    pub interactive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppInfo {
    pub app_name: String,
    pub version: String,
    pub target_platform: String,
    pub webview_runtime: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardState {
    pub current_theme: ThemeMode,
    pub detected_theme: DetectedTheme,
    pub automation_enabled: bool,
    pub last_switched_formatted: String,
    pub next_transition: NextTransition,
    pub schedule_timeline: ScheduleTimeline,
    pub quick_controls: Vec<ControlState>,
    pub app_info: AppInfo,
}

/// Calculate astronomical sunrise and sunset minutes from midnight using standard NOAA solar equations.
///
/// * `day_of_year` - 1-based day of year (1..=366)
/// * `latitude` - Latitude in decimal degrees (North positive, South negative)
/// * `longitude` - Longitude in decimal degrees (East positive, West negative)
/// * `tz_offset_minutes` - Timezone offset from UTC in minutes (e.g., UTC-4 is -240)
///
/// Returns (sunrise_minute, sunset_minute) as minutes from midnight local time.
pub fn calculate_solar_times(
    day_of_year: u32,
    latitude: f64,
    longitude: f64,
    tz_offset_minutes: i32,
) -> (u32, u32) {
    let day = day_of_year.clamp(1, 366) as f64;

    // Fractional year in radians
    let gamma = 2.0 * PI / 365.25 * (day - 1.0);

    // Equation of time in minutes
    let eqtime = 229.18
        * (0.000075 + 0.001868 * gamma.cos() - 0.032077 * gamma.sin()
            - 0.014615 * (2.0 * gamma).cos()
            - 0.040849 * (2.0 * gamma).sin());

    // Solar declination angle in radians
    let decl = 0.006918 - 0.399912 * gamma.cos() + 0.070257 * gamma.sin()
        - 0.006758 * (2.0 * gamma).cos()
        + 0.000907 * (2.0 * gamma).sin()
        - 0.002697 * (3.0 * gamma).cos()
        + 0.00148 * (3.0 * gamma).sin();

    // Standard solar zenith angle for sunrise/sunset (90.833° accounts for atmospheric refraction)
    let zenith_rad = 90.833_f64.to_radians();
    let lat_rad = latitude.to_radians();

    let cos_ha =
        (zenith_rad.cos() - lat_rad.sin() * decl.sin()) / (lat_rad.cos() * decl.cos());

    // Check for polar day or night
    if cos_ha >= 1.0 {
        // Polar night (sun never rises)
        return (720, 720);
    }
    if cos_ha <= -1.0 {
        // Midnight sun (sun never sets)
        return (0, 1439);
    }

    let ha_deg = cos_ha.acos().to_degrees();

    // Solar noon in minutes from local midnight
    let solar_noon = 720.0 - (4.0 * longitude) - eqtime + (tz_offset_minutes as f64);

    let sunrise = (solar_noon - 4.0 * ha_deg).round() as i32;
    let sunset = (solar_noon + 4.0 * ha_deg).round() as i32;

    let sunrise_norm = (sunrise.rem_euclid(1440)) as u32;
    let sunset_norm = (sunset.rem_euclid(1440)) as u32;

    (sunrise_norm, sunset_norm)
}

pub fn format_minutes_as_12h(total_minutes: u32) -> String {
    let minutes = total_minutes % 1440;
    let raw_hour = minutes / 60;
    let minute = minutes % 60;

    let (hour_12, period) = match raw_hour {
        0 => (12, "AM"),
        1..=11 => (raw_hour, "AM"),
        12 => (12, "PM"),
        _ => (raw_hour - 12, "PM"),
    };

    format!("{}:{:02} {}", hour_12, minute, period)
}

pub fn calculate_timeline_metrics(
    sunrise_minute_of_day: u32,
    sunset_minute_of_day: u32,
    current_minute_of_day: u32,
) -> (f32, f32, f32, f32, String) {
    let total_minutes = 1440.0f32;
    let current = current_minute_of_day.min(1440) as f32;
    let needle_pct = (current / total_minutes) * 100.0;

    if sunrise_minute_of_day <= sunset_minute_of_day {
        let sunrise = sunrise_minute_of_day.min(1440) as f32;
        let sunset = sunset_minute_of_day.min(1440) as f32;

        let night1_pct = (sunrise / total_minutes) * 100.0;
        let day_pct = ((sunset - sunrise) / total_minutes) * 100.0;
        let night2_pct = ((total_minutes - sunset) / total_minutes) * 100.0;

        let daylight_duration_minutes = sunset_minute_of_day - sunrise_minute_of_day;
        let hours = daylight_duration_minutes / 60;
        let mins = daylight_duration_minutes % 60;
        let daylight_formatted = format!("{}h {:02}m", hours, mins);

        (night1_pct, day_pct, night2_pct, needle_pct, daylight_formatted)
    } else {
        // Inverted schedule crossing midnight (e.g. Light 8 PM to 6 AM)
        let daylight_duration_minutes = (1440 - sunrise_minute_of_day) + sunset_minute_of_day;
        let dark_duration_minutes = sunrise_minute_of_day - sunset_minute_of_day;

        let day1_pct = (sunset_minute_of_day as f32 / total_minutes) * 100.0;
        let night_pct = (dark_duration_minutes as f32 / total_minutes) * 100.0;
        let day2_pct = ((1440 - sunrise_minute_of_day) as f32 / total_minutes) * 100.0;

        let hours = daylight_duration_minutes / 60;
        let mins = daylight_duration_minutes % 60;
        let daylight_formatted = format!("{}h {:02}m", hours, mins);

        // Represent segments consistently as (start_pct, mid_pct, end_pct)
        (day1_pct, night_pct, day2_pct, needle_pct, daylight_formatted)
    }
}

pub fn build_dashboard_state(config: &AppConfig) -> DashboardState {
    let now = Local::now();
    let current_minute = now.hour() * 60 + now.minute();
    let day_of_year = now.date_naive().ordinal();
    let tz_offset_minutes = now.offset().local_minus_utc() / 60;

    // Calculate active schedule start times based on configured mode
    let (sunrise_minute, sunset_minute, location_note, event_sunrise_name, event_sunset_name) =
        match config.schedule.schedule_type.as_str() {
            "custom_times" => {
                let light = config.schedule.light_start_hour * 60 + config.schedule.light_start_minute;
                let dark = config.schedule.dark_start_hour * 60 + config.schedule.dark_start_minute;
                (
                    light,
                    dark,
                    format!(
                        "Custom schedule (Light: {}, Dark: {})",
                        format_minutes_as_12h(light),
                        format_minutes_as_12h(dark)
                    ),
                    "Scheduled light appearance".to_string(),
                    "Scheduled dark appearance".to_string(),
                )
            }
            "night_light" => {
                let light = 6 * 60;
                let dark = 20 * 60;
                (
                    light,
                    dark,
                    "Windows Night Light schedule synchronization".to_string(),
                    "Night Light schedule off".to_string(),
                    "Night Light schedule on".to_string(),
                )
            }
            _ => {
                // "sunrise_sunset" (default)
                let (raw_sunrise, raw_sunset) = calculate_solar_times(
                    day_of_year,
                    config.schedule.latitude,
                    config.schedule.longitude,
                    tz_offset_minutes,
                );

                let sunrise = ((raw_sunrise as i32 + config.schedule.sunrise_offset_minutes)
                    .rem_euclid(1440)) as u32;
                let sunset = ((raw_sunset as i32 + config.schedule.sunset_offset_minutes)
                    .rem_euclid(1440)) as u32;

                let lat_dir = if config.schedule.latitude >= 0.0 { "N" } else { "S" };
                let lon_dir = if config.schedule.longitude >= 0.0 { "E" } else { "W" };
                let loc_str = format!(
                    "{} ({:.2}° {}, {:.2}° {})",
                    config.schedule.location_name,
                    config.schedule.latitude.abs(),
                    lat_dir,
                    config.schedule.longitude.abs(),
                    lon_dir
                );

                (
                    sunrise,
                    sunset,
                    loc_str,
                    "Astronomical sunrise".to_string(),
                    "Astronomical sunset".to_string(),
                )
            }
        };

    let (night1, day, night2, needle, daylight_str) =
        calculate_timeline_metrics(sunrise_minute, sunset_minute, current_minute);

    let detected = detect_windows_theme().unwrap_or(DetectedTheme {
        apps_theme: config.theme.current,
        system_theme: config.theme.current,
        is_synchronized: true,
        effective_theme: config.theme.current,
    });

    let current_time_formatted = now.format("%I:%M %p").to_string();

    // Determine next transition
    let (target_theme, event_name, time_formatted, mins_remaining) =
        if sunrise_minute <= sunset_minute {
            if current_minute < sunrise_minute {
                (
                    ThemeMode::Light,
                    event_sunrise_name,
                    format_minutes_as_12h(sunrise_minute),
                    sunrise_minute - current_minute,
                )
            } else if current_minute < sunset_minute {
                (
                    ThemeMode::Dark,
                    event_sunset_name,
                    format_minutes_as_12h(sunset_minute),
                    sunset_minute - current_minute,
                )
            } else {
                (
                    ThemeMode::Light,
                    event_sunrise_name,
                    format_minutes_as_12h(sunrise_minute),
                    (1440 - current_minute) + sunrise_minute,
                )
            }
        } else {
            // Inverted schedule crossing midnight (e.g. Light 8 PM, Dark 6 AM)
            if current_minute < sunset_minute {
                (
                    ThemeMode::Dark,
                    event_sunset_name,
                    format_minutes_as_12h(sunset_minute),
                    sunset_minute - current_minute,
                )
            } else if current_minute < sunrise_minute {
                (
                    ThemeMode::Light,
                    event_sunrise_name,
                    format_minutes_as_12h(sunrise_minute),
                    sunrise_minute - current_minute,
                )
            } else {
                (
                    ThemeMode::Dark,
                    event_sunset_name,
                    format_minutes_as_12h(sunset_minute),
                    (1440 - current_minute) + sunset_minute,
                )
            }
        };

    let countdown_formatted = if let Some(ref mo) = config.manual_override {
        if let Some(until) = mo.until_timestamp {
            let now_ts = now.timestamp();
            if until > now_ts {
                let remaining_secs = until - now_ts;
                let rem_m = (remaining_secs + 59) / 60;
                if rem_m >= 60 {
                    format!("Paused ({}h {:02}m left)", rem_m / 60, rem_m % 60)
                } else {
                    format!("Paused ({}m left)", rem_m)
                }
            } else if mins_remaining >= 60 {
                format!("In {}h {:02}m", mins_remaining / 60, mins_remaining % 60)
            } else {
                format!("In {}m", mins_remaining)
            }
        } else {
            format!("Paused until {}", format_minutes_as_12h(if target_theme == ThemeMode::Light { sunrise_minute } else { sunset_minute }))
        }
    } else if mins_remaining >= 60 {
        format!("In {}h {:02}m", mins_remaining / 60, mins_remaining % 60)
    } else {
        format!("In {}m", mins_remaining)
    };

    let last_switched_formatted = if let Some(ts) = config.last_switched_timestamp {
        if let Some(dt) = Local.timestamp_opt(ts, 0).single() {
            let trigger = config
                .last_switched_trigger
                .as_deref()
                .unwrap_or("schedule");
            let today = now.date_naive();
            if dt.date_naive() == today {
                format!("Last switched today at {} via {}", dt.format("%I:%M %p"), trigger)
            } else {
                format!(
                    "Last switched on {} at {} via {}",
                    dt.format("%b %d"),
                    dt.format("%I:%M %p"),
                    trigger
                )
            }
        } else {
            "Synchronized with Windows system settings".to_string()
        }
    } else {
        "Synchronized with Windows system settings".to_string()
    };

    let quick_controls = vec![
        ControlState {
            id: "automatic_switching".to_string(),
            title: "Automatic switching".to_string(),
            description: "Allow AutoDarkMode to manage system theme changes".to_string(),
            enabled: config.theme.automatic,
            supported: true,
            interactive: true,
            badge: None,
        },
        ControlState {
            id: "follow_sun".to_string(),
            title: "Follow sunrise and sunset".to_string(),
            description: "Synchronize schedule with local astronomical daylight".to_string(),
            enabled: config.schedule.schedule_type == "sunrise_sunset" && config.schedule.enabled,
            supported: true,
            interactive: true,
            badge: None,
        },
        ControlState {
            id: "wallpaper".to_string(),
            title: "Wallpaper switching".to_string(),
            description: "Swap desktop backgrounds between day and night variants".to_string(),
            enabled: config.features.wallpaper,
            supported: true,
            interactive: true,
            badge: None,
        },
        ControlState {
            id: "app_themes".to_string(),
            title: "App-specific themes".to_string(),
            description: "Override themes for individual desktop apps and browsers".to_string(),
            enabled: config.features.app_themes,
            supported: true,
            interactive: true,
            badge: None,
        },
    ];

    DashboardState {
        current_theme: detected.effective_theme,
        detected_theme: detected,
        automation_enabled: config.theme.automatic,
        last_switched_formatted,
        next_transition: NextTransition {
            target_theme,
            event_name,
            time_formatted,
            countdown_formatted,
            location_note,
            status: SchedulerStatus::Real,
        },
        schedule_timeline: ScheduleTimeline {
            sunrise_time_formatted: format_minutes_as_12h(sunrise_minute),
            sunset_time_formatted: format_minutes_as_12h(sunset_minute),
            night1_percentage: night1,
            day_percentage: day,
            night2_percentage: night2,
            current_needle_percentage: needle,
            current_time_formatted,
            daylight_window_formatted: daylight_str,
            status: SchedulerStatus::Real,
        },
        quick_controls,
        app_info: AppInfo {
            app_name: "AutoDarkMode".to_string(),
            version: format!("v{}", env!("CARGO_PKG_VERSION")),
            target_platform: "Windows 11 x64".to_string(),
            webview_runtime: "WebView2 Runtime 122.0".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_times_calculation() {
        // Test New York coordinates: 40.7128° N, -74.0060° W, UTC-4 (EDT, -240 min)
        // Day 172 (around Summer Solstice, June 21)
        let (sunrise, sunset) = calculate_solar_times(172, 40.7128, -74.0060, -240);

        // In NYC around summer solstice, sunrise is ~5:25 AM (325 min), sunset ~8:30 PM (1230 min)
        assert!(sunrise >= 310 && sunrise <= 340, "Sunrise expected around 5:25 AM, got {}", format_minutes_as_12h(sunrise));
        assert!(sunset >= 1215 && sunset <= 1245, "Sunset expected around 8:30 PM, got {}", format_minutes_as_12h(sunset));
    }

    #[test]
    fn test_format_minutes_as_12h() {
        assert_eq!(format_minutes_as_12h(0), "12:00 AM");
        assert_eq!(format_minutes_as_12h(402), "6:42 AM");
        assert_eq!(format_minutes_as_12h(720), "12:00 PM");
        assert_eq!(format_minutes_as_12h(1068), "5:48 PM");
        assert_eq!(format_minutes_as_12h(1439), "11:59 PM");
    }

    #[test]
    fn test_calculate_timeline_percentages() {
        let sunrise = 402;
        let sunset = 1068;
        let current = 1338;

        let (night1, day, night2, needle, daylight) =
            calculate_timeline_metrics(sunrise, sunset, current);

        assert!((night1 - 27.91).abs() < 0.1);
        assert!((day - 46.25).abs() < 0.1);
        assert!((night2 - 25.83).abs() < 0.1);
        assert!((needle - 92.91).abs() < 0.1);
        assert_eq!(daylight, "11h 06m");

        let sum = night1 + day + night2;
        assert!((sum - 100.0).abs() < 0.001);
    }
}
