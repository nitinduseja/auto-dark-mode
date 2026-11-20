import type { PersonalizationConfig, ConnectedMonitor, MonitorWallpaperConfig } from '../types/personalization';
import type { ThemeMode } from '../types/theme';
import {
  fetchPersonalizationConfig,
  savePersonalizationConfig,
  testApplyWallpaper,
  selectWallpaperFile,
  fetchConnectedDisplays,
  getDefaultPersonalizationConfig,
} from '../services/tauri';

export class PersonalizationStore {
  config = $state<PersonalizationConfig>(getDefaultPersonalizationConfig());
  connectedMonitors = $state<ConnectedMonitor[]>([]);
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  isTestingWallpaper = $state<boolean>(false);
  isDetectingMonitors = $state<boolean>(false);
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
      await this.loadMonitors();
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load personalization settings';
    } finally {
      this.isLoading = false;
    }
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
}

export const personalizationStore = new PersonalizationStore();
