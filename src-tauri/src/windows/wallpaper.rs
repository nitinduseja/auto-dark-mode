use std::path::Path;
use tracing::{info, warn};

#[cfg(target_os = "windows")]
use std::ffi::c_void;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

use crate::windows::registry::write_string;

const DESKTOP_REGISTRY_SUBKEY: &str = r"Control Panel\Desktop";

pub fn set_desktop_wallpaper(image_path: &str, fit_style: &str) -> Result<(), String> {
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
        // 1. Set fit style in registry
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

        // 2. Call Win32 SystemParametersInfoW
        let path_wide: Vec<u16> = image_path
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let success = unsafe {
            SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                Some(path_wide.as_ptr() as *const c_void as *mut c_void),
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            )
        };

        if success.is_ok() {
            info!("Successfully updated desktop wallpaper to: {}", image_path);
            Ok(())
        } else {
            Err("SystemParametersInfoW failed to set wallpaper".to_string())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("[Non-Windows] Simulated wallpaper update: {}", image_path);
        Ok(())
    }
}
