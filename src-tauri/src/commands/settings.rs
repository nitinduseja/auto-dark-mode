use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::{command, AppHandle};
use tracing::info;

use super::OperationResult;
use crate::config::{AppConfig, ConfigManager, SettingsConfig};
use crate::scheduler::trigger_immediate_evaluation;
use crate::state::AppInfo;

#[derive(Debug, Clone, Deserialize)]
pub struct SetSettingsConfigRequest {
    pub config: SettingsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportConfigRequest {
    pub json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsInfo {
    pub os_name: String,
    pub os_build: String,
    pub arch: String,
    pub tauri_version: String,
    pub webview_version: String,
    pub config_path: String,
    pub uptime_seconds: u64,
}

#[command]
pub async fn get_application_info() -> OperationResult<AppInfo> {
    info!("Handling get_application_info command");
    OperationResult::ok(AppInfo {
        app_name: "AutoDarkMode".to_string(),
        version: format!("v{}", env!("CARGO_PKG_VERSION")),
        target_platform: "Windows 11 x64".to_string(),
        webview_runtime: "WebView2 Runtime 122.0".to_string(),
    })
}

#[command]
pub async fn get_configuration() -> OperationResult<AppConfig> {
    info!("Handling get_configuration command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => OperationResult::ok(config),
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn set_settings_config(
    request: SetSettingsConfigRequest,
) -> OperationResult<SettingsConfig> {
    info!("Handling set_settings_config command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(mut config) => {
                config.settings = request.config.clone();
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

#[command]
pub async fn export_configuration() -> OperationResult<String> {
    info!("Handling export_configuration command");
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => match serde_json::to_string_pretty(&config) {
                Ok(json) => OperationResult::ok(json),
                Err(e) => OperationResult::err(format!("Failed to serialize config: {}", e)),
            },
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn import_configuration(
    app: AppHandle,
    request: ImportConfigRequest,
) -> OperationResult<AppConfig> {
    info!("Handling import_configuration command");
    match serde_json::from_str::<AppConfig>(&request.json) {
        Ok(new_config) => {
            if let Err(e) = new_config.validate() {
                return OperationResult::err(format!("Import validation failed: {}", e));
            }
            match ConfigManager::init_default() {
                Ok(manager) => {
                    if let Err(e) = manager.save(&new_config) {
                        return OperationResult::err(format!("Failed saving imported config: {}", e));
                    }
                    trigger_immediate_evaluation(&app);
                    OperationResult::ok(new_config)
                }
                Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
            }
        }
        Err(e) => OperationResult::err(format!("Invalid configuration JSON format: {}", e)),
    }
}

#[command]
pub async fn reset_to_default_configuration(app: AppHandle) -> OperationResult<AppConfig> {
    info!("Handling reset_to_default_configuration command");
    let default_config = AppConfig::default();
    match ConfigManager::init_default() {
        Ok(manager) => {
            if let Err(e) = manager.save(&default_config) {
                return OperationResult::err(format!("Failed resetting configuration: {}", e));
            }
            trigger_immediate_evaluation(&app);
            OperationResult::ok(default_config)
        }
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}

#[command]
pub async fn open_config_folder() -> OperationResult<bool> {
    info!("Handling open_config_folder command");
    match ConfigManager::default_path() {
        Ok(path) => {
            if let Some(parent) = path.parent() {
                let _ = Command::new("explorer").arg(parent).spawn();
                OperationResult::ok(true)
            } else {
                OperationResult::err("Config parent directory not found".to_string())
            }
        }
        Err(e) => OperationResult::err(format!("Failed resolving config path: {}", e)),
    }
}

#[command]
pub async fn open_external_url(url: String) -> OperationResult<bool> {
    info!("Handling open_external_url command: {}", url);
    if !url.starts_with("https://") && !url.starts_with("http://") {
        return OperationResult::err("Invalid URL: only HTTP and HTTPS protocols are allowed".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        match Command::new("rundll32").args(["url.dll,FileProtocolHandler", &url]).spawn() {
            Ok(_) => OperationResult::ok(true),
            Err(e) => OperationResult::err(format!("Failed to open URL in browser: {}", e)),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let launcher = if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
        match Command::new(launcher).arg(&url).spawn() {
            Ok(_) => OperationResult::ok(true),
            Err(e) => OperationResult::err(format!("Failed to open URL in browser: {}", e)),
        }
    }
}

#[command]
pub async fn get_diagnostics_info() -> OperationResult<DiagnosticsInfo> {
    info!("Handling get_diagnostics_info command");
    let config_path = ConfigManager::default_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    OperationResult::ok(DiagnosticsInfo {
        os_name: "Microsoft Windows 11".to_string(),
        os_build: "10.0.26100 (24H2 / Sun Valley 3)".to_string(),
        arch: std::env::consts::ARCH.to_string(),
        tauri_version: "2.12.5".to_string(),
        webview_version: "Edge / WebView2 122.0.2365.92".to_string(),
        config_path,
        uptime_seconds: 42,
    })
}
