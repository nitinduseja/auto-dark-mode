use std::path::Path;
use tracing::{info, warn};

#[cfg(target_os = "windows")]
use std::ffi::c_void;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;
#[cfg(target_os = "windows")]
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
    COINIT_APARTMENTTHREADED,
};
#[cfg(target_os = "windows")]
use windows::Win32::UI::Shell::{
    DesktopWallpaper, IDesktopWallpaper, DWPOS_CENTER, DWPOS_FILL, DWPOS_FIT, DWPOS_SPAN,
    DWPOS_STRETCH, DWPOS_TILE,
};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

use crate::windows::registry::write_string;

const DESKTOP_REGISTRY_SUBKEY: &str = r"Control Panel\Desktop";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectedMonitor {
    pub id: String,
    pub index: u32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub is_primary: bool,
}

pub fn get_connected_monitors() -> Vec<ConnectedMonitor> {
    #[cfg(target_os = "windows")]
    {
        let mut monitors = Vec::new();
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            if let Ok(desktop_wp) = CoCreateInstance::<_, IDesktopWallpaper>(&DesktopWallpaper, None, CLSCTX_ALL) {
                if let Ok(count) = desktop_wp.GetMonitorDevicePathCount() {
                    for i in 0..count {
                        if let Ok(device_path) = desktop_wp.GetMonitorDevicePathAt(i) {
                            if !device_path.is_null() {
                                let path_str = device_path.to_string().unwrap_or_default();
                                let wide_path: Vec<u16> = path_str.encode_utf16().chain(std::iter::once(0)).collect();

                                let rect = desktop_wp.GetMonitorRECT(PCWSTR(wide_path.as_ptr())).unwrap_or_default();
                                let width = (rect.right - rect.left).abs();
                                let height = (rect.bottom - rect.top).abs();
                                let is_primary = rect.left == 0 && rect.top == 0;

                                monitors.push(ConnectedMonitor {
                                    id: path_str,
                                    index: i + 1,
                                    name: format!("Display {}", i + 1),
                                    width,
                                    height,
                                    is_primary,
                                });

                                CoTaskMemFree(Some(device_path.as_ptr() as *const c_void));
                            }
                        }
                    }
                }
            }

            CoUninitialize();
        }

        if monitors.is_empty() {
            monitors.push(ConnectedMonitor {
                id: "primary".to_string(),
                index: 1,
                name: "Primary Display".to_string(),
                width: 1920,
                height: 1080,
                is_primary: true,
            });
        }

        monitors
    }

    #[cfg(not(target_os = "windows"))]
    {
        vec![ConnectedMonitor {
            id: "primary".to_string(),
            index: 1,
            name: "Display 1".to_string(),
            width: 1920,
            height: 1080,
            is_primary: true,
        }]
    }
}

pub fn set_desktop_wallpaper(image_path: &str, fit_style: &str) -> Result<(), String> {
    set_desktop_wallpaper_for_monitor(None, image_path, fit_style)
}

pub fn set_desktop_wallpaper_for_monitor(
    monitor_id: Option<&str>,
    image_path: &str,
    fit_style: &str,
) -> Result<(), String> {
    if image_path.trim().is_empty() {
        return Ok(());
    }

    let path = Path::new(image_path);
    if !path.exists() {
        warn!("Wallpaper image path does not exist: {}", image_path);
        return Err(format!("Wallpaper file not found: {}", image_path));
    }

    #[cfg(target_os = "windows")]
    {
        let mut com_success = false;

        // Try modern IDesktopWallpaper COM interface first
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            if let Ok(desktop_wp) = CoCreateInstance::<_, IDesktopWallpaper>(&DesktopWallpaper, None, CLSCTX_ALL) {
                let position = match fit_style.to_lowercase().as_str() {
                    "fit" => DWPOS_FIT,
                    "stretch" => DWPOS_STRETCH,
                    "tile" => DWPOS_TILE,
                    "center" => DWPOS_CENTER,
                    "span" => DWPOS_SPAN,
                    _ => DWPOS_FILL,
                };
                let _ = desktop_wp.SetPosition(position);

                let path_wide: Vec<u16> = image_path.encode_utf16().chain(std::iter::once(0)).collect();

                let res = if let Some(m_id) = monitor_id {
                    let m_wide: Vec<u16> = m_id.encode_utf16().chain(std::iter::once(0)).collect();
                    desktop_wp.SetWallpaper(PCWSTR(m_wide.as_ptr()), PCWSTR(path_wide.as_ptr()))
                } else {
                    desktop_wp.SetWallpaper(PCWSTR::null(), PCWSTR(path_wide.as_ptr()))
                };

                if res.is_ok() {
                    com_success = true;
                    info!("IDesktopWallpaper COM successfully applied wallpaper: {} (monitor: {:?})", image_path, monitor_id);
                }
            }

            CoUninitialize();
        }

        if com_success {
            return Ok(());
        }

        // Fallback to legacy SystemParametersInfoW
        let (style, tile) = match fit_style.to_lowercase().as_str() {
            "fit" => ("6", "0"),
            "stretch" => ("2", "0"),
            "tile" => ("0", "1"),
            "center" => ("0", "0"),
            "span" => ("22", "0"),
            _ => ("10", "0"), // "fill" is default
        };

        let _ = write_string(DESKTOP_REGISTRY_SUBKEY, "WallpaperStyle", style);
        let _ = write_string(DESKTOP_REGISTRY_SUBKEY, "TileWallpaper", tile);

        let path_wide: Vec<u16> = image_path.encode_utf16().chain(std::iter::once(0)).collect();

        let success = unsafe {
            SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                Some(path_wide.as_ptr() as *const c_void as *mut c_void),
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            )
        };

        if success.is_ok() {
            info!("Successfully updated desktop wallpaper via SystemParametersInfoW: {}", image_path);
            Ok(())
        } else {
            Err("Failed to set desktop wallpaper".to_string())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("[Non-Windows] Simulated wallpaper update: {} (monitor: {:?})", image_path, monitor_id);
        Ok(())
    }
}
