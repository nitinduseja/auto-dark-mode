use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstalledTheme {
    pub id: String,
    pub name: String,
    pub path: String,
    pub is_system: bool,
}

#[cfg(target_os = "windows")]
fn resolve_indirect_string(source: &str) -> Option<String> {
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::SHLoadIndirectString;

    let source_wide: Vec<u16> = source.encode_utf16().chain(std::iter::once(0)).collect();
    let mut buffer = [0u16; 512];
    let res = unsafe {
        SHLoadIndirectString(
            PCWSTR(source_wide.as_ptr()),
            &mut buffer,
            None,
        )
    };
    if res.is_ok() {
        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        let s = String::from_utf16_lossy(&buffer[..len]).trim().to_string();
        if !s.is_empty() {
            return Some(s);
        }
    }
    None
}

/// Fallback map for common Windows 10/11 built-in theme files
fn fallback_theme_name(file_stem: &str) -> Option<&'static str> {
    match file_stem.to_lowercase().as_str() {
        "aero" => Some("Windows (light)"),
        "dark" => Some("Windows (dark)"),
        "spotlight" => Some("Windows Spotlight"),
        "themea" => Some("Glow"),
        "themeb" => Some("Captured Motion"),
        "themec" => Some("Sunrise"),
        "themed" => Some("Flow"),
        "custom" => Some("Current Custom Theme"),
        _ => None,
    }
}

/// Read file as string handling both UTF-16LE and UTF-8
fn read_theme_content(path: &Path) -> Option<String> {
    if let Ok(bytes) = fs::read(path) {
        if bytes.starts_with(&[0xFF, 0xFE]) {
            // UTF-16 LE
            let u16_slice: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            Some(String::from_utf16_lossy(&u16_slice))
        } else {
            Some(String::from_utf8_lossy(&bytes).to_string())
        }
    } else {
        None
    }
}

/// Helper to expand environment variables like %SystemRoot%, %USERPROFILE%, %LOCALAPPDATA%
fn expand_environment_variables(input: &str) -> String {
    let mut expanded = input.to_string();
    if expanded.contains('%') {
        for (key, val) in std::env::vars() {
            let placeholder_lower = format!("%{}%", key.to_lowercase());
            while let Some(pos) = expanded.to_lowercase().find(&placeholder_lower) {
                let end = pos + placeholder_lower.len();
                expanded.replace_range(pos..end, &val);
            }
        }
    }
    expanded
}

/// Parse the display name from a .theme file
fn parse_theme_display_name(path: &Path) -> String {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown");

    if let Some(content) = read_theme_content(path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("DisplayName=") {
                let val = trimmed["DisplayName=".len()..].trim();
                if val.starts_with('@') {
                    #[cfg(target_os = "windows")]
                    if let Some(resolved) = resolve_indirect_string(val) {
                        return resolved;
                    }
                    if let Some(mapped) = fallback_theme_name(file_stem) {
                        return mapped.to_string();
                    }
                } else if !val.is_empty() {
                    return val.to_string();
                }
            }
        }
    }

    if let Some(mapped) = fallback_theme_name(file_stem) {
        return mapped.to_string();
    }

    // Default to capitalized file stem
    file_stem.replace('_', " ").replace('-', " ")
}

fn scan_theme_dir(dir: &Path, is_system: bool, results: &mut Vec<InstalledTheme>) {
    if !dir.exists() || !dir.is_dir() {
        return;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                // Scan 1 level of subdirectories (e.g. %LOCALAPPDATA%\Themes\<ThemeName>\*.theme)
                if let Ok(sub_entries) = fs::read_dir(&p) {
                    for sub in sub_entries.flatten() {
                        let sub_p = sub.path();
                        if sub_p.is_file()
                            && sub_p
                                .extension()
                                .map_or(false, |ext| ext.eq_ignore_ascii_case("theme"))
                        {
                            add_theme_entry(&sub_p, is_system, results);
                        }
                    }
                }
            } else if p.is_file()
                && p
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("theme"))
            {
                add_theme_entry(&p, is_system, results);
            }
        }
    }
}

fn add_theme_entry(path: &Path, is_system: bool, results: &mut Vec<InstalledTheme>) {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let path_str = canonical.to_string_lossy().to_string();

    // Prevent duplicate entries
    if results.iter().any(|t| t.path.eq_ignore_ascii_case(&path_str)) {
        return;
    }

    let name = parse_theme_display_name(path);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("theme");
    let id = format!("{}_{}", if is_system { "sys" } else { "user" }, stem.to_lowercase());

    results.push(InstalledTheme {
        id,
        name,
        path: path_str,
        is_system,
    });
}

/// Discover all Windows themes installed on the system
pub fn discover_installed_themes() -> Vec<InstalledTheme> {
    let mut themes = Vec::new();

    // 1. System Themes (C:\Windows\Resources\Themes)
    let sys_dir = PathBuf::from(r"C:\Windows\Resources\Themes");
    scan_theme_dir(&sys_dir, true, &mut themes);

    // 2. User Local Themes (%LOCALAPPDATA%\Microsoft\Windows\Themes)
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let user_themes = PathBuf::from(local_app_data).join(r"Microsoft\Windows\Themes");
        scan_theme_dir(&user_themes, false, &mut themes);
    }

    // 3. User Roaming Themes (%APPDATA%\Microsoft\Windows\Themes)
    if let Ok(app_data) = std::env::var("APPDATA") {
        let roaming_themes = PathBuf::from(app_data).join(r"Microsoft\Windows\Themes");
        scan_theme_dir(&roaming_themes, false, &mut themes);
    }

    // Sort: System "Windows (light)" and "Windows (dark)" first, then user themes alphabetically
    themes.sort_by(|a, b| {
        let a_priority = if a.name.contains("Windows (light)") {
            0
        } else if a.name.contains("Windows (dark)") {
            1
        } else if a.is_system {
            2
        } else {
            3
        };

        let b_priority = if b.name.contains("Windows (light)") {
            0
        } else if b.name.contains("Windows (dark)") {
            1
        } else if b.is_system {
            2
        } else {
            3
        };

        a_priority.cmp(&b_priority).then_with(|| a.name.cmp(&b.name))
    });

    info!("Discovered {} installed Windows themes", themes.len());
    themes
}

/// Apply a specific .theme file to Windows silently without opening Windows Settings
pub fn apply_theme_file(theme_path: &str) -> Result<(), String> {
    let path = Path::new(theme_path);
    if !path.exists() {
        return Err(format!("Theme file does not exist: {}", theme_path));
    }

    info!("Applying Windows .theme file silently: {}", theme_path);

    #[cfg(target_os = "windows")]
    {
        if let Some(content) = read_theme_content(path) {
            let mut in_desktop_section = false;
            let mut in_visual_styles_section = false;
            let mut wallpaper_path: Option<String> = None;
            let mut system_mode: Option<crate::windows::theme::ThemeMode> = None;
            let mut app_mode: Option<crate::windows::theme::ThemeMode> = None;
            let mut colorization_color: Option<u32> = None;

            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('[') && trimmed.ends_with(']') {
                    let section = &trimmed[1..trimmed.len() - 1];
                    in_desktop_section = section.eq_ignore_ascii_case(r"Control Panel\Desktop");
                    in_visual_styles_section = section.eq_ignore_ascii_case("VisualStyles");
                    continue;
                }

                if in_desktop_section {
                    if let Some(val) = trimmed.strip_prefix("Wallpaper=") {
                        let cleaned = val.trim();
                        if !cleaned.is_empty() {
                            wallpaper_path = Some(expand_environment_variables(cleaned));
                        }
                    }
                } else if in_visual_styles_section {
                    if let Some(val) = trimmed.strip_prefix("SystemMode=") {
                        if val.trim().eq_ignore_ascii_case("Dark") {
                            system_mode = Some(crate::windows::theme::ThemeMode::Dark);
                        } else if val.trim().eq_ignore_ascii_case("Light") {
                            system_mode = Some(crate::windows::theme::ThemeMode::Light);
                        }
                    } else if let Some(val) = trimmed.strip_prefix("AppMode=") {
                        if val.trim().eq_ignore_ascii_case("Dark") {
                            app_mode = Some(crate::windows::theme::ThemeMode::Dark);
                        } else if val.trim().eq_ignore_ascii_case("Light") {
                            app_mode = Some(crate::windows::theme::ThemeMode::Light);
                        }
                    } else if let Some(val) = trimmed.strip_prefix("ColorizationColor=") {
                        let hex_str = val.trim();
                        if let Some(hex) = hex_str
                            .strip_prefix("0x")
                            .or_else(|| hex_str.strip_prefix("0X"))
                        {
                            if let Ok(color_val) = u32::from_str_radix(hex, 16) {
                                colorization_color = Some(color_val);
                            }
                        }
                    }
                }
            }

            // 1. Apply theme wallpaper silently via SystemParametersInfoW
            if let Some(ref wp) = wallpaper_path {
                if Path::new(wp).exists() {
                    info!("Setting desktop wallpaper from theme: {}", wp);
                    let _ = crate::windows::wallpaper::set_desktop_wallpaper(wp, "fill");
                }
            }

            // 2. Apply System / App theme mode
            let target_mode = system_mode.or(app_mode);
            if let Some(mode) = target_mode {
                info!("Applying theme mode from theme file: {:?}", mode);
                let _ = crate::windows::theme::apply_windows_theme(mode);
            }

            // 3. Update Windows registry theme record
            let _ = crate::windows::registry::write_string(
                r"Software\Microsoft\Windows\CurrentVersion\Themes",
                "CurrentTheme",
                theme_path,
            );
            let _ = crate::windows::registry::write_dword(
                r"Software\Microsoft\Windows\CurrentVersion\Themes",
                "WallpaperSetFromTheme",
                1,
            );
            let _ = crate::windows::registry::write_dword(
                r"Software\Microsoft\Windows\CurrentVersion\Themes",
                "ColorSetFromTheme",
                1,
            );

            // 4. Update ColorizationColor if specified
            if let Some(color) = colorization_color {
                let _ = crate::windows::registry::write_dword(
                    r"Software\Microsoft\Windows\DWM",
                    "ColorizationColor",
                    color,
                );
            }

            // 5. Broadcast theme changes to Windows shell
            let _ = crate::windows::theme::broadcast_theme_change();

            return Ok(());
        }

        Err(format!("Could not read theme file: {}", theme_path))
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("[Non-Windows] Simulated theme file application: {}", theme_path);
        Ok(())
    }
}
