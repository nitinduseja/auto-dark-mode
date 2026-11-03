import type { AppsConfig, CustomAppRule } from '../types/apps';
import {
  fetchAppsConfig,
  saveAppsConfig,
  addCustomAppRule,
  removeCustomAppRule,
  toggleCustomAppRule,
  getDefaultAppsConfig,
} from '../services/tauri';

export class AppsStore {
  config = $state<AppsConfig>(getDefaultAppsConfig());
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchAppsConfig();
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to load apps configuration';
    } finally {
      this.isLoading = false;
    }
  }

  async save(): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    this.successMessage = null;
    try {
      this.config = await saveAppsConfig($state.snapshot(this.config));
      this.successMessage = 'App settings saved successfully';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to save apps configuration';
    } finally {
      this.isSaving = false;
    }
  }

  async addRule(rule: CustomAppRule): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    try {
      this.config = await addCustomAppRule(rule);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to add custom rule';
    } finally {
      this.isSaving = false;
    }
  }

  async removeRule(id: string): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    try {
      this.config = await removeCustomAppRule(id);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to remove custom rule';
    } finally {
      this.isSaving = false;
    }
  }

  async toggleRule(id: string, enabled: boolean): Promise<void> {
    this.errorMessage = null;
    try {
      this.config = await toggleCustomAppRule(id, enabled);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to toggle rule';
    }
  }
}

export const appsStore = new AppsStore();
