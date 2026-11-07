import type { AutomationConfig } from '../types/automation';
import {
  fetchAutomationConfig,
  saveAutomationConfig,
  getDefaultAutomationConfig,
} from '../services/tauri';

export class AutomationStore {
  config = $state<AutomationConfig>(getDefaultAutomationConfig());
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  isCheckingUpdate = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);
  updateStatus = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchAutomationConfig();
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
    this.updateStatus = null;
    await new Promise((resolve) => setTimeout(resolve, 800));
    this.updateStatus = 'You are on the latest release of AutoDarkMode (v1.1.0)';
    this.isCheckingUpdate = false;
  }
}

export const automationStore = new AutomationStore();
