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
pub async fn select_wallpaper_file() -> OperationResult<Option<String>> {
    info!("Handling select_wallpaper_file command");
    #[cfg(target_os = "windows")]
    {
        use windows::core::{PCWSTR, PWSTR};
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::Controls::Dialogs::{
            GetOpenFileNameW, OFN_FILEMUSTEXIST, OFN_PATHMUSTEXIST, OPENFILENAMEW,
        };

        let mut file_buf = [0u16; 1024];
        let filter: Vec<u16> = "Images (*.jpg;*.jpeg;*.png;*.bmp;*.webp)\0*.jpg;*.jpeg;*.png;*.bmp;*.webp\0All Files (*.*)\0*.*\0\0"
            .encode_utf16()
            .collect();
        let title: Vec<u16> = "Select Wallpaper Image\0".encode_utf16().collect();

        let mut ofn = OPENFILENAMEW {
            lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
            hwndOwner: HWND::default(),
            lpstrFilter: PCWSTR(filter.as_ptr()),
            lpstrFile: PWSTR(file_buf.as_mut_ptr()),
            nMaxFile: file_buf.len() as u32,
            lpstrTitle: PCWSTR(title.as_ptr()),
            Flags: OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST,
            ..Default::default()
        };

        let success = unsafe { GetOpenFileNameW(&mut ofn) };
        if success.as_bool() {
            let len = file_buf.iter().position(|&c| c == 0).unwrap_or(file_buf.len());
            let path = String::from_utf16_lossy(&file_buf[..len]);
            if !path.is_empty() {
                return OperationResult::ok(Some(path));
            }
        }
        OperationResult::ok(None)
    }
    #[cfg(not(target_os = "windows"))]
    {
        OperationResult::ok(None)
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
