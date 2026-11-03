pub mod gaming;
pub mod installed_themes;
pub mod power;
pub mod registry;
pub mod startup;
pub mod theme;
pub mod wallpaper;
pub mod window;

pub use gaming::is_fullscreen_active;
pub use installed_themes::{apply_theme_file, discover_installed_themes, InstalledTheme};
pub use power::{get_power_status, PowerStatus};
pub use registry::RegistryError;
pub use startup::{is_run_at_startup, is_run_minimized_at_startup, set_background_priority, set_run_at_startup};
pub use theme::{apply_windows_theme, detect_windows_theme, DetectedTheme, ThemeError, ThemeMode};
pub use wallpaper::set_desktop_wallpaper;
pub use window::{apply_mica_to_hwnd, MicaStatus};
