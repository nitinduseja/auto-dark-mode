export interface SettingsConfig {
  app_ui_theme: 'system' | 'dark' | 'light';
  language: string;
  logging_level: string;
}

export interface DiagnosticsInfo {
  os_name: string;
  os_build: string;
  arch: string;
  tauri_version: string;
  webview_version: string;
  config_path: string;
  uptime_seconds: number;
}
