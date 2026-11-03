use serde::Deserialize;
use tauri::command;
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
    request: SetShortcutsConfigRequest,
) -> OperationResult<ShortcutsConfig> {
    info!("Handling set_shortcuts_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.shortcuts = request.config.clone();
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                OperationResult::ok(request.config)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}
