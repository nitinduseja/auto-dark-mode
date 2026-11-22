use serde::Deserialize;
use tauri::{command, AppHandle};
use tracing::info;

use super::OperationResult;
use crate::config::{ConfigManager, PersonalizationConfig};
use crate::scheduler::trigger_immediate_evaluation;
use crate::windows::{
    get_connected_monitors, get_spotlight_wallpapers as win_get_spotlight,
    set_desktop_solid_color as win_set_solid_color, set_desktop_wallpaper,
    set_desktop_wallpaper_for_monitor, sync_and_apply_spotlight_wallpaper,
    ConnectedMonitor, SpotlightImageInfo, ThemeMode,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SetPersonalizationConfigRequest {
    pub config: PersonalizationConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestWallpaperRequest {
    pub theme: ThemeMode,
    pub monitor_id: Option<String>,
    pub mode: Option<String>,
    pub solid_color: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetSolidColorRequest {
    pub color: String,
}

#[command]
pub async fn get_spotlight_wallpapers() -> OperationResult<Vec<SpotlightImageInfo>> {
    info!("Handling get_spotlight_wallpapers command");
    match win_get_spotlight() {
        Ok(images) => OperationResult::ok(images),
        Err(e) => OperationResult::err(e),
    }
}

#[command]
pub async fn apply_spotlight_wallpaper() -> OperationResult<String> {
    info!("Handling apply_spotlight_wallpaper command");
    match sync_and_apply_spotlight_wallpaper() {
        Ok(path) => OperationResult::ok(path),
        Err(e) => OperationResult::err(e),
    }
}

#[command]
pub async fn set_desktop_solid_color_command(request: SetSolidColorRequest) -> OperationResult<bool> {
    info!("Handling set_desktop_solid_color command with color: {}", request.color);
    match win_set_solid_color(&request.color) {
        Ok(_) => OperationResult::ok(true),
        Err(e) => OperationResult::err(e),
    }
}

#[command]
pub async fn get_connected_displays() -> OperationResult<Vec<ConnectedMonitor>> {
    info!("Handling get_connected_displays command");
    let monitors = get_connected_monitors();
    OperationResult::ok(monitors)
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

                // Apply Taskbar and Titlebar accent colorization immediately
                let _ = crate::windows::apply_taskbar_colorization(request.config.colorize_taskbar);
                let _ = crate::windows::apply_titlebar_colorization(request.config.colorize_titlebars);
                if request.config.sync_accent_color {
                    let current_theme = crate::windows::detect_windows_theme()
                        .map(|t| t.effective_theme)
                        .unwrap_or(ThemeMode::Dark);
                    let accent = match current_theme {
                        ThemeMode::Light => &request.config.light_accent_color,
                        ThemeMode::Dark => &request.config.dark_accent_color,
                    };
                    let _ = crate::windows::apply_accent_color(accent);
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
        "Handling test_apply_wallpaper for theme: {:?} (mode: {:?})",
        request.theme, request.mode
    );
    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => {
                let mode = request.mode.as_deref().unwrap_or(config.personalization.wallpaper_mode.as_str());

                match mode {
                    "solid_color" => {
                        let color = request.solid_color.as_deref().unwrap_or_else(|| {
                            match request.theme {
                                ThemeMode::Light => config.personalization.light_solid_color.as_str(),
                                ThemeMode::Dark => config.personalization.dark_solid_color.as_str(),
                            }
                        });
                        match win_set_solid_color(color) {
                            Ok(_) => OperationResult::ok(true),
                            Err(e) => OperationResult::err(e),
                        }
                    }
                    "spotlight" => {
                        match sync_and_apply_spotlight_wallpaper() {
                            Ok(_) => OperationResult::ok(true),
                            Err(e) => OperationResult::err(e),
                        }
                    }
                    _ => {
                        let path = match request.theme {
                            ThemeMode::Light => &config.personalization.light_wallpaper_path,
                            ThemeMode::Dark => &config.personalization.dark_wallpaper_path,
                        };

                        let mut target_path = path.clone();
                        if let Some(ref m_id) = request.monitor_id {
                            if let Some(mw) = config.personalization.monitor_wallpapers.iter().find(|m| &m.monitor_id == m_id) {
                                let m_path = match request.theme {
                                    ThemeMode::Light => &mw.light_wallpaper_path,
                                    ThemeMode::Dark => &mw.dark_wallpaper_path,
                                };
                                if !m_path.trim().is_empty() {
                                    target_path = m_path.clone();
                                }
                            }
                        }

                        if target_path.trim().is_empty() {
                            return OperationResult::err("No wallpaper file path is specified for this theme mode".to_string());
                        }

                        let res = if let Some(ref m_id) = request.monitor_id {
                            set_desktop_wallpaper_for_monitor(Some(m_id.as_str()), &target_path, &config.personalization.wallpaper_fit)
                        } else {
                            set_desktop_wallpaper(&target_path, &config.personalization.wallpaper_fit)
                        };

                        match res {
                            Ok(_) => OperationResult::ok(true),
                            Err(e) => OperationResult::err(e),
                        }
                    }
                }
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager init failed: {}", e)),
    }
}
