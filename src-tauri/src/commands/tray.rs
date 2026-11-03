use tauri::{command, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tracing::info;

use super::OperationResult;

#[command]
pub async fn open_main_window(app: AppHandle, page: Option<String>) -> OperationResult<bool> {
    let target_page = page.unwrap_or_else(|| "overview".to_string());
    info!(
        "Handling open_main_window from tray popup for page: {}",
        target_page
    );

    // 1. Hide the tray popup
    if let Some(popup) = app.get_webview_window("tray-popup") {
        let _ = popup.hide();
    }

    // Set normal foreground priority for interactive window
    crate::windows::startup::set_background_priority(false);

    // 2. Locate or recreate the main window
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.show();
        let _ = main.unminimize();
        let _ = main.set_focus();
        let _ = main.emit("navigate-page", &target_page);
        OperationResult::ok(true)
    } else {
        const APP_ICON: tauri::image::Image<'_> = tauri::include_image!("icons/128x128.png");
        let builder = WebviewWindowBuilder::new(
            &app,
            "main",
            WebviewUrl::App(format!("index.html?page={}", target_page).into()),
        )
        .title("AutoDarkMode")
        .inner_size(1200.0, 800.0)
        .min_inner_size(1000.0, 700.0)
        .resizable(true)
        .decorations(false)
        .visible(true);

        match builder.build() {
            Ok(new_main) => {
                let _ = new_main.set_icon(APP_ICON);
                let _ = new_main.show();
                let _ = new_main.unminimize();
                let _ = new_main.set_focus();
                #[cfg(target_os = "windows")]
                {
                    if let Ok(hwnd) = new_main.hwnd() {
                        let _ = crate::windows::window::apply_mica_to_hwnd(hwnd.0 as isize);
                    }
                }
                OperationResult::ok(true)
            }
            Err(e) => OperationResult::err(format!("Failed to recreate main window: {}", e)),
        }
    }
}

#[command]
pub async fn hide_main_window(app: AppHandle) -> OperationResult<bool> {
    info!("Hiding main window to system tray");
    crate::windows::startup::set_background_priority(true);
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.hide();
        OperationResult::ok(true)
    } else {
        OperationResult::ok(false)
    }
}

#[command]
pub async fn close_tray_popup(app: AppHandle) -> OperationResult<bool> {
    info!("Handling close_tray_popup");
    if let Some(popup) = app.get_webview_window("tray-popup") {
        let _ = popup.hide();
        OperationResult::ok(true)
    } else {
        OperationResult::ok(false)
    }
}

#[command]
pub async fn exit_app(app: AppHandle) -> OperationResult<bool> {
    info!("Exiting AutoDarkMode via system tray menu");
    app.exit(0);
    OperationResult::ok(true)
}
