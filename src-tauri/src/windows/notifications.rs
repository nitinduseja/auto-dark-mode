use std::process::Command;
use tracing::info;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Displays a Windows system toast / balloon notification when theme transition occurs
pub fn show_theme_switch_notification(theme_name: &str) {
    let theme = theme_name.to_string();
    let script = format!(
        "[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms') | Out-Null; \
         $b = New-Object System.Windows.Forms.NotifyIcon; \
         $b.Icon = [System.Drawing.SystemIcons]::Information; \
         $b.Visible = $true; \
         $b.ShowBalloonTip(2500, 'AutoDarkMode', 'Switched appearance to {} Mode', [System.Windows.Forms.ToolTipIcon]::Info); \
         Start-Sleep -Milliseconds 700; \
         $b.Dispose()",
        theme
    );

    std::thread::spawn(move || {
        let mut cmd = Command::new("powershell.exe");
        cmd.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", &script]);

        #[cfg(target_os = "windows")]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let _ = cmd.status();
        info!("Theme notification dispatched for {} Mode", theme);
    });
}
