import type { ConditionsConfig, SystemConditionsStatus } from '../types/conditions';
import {
  fetchConditionsConfig,
  saveConditionsConfig,
  fetchSystemConditionsStatus,
  getDefaultConditionsConfig,
} from '../services/tauri';

export class ConditionsStore {
  config = $state<ConditionsConfig>(getDefaultConditionsConfig());
  systemStatus = $state<SystemConditionsStatus | null>(null);
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);

  private pollIntervalId: number | null = null;

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchConditionsConfig();
      await this.refreshStatus();
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load conditions settings';
    } finally {
      this.isLoading = false;
    }

    if (!this.pollIntervalId && typeof window !== 'undefined') {
      this.pollIntervalId = window.setInterval(() => {
        this.refreshStatus();
      }, 5000);
    }
  }

  async refreshStatus(): Promise<void> {
    try {
      this.systemStatus = await fetchSystemConditionsStatus();
    } catch {
      // Ignore background status polling errors
    }
  }

  async save(): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    this.successMessage = null;
    try {
      this.config = await saveConditionsConfig($state.snapshot(this.config));
      this.successMessage = 'Conditions saved successfully';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to save conditions settings';
    } finally {
      this.isSaving = false;
    }
  }

  destroy(): void {
    if (this.pollIntervalId && typeof window !== 'undefined') {
      window.clearInterval(this.pollIntervalId);
      this.pollIntervalId = null;
    }
  }
}

export const conditionsStore = new ConditionsStore();
