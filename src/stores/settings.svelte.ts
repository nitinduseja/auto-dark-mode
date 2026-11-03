import type { SettingsConfig, DiagnosticsInfo } from '../types/settings';
import {
  fetchSettingsConfig,
  saveSettingsConfig,
  exportConfigurationJson,
  importConfigurationJson,
  resetToDefaultConfig,
  openConfigFolder,
  fetchDiagnosticsInfo,
  getDefaultSettingsConfig,
} from '../services/tauri';

export class SettingsStore {
  config = $state<SettingsConfig>(getDefaultSettingsConfig());
  diagnostics = $state<DiagnosticsInfo | null>(null);
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);
  exportedJson = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchSettingsConfig();
      this.diagnostics = await fetchDiagnosticsInfo();
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load application settings';
    } finally {
      this.isLoading = false;
    }
  }

  async save(): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    this.successMessage = null;
    try {
      this.config = await saveSettingsConfig($state.snapshot(this.config));
      this.successMessage = 'Settings saved successfully';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to save application settings';
    } finally {
      this.isSaving = false;
    }
  }

  async exportConfig(): Promise<string> {
    try {
      const json = await exportConfigurationJson();
      this.exportedJson = json;
      return json;
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to export configuration';
      throw err;
    }
  }

  async importConfig(json: string): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      await importConfigurationJson(json);
      this.successMessage = 'Configuration restored successfully';
      await this.init();
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to import configuration';
    } finally {
      this.isLoading = false;
    }
  }

  async resetDefaults(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      await resetToDefaultConfig();
      this.successMessage = 'Configuration reset to defaults';
      await this.init();
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to reset configuration';
    } finally {
      this.isLoading = false;
    }
  }

  async openFolder(): Promise<void> {
    try {
      await openConfigFolder();
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to open configuration folder';
    }
  }
}

export const settingsStore = new SettingsStore();
