import type { ShortcutsConfig } from '../types/shortcuts';
import {
  fetchShortcutsConfig,
  saveShortcutsConfig,
  getDefaultShortcutsConfig,
} from '../services/tauri';

export class ShortcutsStore {
  config = $state<ShortcutsConfig>(getDefaultShortcutsConfig());
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchShortcutsConfig();
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load shortcuts configuration';
    } finally {
      this.isLoading = false;
    }
  }

  async save(): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    this.successMessage = null;
    try {
      this.config = await saveShortcutsConfig($state.snapshot(this.config));
      this.successMessage = 'Keyboard shortcuts saved successfully';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to save shortcuts configuration';
    } finally {
      this.isSaving = false;
    }
  }
}

export const shortcutsStore = new ShortcutsStore();
