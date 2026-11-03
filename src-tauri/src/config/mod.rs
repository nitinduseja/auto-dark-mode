use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use thiserror::Error;
use tracing::{info, warn};

use crate::windows::ThemeMode;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    #[error("Configuration directory could not be determined")]
    DirectoryNotFound,

    #[error("Failed to read configuration file: {0}")]
    ReadFailed(String),

    #[error("Configuration file is corrupted: {0}")]
    Corrupted(String),

    #[error("Failed to write temporary configuration file: {0}")]
    TempWriteFailed(String),

    #[error("Failed to atomically replace configuration file: {0}")]
    AtomicReplaceFailed(String),

    #[error("Unsupported configuration version: {0}")]
    UnsupportedVersion(u32),

    #[error("Validation failed: {0}")]
    ValidationError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub current: ThemeMode,
    pub automatic: bool,
    pub use_theme_files: bool,
    pub light_theme_path: String,
    pub light_theme_name: String,
    pub dark_theme_path: String,
    pub dark_theme_name: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            current: ThemeMode::Dark,
            automatic: true,
            use_theme_files: false,
            light_theme_path: r"C:\Windows\Resources\Themes\aero.theme".to_string(),
            light_theme_name: "Windows (light)".to_string(),
            dark_theme_path: r"C:\Windows\Resources\Themes\dark.theme".to_string(),
            dark_theme_name: "Windows (dark)".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ScheduleConfig {
    #[serde(rename = "type")]
    pub schedule_type: String, // "sunrise_sunset", "custom_times", "night_light"
    pub enabled: bool,
    pub light_start_hour: u32,
    pub light_start_minute: u32,
    pub dark_start_hour: u32,
    pub dark_start_minute: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub location_name: String,
    pub use_location_service: bool,
    pub sunrise_offset_minutes: i32,
    pub sunset_offset_minutes: i32,
    pub idle_delay_minutes: u32,
    pub pause_on_battery: bool,
    pub pause_on_gaming: bool,
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            schedule_type: "sunrise_sunset".to_string(),
            enabled: true,
            light_start_hour: 7,
            light_start_minute: 0,
            dark_start_hour: 19,
            dark_start_minute: 0,
            latitude: 40.7128,
            longitude: -74.0060,
            location_name: "New York, USA".to_string(),
            use_location_service: false,
            sunrise_offset_minutes: 0,
            sunset_offset_minutes: 0,
            idle_delay_minutes: 0,
            pause_on_battery: false,
            pause_on_gaming: false,
        }
    }
}

impl ScheduleConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.light_start_hour > 23 || self.dark_start_hour > 23 {
            return Err(ConfigError::ValidationError(
                "Hours must be between 0 and 23".to_string(),
            ));
        }
        if self.light_start_minute > 59 || self.dark_start_minute > 59 {
            return Err(ConfigError::ValidationError(
                "Minutes must be between 0 and 59".to_string(),
            ));
        }
        if self.latitude < -90.0 || self.latitude > 90.0 {
            return Err(ConfigError::ValidationError(
                "Latitude must be between -90.0 and +90.0".to_string(),
            ));
        }
        if self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(ConfigError::ValidationError(
                "Longitude must be between -180.0 and +180.0".to_string(),
            ));
        }
        if self.sunrise_offset_minutes < -240 || self.sunrise_offset_minutes > 240 {
            return Err(ConfigError::ValidationError(
                "Sunrise offset must be between -240 and +240 minutes".to_string(),
            ));
        }
        if self.sunset_offset_minutes < -240 || self.sunset_offset_minutes > 240 {
            return Err(ConfigError::ValidationError(
                "Sunset offset must be between -240 and +240 minutes".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FeaturesConfig {
    pub wallpaper: bool,
    pub app_themes: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomAppRule {
    pub id: String,
    pub name: String,
    pub process_name: String,
    pub target_theme: String, // "system", "dark", "light"
    pub enabled: bool,
}

impl Default for CustomAppRule {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Notepad".to_string(),
            process_name: "notepad.exe".to_string(),
            target_theme: "system".to_string(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppsConfig {
    pub apps_separate_from_system: bool,
    pub apps_theme: ThemeMode,
    pub system_theme: ThemeMode,
    pub sync_office: bool,
    pub sync_browsers: bool,
    pub custom_rules: Vec<CustomAppRule>,
}

impl Default for AppsConfig {
    fn default() -> Self {
        Self {
            apps_separate_from_system: false,
            apps_theme: ThemeMode::Dark,
            system_theme: ThemeMode::Dark,
            sync_office: true,
            sync_browsers: true,
            custom_rules: vec![
                CustomAppRule {
                    id: "rule_1".to_string(),
                    name: "Visual Studio Code".to_string(),
                    process_name: "code.exe".to_string(),
                    target_theme: "dark".to_string(),
                    enabled: true,
                },
                CustomAppRule {
                    id: "rule_2".to_string(),
                    name: "Windows Terminal".to_string(),
                    process_name: "windowsterminal.exe".to_string(),
                    target_theme: "system".to_string(),
                    enabled: true,
                },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PersonalizationConfig {
    pub wallpaper_enabled: bool,
    pub light_wallpaper_path: String,
    pub dark_wallpaper_path: String,
    pub wallpaper_fit: String, // "fill", "fit", "stretch", "tile", "center", "span"
    pub sync_accent_color: bool,
    pub light_accent_color: String,
    pub dark_accent_color: String,
    pub sync_cursor: bool,
    pub sync_sounds: bool,
}

impl Default for PersonalizationConfig {
    fn default() -> Self {
        Self {
            wallpaper_enabled: false,
            light_wallpaper_path: String::new(),
            dark_wallpaper_path: String::new(),
            wallpaper_fit: "fill".to_string(),
            sync_accent_color: false,
            light_accent_color: "#0078D4".to_string(),
            dark_accent_color: "#60CDFF".to_string(),
            sync_cursor: false,
            sync_sounds: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConditionsConfig {
    pub pause_on_battery: bool,
    pub force_dark_on_battery_saver: bool,
    pub pause_on_gaming: bool,
    pub idle_delay_minutes: u32,
    pub pause_on_metered: bool,
}

impl Default for ConditionsConfig {
    fn default() -> Self {
        Self {
            pause_on_battery: false,
            force_dark_on_battery_saver: false,
            pause_on_gaming: false,
            idle_delay_minutes: 0,
            pause_on_metered: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AutomationConfig {
    pub launch_on_startup: bool,
    pub start_minimized: bool,
    pub daemon_mode: bool,
    pub notify_on_switch: bool,
    pub check_updates: bool,
}

impl Default for AutomationConfig {
    fn default() -> Self {
        Self {
            launch_on_startup: false,
            start_minimized: true,
            daemon_mode: true,
            notify_on_switch: true,
            check_updates: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutsConfig {
    pub toggle_shortcut: String,
    pub force_light_shortcut: String,
    pub force_dark_shortcut: String,
    pub postpone_shortcut: String,
    pub shortcuts_enabled: bool,
}

impl Default for ShortcutsConfig {
    fn default() -> Self {
        Self {
            toggle_shortcut: "Ctrl+Alt+T".to_string(),
            force_light_shortcut: "Ctrl+Alt+L".to_string(),
            force_dark_shortcut: "Ctrl+Alt+D".to_string(),
            postpone_shortcut: "Ctrl+Alt+P".to_string(),
            shortcuts_enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SettingsConfig {
    pub app_ui_theme: String, // "system", "dark", "light"
    pub language: String,
    pub logging_level: String,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            app_ui_theme: "system".to_string(),
            language: "en-US".to_string(),
            logging_level: "info".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub version: u32,
    pub theme: ThemeConfig,
    pub schedule: ScheduleConfig,
    pub features: FeaturesConfig,
    pub apps: AppsConfig,
    pub personalization: PersonalizationConfig,
    pub conditions: ConditionsConfig,
    pub automation: AutomationConfig,
    pub shortcuts: ShortcutsConfig,
    pub settings: SettingsConfig,
    pub last_switched_timestamp: Option<i64>,
    pub last_switched_trigger: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            theme: ThemeConfig::default(),
            schedule: ScheduleConfig::default(),
            features: FeaturesConfig::default(),
            apps: AppsConfig::default(),
            personalization: PersonalizationConfig::default(),
            conditions: ConditionsConfig::default(),
            automation: AutomationConfig::default(),
            shortcuts: ShortcutsConfig::default(),
            settings: SettingsConfig::default(),
            last_switched_timestamp: None,
            last_switched_trigger: None,
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.version != 1 {
            return Err(ConfigError::UnsupportedVersion(self.version));
        }
        self.schedule.validate()?;
        Ok(())
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new_with_path(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn default_path() -> Result<PathBuf, ConfigError> {
        let app_data = std::env::var("APPDATA")
            .map(PathBuf::from)
            .or_else(|_| {
                std::env::var("USERPROFILE")
                    .map(|p| PathBuf::from(p).join("AppData").join("Roaming"))
            })
            .map_err(|_| ConfigError::DirectoryNotFound)?;

        let dir = app_data.join("AutoDarkMode");
        Ok(dir.join("config.json"))
    }

    pub fn init_default() -> Result<Self, ConfigError> {
        let path = Self::default_path()?;
        Ok(Self::new_with_path(path))
    }

    pub fn load_or_create(&self) -> Result<AppConfig, ConfigError> {
        if !self.config_path.exists() {
            info!(
                "No existing configuration found; initializing defaults at {:?}",
                self.config_path
            );
            let default_config = AppConfig::default();
            self.save(&default_config)?;
            return Ok(default_config);
        }

        let mut file =
            File::open(&self.config_path).map_err(|e| ConfigError::ReadFailed(e.to_string()))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| ConfigError::ReadFailed(e.to_string()))?;

        // 1. Attempt deserialization with default filling for any missing schema fields
        match serde_json::from_str::<AppConfig>(&content) {
            Ok(parsed) => {
                if let Err(val_err) = parsed.validate() {
                    warn!(
                        "Configuration validation failed: {}. Re-saving sanitized defaults.",
                        val_err
                    );
                    let default_config = AppConfig::default();
                    let _ = self.save(&default_config);
                    return Ok(default_config);
                }
                Ok(parsed)
            }
            Err(e) => {
                // 2. Resilient recovery: backup corrupted file and initialize fresh default
                warn!(
                    "Config file at {:?} failed to parse ({}). Backing up and restoring defaults.",
                    self.config_path, e
                );

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let backup_path = self
                    .config_path
                    .with_file_name(format!("config.corrupted.{}.json", timestamp));
                let _ = fs::copy(&self.config_path, backup_path);

                let default_config = AppConfig::default();
                self.save(&default_config)?;
                Ok(default_config)
            }
        }
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), ConfigError> {
        config.validate()?;

        if let Some(parent) = self.config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| ConfigError::TempWriteFailed(e.to_string()))?;
            }
        }

        let serialized = serde_json::to_string_pretty(config)
            .map_err(|e| ConfigError::Corrupted(e.to_string()))?;

        let temp_filename = format!(
            "config.tmp.{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );
        let temp_path = self.config_path.with_file_name(temp_filename);

        {
            let mut temp_file = File::create(&temp_path)
                .map_err(|e| ConfigError::TempWriteFailed(e.to_string()))?;

            temp_file
                .write_all(serialized.as_bytes())
                .map_err(|e| ConfigError::TempWriteFailed(e.to_string()))?;

            temp_file
                .sync_all()
                .map_err(|e| ConfigError::TempWriteFailed(e.to_string()))?;
        }

        if self.config_path.exists() {
            let _ = fs::remove_file(&self.config_path);
        }

        fs::rename(&temp_path, &self.config_path).map_err(|e| {
            let _ = fs::remove_file(&temp_path);
            ConfigError::AtomicReplaceFailed(e.to_string())
        })?;

        info!("Configuration saved safely to {:?}", self.config_path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.theme.current, ThemeMode::Dark);
        assert!(config.theme.automatic);
        assert_eq!(config.schedule.schedule_type, "sunrise_sunset");
        assert!(config.schedule.enabled);
        assert_eq!(config.schedule.light_start_hour, 7);
        assert_eq!(config.schedule.dark_start_hour, 19);
        assert!(!config.features.wallpaper);
        assert!(!config.features.app_themes);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_backwards_compatible_deserialization() {
        let legacy_json = r#"{
          "version": 1,
          "theme": {
            "current": "light",
            "automatic": true
          },
          "schedule": {
            "type": "sunrise_sunset",
            "enabled": true,
            "demo_mode": true
          },
          "features": {
            "wallpaper": true,
            "app_themes": true
          }
        }"#;

        let parsed: AppConfig = serde_json::from_str(legacy_json).unwrap();
        assert_eq!(parsed.theme.current, ThemeMode::Light);
        assert!(parsed.theme.automatic);
        assert_eq!(parsed.schedule.schedule_type, "sunrise_sunset");
        assert!(parsed.schedule.enabled);
        assert_eq!(parsed.schedule.light_start_hour, 7);
        assert_eq!(parsed.schedule.dark_start_hour, 19);
        assert_eq!(parsed.schedule.latitude, 40.7128);
        assert!(parsed.features.wallpaper);
        assert!(parsed.features.app_themes);
        assert!(parsed.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_unsupported_version() {
        let config = AppConfig {
            version: 99,
            ..Default::default()
        };
        assert_eq!(config.validate(), Err(ConfigError::UnsupportedVersion(99)));
    }

    #[test]
    fn test_invalid_schedule_hours() {
        let mut config = AppConfig::default();
        config.schedule.light_start_hour = 25;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_atomic_save_and_load() {
        let test_dir = temp_dir().join(format!(
            "adm_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let config_file = test_dir.join("config.json");
        let manager = ConfigManager::new_with_path(config_file.clone());

        let loaded = manager.load_or_create().unwrap();
        assert_eq!(loaded, AppConfig::default());
        assert!(config_file.exists());

        let mut modified = loaded.clone();
        modified.theme.current = ThemeMode::Light;
        modified.theme.automatic = false;
        modified.schedule.light_start_hour = 8;
        manager.save(&modified).unwrap();

        let reloaded = manager.load_or_create().unwrap();
        assert_eq!(reloaded.theme.current, ThemeMode::Light);
        assert!(!reloaded.theme.automatic);
        assert_eq!(reloaded.schedule.light_start_hour, 8);

        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_corrupted_config_automatic_recovery() {
        let test_dir = temp_dir().join(format!(
            "adm_corrupt_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let config_file = test_dir.join("config.json");
        fs::create_dir_all(&test_dir).unwrap();
        fs::write(&config_file, "{ invalid json text here").unwrap();

        let manager = ConfigManager::new_with_path(config_file.clone());
        let recovered = manager.load_or_create().unwrap();
        assert_eq!(recovered, AppConfig::default());

        let _ = fs::remove_dir_all(&test_dir);
    }
}
