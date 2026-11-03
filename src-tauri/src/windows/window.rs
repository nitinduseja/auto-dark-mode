use tracing::{info, warn};

pub const DWMWA_USE_IMMERSIVE_DARK_MODE: u32 = 20;
pub const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
pub const DWMWA_MICA_EFFECT: u32 = 1029;

pub const DWMSBT_AUTO: u32 = 0;
pub const DWMSBT_NONE: u32 = 1;
pub const DWMSBT_MAINWINDOW: u32 = 2; // Mica
pub const DWMSBT_TRANSIENTWINDOW: u32 = 3; // Acrylic
pub const DWMSBT_TABBEDWINDOW: u32 = 4; // Mica Alt

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MicaStatus {
    AppliedSystemBackdrop,
    AppliedLegacyMica,
    Unsupported,
    Failed(String),
}

pub fn apply_mica_to_hwnd(hwnd_isize: isize) -> MicaStatus {
    #[cfg(target_os = "windows")]
    {
        win_impl::apply_mica(hwnd_isize)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = hwnd_isize;
        MicaStatus::Unsupported
    }
}

pub fn update_window_theme(hwnd_isize: isize, is_dark: bool) {
    #[cfg(target_os = "windows")]
    {
        win_impl::set_immersive_dark_mode(hwnd_isize, is_dark);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (hwnd_isize, is_dark);
    }
}

#[cfg(target_os = "windows")]
mod win_impl {
    use super::*;
    use windows::Win32::Foundation::{BOOL, HWND};
    use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};

    pub fn set_immersive_dark_mode(hwnd_isize: isize, is_dark: bool) {
        let hwnd = HWND(hwnd_isize as *mut std::ffi::c_void);
        let dark_val = if is_dark { BOOL(1) } else { BOOL(0) };
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWINDOWATTRIBUTE(DWMWA_USE_IMMERSIVE_DARK_MODE as i32),
                &dark_val as *const BOOL as *const std::ffi::c_void,
                std::mem::size_of::<BOOL>() as u32,
            );
        }
        info!("Updated DWM immersive dark mode to: {}", is_dark);
    }

    pub fn apply_mica(hwnd_isize: isize) -> MicaStatus {
        let hwnd = HWND(hwnd_isize as *mut std::ffi::c_void);

        // 1. First enable Immersive Dark Mode for window title and border
        set_immersive_dark_mode(hwnd_isize, true);

        // 2. Try Windows 11 22H2+ System Backdrop (Attribute 38 = DWMSBT_MAINWINDOW)
        let backdrop: u32 = DWMSBT_MAINWINDOW;
        let res_backdrop = unsafe {
            DwmSetWindowAttribute(
                hwnd,
                DWMWINDOWATTRIBUTE(DWMWA_SYSTEMBACKDROP_TYPE as i32),
                &backdrop as *const u32 as *const std::ffi::c_void,
                std::mem::size_of::<u32>() as u32,
            )
        };

        if res_backdrop.is_ok() {
            info!("Applied Windows 11 DWM Mica system backdrop successfully");
            return MicaStatus::AppliedSystemBackdrop;
        }

        // 3. Fallback to Windows 11 21H2 Mica effect (Attribute 1029)
        let mica_legacy: BOOL = BOOL(1);
        let res_legacy = unsafe {
            DwmSetWindowAttribute(
                hwnd,
                DWMWINDOWATTRIBUTE(DWMWA_MICA_EFFECT as i32),
                &mica_legacy as *const BOOL as *const std::ffi::c_void,
                std::mem::size_of::<BOOL>() as u32,
            )
        };

        if res_legacy.is_ok() {
            info!("Applied Windows 11 21H2 legacy Mica effect");
            return MicaStatus::AppliedLegacyMica;
        }

        warn!("DWM Mica effect unsupported or failed on this Windows build; using visual fallback");
        MicaStatus::Failed(format!(
            "Backdrop: {:?}, Legacy: {:?}",
            res_backdrop, res_legacy
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwm_constants() {
        assert_eq!(DWMWA_USE_IMMERSIVE_DARK_MODE, 20);
        assert_eq!(DWMWA_SYSTEMBACKDROP_TYPE, 38);
        assert_eq!(DWMSBT_MAINWINDOW, 2);
    }
}
