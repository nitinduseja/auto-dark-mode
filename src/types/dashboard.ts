import type { DetectedTheme, ThemeConfig, ThemeMode } from './theme';
import type { ScheduleConfig } from './schedule';
import type { AppsConfig } from './apps';
import type { PersonalizationConfig } from './personalization';
import type { ConditionsConfig } from './conditions';
import type { AutomationConfig } from './automation';
import type { ShortcutsConfig } from './shortcuts';
import type { SettingsConfig } from './settings';

export type SchedulerStatus = 'real' | 'demonstration' | 'unavailable' | 'unsupported';

export interface NextTransition {
  target_theme: ThemeMode;
  event_name: string;
  time_formatted: string;
  countdown_formatted: string;
  location_note: string;
  status: SchedulerStatus;
}

export interface ScheduleTimeline {
  sunrise_time_formatted: string;
  sunset_time_formatted: string;
  night1_percentage: number;
  day_percentage: number;
  night2_percentage: number;
  current_needle_percentage: number;
  current_time_formatted: string;
  daylight_window_formatted: string;
  status: SchedulerStatus;
}

export interface ControlState {
  id: string;
  title: string;
  description: string;
  enabled: boolean;
  supported: boolean;
  interactive: boolean;
  badge?: string;
}

export interface AppInfo {
  app_name: string;
  version: string;
  target_platform: string;
  webview_runtime: string;
}

export interface DashboardState {
  current_theme: ThemeMode;
  detected_theme: DetectedTheme;
  automation_enabled: boolean;
  last_switched_formatted: string;
  next_transition: NextTransition;
  schedule_timeline: ScheduleTimeline;
  quick_controls: ControlState[];
  app_info: AppInfo;
}

export interface AppConfig {
  version: number;
  theme: ThemeConfig;
  schedule: ScheduleConfig;
  features: {
    wallpaper: boolean;
    app_themes: boolean;
  };
  apps?: AppsConfig;
  personalization?: PersonalizationConfig;
  conditions?: ConditionsConfig;
  automation?: AutomationConfig;
  shortcuts?: ShortcutsConfig;
  settings?: SettingsConfig;
  last_switched_timestamp?: number;
  last_switched_trigger?: string;
}
