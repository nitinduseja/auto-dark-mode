#[cfg(target_os = "windows")]
use windows::Win32::UI::Shell::{
    SHQueryUserNotificationState, QUNS_BUSY, QUNS_PRESENTATION_MODE, QUNS_RUNNING_D3D_FULL_SCREEN,
};
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
#[cfg(target_os = "windows")]
use windows::Win32::System::SystemInformation::GetTickCount64;

/// Detects whether a genuine 3D fullscreen game or presentation is running using
/// the official Windows Shell notification state API.
/// This prevents false positives from standard maximized windows (browsers, editors).
pub fn is_fullscreen_active() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            if let Ok(state) = SHQueryUserNotificationState() {
                return state == QUNS_RUNNING_D3D_FULL_SCREEN
                    || state == QUNS_PRESENTATION_MODE
                    || state == QUNS_BUSY;
            }
            false
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

/// Returns the number of seconds since the user last interacted with the system
/// (mouse movement, clicks, or keyboard input).
pub fn get_user_idle_seconds() -> u64 {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let mut lii = LASTINPUTINFO {
                cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                dwTime: 0,
            };
            if GetLastInputInfo(&mut lii).as_bool() {
                let current_tick = GetTickCount64();
                let last_tick = lii.dwTime as u64;
                if current_tick >= last_tick {
                    return (current_tick - last_tick) / 1000;
                }
            }
            0
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        0
    }
}
