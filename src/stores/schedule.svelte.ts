import type {
  ScheduleConfig,
  ScheduleType,
  SolarPreviewResult,
  CityPreset,
} from '../types/schedule';
import {
  fetchScheduleConfig,
  saveScheduleConfig,
  calculateSolarPreview,
  getDefaultScheduleConfig,
} from '../services/tauri';
import { dashboard } from './dashboard.svelte';

export class ScheduleStore {
  config = $state<ScheduleConfig>(getDefaultScheduleConfig());
  savedConfig = $state<ScheduleConfig>(getDefaultScheduleConfig());
  preview = $state<SolarPreviewResult | null>(null);
  isLoading = $state<boolean>(true);
  isSaving = $state<boolean>(false);
  isDirty = $state<boolean>(false);
  saveSuccess = $state<boolean>(false);
  errorMessage = $state<string | null>(null);

  private saveSuccessTimeout: number | null = null;

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;

    try {
      const loaded = await fetchScheduleConfig();
      this.config = { ...loaded };
      this.savedConfig = { ...loaded };
      this.isDirty = false;
      await this.refreshPreview();
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load schedule settings';
    } finally {
      this.isLoading = false;
    }
  }

  async refreshPreview(): Promise<void> {
    try {
      const res = await calculateSolarPreview({
        latitude: this.config.latitude,
        longitude: this.config.longitude,
        sunrise_offset_minutes: this.config.sunrise_offset_minutes,
        sunset_offset_minutes: this.config.sunset_offset_minutes,
      });
      this.preview = res;
    } catch {
      // Non-critical preview calculation fallback
    }
  }

  updateField<K extends keyof ScheduleConfig>(key: K, value: ScheduleConfig[K]): void {
    this.config[key] = value;
    this.checkDirty();

    if (
      key === 'latitude' ||
      key === 'longitude' ||
      key === 'sunrise_offset_minutes' ||
      key === 'sunset_offset_minutes'
    ) {
      this.refreshPreview();
    }
  }

  setScheduleType(type: ScheduleType): void {
    this.config.type = type;
    this.checkDirty();
  }

  setCityPreset(city: CityPreset): void {
    this.config.location_name = city.name;
    this.config.latitude = city.latitude;
    this.config.longitude = city.longitude;
    this.checkDirty();
    this.refreshPreview();
  }

  private checkDirty(): void {
    this.isDirty = JSON.stringify(this.config) !== JSON.stringify(this.savedConfig);
  }

  async save(): Promise<boolean> {
    if (this.isSaving) return false;
    this.isSaving = true;
    this.errorMessage = null;

    try {
      const saved = await saveScheduleConfig(this.config);
      this.config = { ...saved };
      this.savedConfig = { ...saved };
      this.isDirty = false;
      this.saveSuccess = true;

      // Also refresh dashboard state so overview timeline immediately matches
      await dashboard.refresh();

      if (this.saveSuccessTimeout && typeof window !== 'undefined') {
        window.clearTimeout(this.saveSuccessTimeout);
      }
      if (typeof window !== 'undefined') {
        this.saveSuccessTimeout = window.setTimeout(() => {
          this.saveSuccess = false;
        }, 3000);
      }
      return true;
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to save schedule settings';
      return false;
    } finally {
      this.isSaving = false;
    }
  }

  reset(): void {
    this.config = { ...this.savedConfig };
    this.isDirty = false;
    this.errorMessage = null;
    this.refreshPreview();
  }
}

export const scheduleStore = new ScheduleStore();
