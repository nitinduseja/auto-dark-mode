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

pub fn apply_theme_internally(app: &AppHandle, target_theme: ThemeMode, trigger: &str) -> Result<DetectedTheme, String> {
    info!("Executing theme application for target: {:?}, trigger: {}", target_theme, trigger);

    // 1. Apply to Windows personalization settings
    match apply_windows_theme(target_theme) {
        Ok(detected) => {
            // 2. Update native window DWM dark mode frame across all open windows (main and tray-popup)
            crate::windows::window::update_all_windows_theme(app, target_theme == ThemeMode::Dark);

            // 3. Persist in configuration and apply Windows .theme file / wallpaper if configured
            let mut updated_config = None;
            if let Some(state) = app.try_state::<AppState>() {
                let _ = state.update_config(|config| {
                    config.theme.current = target_theme;
                    config.last_switched_timestamp = Some(Local::now().timestamp());
                    config.last_switched_trigger = Some(trigger.to_string());
                    if config.theme.automatic {
                        config.manual_override = Some(ManualOverride {
                            target_theme,
                            until_timestamp: None, // Respect until next scheduled transition
                            reason: trigger.to_string(),
                        });
                    }
                    updated_config = Some(config.clone());
                });
            } else if let Ok(manager) = ConfigManager::init_default() {
                if let Ok(mut config) = manager.load_or_create() {
                    config.theme.current = target_theme;
                    config.last_switched_timestamp = Some(Local::now().timestamp());
                    config.last_switched_trigger = Some(trigger.to_string());
                    if config.theme.automatic {
                        config.manual_override = Some(ManualOverride {
                            target_theme,
                            until_timestamp: None,
                            reason: trigger.to_string(),
                        });
                    }
                    let _ = manager.save(&config);
                    updated_config = Some(config);
                }
            }

            if let Some(config) = updated_config {
                // If user configured a specific .theme file, apply it
                if config.theme.use_theme_files {
                    let theme_file = match target_theme {
                        ThemeMode::Light => &config.theme.light_theme_path,
                        ThemeMode::Dark => &config.theme.dark_theme_path,
                    };
                    if !theme_file.is_empty() {
                        let _ = apply_theme_file(theme_file);
                    }
                }

                // Apply Wallpaper if enabled
                if config.personalization.wallpaper_enabled || config.features.wallpaper {
                    let wp_path = match target_theme {
                        ThemeMode::Light => &config.personalization.light_wallpaper_path,
                        ThemeMode::Dark => &config.personalization.dark_wallpaper_path,
                    };
                    if !wp_path.is_empty() {
                        let _ = set_desktop_wallpaper(wp_path, &config.personalization.wallpaper_fit);
                    }
                }

                // Apply Accent Color if enabled
                if config.personalization.sync_accent_color {
                    let accent = match target_theme {
                        ThemeMode::Light => &config.personalization.light_accent_color,
                        ThemeMode::Dark => &config.personalization.dark_accent_color,
                    };
                    let _ = apply_accent_color(accent);
                }

                if config.personalization.sync_cursor {
                    let _ = apply_cursor_scheme(target_theme == ThemeMode::Dark);
                }

                if config.personalization.sync_sounds {
                    let _ = apply_sound_scheme(target_theme == ThemeMode::Dark);
                }

                if config.personalization.sync_color_filter {
                    let _ = crate::windows::sync_color_filter(target_theme == ThemeMode::Dark);
                }

                if config.personalization.sync_touch_keyboard {
                    let _ = crate::windows::sync_touch_keyboard_theme(target_theme == ThemeMode::Dark);
                }

                if config.apps.sync_office {
                    let _ = crate::windows::sync_office_theme(target_theme == ThemeMode::Dark);
                }

                if config.apps.sync_browsers {
                    let _ = crate::windows::sync_vscode_theme(target_theme == ThemeMode::Dark);
                    let _ = crate::windows::sync_windows_terminal_theme(target_theme == ThemeMode::Dark);
                }

                let target_theme_str = match target_theme {
                    ThemeMode::Light => "light",
                    ThemeMode::Dark => "dark",
                };
                for rule in &config.apps.custom_rules {
                    if rule.enabled && (rule.target_theme == "any" || rule.target_theme == target_theme_str) {
                        let _ = crate::windows::execute_custom_script(&rule.process_name, "");
                    }
                }

                if config.automation.notify_on_switch {
                    crate::windows::show_theme_switch_notification(match target_theme {
                        ThemeMode::Light => "Light",
                        ThemeMode::Dark => "Dark",
                    });
                }
            }

            Ok(detected)
        }
        Err(e) => {
            error!("Failed to apply theme {:?}: {}", target_theme, e);
            Err(format!("Windows theme switch error: {}", e))
        }
    }
}

pub fn postpone_internally(app: &AppHandle, minutes: u32) -> Result<bool, String> {
    info!("Executing internal postpone for {} minutes", minutes);

    let detected = match crate::windows::detect_windows_theme() {
        Ok(d) => d,
        Err(e) => return Err(format!("Failed to detect theme: {}", e)),
    };

    let until = Local::now().timestamp() + (minutes as i64 * 60);

    let res = if let Some(state) = app.try_state::<AppState>() {
        state.update_config(|config| {
            config.manual_override = Some(ManualOverride {
                target_theme: detected.effective_theme,
                until_timestamp: Some(until),
                reason: format!("Postponed for {}m", minutes),
            });
        }).map_err(|e| e.to_string())
    } else if let Ok(manager) = ConfigManager::init_default() {
        match manager.load_or_create() {
            Ok(mut config) => {
                config.manual_override = Some(ManualOverride {
                    target_theme: detected.effective_theme,
                    until_timestamp: Some(until),
                    reason: format!("Postponed for {}m", minutes),
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
            trigger_immediate_evaluation(app);
            Ok(true)
        }
        Err(e) => Err(e),
    }
}

#[command]
pub async fn set_theme(app: AppHandle, request: SetThemeRequest) -> OperationResult<DetectedTheme> {
    info!("Handling set_theme request for target: {:?}", request.theme);
    match apply_theme_internally(&app, request.theme, "Manual switch") {
        Ok(detected) => OperationResult::ok(detected),
        Err(e) => OperationResult::err(e),
    }
}

#[command]
pub async fn postpone_switch(app: AppHandle, request: PostponeRequest) -> OperationResult<bool> {
    info!("Handling postpone_switch command for {} minutes", request.minutes);
    match postpone_internally(&app, request.minutes) {
        Ok(success) => OperationResult::ok(success),
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
