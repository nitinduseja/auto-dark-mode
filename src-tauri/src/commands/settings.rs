use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::LazyLock;
use std::time::Instant;
use tauri::{command, AppHandle};
use tracing::info;

static APP_START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

pub fn init_app_start_time() {
    let _ = *APP_START_TIME;
}

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
        match Command::new("cmd").args(["/c", "start", "", &url]).spawn() {
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

    let (os_name, os_build) = {
        #[cfg(target_os = "windows")]
        {
            let product = crate::windows::registry::read_hklm_string(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
                "ProductName",
            ).unwrap_or_else(|_| "Microsoft Windows".to_string());

            let display_version = crate::windows::registry::read_hklm_string(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
                "DisplayVersion",
            ).unwrap_or_default();

            let build_num = crate::windows::registry::read_hklm_string(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
                "CurrentBuildNumber",
            ).unwrap_or_else(|_| "Unknown Build".to_string());

            let build_str = if display_version.is_empty() {
                format!("Build {}", build_num)
            } else {
                format!("{} (Build {})", display_version, build_num)
            };

            (product, build_str)
        }
        #[cfg(not(target_os = "windows"))]
        {
            ("Desktop OS".to_string(), "Development Build".to_string())
        }
    };

    let uptime = APP_START_TIME.elapsed().as_secs();

    OperationResult::ok(DiagnosticsInfo {
        os_name,
        os_build,
        arch: std::env::consts::ARCH.to_string(),
        tauri_version: tauri::VERSION.to_string(),
        webview_version: "Edge / WebView2 (Chromium Evergreen)".to_string(),
        config_path,
        uptime_seconds: uptime,
    })
}
