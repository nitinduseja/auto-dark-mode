use serde::Deserialize;
use tauri::{command, AppHandle};
use tracing::info;

use super::OperationResult;
use crate::config::{ConfigManager, PersonalizationConfig};
use crate::scheduler::trigger_immediate_evaluation;
use crate::windows::{set_desktop_wallpaper, ThemeMode};

#[derive(Debug, Clone, Deserialize)]
pub struct SetPersonalizationConfigRequest {
    pub config: PersonalizationConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestWallpaperRequest {
    pub theme: ThemeMode,
}

#[command]
pub async fn get_personalization_config() -> OperationResult<PersonalizationConfig> {
    info!("Handling get_personalization_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => OperationResult::ok(config.personalization),
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_personalization_config(
    app: AppHandle,
    request: SetPersonalizationConfigRequest,
) -> OperationResult<PersonalizationConfig> {
    info!("Handling set_personalization_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.personalization = request.config.clone();
                config.features.wallpaper = request.config.wallpaper_enabled;
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(request.config)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn test_apply_wallpaper(request: TestWallpaperRequest) -> OperationResult<bool> {
    info!(
        "Handling test_apply_wallpaper for theme: {:?}",
        request.theme
    );
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => {
                let path = match request.theme {
                    ThemeMode::Light => &config.personalization.light_wallpaper_path,
                    ThemeMode::Dark => &config.personalization.dark_wallpaper_path,
                };

                if path.trim().is_empty() {
                    return OperationResult::err("No wallpaper file path is specified for this theme mode".to_string());
                }

                match set_desktop_wallpaper(path, &config.personalization.wallpaper_fit) {
                    Ok(_) => OperationResult::ok(true),
                    Err(e) => OperationResult::err(e),
                }
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}
