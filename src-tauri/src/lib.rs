pub mod commands;
pub mod config;
pub mod scheduler;
pub mod state;
pub mod tray;
pub mod windows;

use tauri::Manager;
use tracing::info;

pub fn should_start_in_background() -> bool {
    // Only start minimized/in background if explicitly passed via command line flags
    // (e.g. Windows startup registry entry, scheduled task, or silent daemon launch)
    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        let lower = arg.to_lowercase();
        if lower == "--minimized" || lower == "--silent" || lower == "--background" || lower == "-m" {
            return true;
        }
    }

    false
}

pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();

    info!("Initializing AutoDarkMode Tauri application...");

    const APP_ICON: tauri::image::Image<'_> = tauri::include_image!("icons/128x128.png");

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            info!("Second instance detected with args: {:?}", argv);
            let is_minimized_arg = argv.iter().any(|arg| {
                let lower = arg.to_lowercase();
                lower == "--minimized" || lower == "--silent" || lower == "--background" || lower == "-m"
            });

            // If the user launched the second instance normally (without minimized flag),
            // bring the existing main window to the foreground.
            if !is_minimized_arg {
                if let Some(main) = app.get_webview_window("main") {
                    windows::startup::set_background_priority(false);
                    let _ = main.show();
                    let _ = main.unminimize();
                    let _ = main.set_focus();
                }
            }
        }))
        .setup(|app| {
            let start_background = should_start_in_background();
            info!("Application starting up. Background daemon mode: {}", start_background);

            if let Some(window) = app.get_webview_window("main") {
                // Explicitly set window icon so taskbar and window switchers display the app icon
                let _ = window.set_icon(APP_ICON.clone());

                // When close is requested, hide to system tray so background daemon stays active
                let main_window_for_close = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        windows::startup::set_background_priority(true);
                        let _ = main_window_for_close.hide();
                    }
                });

                #[cfg(target_os = "windows")]
                {
                    if let Ok(hwnd) = window.hwnd() {
                        let status = windows::apply_mica_to_hwnd(hwnd.0 as isize);
                        info!("Mica initialization status: {:?}", status);
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = window;
                }

                if start_background {
                    info!("Startup flag detected: keeping main window hidden in system tray with background priority");
                    windows::startup::set_background_priority(true);
                    let _ = window.hide();
                } else {
                    info!("User interactive launch: displaying main window in foreground");
                    windows::startup::set_background_priority(false);
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }

            // Initialize System Tray (quick flyout popup is loaded lazily on-demand)
            if let Err(e) = tray::setup_tray(&app.handle().clone()) {
                tracing::warn!("Failed to setup system tray: {}", e);
            }

            // Start reliable background scheduler daemon
            scheduler::start_scheduler(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Dashboard & Theme
            commands::dashboard::get_dashboard_state,
            commands::dashboard::sync_theme_state,
            commands::theme::set_theme,
            commands::theme::set_automation_enabled,
            commands::theme::set_follow_sun_enabled,
            commands::theme::set_quick_control,
            commands::theme::get_installed_themes,
            commands::theme::set_mode_themes,
            commands::theme::apply_specific_theme,
            // Schedule
            commands::schedule::get_schedule_config,
            commands::schedule::set_schedule_config,
            commands::schedule::calculate_solar_preview,
            // Apps
            commands::apps::get_apps_config,
            commands::apps::set_apps_config,
            commands::apps::add_custom_app_rule,
            commands::apps::remove_custom_app_rule,
            commands::apps::toggle_custom_app_rule,
            // Personalization
            commands::personalization::get_personalization_config,
            commands::personalization::set_personalization_config,
            commands::personalization::test_apply_wallpaper,
            // Conditions
            commands::conditions::get_conditions_config,
            commands::conditions::set_conditions_config,
            commands::conditions::get_system_conditions_status,
            // Automation
            commands::automation::get_automation_config,
            commands::automation::set_automation_config,
            // Shortcuts
            commands::shortcuts::get_shortcuts_config,
            commands::shortcuts::set_shortcuts_config,
            // Settings & Diagnostics
            commands::settings::get_application_info,
            commands::settings::get_configuration,
            commands::settings::set_settings_config,
            commands::settings::export_configuration,
            commands::settings::import_configuration,
            commands::settings::reset_to_default_configuration,
            commands::settings::open_config_folder,
            commands::settings::open_external_url,
            commands::settings::get_diagnostics_info,
            // Tray Flyout Commands
            commands::tray::open_main_window,
            commands::tray::hide_main_window,
            commands::tray::close_tray_popup,
            commands::tray::exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running AutoDarkMode application");
}
