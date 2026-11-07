use chrono::Local;
use serde::Deserialize;
use tauri::{command, AppHandle, Manager};
use tracing::{error, info};

use super::OperationResult;
use crate::config::{AppState, ConfigManager, ManualOverride, ThemeConfig};
use crate::scheduler::trigger_immediate_evaluation;
use crate::windows::{
    apply_accent_color, apply_cursor_scheme, apply_sound_scheme, apply_theme_file,
    apply_windows_theme, discover_installed_themes, set_desktop_wallpaper, DetectedTheme,
    InstalledTheme, ThemeMode,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SetThemeRequest {
    pub theme: ThemeMode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostponeRequest {
    pub minutes: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetAutomationRequest {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetFollowSunRequest {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetQuickControlRequest {
    pub id: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetModeThemesRequest {
    pub use_theme_files: bool,
    pub light_theme_path: String,
    pub light_theme_name: String,
    pub dark_theme_path: String,
    pub dark_theme_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplySpecificThemeRequest {
    pub path: String,
}

#[command]
pub async fn set_theme(app: AppHandle, request: SetThemeRequest) -> OperationResult<DetectedTheme> {
    info!("Handling set_theme request for target: {:?}", request.theme);

    // 1. Apply to Windows personalization settings
    match apply_windows_theme(request.theme) {
        Ok(detected) => {
            // 2. Update native window DWM dark mode frame
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "windows")]
                {
                    if let Ok(hwnd) = window.hwnd() {
                        crate::windows::window::update_window_theme(
                            hwnd.0 as isize,
                            request.theme == ThemeMode::Dark,
                        );
                    }
                }
            }

            // 3. Persist in configuration and apply Windows .theme file / wallpaper if configured
            let mut updated_config = None;
            if let Some(state) = app.try_state::<AppState>() {
                let _ = state.update_config(|config| {
                    config.theme.current = request.theme;
                    config.last_switched_timestamp = Some(Local::now().timestamp());
                    config.last_switched_trigger = Some("Manual switch".to_string());
                    if config.theme.automatic {
                        config.manual_override = Some(ManualOverride {
                            target_theme: request.theme,
                            until_timestamp: None, // Respect until next scheduled transition
                            reason: "Manual switch".to_string(),
                        });
                    }
                    updated_config = Some(config.clone());
                });
            } else if let Ok(manager) = ConfigManager::init_default() {
                if let Ok(mut config) = manager.load_or_create() {
                    config.theme.current = request.theme;
                    config.last_switched_timestamp = Some(Local::now().timestamp());
                    config.last_switched_trigger = Some("Manual switch".to_string());
                    if config.theme.automatic {
                        config.manual_override = Some(ManualOverride {
                            target_theme: request.theme,
                            until_timestamp: None,
                            reason: "Manual switch".to_string(),
                        });
                    }
                    let _ = manager.save(&config);
                    updated_config = Some(config);
                }
            }

            if let Some(config) = updated_config {
                // If user configured a specific .theme file, apply it
                if config.theme.use_theme_files {
                    let theme_file = match request.theme {
                        ThemeMode::Light => &config.theme.light_theme_path,
                        ThemeMode::Dark => &config.theme.dark_theme_path,
                    };
                    if !theme_file.is_empty() {
                        let _ = apply_theme_file(theme_file);
                    }
                }

                // Apply Wallpaper if enabled
                if config.personalization.wallpaper_enabled || config.features.wallpaper {
                    let wp_path = match request.theme {
                        ThemeMode::Light => &config.personalization.light_wallpaper_path,
                        ThemeMode::Dark => &config.personalization.dark_wallpaper_path,
                    };
                    if !wp_path.is_empty() {
                        let _ = set_desktop_wallpaper(wp_path, &config.personalization.wallpaper_fit);
                    }
                }

                // Apply Accent Color if enabled
                if config.personalization.sync_accent_color {
                    let accent = match request.theme {
                        ThemeMode::Light => &config.personalization.light_accent_color,
                        ThemeMode::Dark => &config.personalization.dark_accent_color,
                    };
                    let _ = apply_accent_color(accent);
                }

                if config.personalization.sync_cursor {
                    let _ = apply_cursor_scheme(request.theme == ThemeMode::Dark);
                }

                if config.personalization.sync_sounds {
                    let _ = apply_sound_scheme(request.theme == ThemeMode::Dark);
                }
            }

            OperationResult::ok(detected)
        }
        Err(e) => {
            error!("Failed to apply theme {:?}: {}", request.theme, e);
            OperationResult::err(format!("Windows theme switch error: {}", e))
        }
    }
}

#[command]
pub async fn postpone_switch(app: AppHandle, request: PostponeRequest) -> OperationResult<bool> {
    info!("Handling postpone_switch command for {} minutes", request.minutes);

    let detected = match crate::windows::detect_windows_theme() {
        Ok(d) => d,
        Err(e) => return OperationResult::err(format!("Failed to detect theme: {}", e)),
    };

    let until = Local::now().timestamp() + (request.minutes as i64 * 60);

    let res = if let Some(state) = app.try_state::<AppState>() {
        state.update_config(|config| {
            config.manual_override = Some(ManualOverride {
                target_theme: detected.effective_theme,
                until_timestamp: Some(until),
                reason: format!("Postponed for {}m", request.minutes),
            });
        }).map_err(|e| e.to_string())
    } else if let Ok(manager) = ConfigManager::init_default() {
        match manager.load_or_create() {
            Ok(mut config) => {
                config.manual_override = Some(ManualOverride {
                    target_theme: detected.effective_theme,
                    until_timestamp: Some(until),
                    reason: format!("Postponed for {}m", request.minutes),
                });
                manager.save(&config).map_err(|e| e.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Failed to initialize config manager".to_string())
    };

    match res {
        Ok(_) => {
            trigger_immediate_evaluation(&app);
            OperationResult::ok(true)
        }
        Err(e) => OperationResult::err(e),
    }
}

#[command]
pub async fn resume_automation(app: AppHandle) -> OperationResult<bool> {
    info!("Handling resume_automation command (clearing manual override)");

    let res = if let Some(state) = app.try_state::<AppState>() {
        state.update_config(|config| {
            config.manual_override = None;
            config.theme.automatic = true;
        }).map_err(|e| e.to_string())
    } else if let Ok(manager) = ConfigManager::init_default() {
        match manager.load_or_create() {
            Ok(mut config) => {
                config.manual_override = None;
                config.theme.automatic = true;
                manager.save(&config).map_err(|e| e.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Failed to initialize config manager".to_string())
    };

    match res {
        Ok(_) => {
            trigger_immediate_evaluation(&app);
            OperationResult::ok(true)
        }
        Err(e) => OperationResult::err(e),
    }
}

#[command]
pub async fn set_automation_enabled(
    app: AppHandle,
    request: SetAutomationRequest,
) -> OperationResult<bool> {
    info!(
        "Handling set_automation_enabled request: {}",
        request.enabled
    );

    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.theme.automatic = request.enabled;
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(request.enabled)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_follow_sun_enabled(
    app: AppHandle,
    request: SetFollowSunRequest,
) -> OperationResult<bool> {
    info!(
        "Handling set_follow_sun_enabled request: {}",
        request.enabled
    );

    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.schedule.enabled = request.enabled;
                if request.enabled {
                    config.schedule.schedule_type = "sunrise_sunset".to_string();
                }
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(request.enabled)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_quick_control(
    app: AppHandle,
    request: SetQuickControlRequest,
) -> OperationResult<bool> {
    info!(
        "Handling set_quick_control request for id: {}, enabled: {}",
        request.id, request.enabled
    );

    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                match request.id.as_str() {
                    "automatic_switching" => config.theme.automatic = request.enabled,
                    "follow_sun" => {
                        config.schedule.enabled = request.enabled;
                        if request.enabled {
                            config.schedule.schedule_type = "sunrise_sunset".to_string();
                        }
                    }
                    "wallpaper" => {
                        config.features.wallpaper = request.enabled;
                        config.personalization.wallpaper_enabled = request.enabled;
                    }
                    "app_themes" => config.features.app_themes = request.enabled,
                    unknown => {
                        return OperationResult::err(format!("Unknown control identifier: {}", unknown));
                    }
                }
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(request.enabled)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn get_installed_themes() -> OperationResult<Vec<InstalledTheme>> {
    info!("Handling get_installed_themes command");
    let themes = discover_installed_themes();
    OperationResult::ok(themes)
}

#[command]
pub async fn set_mode_themes(
    app: AppHandle,
    request: SetModeThemesRequest,
) -> OperationResult<ThemeConfig> {
    info!(
        "Handling set_mode_themes command: Light='{}', Dark='{}'",
        request.light_theme_name, request.dark_theme_name
    );

    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.theme.use_theme_files = request.use_theme_files;
                config.theme.light_theme_path = request.light_theme_path;
                config.theme.light_theme_name = request.light_theme_name;
                config.theme.dark_theme_path = request.dark_theme_path;
                config.theme.dark_theme_name = request.dark_theme_name;

                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(config.theme)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn apply_specific_theme(
    request: ApplySpecificThemeRequest,
) -> OperationResult<bool> {
    info!("Handling apply_specific_theme for path: {}", request.path);
    match apply_theme_file(&request.path) {
        Ok(_) => OperationResult::ok(true),
        Err(e) => OperationResult::err(e),
    }
}
