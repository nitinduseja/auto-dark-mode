import type { DashboardState, AppInfo, AppConfig } from '../types/dashboard';
import type {
  DetectedTheme,
  InstalledTheme,
  OperationResult,
  SetAutomationRequest,
  SetFollowSunRequest,
  SetModeThemesRequest,
  SetThemeRequest,
  ThemeConfig,
  ThemeMode,
} from '../types/theme';
import type {
  ScheduleConfig,
  SolarPreviewRequest,
  SolarPreviewResult,
} from '../types/schedule';
import type { AppsConfig, CustomAppRule } from '../types/apps';
import type { PersonalizationConfig } from '../types/personalization';
import type { ConditionsConfig, SystemConditionsStatus } from '../types/conditions';
import type { AutomationConfig } from '../types/automation';
import type { ShortcutsConfig } from '../types/shortcuts';
import type { SettingsConfig, DiagnosticsInfo } from '../types/settings';

function isTauriEnvironment(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

// ==================== DASHBOARD & THEME ====================

export async function fetchDashboardState(): Promise<DashboardState> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<DashboardState>>('get_dashboard_state');
    if (res.success && res.data) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to fetch dashboard state');
  }
  return getFallbackDashboardState();
}

export async function executeSetTheme(theme: ThemeMode): Promise<DetectedTheme> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const req: SetThemeRequest = { theme };
    const res = await invoke<OperationResult<DetectedTheme>>('set_theme', { request: req });
    if (res.success && res.data) {
      return res.data;
    }
    throw new Error(res.error || `Failed to set theme to ${theme}`);
  }
  return {
    apps_theme: theme,
    system_theme: theme,
    is_synchronized: true,
    effective_theme: theme,
  };
}

export async function executeSetAutomation(enabled: boolean): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const req: SetAutomationRequest = { enabled };
    const res = await invoke<OperationResult<boolean>>('set_automation_enabled', { request: req });
    if (res.success && res.data !== undefined) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to update automation state');
  }
  return enabled;
}

export async function executeSetFollowSun(enabled: boolean): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const req: SetFollowSunRequest = { enabled };
    const res = await invoke<OperationResult<boolean>>('set_follow_sun_enabled', { request: req });
    if (res.success && res.data !== undefined) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to update follow sunrise/sunset state');
  }
  return enabled;
}

export async function executeSetQuickControl(id: string, enabled: boolean): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('set_quick_control', {
      request: { id, enabled },
    });
    if (res.success && res.data !== undefined) {
      return res.data;
    }
    throw new Error(res.error || `Failed to update ${id} state`);
  }
  return enabled;
}

export async function executePostponeSwitch(minutes: number): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('postpone_switch', {
      request: { minutes },
    });
    if (res.success && res.data !== undefined) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to postpone switch');
  }
  return true;
}

export async function executeResumeAutomation(): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('resume_automation');
    if (res.success && res.data !== undefined) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to resume automation');
  }
  return true;
}

export async function syncThemeState(): Promise<DashboardState> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<DashboardState>>('sync_theme_state');
    if (res.success && res.data) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to sync theme state');
  }
  return getFallbackDashboardState();
}

// ==================== INSTALLED WINDOWS THEMES ====================

export async function fetchInstalledThemes(): Promise<InstalledTheme[]> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<InstalledTheme[]>>('get_installed_themes');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch installed themes');
  }
  return [
    {
      id: 'sys_aero',
      name: 'Windows (light)',
      path: 'C:\\Windows\\Resources\\Themes\\aero.theme',
      is_system: true,
    },
    {
      id: 'sys_dark',
      name: 'Windows (dark)',
      path: 'C:\\Windows\\Resources\\Themes\\dark.theme',
      is_system: true,
    },
    {
      id: 'sys_themea',
      name: 'Glow',
      path: 'C:\\Windows\\Resources\\Themes\\themeA.theme',
      is_system: true,
    },
    {
      id: 'sys_themeb',
      name: 'Captured Motion',
      path: 'C:\\Windows\\Resources\\Themes\\themeB.theme',
      is_system: true,
    },
    {
      id: 'sys_themec',
      name: 'Sunrise',
      path: 'C:\\Windows\\Resources\\Themes\\themeC.theme',
      is_system: true,
    },
    {
      id: 'sys_themed',
      name: 'Flow',
      path: 'C:\\Windows\\Resources\\Themes\\themeD.theme',
      is_system: true,
    },
  ];
}

export async function saveModeThemes(req: SetModeThemesRequest): Promise<ThemeConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<ThemeConfig>>('set_mode_themes', { request: req });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to save theme file configuration');
  }
  return {
    current: 'dark',
    automatic: true,
    ...req,
  };
}

export async function applySpecificTheme(path: string): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('apply_specific_theme', {
      request: { path },
    });
    if (res.success && res.data !== undefined) return res.data;
    throw new Error(res.error || 'Failed to apply theme');
  }
  return true;
}

// ==================== TRAY POPUP CONTROLS ====================

export async function openMainWindow(page: string = 'overview'): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('open_main_window', { page });
    if (res.success && res.data !== undefined) return res.data;
  }
  return true;
}

export async function closeTrayPopup(): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('close_tray_popup');
    if (res.success && res.data !== undefined) return res.data;
  }
  return true;
}

export async function exitApp(): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('exit_app');
    if (res.success && res.data !== undefined) return res.data;
  }
  return true;
}

// ==================== REAL-TIME TAURI EVENTS ====================

export async function listenToSchedulerEvents(
  onThemeChanged?: (theme: DetectedTheme) => void,
  onDashboardUpdated?: (state: DashboardState) => void
): Promise<() => void> {
  if (!isTauriEnvironment()) {
    return () => {};
  }

  const { listen } = await import('@tauri-apps/api/event');
  const unlistenTheme = await listen<DetectedTheme>('theme-changed', (event) => {
    if (onThemeChanged) onThemeChanged(event.payload);
  });
  const unlistenDashboard = await listen<DashboardState>('dashboard-updated', (event) => {
    if (onDashboardUpdated) onDashboardUpdated(event.payload);
  });

  return () => {
    unlistenTheme();
    unlistenDashboard();
  };
}

// ==================== SCHEDULE ====================

export async function fetchScheduleConfig(): Promise<ScheduleConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<ScheduleConfig>>('get_schedule_config');
    if (res.success && res.data) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to fetch schedule configuration');
  }
  return getDefaultScheduleConfig();
}

export async function saveScheduleConfig(schedule: ScheduleConfig): Promise<ScheduleConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<ScheduleConfig>>('set_schedule_config', {
      request: { schedule },
    });
    if (res.success && res.data) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to save schedule configuration');
  }
  return schedule;
}

export async function calculateSolarPreview(
  request: SolarPreviewRequest
): Promise<SolarPreviewResult> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<SolarPreviewResult>>('calculate_solar_preview', {
      request,
    });
    if (res.success && res.data) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to calculate solar preview');
  }

  const sunriseMin = 402 + request.sunrise_offset_minutes;
  const sunsetMin = 1068 + request.sunset_offset_minutes;
  return {
    sunrise_minute: sunriseMin,
    sunset_minute: sunsetMin,
    sunrise_time_formatted: '6:42 AM',
    sunset_time_formatted: '5:48 PM',
    daylight_window_formatted: '11h 06m',
  };
}

export async function detectUserLocation(): Promise<{
  location_name: string;
  latitude: number;
  longitude: number;
}> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<
      OperationResult<{ location_name: string; latitude: number; longitude: number }>
    >('detect_user_location');
    if (res.success && res.data) {
      return res.data;
    }
    throw new Error(res.error || 'Failed to detect user location');
  }
  return {
    location_name: 'Local Timezone Location',
    latitude: 40.7128,
    longitude: -74.006,
  };
}

// ==================== APPS ====================

export async function fetchAppsConfig(): Promise<AppsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppsConfig>>('get_apps_config');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch apps configuration');
  }
  return getDefaultAppsConfig();
}

export async function saveAppsConfig(config: AppsConfig): Promise<AppsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppsConfig>>('set_apps_config', {
      request: { config },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to save apps configuration');
  }
  return config;
}

export async function addCustomAppRule(rule: CustomAppRule): Promise<AppsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppsConfig>>('add_custom_app_rule', {
      request: { rule },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to add custom rule');
  }
  return getDefaultAppsConfig();
}

export async function removeCustomAppRule(id: string): Promise<AppsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppsConfig>>('remove_custom_app_rule', {
      request: { id },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to remove custom rule');
  }
  return getDefaultAppsConfig();
}

export async function toggleCustomAppRule(id: string, enabled: boolean): Promise<AppsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppsConfig>>('toggle_custom_app_rule', {
      request: { id, enabled },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to toggle rule');
  }
  return getDefaultAppsConfig();
}

// ==================== PERSONALIZATION ====================

export async function fetchPersonalizationConfig(): Promise<PersonalizationConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<PersonalizationConfig>>('get_personalization_config');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch personalization config');
  }
  return getDefaultPersonalizationConfig();
}

export async function savePersonalizationConfig(
  config: PersonalizationConfig
): Promise<PersonalizationConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<PersonalizationConfig>>('set_personalization_config', {
      request: { config },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to save personalization config');
  }
  return config;
}

export async function testApplyWallpaper(theme: ThemeMode): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('test_apply_wallpaper', {
      request: { theme },
    });
    if (res.success && res.data !== undefined) return res.data;
    throw new Error(res.error || 'Failed to apply test wallpaper');
  }
  return true;
}

export async function selectWallpaperFile(): Promise<string | null> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<string | null>>('select_wallpaper_file');
    if (res.success) {
      return res.data ?? null;
    }
    throw new Error(res.error || 'Failed to select wallpaper file');
  }
  return null;
}

// ==================== CONDITIONS ====================

export async function fetchConditionsConfig(): Promise<ConditionsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<ConditionsConfig>>('get_conditions_config');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch conditions config');
  }
  return getDefaultConditionsConfig();
}

export async function saveConditionsConfig(config: ConditionsConfig): Promise<ConditionsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<ConditionsConfig>>('set_conditions_config', {
      request: { config },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to save conditions config');
  }
  return config;
}

export async function fetchSystemConditionsStatus(): Promise<SystemConditionsStatus> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<SystemConditionsStatus>>('get_system_conditions_status');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch conditions status');
  }
  return {
    power: { is_on_battery: false, battery_percent: 100, is_charging: true, is_battery_saver_on: false },
    is_gaming_active: false,
    is_switch_allowed: true,
    restriction_reason: null,
  };
}

// ==================== AUTOMATION ====================

export async function fetchAutomationConfig(): Promise<AutomationConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AutomationConfig>>('get_automation_config');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch automation config');
  }
  return getDefaultAutomationConfig();
}

export async function saveAutomationConfig(config: AutomationConfig): Promise<AutomationConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AutomationConfig>>('set_automation_config', {
      request: { config },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to save automation config');
  }
  return config;
}

// ==================== SHORTCUTS ====================

export async function fetchShortcutsConfig(): Promise<ShortcutsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<ShortcutsConfig>>('get_shortcuts_config');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch shortcuts config');
  }
  return getDefaultShortcutsConfig();
}

export async function saveShortcutsConfig(config: ShortcutsConfig): Promise<ShortcutsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<ShortcutsConfig>>('set_shortcuts_config', {
      request: { config },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to save shortcuts config');
  }
  return config;
}

// ==================== SETTINGS & DIAGNOSTICS ====================

export async function fetchFullConfiguration(): Promise<AppConfig | null> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppConfig>>('get_configuration');
    if (res.success && res.data) return res.data;
  }
  return null;
}

export async function fetchSettingsConfig(): Promise<SettingsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppConfig>>('get_configuration');
    if (res.success && res.data && res.data.settings) return res.data.settings;
  }
  return getDefaultSettingsConfig();
}

export async function saveSettingsConfig(config: SettingsConfig): Promise<SettingsConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<SettingsConfig>>('set_settings_config', {
      request: { config },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to save settings config');
  }
  return config;
}

export async function exportConfigurationJson(): Promise<string> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<string>>('export_configuration');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to export configuration');
  }
  return '{}';
}

export async function importConfigurationJson(json: string): Promise<AppConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppConfig>>('import_configuration', {
      request: { json },
    });
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to import configuration');
  }
  throw new Error('Not supported in browser environment');
}

export async function resetToDefaultConfig(): Promise<AppConfig> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<AppConfig>>('reset_to_default_configuration');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to reset configuration');
  }
  throw new Error('Not supported in browser environment');
}

export async function openConfigFolder(): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('open_config_folder');
    if (res.success && res.data !== undefined) return res.data;
    throw new Error(res.error || 'Failed to open config folder');
  }
  return false;
}

export async function openExternalUrl(url: string): Promise<boolean> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<boolean>>('open_external_url', { url });
    if (res.success && res.data !== undefined) return res.data;
    throw new Error(res.error || 'Failed to open external URL');
  }
  if (typeof window !== 'undefined') {
    window.open(url, '_blank');
    return true;
  }
  return false;
}

export async function fetchDiagnosticsInfo(): Promise<DiagnosticsInfo> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    const res = await invoke<OperationResult<DiagnosticsInfo>>('get_diagnostics_info');
    if (res.success && res.data) return res.data;
    throw new Error(res.error || 'Failed to fetch diagnostics info');
  }
  return {
    os_name: 'Microsoft Windows 11',
    os_build: '10.0.26100',
    arch: 'x86_64',
    tauri_version: '2.12.5',
    webview_version: 'Edge / WebView2 122.0',
    config_path: '%APPDATA%\\AutoDarkMode\\config.json',
    uptime_seconds: 42,
  };
}

// ==================== WINDOW CONTROLS ====================

export async function minimizeWindow(): Promise<void> {
  if (isTauriEnvironment()) {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    await getCurrentWindow().minimize();
  }
}

export async function toggleMaximizeWindow(): Promise<void> {
  if (isTauriEnvironment()) {
    const { getCurrentWindow } = await import('@tauri-apps/api/window');
    await getCurrentWindow().toggleMaximize();
  }
}

export async function closeWindow(): Promise<void> {
  if (isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('hide_main_window');
  } else if (typeof window !== 'undefined') {
    window.close();
  }
}

// ==================== DEFAULTS ====================

export function getDefaultScheduleConfig(): ScheduleConfig {
  return {
    type: 'sunrise_sunset',
    enabled: true,
    light_start_hour: 7,
    light_start_minute: 0,
    dark_start_hour: 19,
    dark_start_minute: 0,
    latitude: 40.7128,
    longitude: -74.006,
    location_name: 'New York, USA',
    use_location_service: false,
    sunrise_offset_minutes: 0,
    sunset_offset_minutes: 0,
    idle_delay_minutes: 0,
    pause_on_battery: false,
    pause_on_gaming: false,
  };
}

export function getDefaultAppsConfig(): AppsConfig {
  return {
    apps_separate_from_system: false,
    apps_theme: 'dark',
    system_theme: 'dark',
    sync_office: true,
    sync_browsers: true,
    custom_rules: [
      {
        id: 'rule_1',
        name: 'Visual Studio Code',
        process_name: 'code.exe',
        target_theme: 'dark',
        enabled: true,
      },
      {
        id: 'rule_2',
        name: 'Windows Terminal',
        process_name: 'windowsterminal.exe',
        target_theme: 'system',
        enabled: true,
      },
    ],
  };
}

export function getDefaultPersonalizationConfig(): PersonalizationConfig {
  return {
    wallpaper_enabled: false,
    light_wallpaper_path: '',
    dark_wallpaper_path: '',
    wallpaper_fit: 'fill',
    sync_accent_color: false,
    light_accent_color: '#0078D4',
    dark_accent_color: '#60CDFF',
    sync_cursor: false,
    sync_sounds: false,
  };
}

export function getDefaultConditionsConfig(): ConditionsConfig {
  return {
    pause_on_battery: false,
    force_dark_on_battery_saver: false,
    pause_on_gaming: false,
    idle_delay_minutes: 0,
    pause_on_metered: false,
  };
}

export function getDefaultAutomationConfig(): AutomationConfig {
  return {
    launch_on_startup: false,
    start_minimized: true,
    daemon_mode: true,
    notify_on_switch: true,
    check_updates: true,
  };
}

export function getDefaultShortcutsConfig(): ShortcutsConfig {
  return {
    toggle_shortcut: 'Ctrl+Alt+T',
    force_light_shortcut: 'Ctrl+Alt+L',
    force_dark_shortcut: 'Ctrl+Alt+D',
    postpone_shortcut: 'Ctrl+Alt+P',
    shortcuts_enabled: true,
  };
}

export function getDefaultSettingsConfig(): SettingsConfig {
  return {
    app_ui_theme: 'system',
    language: 'en-US',
    logging_level: 'info',
  };
}

function getFallbackDashboardState(): DashboardState {
  return {
    current_theme: 'dark',
    detected_theme: {
      apps_theme: 'dark',
      system_theme: 'dark',
      is_synchronized: true,
      effective_theme: 'dark',
    },
    automation_enabled: true,
    last_switched_formatted: 'Synchronized with Windows system settings',
    next_transition: {
      target_theme: 'light',
      event_name: 'Astronomical sunrise',
      time_formatted: '6:42 AM',
      countdown_formatted: 'In 8h 24m',
      location_note: 'New York, USA (40.71° N, 74.01° W)',
      status: 'real',
    },
    schedule_timeline: {
      sunrise_time_formatted: '6:42 AM',
      sunset_time_formatted: '5:48 PM',
      night1_percentage: 27.9,
      day_percentage: 46.3,
      night2_percentage: 25.8,
      current_needle_percentage: 92.9,
      current_time_formatted: '10:18 PM',
      daylight_window_formatted: '11h 06m',
      status: 'real',
    },
    quick_controls: [
      {
        id: 'automatic_switching',
        title: 'Automatic switching',
        description: 'Allow AutoDarkMode to manage system theme changes',
        enabled: true,
        supported: true,
        interactive: true,
      },
      {
        id: 'follow_sun',
        title: 'Follow sunrise and sunset',
        description: 'Synchronize schedule with local astronomical daylight',
        enabled: true,
        supported: true,
        interactive: true,
      },
      {
        id: 'wallpaper',
        title: 'Wallpaper switching',
        description: 'Swap desktop backgrounds between day and night variants',
        enabled: false,
        supported: true,
        interactive: true,
      },
      {
        id: 'app_themes',
        title: 'App-specific themes',
        description: 'Override themes for individual desktop apps and browsers',
        enabled: false,
        supported: true,
        interactive: true,
      },
    ],
    app_info: {
      app_name: 'AutoDarkMode',
      version: 'v1.1.0',
      target_platform: 'Windows 11 x64',
      webview_runtime: 'WebView2 Runtime 122.0',
    },
  };
}
