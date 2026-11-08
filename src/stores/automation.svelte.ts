import type { AutomationConfig } from '../types/automation';
import {
  fetchAutomationConfig,
  saveAutomationConfig,
  getDefaultAutomationConfig,
} from '../services/tauri';
import {
  checkForUpdates,
  type UpdateCheckResult,
  APP_VERSION,
} from '../services/updater';

export class AutomationStore {
  config = $state<AutomationConfig>(getDefaultAutomationConfig());
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  isCheckingUpdate = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);
  updateStatus = $state<string | null>(null);
  updateResult = $state<UpdateCheckResult | null>(null);
  lastCheckedTime = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchAutomationConfig();
      if (this.config.check_updates && !this.updateResult) {
        // Run background update check silently at startup if enabled
        this.checkUpdatesSilently();
      }
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load automation configuration';
    } finally {
      this.isLoading = false;
    }
  }

  async save(): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    this.successMessage = null;
    try {
      this.config = await saveAutomationConfig($state.snapshot(this.config));
      this.successMessage = 'Automation settings applied successfully';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to save automation configuration';
    } finally {
      this.isSaving = false;
    }
  }

  async checkUpdates(): Promise<void> {
    this.isCheckingUpdate = true;
    this.errorMessage = null;
    try {
      const result = await checkForUpdates(APP_VERSION);
      this.updateResult = result;
      this.updateStatus = result.message;
      this.lastCheckedTime = result.checkedAt;
    } catch (err: unknown) {
      this.updateStatus = 'Failed to check for updates';
    } finally {
      this.isCheckingUpdate = false;
    }
  }

  async checkUpdatesSilently(): Promise<void> {
    try {
      const result = await checkForUpdates(APP_VERSION);
      this.updateResult = result;
      this.updateStatus = result.message;
      this.lastCheckedTime = result.checkedAt;
    } catch {
      // Fail silently in background
    }
  }
}

export const automationStore = new AutomationStore();
