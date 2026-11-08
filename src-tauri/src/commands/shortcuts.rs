use serde::Deserialize;
use tauri::{command, Manager};
use tracing::info;

use super::OperationResult;
use crate::config::{ConfigManager, ShortcutsConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct SetShortcutsConfigRequest {
    pub config: ShortcutsConfig,
}

#[command]
pub async fn get_shortcuts_config() -> OperationResult<ShortcutsConfig> {
    info!("Handling get_shortcuts_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => OperationResult::ok(config.shortcuts),
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_shortcuts_config(
    app: tauri::AppHandle,
    request: SetShortcutsConfigRequest,
) -> OperationResult<ShortcutsConfig> {
    info!("Handling set_shortcuts_config command");
    let mut saved = false;
    if let Some(state) = app.try_state::<crate::config::AppState>() {
        let _ = state.update_config(|config| {
            config.shortcuts = request.config.clone();
            saved = true;
        });
    } else if let Ok(manager) = ConfigManager::init_default() {
        if let Ok(mut config) = manager.load_or_create() {
            config.shortcuts = request.config.clone();
            if let Err(e) = manager.save(&config) {
                return OperationResult::err(format!("Failed saving configuration: {}", e));
            }
            saved = true;
        }
    }

    if saved {
        crate::windows::hotkeys::update_registered_hotkeys();
        OperationResult::ok(request.config)
    } else {
        OperationResult::err("Failed to persist shortcuts configuration".to_string())
    }
}
