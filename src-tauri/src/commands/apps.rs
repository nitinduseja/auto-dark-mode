use serde::Deserialize;
use tauri::{command, AppHandle};
use tracing::info;

use super::OperationResult;
use crate::config::{AppsConfig, ConfigManager, CustomAppRule};
use crate::scheduler::trigger_immediate_evaluation;

#[derive(Debug, Clone, Deserialize)]
pub struct SetAppsConfigRequest {
    pub config: AppsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddRuleRequest {
    pub rule: CustomAppRule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleIdRequest {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToggleRuleRequest {
    pub id: String,
    pub enabled: bool,
}

#[command]
pub async fn get_apps_config() -> OperationResult<AppsConfig> {
    info!("Handling get_apps_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => OperationResult::ok(config.apps),
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_apps_config(
    app: AppHandle,
    request: SetAppsConfigRequest,
) -> OperationResult<AppsConfig> {
    info!("Handling set_apps_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.apps = request.config.clone();
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
pub async fn add_custom_app_rule(
    app: AppHandle,
    request: AddRuleRequest,
) -> OperationResult<AppsConfig> {
    info!("Handling add_custom_app_rule command: {}", request.rule.name);
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                // Remove if already exists with same ID
                config.apps.custom_rules.retain(|r| r.id != request.rule.id);
                config.apps.custom_rules.push(request.rule);
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(config.apps)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn remove_custom_app_rule(
    app: AppHandle,
    request: RuleIdRequest,
) -> OperationResult<AppsConfig> {
    info!("Handling remove_custom_app_rule command: {}", request.id);
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.apps.custom_rules.retain(|r| r.id != request.id);
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(config.apps)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn toggle_custom_app_rule(
    app: AppHandle,
    request: ToggleRuleRequest,
) -> OperationResult<AppsConfig> {
    info!(
        "Handling toggle_custom_app_rule command: {} -> {}",
        request.id, request.enabled
    );
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                if let Some(rule) = config.apps.custom_rules.iter_mut().find(|r| r.id == request.id)
                {
                    rule.enabled = request.enabled;
                }
                if let Err(e) = manager.save(&config) {
                    return OperationResult::err(format!("Failed saving configuration: {}", e));
                }
                trigger_immediate_evaluation(&app);
                OperationResult::ok(config.apps)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}
