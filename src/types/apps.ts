import type { ThemeMode } from './theme';

export interface CustomAppRule {
  id: string;
  name: string;
  process_name: string;
  target_theme: 'system' | 'dark' | 'light';
  enabled: boolean;
}

export interface AppsConfig {
  apps_separate_from_system: boolean;
  apps_theme: ThemeMode;
  system_theme: ThemeMode;
  sync_office: boolean;
  sync_browsers: boolean;
  custom_rules: CustomAppRule[];
}
