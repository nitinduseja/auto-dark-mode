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
    let target_dword = target.to_dword();

    // 1. Write AppsUseLightTheme
    write_dword(PERSONALIZE_SUBKEY, APPS_USE_LIGHT_THEME, target_dword)
        .map_err(|e| ThemeError::Registry(format!("Failed writing AppsUseLightTheme: {}", e)))?;

    // 2. Write SystemUsesLightTheme
    if let Err(e) = write_dword(PERSONALIZE_SUBKEY, SYSTEM_USES_LIGHT_THEME, target_dword) {
        return Err(ThemeError::PartialSuccess(
            target.to_string(),
            e.to_string(),
        ));
    }

    // 3. Notify Windows system and running apps
    broadcast_theme_change()?;

    // 4. Return fresh verified state
    detect_windows_theme()
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
