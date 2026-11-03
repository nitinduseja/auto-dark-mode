use tracing::info;
use crate::windows::registry::{delete_value, read_string, write_string};

const RUN_SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_RUN_NAME: &str = "AutoDarkMode";

pub fn is_run_at_startup() -> bool {
    read_string(RUN_SUBKEY, APP_RUN_NAME).is_ok()
}

pub fn is_run_minimized_at_startup() -> bool {
    if let Ok(cmd) = read_string(RUN_SUBKEY, APP_RUN_NAME) {
        let cmd_lower = cmd.to_lowercase();
        cmd_lower.contains("--minimized")
            || cmd_lower.contains("--background")
            || cmd_lower.contains("--silent")
    } else {
        false
    }
}

#[cfg(target_os = "windows")]
pub fn set_background_priority(is_background: bool) {
    use windows::Win32::System::Threading::{
        GetCurrentProcess, SetPriorityClass, BELOW_NORMAL_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
    };
    unsafe {
        let priority = if is_background {
            BELOW_NORMAL_PRIORITY_CLASS
        } else {
            NORMAL_PRIORITY_CLASS
        };
        let _ = SetPriorityClass(GetCurrentProcess(), priority);
        info!(
            "Windows process priority set to: {}",
            if is_background {
                "BELOW_NORMAL (Background Daemon)"
            } else {
                "NORMAL (Foreground Interactive)"
            }
        );
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_background_priority(_is_background: bool) {}

pub fn set_run_at_startup(enabled: bool, start_minimized: bool) -> Result<(), String> {
    if enabled {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to resolve executable path: {}", e))?;
        let exe_str = exe_path.to_string_lossy();
        let cmd = if start_minimized {
            format!("\"{}\" --minimized", exe_str)
        } else {
            format!("\"{}\"", exe_str)
        };

        write_string(RUN_SUBKEY, APP_RUN_NAME, &cmd)
            .map_err(|e| format!("Failed to register startup entry: {:?}", e))?;
        info!("Registered AutoDarkMode to run on Windows startup: {}", cmd);
    } else {
        delete_value(RUN_SUBKEY, APP_RUN_NAME)
            .map_err(|e| format!("Failed to remove startup entry: {:?}", e))?;
        info!("Removed AutoDarkMode from Windows startup");
    }
    Ok(())
}
