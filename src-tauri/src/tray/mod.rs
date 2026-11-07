use std::sync::atomic::{AtomicI64, Ordering};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, PhysicalPosition, Position, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tracing::info;

pub const TRAY_WIDTH: i32 = 360;
pub const TRAY_HEIGHT: i32 = 440;

static LAST_BLUR_TIME: AtomicI64 = AtomicI64::new(0);

fn current_time_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[cfg(target_os = "windows")]
pub fn calculate_tray_popup_position_at(click_pos: Option<tauri::PhysicalPosition<f64>>) -> (i32, i32) {
    use std::ffi::c_void;
    use windows::Win32::Foundation::{POINT, RECT};
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SPI_GETWORKAREA,
    };

    if let Some(pos) = click_pos {
        let pt = POINT {
            x: pos.x as i32,
            y: pos.y as i32,
        };
        let hmon = unsafe { MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST) };
        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        let ok = unsafe { GetMonitorInfoW(hmon, &mut mi) };
        if ok.as_bool() {
            let x = (mi.rcWork.right - TRAY_WIDTH - 12).max(mi.rcWork.left);
            let y = (mi.rcWork.bottom - TRAY_HEIGHT - 12).max(mi.rcWork.top);
            return (x, y);
        }
    }

    let mut rect = RECT::default();
    let res = unsafe {
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut rect as *mut _ as *mut c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };

    if res.is_ok() {
        let x = rect.right - TRAY_WIDTH - 12;
        let y = rect.bottom - TRAY_HEIGHT - 12;
        (x, y)
    } else {
        (100, 100)
    }
}

#[cfg(not(target_os = "windows"))]
pub fn calculate_tray_popup_position_at(_click_pos: Option<tauri::PhysicalPosition<f64>>) -> (i32, i32) {
    (100, 100)
}

pub fn calculate_tray_popup_position() -> (i32, i32) {
    calculate_tray_popup_position_at(None)
}

pub fn create_tray_popup_window(app: &AppHandle) -> Result<WebviewWindow, tauri::Error> {
    let (x, y) = calculate_tray_popup_position();
    let popup = WebviewWindowBuilder::new(
        app,
        "tray-popup",
        WebviewUrl::App("index.html?view=tray".into()),
    )
    .title("AutoDarkMode Quick Flyout")
    .inner_size(TRAY_WIDTH as f64, TRAY_HEIGHT as f64)
    .resizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .visible(false)
    .build()?;

    let _ = popup.set_position(Position::Physical(PhysicalPosition { x, y }));
    #[cfg(target_os = "windows")]
    {
        if let Ok(hwnd) = popup.hwnd() {
            let _ = crate::windows::window::apply_mica_to_hwnd(hwnd.0 as isize);
        }
    }

    // Hide tray popup automatically whenever it loses focus (OneDrive/Action Center behavior)
    let popup_for_blur = popup.clone();
    popup.on_window_event(move |event| {
        if let tauri::WindowEvent::Focused(false) = event {
            if popup_for_blur.is_visible().unwrap_or(false) {
                LAST_BLUR_TIME.store(current_time_millis(), Ordering::SeqCst);
                let _ = popup_for_blur.hide();
            }
        }
    });

    Ok(popup)
}

pub fn toggle_tray_popup(app: &AppHandle, click_pos: Option<tauri::PhysicalPosition<f64>>) {
    // Debounce check: if popup just lost focus because user clicked the tray icon to dismiss it,
    // don't immediately reopen it.
    let now = current_time_millis();
    let last_blur = LAST_BLUR_TIME.load(Ordering::SeqCst);
    if now - last_blur < 250 {
        return;
    }

    let popup = match app.get_webview_window("tray-popup") {
        Some(w) => w,
        None => match create_tray_popup_window(app) {
            Ok(w) => w,
            Err(e) => {
                tracing::warn!("Failed to create tray popup window: {}", e);
                return;
            }
        },
    };

    let is_visible = popup.is_visible().unwrap_or(false);
    if is_visible {
        let _ = popup.hide();
    } else {
        let (x, y) = calculate_tray_popup_position_at(click_pos);
        let _ = popup.set_position(Position::Physical(PhysicalPosition { x, y }));
        let _ = popup.show();
        let _ = popup.set_focus();
    }
}

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing AutoDarkMode system tray icon and menus...");

    // Pre-create the tray flyout window in the background (hidden) so WebView2 completes
    // initialization before the first user click, preventing startup focus flicker
    if let Err(e) = create_tray_popup_window(app) {
        tracing::warn!("Failed to pre-initialize tray popup window: {}", e);
    }

    // Build right-click native context menu
    let menu = tauri::menu::MenuBuilder::new(app)
        .text("open_dashboard", "Open Dashboard")
        .separator()
        .text("switch_light", "Switch to Light Mode")
        .text("switch_dark", "Switch to Dark Mode")
        .separator()
        .text("postpone_1h", "Postpone 1 Hour")
        .text("resume_auto", "Resume Automation")
        .separator()
        .text("settings", "Settings")
        .text("exit", "Exit AutoDarkMode")
        .build()?;

    // Build Tray Icon
    const APP_ICON: tauri::image::Image<'_> = tauri::include_image!("icons/32x32.png");

    let _tray = TrayIconBuilder::new()
        .icon(APP_ICON)
        .tooltip("AutoDarkMode - Automatic Windows 11 Appearance")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open_dashboard" => {
                let _ = crate::commands::tray::open_main_window(
                    app.clone(),
                    Some("overview".to_string()),
                );
            }
            "switch_light" => {
                let app_c = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = crate::commands::theme::set_theme(
                        app_c,
                        crate::commands::theme::SetThemeRequest {
                            theme: crate::windows::ThemeMode::Light,
                        },
                    )
                    .await;
                });
            }
            "switch_dark" => {
                let app_c = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = crate::commands::theme::set_theme(
                        app_c,
                        crate::commands::theme::SetThemeRequest {
                            theme: crate::windows::ThemeMode::Dark,
                        },
                    )
                    .await;
                });
            }
            "postpone_1h" => {
                let app_c = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = crate::commands::theme::postpone_switch(
                        app_c,
                        crate::commands::theme::PostponeRequest { minutes: 60 },
                    )
                    .await;
                });
            }
            "resume_auto" => {
                let app_c = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = crate::commands::theme::resume_automation(app_c).await;
                });
            }
            "settings" => {
                let _ = crate::commands::tray::open_main_window(
                    app.clone(),
                    Some("settings".to_string()),
                );
            }
            "exit" => {
                let app_c = app.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = crate::commands::tray::exit_app(app_c).await;
                });
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                position,
                ..
            } => {
                toggle_tray_popup(tray.app_handle(), Some(position));
            }
            TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                let app = tray.app_handle();
                if let Some(popup) = app.get_webview_window("tray-popup") {
                    let _ = popup.hide();
                }
                if let Some(main) = app.get_webview_window("main") {
                    crate::windows::startup::set_background_priority(false);
                    let _ = main.show();
                    let _ = main.unminimize();
                    let _ = main.set_focus();
                }
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
