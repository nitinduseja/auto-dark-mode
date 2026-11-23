import type {
  PersonalizationConfig,
  ConnectedMonitor,
  MonitorWallpaperConfig,
  SpotlightImageInfo,
  WallpaperMode,
  CursorSchemeInfo,
} from '../types/personalization';
import type { ThemeMode } from '../types/theme';
import {
  fetchPersonalizationConfig,
  savePersonalizationConfig,
  testApplyWallpaper,
  selectWallpaperFile,
  fetchConnectedDisplays,
  fetchSpotlightWallpapers,
  executeApplySpotlightWallpaper,
  executeSetSolidColor,
  fetchInstalledCursorSchemes,
  executeTestApplyCursorScheme,
  getDefaultPersonalizationConfig,
} from '../services/tauri';

export class PersonalizationStore {
  config = $state<PersonalizationConfig>(getDefaultPersonalizationConfig());
  connectedMonitors = $state<ConnectedMonitor[]>([]);
  spotlightImages = $state<SpotlightImageInfo[]>([]);
  installedCursorSchemes = $state<CursorSchemeInfo[]>([]);
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  isTestingWallpaper = $state<boolean>(false);
  isTestingCursor = $state<boolean>(false);
  isDetectingMonitors = $state<boolean>(false);
  isLoadingSpotlight = $state<boolean>(false);
  isLoadingCursors = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchPersonalizationConfig();
      if (!this.config.monitor_wallpapers) {
        this.config.monitor_wallpapers = [];
      }
      await Promise.all([this.loadMonitors(), this.loadSpotlightImages(), this.loadCursorSchemes()]);
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load personalization settings';
    } finally {
      this.isLoading = false;
    }
  }

  async loadSpotlightImages(): Promise<void> {
    this.isLoadingSpotlight = true;
    try {
      this.spotlightImages = await fetchSpotlightWallpapers();
    } catch (err: unknown) {
      console.warn('Failed to load spotlight wallpapers:', err);
    } finally {
      this.isLoadingSpotlight = false;
    }
  }

  async applySpotlight(): Promise<void> {
    this.isTestingWallpaper = true;
    this.errorMessage = null;
    try {
      const appliedPath = await executeApplySpotlightWallpaper();
      this.config.spotlight_wallpaper_path = appliedPath;
      this.successMessage = 'Applied Windows Spotlight wallpaper';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to apply Spotlight wallpaper';
    } finally {
      this.isTestingWallpaper = false;
    }
  }

  async testSolidColor(theme: ThemeMode, color: string): Promise<void> {
    this.isTestingWallpaper = true;
    this.errorMessage = null;
    try {
      await executeSetSolidColor(color);
      this.successMessage = `Applied ${theme} solid color (${color}) successfully`;
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to apply solid color';
    } finally {
      this.isTestingWallpaper = false;
    }
  }

  setWallpaperMode(mode: WallpaperMode): void {
    this.config.wallpaper_mode = mode;
    this.save();
  }

  async loadMonitors(): Promise<void> {
    this.isDetectingMonitors = true;
    try {
      this.connectedMonitors = await fetchConnectedDisplays();
      if (!this.config.monitor_wallpapers) {
        this.config.monitor_wallpapers = [];
      }
      for (const mon of this.connectedMonitors) {
        const existing = this.config.monitor_wallpapers.find((w) => w.monitor_id === mon.id);
        if (!existing) {
          this.config.monitor_wallpapers.push({
            monitor_id: mon.id,
            monitor_name: mon.name,
            light_wallpaper_path: '',
            dark_wallpaper_path: '',
          });
        } else {
          existing.monitor_name = mon.name;
        }
      }
    } catch (err: unknown) {
      console.error('Failed to load connected monitors:', err);
    } finally {
      this.isDetectingMonitors = false;
    }
  }

  getMonitorConfig(monitorId: string): MonitorWallpaperConfig {
    let entry = this.config.monitor_wallpapers?.find((w) => w.monitor_id === monitorId);
    if (!entry) {
      const mon = this.connectedMonitors.find((m) => m.id === monitorId);
      entry = {
        monitor_id: monitorId,
        monitor_name: mon ? mon.name : 'Display',
        light_wallpaper_path: '',
        dark_wallpaper_path: '',
      };
      if (!this.config.monitor_wallpapers) {
        this.config.monitor_wallpapers = [];
      }
      this.config.monitor_wallpapers.push(entry);
    }
    return entry;
  }

  async save(): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    this.successMessage = null;
    try {
      this.config = await savePersonalizationConfig($state.snapshot(this.config));
      this.successMessage = 'Personalization settings saved successfully';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to save personalization settings';
    } finally {
      this.isSaving = false;
    }
  }

  async testWallpaper(theme: ThemeMode, monitorId?: string): Promise<void> {
    this.isTestingWallpaper = true;
    this.errorMessage = null;
    try {
      await testApplyWallpaper(theme, monitorId);
      const target = monitorId ? `for selected display` : `wallpaper`;
      this.successMessage = `Applied ${theme} ${target} successfully`;
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to apply test wallpaper';
    } finally {
      this.isTestingWallpaper = false;
    }
  }

  async browseWallpaper(mode: 'light' | 'dark'): Promise<void> {
    this.errorMessage = null;
    try {
      const selected = await selectWallpaperFile();
      if (selected) {
        if (mode === 'light') {
          this.config.light_wallpaper_path = selected;
        } else {
          this.config.dark_wallpaper_path = selected;
        }
        await this.save();
      }
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to select wallpaper file';
    }
  }

  async browseMonitorWallpaper(monitorId: string, mode: 'light' | 'dark'): Promise<void> {
    this.errorMessage = null;
    try {
      const selected = await selectWallpaperFile();
      if (selected) {
        const entry = this.getMonitorConfig(monitorId);
        if (mode === 'light') {
          entry.light_wallpaper_path = selected;
        } else {
          entry.dark_wallpaper_path = selected;
        }
        await this.save();
      }
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to select wallpaper file';
    }
  }

  async loadCursorSchemes(): Promise<void> {
    this.isLoadingCursors = true;
    try {
      this.installedCursorSchemes = await fetchInstalledCursorSchemes();
      if (!this.config.light_cursor_scheme) {
        this.config.light_cursor_scheme = 'Windows Aero';
      }
      if (!this.config.dark_cursor_scheme) {
        this.config.dark_cursor_scheme = 'Windows Black';
      }
    } catch (err: unknown) {
      console.warn('Failed to load cursor schemes:', err);
    } finally {
      this.isLoadingCursors = false;
    }
  }

  async testCursorScheme(schemeName: string): Promise<void> {
    this.isTestingCursor = true;
    this.errorMessage = null;
    try {
      await executeTestApplyCursorScheme(schemeName);
      this.successMessage = `Applied cursor scheme "${schemeName}" successfully`;
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to apply cursor scheme';
    } finally {
      this.isTestingCursor = false;
    }
  }

  setLightCursorScheme(scheme: string): void {
    this.config.light_cursor_scheme = scheme;
    this.save();
  }

  setDarkCursorScheme(scheme: string): void {
    this.config.dark_cursor_scheme = scheme;
    this.save();
  }
}

export const personalizationStore = new PersonalizationStore();
