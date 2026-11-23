use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorSchemeInfo {
    pub name: String,
    pub is_system: bool,
    pub sample_arrow_path: Option<String>,
}

const CURSOR_ROLES: &[&str] = &[
    "Arrow",
    "Help",
    "AppStarting",
    "Wait",
    "Crosshair",
    "IBeam",
    "NWPen",
    "No",
    "SizeNS",
    "SizeWE",
    "SizeNWSE",
    "SizeNESW",
    "SizeAll",
    "UpArrow",
    "Hand",
    "Pin",
    "Person",
];

const CURSORS_SUBKEY: &str = r"Control Panel\Cursors";
const HKLM_SCHEMES_SUBKEY: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Control Panel\Cursors\Schemes";
const HKCU_SCHEMES_SUBKEY: &str = r"Control Panel\Cursors\Schemes";

pub fn discover_cursor_schemes() -> Vec<CursorSchemeInfo> {
    #[cfg(target_os = "windows")]
    {
        use crate::windows::registry::{
            list_value_names_hkcu, list_value_names_hklm, read_hklm_string, read_string,
        };

        let mut schemes = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // 1. User defined schemes in HKCU
        if let Ok(user_names) = list_value_names_hkcu(HKCU_SCHEMES_SUBKEY) {
            for name in user_names {
                let sample_arrow = read_string(HKCU_SCHEMES_SUBKEY, &name)
                    .ok()
                    .and_then(|val| val.split(',').next().map(|s| s.trim().to_string()))
                    .filter(|s| !s.is_empty());

                seen.insert(name.clone());
                schemes.push(CursorSchemeInfo {
                    name,
                    is_system: false,
                    sample_arrow_path: sample_arrow,
                });
            }
        }

        // 2. System schemes in HKLM
        if let Ok(sys_names) = list_value_names_hklm(HKLM_SCHEMES_SUBKEY) {
            for name in sys_names {
                if !seen.contains(&name) {
                    let sample_arrow = read_hklm_string(HKLM_SCHEMES_SUBKEY, &name)
                        .ok()
                        .and_then(|val| val.split(',').next().map(|s| s.trim().to_string()))
                        .filter(|s| !s.is_empty());

                    seen.insert(name.clone());
                    schemes.push(CursorSchemeInfo {
                        name,
                        is_system: true,
                        sample_arrow_path: sample_arrow,
                    });
                }
            }
        }

        // Fallback standard Windows schemes if none found
        if schemes.is_empty() {
            schemes.push(CursorSchemeInfo {
                name: "Windows Aero".to_string(),
                is_system: true,
                sample_arrow_path: Some(r"C:\Windows\Cursors\aero_arrow.cur".to_string()),
            });
            schemes.push(CursorSchemeInfo {
                name: "Windows Black".to_string(),
                is_system: true,
                sample_arrow_path: Some(r"C:\Windows\Cursors\arrow_r.cur".to_string()),
            });
            schemes.push(CursorSchemeInfo {
                name: "Windows Inverted".to_string(),
                is_system: true,
                sample_arrow_path: None,
            });
        }

        schemes.sort_by(|a, b| {
            let a_priority = match a.name.as_str() {
                "Windows Aero" => 0,
                "Windows Black" => 1,
                "Windows Inverted" => 2,
                _ => 3,
            };
            let b_priority = match b.name.as_str() {
                "Windows Aero" => 0,
                "Windows Black" => 1,
                "Windows Inverted" => 2,
                _ => 3,
            };
            a_priority.cmp(&b_priority).then(a.name.cmp(&b.name))
        });

        schemes
    }

    #[cfg(not(target_os = "windows"))]
    {
        vec![
            CursorSchemeInfo {
                name: "Windows Aero".to_string(),
                is_system: true,
                sample_arrow_path: None,
            },
            CursorSchemeInfo {
                name: "Windows Black".to_string(),
                is_system: true,
                sample_arrow_path: None,
            },
        ]
    }
}

pub fn apply_cursor_scheme_by_name(scheme_name: &str) -> Result<(), String> {
    if scheme_name.trim().is_empty() {
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{
            SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETCURSORS,
        };
        use crate::windows::registry::{read_hklm_string, read_string, write_string};

        let scheme_data = read_string(HKCU_SCHEMES_SUBKEY, scheme_name)
            .or_else(|_| read_hklm_string(HKLM_SCHEMES_SUBKEY, scheme_name))
            .ok();

        if let Some(data) = scheme_data {
            let paths: Vec<&str> = data.split(',').collect();
            for (idx, role) in CURSOR_ROLES.iter().enumerate() {
                if let Some(cursor_path) = paths.get(idx) {
                    let _ = write_string(CURSORS_SUBKEY, role, cursor_path.trim());
                }
            }
        }

        let _ = write_string(CURSORS_SUBKEY, "", scheme_name);

        unsafe {
            let res = SystemParametersInfoW(
                SPI_SETCURSORS,
                0,
                None,
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            );
            if res.is_err() {
                warn!("SystemParametersInfoW SPI_SETCURSORS returned error");
            }
        }

        let _ = crate::windows::theme::broadcast_theme_change();
        info!("Applied Windows cursor scheme: {}", scheme_name);
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        info!("[Non-Windows] Simulated cursor scheme switch: {}", scheme_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_roles_list() {
        assert_eq!(CURSOR_ROLES[0], "Arrow");
        assert!(CURSOR_ROLES.len() >= 15);
    }
}
