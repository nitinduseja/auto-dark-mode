#[cfg(target_os = "windows")]
use windows::Win32::Foundation::RECT;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    GetDesktopWindow, GetForegroundWindow, GetSystemMetrics, GetWindowRect, SM_CXSCREEN, SM_CYSCREEN,
};

pub fn is_fullscreen_active() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let foreground = GetForegroundWindow();
            if foreground.0.is_null() {
                return false;
            }

            let desktop = GetDesktopWindow();
            if foreground == desktop {
                return false;
            }

            let mut rect = RECT::default();
            if GetWindowRect(foreground, &mut rect).is_err() {
                return false;
            }

            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);

            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            // If window bounds match or cover the primary monitor
            width >= screen_width && height >= screen_height && rect.left <= 0 && rect.top <= 0
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}
