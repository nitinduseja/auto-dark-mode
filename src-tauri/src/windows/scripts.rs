use std::process::Command;
use tracing::{info, warn};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Executes an external script or executable in the background upon a theme switch
pub fn execute_custom_script(script_path: &str, args: &str) -> Result<(), String> {
    let path = std::path::Path::new(script_path);
    if !path.exists() {
        warn!("Custom script path does not exist: {}", script_path);
        return Err(format!("Script not found: {}", script_path));
    }

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    let mut cmd = match ext.as_str() {
        "ps1" => {
            let mut c = Command::new("powershell.exe");
            c.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File", script_path]);
            if !args.trim().is_empty() {
                c.arg(args);
            }
            c
        }
        "bat" | "cmd" => {
            let mut c = Command::new("cmd.exe");
            c.args(["/c", script_path]);
            if !args.trim().is_empty() {
                c.arg(args);
            }
            c
        }
        _ => {
            let mut c = Command::new(script_path);
            if !args.trim().is_empty() {
                c.arg(args);
            }
            c
        }
    };

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    match cmd.spawn() {
        Ok(_) => {
            info!("Successfully launched custom script: {}", script_path);
            Ok(())
        }
        Err(e) => {
            warn!("Failed to spawn custom script {}: {}", script_path, e);
            Err(e.to_string())
        }
    }
}
