use chrono::{Datelike, Local, Timelike};
use tauri::{AppHandle, Emitter, Manager};
use tracing::{info, warn};

use crate::config::{AppConfig, ConfigManager};
use crate::state::{build_dashboard_state, calculate_solar_times};
use crate::windows::{
    apply_theme_file, apply_windows_theme, detect_windows_theme, is_fullscreen_active,
    power::get_power_status, set_desktop_wallpaper, ThemeMode,
};

/// Calculate the theme that should be active right now according to the schedule configuration.
pub fn evaluate_expected_theme(config: &AppConfig, now: &chrono::DateTime<Local>) -> ThemeMode {
    let now_minute = now.hour() * 60 + now.minute();

    match config.schedule.schedule_type.as_str() {
        "custom_times" => {
            let light_minute =
                config.schedule.light_start_hour * 60 + config.schedule.light_start_minute;
            let dark_minute =
                config.schedule.dark_start_hour * 60 + config.schedule.dark_start_minute;

            if light_minute < dark_minute {
                // Daytime is light (e.g., 07:00 to 19:00)
                if now_minute >= light_minute && now_minute < dark_minute {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                }
            } else if light_minute > dark_minute {
                // Inverted schedule (e.g., 19:00 to 07:00 is light)
                if now_minute >= dark_minute && now_minute < light_minute {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                }
            } else {
                config.theme.current
            }
        }
        "night_light" => {
            // Windows Night Light schedule (default: 06:00 to 20:00)
            let light_minute = 6 * 60;
            let dark_minute = 20 * 60;
            if now_minute >= light_minute && now_minute < dark_minute {
                ThemeMode::Light
            } else {
                ThemeMode::Dark
            }
        }
        _ => {
            // "sunrise_sunset" (default)
            let day_of_year = now.date_naive().ordinal();
            let tz_offset_minutes = now.offset().local_minus_utc() / 60;

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

            if sunrise < sunset {
                if now_minute >= sunrise && now_minute < sunset {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                }
            } else if sunrise > sunset {
                if now_minute >= sunset && now_minute < sunrise {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                }
            } else {
                config.theme.current
            }
        }
    }
}

/// Verify whether conditions permit switching themes right now.
pub fn check_conditions_allow_switch(config: &AppConfig) -> (bool, Option<&'static str>) {
    // 1. Power / Battery Condition
    if config.conditions.pause_on_battery || config.schedule.pause_on_battery {
        let power = get_power_status();
        if power.is_on_battery {
            if config.conditions.force_dark_on_battery_saver && power.is_battery_saver_on {
                // Force dark is permitted on battery saver
                return (true, None);
            }
            return (false, Some("Paused while running on battery power"));
        }
    }

    // 2. Gaming / Fullscreen Condition
    if config.conditions.pause_on_gaming || config.schedule.pause_on_gaming {
        if is_fullscreen_active() {
            return (false, Some("Paused while gaming or fullscreen application active"));
        }
    }

    (true, None)
}

/// Executes a single evaluation pass. If a switch is required and allowed, applies it immediately.
pub fn perform_evaluation_tick(app: &AppHandle) {
    let manager = match ConfigManager::init_default() {
        Ok(m) => m,
        Err(e) => {
            warn!("Scheduler failed to initialize ConfigManager: {}", e);
            return;
        }
    };

    let mut config = match manager.load_or_create() {
        Ok(c) => c,
        Err(e) => {
            warn!("Scheduler failed to load configuration: {}", e);
            return;
        }
    };

    // If automatic switching is disabled globally or in schedule, take no action
    if !config.theme.automatic || !config.schedule.enabled {
        return;
    }

    let now = Local::now();
    let expected_theme = evaluate_expected_theme(&config, &now);

    let detected = match detect_windows_theme() {
        Ok(d) => d,
        Err(e) => {
            warn!("Scheduler failed to detect Windows theme: {}", e);
            return;
        }
    };

    // If already in the expected theme, nothing to change
    if detected.effective_theme == expected_theme {
        return;
    }

    // Verify conditions
    let (allowed, reason) = check_conditions_allow_switch(&config);
    if !allowed {
        info!(
            "Theme transition to {:?} postponed by condition: {:?}",
            expected_theme, reason
        );
        return;
    }

    info!(
        "Triggering scheduled theme transition from {:?} to {:?}",
        detected.effective_theme, expected_theme
    );

    // 1. Apply to Windows personalization
    if let Err(e) = apply_windows_theme(expected_theme) {
        warn!("Failed to apply scheduled theme switch: {}", e);
        return;
    }

    // 1b. Apply configured Windows .theme file if enabled
    if config.theme.use_theme_files {
        let theme_file = match expected_theme {
            ThemeMode::Light => &config.theme.light_theme_path,
            ThemeMode::Dark => &config.theme.dark_theme_path,
        };
        if !theme_file.is_empty() {
            let _ = apply_theme_file(theme_file);
        }
    }

    // 2. Update native window DWM theme
    if let Some(window) = app.get_webview_window("main") {
        #[cfg(target_os = "windows")]
        {
            if let Ok(hwnd) = window.hwnd() {
                crate::windows::window::update_window_theme(
                    hwnd.0 as isize,
                    expected_theme == ThemeMode::Dark,
                );
            }
        }
    }

    // 3. Apply Wallpaper if enabled
    if config.personalization.wallpaper_enabled || config.features.wallpaper {
        let wp_path = match expected_theme {
            ThemeMode::Light => &config.personalization.light_wallpaper_path,
            ThemeMode::Dark => &config.personalization.dark_wallpaper_path,
        };
        if !wp_path.is_empty() {
            let _ = set_desktop_wallpaper(wp_path, &config.personalization.wallpaper_fit);
        }
    }

    // 4. Update configuration with timestamp and trigger source
    config.theme.current = expected_theme;
    config.last_switched_timestamp = Some(now.timestamp());
    config.last_switched_trigger = Some(match config.schedule.schedule_type.as_str() {
        "custom_times" => "Custom schedule".to_string(),
        "night_light" => "Night Light schedule".to_string(),
        _ => "Solar schedule".to_string(),
    });
    let _ = manager.save(&config);

    // 5. Emit real-time events to frontend
    let new_detected = detect_windows_theme().unwrap_or(detected);
    let _ = app.emit("theme-changed", &new_detected);

    let dashboard_state = build_dashboard_state(&config);
    let _ = app.emit("dashboard-updated", &dashboard_state);
}

/// Spawns the background scheduler thread inside Tauri's async runtime.
pub fn start_scheduler(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        info!("Starting AutoDarkMode background scheduler daemon...");

        // Startup evaluation: immediately synchronize if the app was closed during transition
        perform_evaluation_tick(&app);

        // Evaluation ticker loop: every 2 seconds
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
        loop {
            interval.tick().await;
            perform_evaluation_tick(&app);
        }
    });
}

/// Triggers an immediate evaluation without waiting for the next 2-second tick.
pub fn trigger_immediate_evaluation(app: &AppHandle) {
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        perform_evaluation_tick(&app_clone);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

    #[test]
    fn test_custom_times_day_evaluation() {
        let mut config = AppConfig::default();
        config.schedule.schedule_type = "custom_times".to_string();
        config.schedule.light_start_hour = 8;
        config.schedule.light_start_minute = 0;
        config.schedule.dark_start_hour = 20;
        config.schedule.dark_start_minute = 0;

        // 12:00 PM should be Light
        let dt = Local
            .from_local_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ))
            .single()
            .unwrap();
        assert_eq!(evaluate_expected_theme(&config, &dt), ThemeMode::Light);

        // 21:00 (9:00 PM) should be Dark
        let dt_night = Local
            .from_local_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
                NaiveTime::from_hms_opt(21, 0, 0).unwrap(),
            ))
            .single()
            .unwrap();
        assert_eq!(evaluate_expected_theme(&config, &dt_night), ThemeMode::Dark);

        // 06:30 AM should be Dark
        let dt_early = Local
            .from_local_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
                NaiveTime::from_hms_opt(6, 30, 0).unwrap(),
            ))
            .single()
            .unwrap();
        assert_eq!(evaluate_expected_theme(&config, &dt_early), ThemeMode::Dark);
    }

    #[test]
    fn test_custom_times_inverted_evaluation() {
        let mut config = AppConfig::default();
        config.schedule.schedule_type = "custom_times".to_string();
        config.schedule.light_start_hour = 21; // Light at 9:00 PM
        config.schedule.light_start_minute = 0;
        config.schedule.dark_start_hour = 5;  // Dark at 5:00 AM
        config.schedule.dark_start_minute = 0;

        // 12:00 PM (noon) should be Dark
        let dt_noon = Local
            .from_local_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ))
            .single()
            .unwrap();
        assert_eq!(evaluate_expected_theme(&config, &dt_noon), ThemeMode::Dark);

        // 23:00 (11:00 PM) should be Light
        let dt_night = Local
            .from_local_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
                NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
            ))
            .single()
            .unwrap();
        assert_eq!(evaluate_expected_theme(&config, &dt_night), ThemeMode::Light);
    }
}
