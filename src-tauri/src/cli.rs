use chrono::Local;
use crate::config::{ConfigManager, ManualOverride};
use crate::windows::{apply_windows_theme, detect_windows_theme, ThemeMode};

#[cfg(target_os = "windows")]
pub fn attach_console() {
    unsafe {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

/// Parses CLI arguments and executes headless commands if provided.
/// Returns `true` if a CLI action was handled and the application should exit,
/// or `false` to proceed with full GUI/daemon launch.
pub fn handle_cli_args(args: &[String]) -> bool {
    if args.len() <= 1 {
        return false;
    }

    let flag = args[1].to_lowercase();
    match flag.as_str() {
        "--help" | "-h" | "/?" => {
            #[cfg(target_os = "windows")]
            attach_console();
            println!("\nAutoDarkMode CLI v1.2.0 - Automatic Windows 11 Appearance Switcher");
            println!("Usage: autodarkmode.exe [OPTIONS]\n");
            println!("Options:");
            println!("  -d, --dark              Switch Windows to Dark Mode");
            println!("  -l, --light             Switch Windows to Light Mode");
            println!("  -t, --toggle            Toggle between Light and Dark Mode");
            println!("  -s, --status            Print current active theme (Light or Dark)");
            println!("  -p, --postpone <MIN>    Postpone automatic transitions for N minutes");
            println!("  -r, --resume            Clear postponement and resume automated scheduling");
            println!("  -m, --minimized         Launch in background minimized to system tray");
            println!("  -v, --version           Display application version");
            println!("  -h, --help              Display this help message\n");
            true
        }
        "--version" | "-v" => {
            #[cfg(target_os = "windows")]
            attach_console();
            println!("AutoDarkMode v1.2.0");
            true
        }
        "--status" | "-s" => {
            #[cfg(target_os = "windows")]
            attach_console();
            let theme = detect_windows_theme()
                .map(|t| format!("{:?}", t.effective_theme))
                .unwrap_or_else(|_| "Unknown".to_string());
            println!("Current theme: {}", theme);
            true
        }
        "--dark" | "-d" => {
            #[cfg(target_os = "windows")]
            attach_console();
            match apply_windows_theme(ThemeMode::Dark) {
                Ok(_) => println!("Successfully switched Windows to Dark Mode"),
                Err(e) => eprintln!("Failed switching to Dark Mode: {}", e),
            }
            true
        }
        "--light" | "-l" => {
            #[cfg(target_os = "windows")]
            attach_console();
            match apply_windows_theme(ThemeMode::Light) {
                Ok(_) => println!("Successfully switched Windows to Light Mode"),
                Err(e) => eprintln!("Failed switching to Light Mode: {}", e),
            }
            true
        }
        "--toggle" | "-t" => {
            #[cfg(target_os = "windows")]
            attach_console();
            let target = match detect_windows_theme() {
                Ok(t) => match t.effective_theme {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                },
                Err(_) => ThemeMode::Dark,
            };
            match apply_windows_theme(target) {
                Ok(_) => println!("Toggled theme to {:?}", target),
                Err(e) => eprintln!("Failed toggling theme: {}", e),
            }
            true
        }
        "--postpone" | "-p" => {
            #[cfg(target_os = "windows")]
            attach_console();
            let minutes: u32 = args.get(2).and_then(|m| m.parse().ok()).unwrap_or(60);
            if let Ok(manager) = ConfigManager::init_default() {
                if let Ok(mut config) = manager.load_or_create() {
                    let current_theme = detect_windows_theme()
                        .map(|t| t.effective_theme)
                        .unwrap_or(ThemeMode::Dark);
                    let until = Local::now().timestamp() + (minutes as i64 * 60);
                    config.manual_override = Some(ManualOverride {
                        target_theme: current_theme,
                        until_timestamp: Some(until),
                        reason: format!("CLI postpone for {}m", minutes),
                    });
                    let _ = manager.save(&config);
                    println!("Automatic theme switching postponed for {} minutes", minutes);
                }
            }
            true
        }
        "--resume" | "-r" => {
            #[cfg(target_os = "windows")]
            attach_console();
            if let Ok(manager) = ConfigManager::init_default() {
                if let Ok(mut config) = manager.load_or_create() {
                    config.manual_override = None;
                    let _ = manager.save(&config);
                    println!("Cleared postponement. Automatic scheduling resumed.");
                }
            }
            true
        }
        "--minimized" | "--silent" | "--background" | "-m" => {
            // Background daemon flag - proceed to full GUI/daemon launch
            false
        }
        _ => false,
    }
}
