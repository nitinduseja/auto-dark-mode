export type ThemeMode = 'light' | 'dark';

export interface DetectedTheme {
  apps_theme: ThemeMode;
  system_theme: ThemeMode;
  is_synchronized: boolean;
  effective_theme: ThemeMode;
}

export interface SetThemeRequest {
  theme: ThemeMode;
}

export interface SetAutomationRequest {
  enabled: boolean;
}

export interface SetFollowSunRequest {
  enabled: boolean;
}

export interface InstalledTheme {
  id: string;
  name: string;
  path: string;
  is_system: boolean;
}

export interface ThemeConfig {
  current: ThemeMode;
  automatic: boolean;
  use_theme_files: boolean;
  light_theme_path: string;
  light_theme_name: string;
  dark_theme_path: string;
  dark_theme_name: string;
}

export interface SetModeThemesRequest {
  use_theme_files: boolean;
  light_theme_path: string;
  light_theme_name: string;
  dark_theme_path: string;
  dark_theme_name: string;
}

export interface OperationResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}
