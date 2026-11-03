use serde::Deserialize;
use tauri::{command, AppHandle};
use tracing::info;

use super::OperationResult;
use crate::config::{AutomationConfig, ConfigManager};
use crate::windows::{is_run_at_startup, set_run_at_startup};

#[derive(Debug, Clone, Deserialize)]
pub struct SetAutomationConfigRequest {
    pub config: AutomationConfig,
}

#[command]
pub async fn get_automation_config() -> OperationResult<AutomationConfig> {
    info!("Handling get_automation_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                // Synchronize startup flag with actual Windows registry state
                config.automation.launch_on_startup = is_run_at_startup();
                OperationResult::ok(config.automation)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_automation_config(
    _app: AppHandle,
    request: SetAutomationConfigRequest,
) -> OperationResult<AutomationConfig> {
    info!("Handling set_automation_config command");

    // Apply Windows startup registry change
    if let Err(e) = set_run_at_startup(
        request.config.launch_on_startup,
        request.config.start_minimized,
    ) {
        return OperationResult::err(format!("Failed to update Windows startup: {}", e));
    }

    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.automation = request.config.clone();
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
