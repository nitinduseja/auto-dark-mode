use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle};
use tracing::info;

use super::OperationResult;
use crate::config::{ConditionsConfig, ConfigManager};
use crate::scheduler::trigger_immediate_evaluation;
use crate::windows::{get_power_status, is_fullscreen_active, PowerStatus};

#[derive(Debug, Clone, Deserialize)]
pub struct SetConditionsConfigRequest {
    pub config: ConditionsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConditionsStatus {
    pub power: PowerStatus,
    pub is_gaming_active: bool,
    pub is_switch_allowed: bool,
    pub restriction_reason: Option<String>,
}

#[command]
pub async fn get_conditions_config() -> OperationResult<ConditionsConfig> {
    info!("Handling get_conditions_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => OperationResult::ok(config.conditions),
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_conditions_config(
    app: AppHandle,
    request: SetConditionsConfigRequest,
) -> OperationResult<ConditionsConfig> {
    info!("Handling set_conditions_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.conditions = request.config.clone();
                // Synchronize with legacy schedule subfields
                config.schedule.pause_on_battery = request.config.pause_on_battery;
                config.schedule.pause_on_gaming = request.config.pause_on_gaming;
                config.schedule.idle_delay_minutes = request.config.idle_delay_minutes;

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
pub async fn get_system_conditions_status() -> OperationResult<SystemConditionsStatus> {
    let power = get_power_status();
    let is_gaming = is_fullscreen_active();

    let mut allowed = true;
    let mut reason = None;

    if let Ok(manager) = ConfigManager::init_default() {
        if let Ok(config) = manager.load_or_create() {
            if config.conditions.pause_on_battery && power.is_on_battery {
                if !(config.conditions.force_dark_on_battery_saver && power.is_battery_saver_on) {
                    allowed = false;
                    reason = Some("Running on battery power".to_string());
                }
            } else if config.conditions.pause_on_gaming && is_gaming {
                allowed = false;
                reason = Some("Fullscreen / gaming application active".to_string());
            }
        }
    }

    OperationResult::ok(SystemConditionsStatus {
        power,
        is_gaming_active: is_gaming,
        is_switch_allowed: allowed,
        restriction_reason: reason,
    })
}
