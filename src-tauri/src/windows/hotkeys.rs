use std::sync::atomic::{AtomicU32, Ordering};
use tauri::{AppHandle, Emitter, Manager};
use tracing::{info, warn};

use crate::commands::theme::{apply_theme_internally, postpone_internally};
use crate::config::{AppState, ConfigManager, ShortcutsConfig};
use crate::windows::theme::{detect_windows_theme, ThemeMode};

pub const HOTKEY_ID_TOGGLE: i32 = 1001;
pub const HOTKEY_ID_FORCE_LIGHT: i32 = 1002;
pub const HOTKEY_ID_FORCE_DARK: i32 = 1003;
pub const HOTKEY_ID_POSTPONE: i32 = 1004;

pub const WM_RELOAD_HOTKEYS: u32 = 0x0400 + 100; // WM_USER + 100

static HOTKEY_THREAD_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, serde::Serialize)]
pub struct HotkeyTriggerEvent {
    pub action: String,
    pub target_theme: Option<String>,
    pub timestamp: i64,
}

/// Parses a human-readable shortcut string (e.g., "Ctrl+Alt+T", "Win+Shift+D")
/// into Win32 (fsModifiers, vkCode).
pub fn parse_shortcut_string(s: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = s.split('+').map(|p| p.trim()).collect();
    if parts.is_empty() {
        return None;
    }

    let mut modifiers: u32 = 0x4000; // MOD_NOREPEAT
    let mut vk: Option<u32> = None;

    for part in parts {
        let lower = part.to_lowercase();
        match lower.as_str() {
            "ctrl" | "control" => modifiers |= 0x0002, // MOD_CONTROL
            "alt" => modifiers |= 0x0001,             // MOD_ALT
            "shift" => modifiers |= 0x0004,           // MOD_SHIFT
            "win" | "windows" | "super" => modifiers |= 0x0008, // MOD_WIN
            single if single.len() == 1 => {
                let c = single.chars().next().unwrap().to_ascii_uppercase();
                if c.is_ascii_alphanumeric() {
                    vk = Some(c as u32);
                }
            }
            f_key if f_key.starts_with('f') && f_key.len() <= 3 => {
                if let Ok(num) = f_key[1..].parse::<u32>() {
                    if (1..=24).contains(&num) {
                        vk = Some(0x70 + num - 1); // VK_F1 is 0x70
                    }
                }
            }
            "space" => vk = Some(0x20),
            "tab" => vk = Some(0x09),
            "esc" | "escape" => vk = Some(0x1B),
            "enter" | "return" => vk = Some(0x0D),
            "insert" => vk = Some(0x2D),
            "delete" | "del" => vk = Some(0x2E),
            "home" => vk = Some(0x24),
            "end" => vk = Some(0x23),
            "pageup" | "pgup" => vk = Some(0x21),
            "pagedown" | "pgdn" => vk = Some(0x22),
            _ => {}
        }
    }

    vk.map(|k| (modifiers, k))
}

/// Spawns the background Win32 message queue thread that manages global hotkeys.
pub fn start_hotkey_manager(app: AppHandle) {
    #[cfg(target_os = "windows")]
    {
        std::thread::Builder::new()
            .name("autodarkmode-hotkeys".to_string())
            .spawn(move || {
                run_hotkey_loop(app);
            })
            .expect("Failed to spawn hotkey manager thread");
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = app;
    }
}

/// Signals the hotkey manager thread to reload configuration and re-register hotkeys.
pub fn update_registered_hotkeys() {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Foundation::{LPARAM, WPARAM};
        use windows::Win32::UI::WindowsAndMessaging::PostThreadMessageW;
        let thread_id = HOTKEY_THREAD_ID.load(Ordering::SeqCst);
        if thread_id != 0 {
            unsafe {
                let _ = PostThreadMessageW(thread_id, WM_RELOAD_HOTKEYS, WPARAM(0), LPARAM(0));
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn run_hotkey_loop(app: AppHandle) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::System::Threading::GetCurrentThreadId;
    use windows::Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, GetMessageW, PeekMessageW, TranslateMessage, MSG, PM_NOREMOVE,
        WM_HOTKEY, WM_QUIT,
    };

    let thread_id = unsafe { GetCurrentThreadId() };
    HOTKEY_THREAD_ID.store(thread_id, Ordering::SeqCst);
    info!("Global Hotkey manager thread started with ThreadId: {}", thread_id);

    // Force creation of a Win32 message queue for this thread
    let mut dummy_msg = MSG::default();
    unsafe {
        let _ = PeekMessageW(&mut dummy_msg, HWND(std::ptr::null_mut()), 0, 0, PM_NOREMOVE);
    }

    // Initial registration of hotkeys from active configuration
    register_all_hotkeys(&app);

    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).as_bool() {
            if msg.message == WM_HOTKEY {
                let hotkey_id = msg.wParam.0 as i32;
                info!("Global hotkey triggered with ID: {}", hotkey_id);
                handle_hotkey_action(&app, hotkey_id);
            } else if msg.message == WM_RELOAD_HOTKEYS {
                info!("Reloading global hotkeys from updated configuration...");
                unregister_all_hotkeys();
                register_all_hotkeys(&app);
            } else if msg.message == WM_QUIT {
                info!("Exiting global hotkey message loop...");
                break;
            }

            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    unregister_all_hotkeys();
    HOTKEY_THREAD_ID.store(0, Ordering::SeqCst);
    info!("Global hotkey manager thread terminated cleanly.");
}

#[cfg(target_os = "windows")]
fn register_all_hotkeys(app: &AppHandle) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS};

    let config = if let Some(state) = app.try_state::<AppState>() {
        state.get_config().shortcuts.clone()
    } else if let Ok(manager) = ConfigManager::init_default() {
        manager.load_or_create().map(|c| c.shortcuts).unwrap_or_default()
    } else {
        ShortcutsConfig::default()
    };

    if !config.shortcuts_enabled {
        info!("Global hotkeys are disabled in configuration. Skipping registration.");
        return;
    }

    let shortcuts_to_register = [
        (HOTKEY_ID_TOGGLE, "Toggle Theme", &config.toggle_shortcut),
        (HOTKEY_ID_FORCE_LIGHT, "Force Light", &config.force_light_shortcut),
        (HOTKEY_ID_FORCE_DARK, "Force Dark", &config.force_dark_shortcut),
        (HOTKEY_ID_POSTPONE, "Postpone Transition", &config.postpone_shortcut),
    ];

    for (id, label, shortcut_str) in shortcuts_to_register {
        if shortcut_str.trim().is_empty() {
            continue;
        }

        if let Some((mods, vk)) = parse_shortcut_string(shortcut_str) {
            let res = unsafe {
                RegisterHotKey(
                    HWND(std::ptr::null_mut()),
                    id,
                    HOT_KEY_MODIFIERS(mods),
                    vk,
                )
            };

            if res.is_ok() {
                info!("Successfully registered global hotkey: '{}' ({}) -> ID {}", shortcut_str, label, id);
            } else {
                warn!("Failed to register global hotkey: '{}' ({}) - key combination may be occupied by another app.", shortcut_str, label);
            }
        } else {
            warn!("Could not parse hotkey key combination: '{}'", shortcut_str);
        }
    }
}

#[cfg(target_os = "windows")]
fn unregister_all_hotkeys() {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::Input::KeyboardAndMouse::UnregisterHotKey;

    let ids = [
        HOTKEY_ID_TOGGLE,
        HOTKEY_ID_FORCE_LIGHT,
        HOTKEY_ID_FORCE_DARK,
        HOTKEY_ID_POSTPONE,
    ];

    for id in ids {
        unsafe {
            let _ = UnregisterHotKey(HWND(std::ptr::null_mut()), id);
        }
    }
}

fn handle_hotkey_action(app: &AppHandle, hotkey_id: i32) {
    let now = chrono::Local::now().timestamp();

    match hotkey_id {
        HOTKEY_ID_TOGGLE => {
            let current = detect_windows_theme()
                .map(|t| t.effective_theme)
                .unwrap_or(ThemeMode::Dark);
            let next_theme = match current {
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::Light,
            };

            info!("Hotkey Toggle: Switching theme from {:?} to {:?}", current, next_theme);
            let _ = apply_theme_internally(app, next_theme, "Hotkey: Toggle Theme");
            let _ = app.emit("theme-changed", HotkeyTriggerEvent {
                action: "Toggle Theme".to_string(),
                target_theme: Some(format!("{:?}", next_theme)),
                timestamp: now,
            });
        }
        HOTKEY_ID_FORCE_LIGHT => {
            info!("Hotkey Force Light: Activating Light Mode");
            let _ = apply_theme_internally(app, ThemeMode::Light, "Hotkey: Force Light Mode");
            let _ = app.emit("theme-changed", HotkeyTriggerEvent {
                action: "Force Light Mode".to_string(),
                target_theme: Some("Light".to_string()),
                timestamp: now,
            });
        }
        HOTKEY_ID_FORCE_DARK => {
            info!("Hotkey Force Dark: Activating Dark Mode");
            let _ = apply_theme_internally(app, ThemeMode::Dark, "Hotkey: Force Dark Mode");
            let _ = app.emit("theme-changed", HotkeyTriggerEvent {
                action: "Force Dark Mode".to_string(),
                target_theme: Some("Dark".to_string()),
                timestamp: now,
            });
        }
        HOTKEY_ID_POSTPONE => {
            info!("Hotkey Postpone: Delaying scheduled theme switch by 60 minutes");
            let _ = postpone_internally(app, 60);
            let _ = app.emit("theme-changed", HotkeyTriggerEvent {
                action: "Postpone +60m".to_string(),
                target_theme: None,
                timestamp: now,
            });
        }
        _ => {
            warn!("Received unrecognized hotkey ID: {}", hotkey_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shortcut_string() {
        let parsed = parse_shortcut_string("Ctrl+Alt+T");
        assert!(parsed.is_some());
        let (mods, vk) = parsed.unwrap();
        assert_eq!(mods & 0x0002, 0x0002); // MOD_CONTROL
        assert_eq!(mods & 0x0001, 0x0001); // MOD_ALT
        assert_eq!(vk, 'T' as u32);

        let parsed_f = parse_shortcut_string("Ctrl+Shift+F5");
        assert!(parsed_f.is_some());
        let (mods_f, vk_f) = parsed_f.unwrap();
        assert_eq!(mods_f & 0x0002, 0x0002); // MOD_CONTROL
        assert_eq!(mods_f & 0x0004, 0x0004); // MOD_SHIFT
        assert_eq!(vk_f, 0x74); // VK_F5
    }

    #[test]
    fn test_parse_invalid_shortcut() {
        assert!(parse_shortcut_string("").is_none());
        assert!(parse_shortcut_string("InvalidModifierOnly").is_none());
    }
}
