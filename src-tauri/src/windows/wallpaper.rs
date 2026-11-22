use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

#[cfg(target_os = "windows")]
use std::ffi::c_void;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::COLORREF;
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpotlightImageInfo {
    pub file_name: String,
    pub cached_path: String,
    pub width: u32,
    pub height: u32,
    pub modified_timestamp: i64,
}

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() == 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
        let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
        let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

pub fn set_desktop_solid_color(hex_color: &str) -> Result<(), String> {
    let (r, g, b) = hex_to_rgb(hex_color)
        .ok_or_else(|| format!("Invalid hex color: {}", hex_color))?;

    #[cfg(target_os = "windows")]
    {
        let _ = write_string(r"Control Panel\Colors", "Background", &format!("{} {} {}", r, g, b));
        let _ = write_string(DESKTOP_REGISTRY_SUBKEY, "Wallpaper", "");
        let _ = crate::windows::registry::write_dword(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\Wallpapers",
            "BackgroundType",
            1,
        );

        let colorref = COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16));

        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

            if let Ok(desktop_wp) = CoCreateInstance::<_, IDesktopWallpaper>(&DesktopWallpaper, None, CLSCTX_ALL) {
                let _ = desktop_wp.SetBackgroundColor(colorref);
                let empty: [u16; 1] = [0];
                let _ = desktop_wp.SetWallpaper(PCWSTR::null(), PCWSTR(empty.as_ptr()));
            }

            let empty_path: [u16; 1] = [0];
            let _ = SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                Some(empty_path.as_ptr() as *const c_void as *mut c_void),
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            );

            CoUninitialize();
        }

        info!("Successfully applied solid background color: {} (rgb: {}, {}, {})", hex_color, r, g, b);
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("[Non-Windows] Simulated solid color wallpaper: {}", hex_color);
        Ok(())
    }
}

pub fn parse_jpeg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 4 || data[0] != 0xFF || data[1] != 0xD8 {
        return None;
    }
    let mut i = 2;
    while i + 8 < data.len() {
        if data[i] == 0xFF {
            let marker = data[i + 1];
            if marker == 0xC0 || marker == 0xC1 || marker == 0xC2 {
                let h = ((data[i + 5] as u32) << 8) | (data[i + 6] as u32);
                let w = ((data[i + 7] as u32) << 8) | (data[i + 8] as u32);
                return Some((w, h));
            }
            if i + 3 < data.len() {
                let len = ((data[i + 2] as usize) << 8) | (data[i + 3] as usize);
                if len < 2 {
                    i += 1;
                } else {
                    i += 2 + len;
                }
            } else {
                break;
            }
        } else {
            i += 1;
        }
    }
    None
}

pub fn get_spotlight_assets_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA").ok().map(|local_app_data| {
            PathBuf::from(local_app_data)
                .join(r"Packages\Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy\LocalState\Assets")
        })
    }
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

pub fn get_spotlight_cache_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("LOCALAPPDATA")
            .map(|l| PathBuf::from(l).join(r"AutoDarkMode\spotlight_cache"))
            .unwrap_or_else(|_| PathBuf::from(r"C:\AutoDarkMode\spotlight_cache"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::temp_dir().join("autodarkmode_spotlight")
    }
}

pub fn get_spotlight_wallpapers() -> Result<Vec<SpotlightImageInfo>, String> {
    let assets_dir = match get_spotlight_assets_dir() {
        Some(d) if d.exists() => d,
        _ => return Ok(Vec::new()),
    };

    let cache_dir = get_spotlight_cache_dir();
    let _ = fs::create_dir_all(&cache_dir);

    let mut images = Vec::new();
    let entries = fs::read_dir(&assets_dir).map_err(|e| format!("Failed reading Spotlight assets: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Ok(meta) = path.metadata() {
            if meta.len() < 200 * 1024 {
                continue;
            }

            if let Ok(data) = fs::read(&path) {
                if let Some((w, h)) = parse_jpeg_dimensions(&data) {
                    if w >= h && w >= 1080 {
                        let file_stem = path.file_name().and_then(|f| f.to_str()).unwrap_or("spotlight");
                        let cached_filename = format!("{}.jpg", file_stem);
                        let cached_path = cache_dir.join(&cached_filename);

                        if !cached_path.exists() {
                            let _ = fs::copy(&path, &cached_path);
                        }

                        let modified_timestamp = meta
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(0);

                        images.push(SpotlightImageInfo {
                            file_name: cached_filename,
                            cached_path: cached_path.to_string_lossy().to_string(),
                            width: w,
                            height: h,
                            modified_timestamp,
                        });
                    }
                }
            }
        }
    }

    images.sort_by(|a, b| b.modified_timestamp.cmp(&a.modified_timestamp));
    Ok(images)
}

pub fn sync_and_apply_spotlight_wallpaper() -> Result<String, String> {
    let images = get_spotlight_wallpapers()?;
    if let Some(latest) = images.first() {
        set_desktop_wallpaper(&latest.cached_path, "fill")?;
        info!("Applied Windows Spotlight wallpaper: {}", latest.cached_path);
        Ok(latest.cached_path.clone())
    } else {
        Err("No Windows Spotlight desktop wallpapers found in system cache".to_string())
    }
}

pub fn apply_configured_wallpaper(
    config: &crate::config::PersonalizationConfig,
    target_theme: crate::windows::ThemeMode,
) -> Result<(), String> {
    if !config.wallpaper_enabled {
        return Ok(());
    }

    match config.wallpaper_mode.as_str() {
        "solid_color" => {
            let color = match target_theme {
                crate::windows::ThemeMode::Light => &config.light_solid_color,
                crate::windows::ThemeMode::Dark => &config.dark_solid_color,
            };
            if !color.trim().is_empty() {
                set_desktop_solid_color(color)?;
            }
            Ok(())
        }
        "spotlight" => {
            let _ = sync_and_apply_spotlight_wallpaper()?;
            Ok(())
        }
        _ => {
            // "picture" (default)
            let default_wp_path = match target_theme {
                crate::windows::ThemeMode::Light => &config.light_wallpaper_path,
                crate::windows::ThemeMode::Dark => &config.dark_wallpaper_path,
            };

            if config.multi_monitor_enabled && !config.monitor_wallpapers.is_empty() {
                for mw in &config.monitor_wallpapers {
                    let m_path = match target_theme {
                        crate::windows::ThemeMode::Light => &mw.light_wallpaper_path,
                        crate::windows::ThemeMode::Dark => &mw.dark_wallpaper_path,
                    };
                    let chosen_path = if !m_path.trim().is_empty() {
                        m_path
                    } else {
                        default_wp_path
                    };
                    if !chosen_path.trim().is_empty() {
                        let _ = set_desktop_wallpaper_for_monitor(
                            Some(&mw.monitor_id),
                            chosen_path,
                            &config.wallpaper_fit,
                        );
                    }
                }
                Ok(())
            } else if !default_wp_path.trim().is_empty() {
                set_desktop_wallpaper(default_wp_path, &config.wallpaper_fit)
            } else {
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FFFFFF"), Some((255, 255, 255)));
        assert_eq!(hex_to_rgb("#000000"), Some((0, 0, 0)));
        assert_eq!(hex_to_rgb("1E1E1E"), Some((30, 30, 30)));
        assert_eq!(hex_to_rgb("invalid"), None);
    }
}
