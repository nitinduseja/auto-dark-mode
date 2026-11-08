use std::path::PathBuf;
use tracing::info;

#[cfg(target_os = "windows")]
use super::registry::{write_dword, write_string};

/// Synchronizes Microsoft Office 2016/2019/2021/365 UI Theme
/// UI Theme values:
/// 0 = Colorful (Light)
/// 3 = Dark Gray
/// 4 = Black (Dark)
/// 5 = White
/// 6 = Use System Setting
pub fn sync_office_theme(is_dark: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let key = r"Software\Microsoft\Office\16.0\Common";
        let target_value = if is_dark { 4u32 } else { 0u32 };
        if let Err(e) = write_dword(key, "UI Theme", target_value) {
            return Err(format!("Failed to update Office theme registry: {}", e));
        }
        info!("Office UI Theme updated to: {}", if is_dark { "Black" } else { "Colorful" });
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = is_dark;
        Ok(())
    }
}

/// Synchronizes Visual Studio Code workbench color theme
pub fn sync_vscode_theme(is_dark: bool) -> Result<(), String> {
    let appdata = match std::env::var("APPDATA") {
        Ok(v) => v,
        Err(_) => return Err("APPDATA environment variable not found".to_string()),
    };

    let settings_path = PathBuf::from(appdata).join("Code").join("User").join("settings.json");
    if !settings_path.exists() {
        return Ok(()); // VS Code not installed or no settings.json yet
    }

    let content = match std::fs::read_to_string(&settings_path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to read VS Code settings: {}", e)),
    };

    // Parse or update JSON
    let mut json_val: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return Err("Failed to parse VS Code settings.json".to_string()),
    };

    let target_theme = if is_dark {
        "Default Dark Modern"
    } else {
        "Default Light Modern"
    };

    if let Some(obj) = json_val.as_object_mut() {
        obj.insert("workbench.colorTheme".to_string(), serde_json::Value::String(target_theme.to_string()));
    }

    let serialized = match serde_json::to_string_pretty(&json_val) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to serialize VS Code settings: {}", e)),
    };

    if let Err(e) = std::fs::write(&settings_path, serialized) {
        return Err(format!("Failed to write VS Code settings.json: {}", e));
    }

    info!("VS Code theme updated to: {}", target_theme);
    Ok(())
}

/// Synchronizes Windows Terminal theme (light / dark)
pub fn sync_windows_terminal_theme(is_dark: bool) -> Result<(), String> {
    let localappdata = match std::env::var("LOCALAPPDATA") {
        Ok(v) => v,
        Err(_) => return Err("LOCALAPPDATA environment variable not found".to_string()),
    };

    // Find Windows Terminal settings path (packaged or unpackaged)
    let packages_dir = PathBuf::from(&localappdata).join("Packages");
    let mut settings_path = None;

    if packages_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&packages_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("Microsoft.WindowsTerminal_") {
                    let candidate = entry.path().join("LocalState").join("settings.json");
                    if candidate.exists() {
                        settings_path = Some(candidate);
                        break;
                    }
                }
            }
        }
    }

    if settings_path.is_none() {
        let unpackaged = PathBuf::from(localappdata).join("Microsoft").join("Windows Terminal").join("settings.json");
        if unpackaged.exists() {
            settings_path = Some(unpackaged);
        }
    }

    let path = match settings_path {
        Some(p) => p,
        None => return Ok(()), // Windows Terminal settings file not found
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to read Windows Terminal settings: {}", e)),
    };

    let mut json_val: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return Err("Failed to parse Windows Terminal settings.json".to_string()),
    };

    let target_theme = if is_dark { "dark" } else { "light" };

    if let Some(obj) = json_val.as_object_mut() {
        obj.insert("theme".to_string(), serde_json::Value::String(target_theme.to_string()));
    }

    let serialized = match serde_json::to_string_pretty(&json_val) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to serialize Windows Terminal settings: {}", e)),
    };

    if let Err(e) = std::fs::write(&path, serialized) {
        return Err(format!("Failed to write Windows Terminal settings.json: {}", e));
    }

    info!("Windows Terminal theme updated to: {}", target_theme);
    Ok(())
}

/// Synchronizes Windows 10/11 Touch Keyboard Theme
pub fn sync_touch_keyboard_theme(is_dark: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let key = r"Software\Microsoft\TabletTip\1.7";
        let target_theme = if is_dark { "DefaultDark" } else { "DefaultLight" };
        let _ = write_string(key, "SelectedTheme", target_theme);
        info!("Touch keyboard theme updated to: {}", target_theme);
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = is_dark;
        Ok(())
    }
}

/// Synchronizes Windows Grayscale Color Filter
pub fn sync_color_filter(active: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let key = r"Software\Microsoft\ColorFiltering";
        let _ = write_dword(key, "Active", if active { 1 } else { 0 });
        let _ = write_dword(key, "FilterType", 0); // 0 = Grayscale
        let _ = crate::windows::theme::broadcast_theme_change();
        info!("Windows Grayscale color filter set to: {}", active);
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = active;
        Ok(())
    }
}
