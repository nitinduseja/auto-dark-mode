use super::registry::{
    read_dword, write_dword, RegistryError, APPS_USE_LIGHT_THEME, PERSONALIZE_SUBKEY,
    SYSTEM_USES_LIGHT_THEME,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Light => write!(f, "light"),
            ThemeMode::Dark => write!(f, "dark"),
        }
    }
}

impl ThemeMode {
    pub fn to_dword(self) -> u32 {
        match self {
            ThemeMode::Light => 1,
            ThemeMode::Dark => 0,
        }
    }

    pub fn from_dword(value: u32) -> Self {
        if value == 1 {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedTheme {
    pub apps_theme: ThemeMode,
    pub system_theme: ThemeMode,
    pub is_synchronized: bool,
    pub effective_theme: ThemeMode,
}

#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum ThemeError {
    #[error("Registry operation failed: {0}")]
    Registry(String),

    #[error("Failed to detect theme: {0}")]
    DetectionFailed(String),

    #[error(
        "Partial theme switch applied: apps was set to {0}, but system setting failed with {1}"
    )]
    PartialSuccess(String, String),

    #[error("Windows notification broadcast failed: {0}")]
    NotificationFailed(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),
}

impl From<RegistryError> for ThemeError {
    fn from(err: RegistryError) -> Self {
        ThemeError::Registry(err.to_string())
    }
}

pub fn detect_windows_theme() -> Result<DetectedTheme, ThemeError> {
    let apps_val = read_dword(PERSONALIZE_SUBKEY, APPS_USE_LIGHT_THEME).map_err(|e| {
        ThemeError::DetectionFailed(format!("Failed to read {}: {}", APPS_USE_LIGHT_THEME, e))
    })?;

    let system_val = read_dword(PERSONALIZE_SUBKEY, SYSTEM_USES_LIGHT_THEME).map_err(|e| {
        ThemeError::DetectionFailed(format!("Failed to read {}: {}", SYSTEM_USES_LIGHT_THEME, e))
    })?;

    let apps_theme = ThemeMode::from_dword(apps_val);
    let system_theme = ThemeMode::from_dword(system_val);
    let is_synchronized = apps_theme == system_theme;
    let effective_theme = apps_theme;

    Ok(DetectedTheme {
        apps_theme,
        system_theme,
        is_synchronized,
        effective_theme,
    })
}

pub fn apply_windows_theme(target: ThemeMode) -> Result<DetectedTheme, ThemeError> {
    apply_windows_theme_independent(target, target)
}

pub fn apply_windows_theme_independent(
    apps_target: ThemeMode,
    system_target: ThemeMode,
) -> Result<DetectedTheme, ThemeError> {
    let apps_dword = apps_target.to_dword();
    let system_dword = system_target.to_dword();

    // 1. Write AppsUseLightTheme
    write_dword(PERSONALIZE_SUBKEY, APPS_USE_LIGHT_THEME, apps_dword)
        .map_err(|e| ThemeError::Registry(format!("Failed writing AppsUseLightTheme: {}", e)))?;

    // 2. Write SystemUsesLightTheme
    if let Err(e) = write_dword(PERSONALIZE_SUBKEY, SYSTEM_USES_LIGHT_THEME, system_dword) {
        return Err(ThemeError::PartialSuccess(
            apps_target.to_string(),
            e.to_string(),
        ));
    }

    // 3. Notify Windows system and running apps
    broadcast_theme_change()?;

    // 4. Return fresh verified state
    detect_windows_theme()
}

pub fn apply_accent_color(hex_color: &str) -> Result<(), ThemeError> {
    let clean_hex = hex_color.trim().trim_start_matches('#');
    if clean_hex.len() != 6 {
        return Ok(());
    }

    if let Ok(rgb) = u32::from_str_radix(clean_hex, 16) {
        let r = (rgb >> 16) & 0xFF;
        let g = (rgb >> 8) & 0xFF;
        let b = rgb & 0xFF;

        let colorization_color = 0xFF000000 | (r << 16) | (g << 8) | b;
        let accent_color = 0xFF000000 | (b << 16) | (g << 8) | r;

        let _ = write_dword(r"Software\Microsoft\Windows\DWM", "ColorizationColor", colorization_color);
        let _ = write_dword(r"Software\Microsoft\Windows\DWM", "AccentColor", accent_color);
        let _ = broadcast_theme_change();
    }

    Ok(())
}

pub fn apply_cursor_scheme(is_dark: bool) -> Result<(), ThemeError> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{
            SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETCURSORS,
        };
        use super::registry::write_string;

        let scheme_name = if is_dark {
            "Windows Black"
        } else {
            "Windows Default"
        };

        let _ = write_string(r"Control Panel\Cursors", "", scheme_name);
        unsafe {
            let _ = SystemParametersInfoW(
                SPI_SETCURSORS,
                0,
                None,
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            );
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = is_dark;
    }
    Ok(())
}

pub fn apply_sound_scheme(is_dark: bool) -> Result<(), ThemeError> {
    #[cfg(target_os = "windows")]
    {
        use super::registry::write_string;
        let scheme = if is_dark {
            ".None"
        } else {
            ".Default"
        };
        let _ = write_string(r"AppEvents\Schemes", "", scheme);
        let _ = broadcast_theme_change();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = is_dark;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn broadcast_theme_change() -> Result<(), ThemeError> {
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
    };

    fn to_wide_null(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    let param1 = to_wide_null("ImmersiveColorSet");
    let param2 = to_wide_null("WindowsThemeElement");

    unsafe {
        let mut result: usize = 0;
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(param1.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            1000,
            Some(&mut result),
        );

        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(param2.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            1000,
            Some(&mut result),
        );
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn broadcast_theme_change() -> Result<(), ThemeError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_conversion() {
        assert_eq!(ThemeMode::Light.to_dword(), 1);
        assert_eq!(ThemeMode::Dark.to_dword(), 0);
        assert_eq!(ThemeMode::from_dword(1), ThemeMode::Light);
        assert_eq!(ThemeMode::from_dword(0), ThemeMode::Dark);
        assert_eq!(ThemeMode::from_dword(42), ThemeMode::Dark);
    }

    #[test]
    fn test_theme_mode_serialization() {
        let light_json = serde_json::to_string(&ThemeMode::Light).unwrap();
        assert_eq!(light_json, "\"light\"");

        let dark_json = serde_json::to_string(&ThemeMode::Dark).unwrap();
        assert_eq!(dark_json, "\"dark\"");

        let parsed: ThemeMode = serde_json::from_str("\"light\"").unwrap();
        assert_eq!(parsed, ThemeMode::Light);
    }
}
