import type { PersonalizationConfig } from '../types/personalization';
import type { ThemeMode } from '../types/theme';
import {
  fetchPersonalizationConfig,
  savePersonalizationConfig,
  testApplyWallpaper,
  getDefaultPersonalizationConfig,
} from '../services/tauri';

export class PersonalizationStore {
  config = $state<PersonalizationConfig>(getDefaultPersonalizationConfig());
  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  isTestingWallpaper = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      this.config = await fetchPersonalizationConfig();
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to load personalization settings';
    } finally {
      this.isLoading = false;
    }
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

  async testWallpaper(theme: ThemeMode): Promise<void> {
    this.isTestingWallpaper = true;
    this.errorMessage = null;
    try {
      await testApplyWallpaper(theme);
      this.successMessage = `Applied ${theme} wallpaper successfully`;
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to apply test wallpaper';
    } finally {
      this.isTestingWallpaper = false;
    }
  }
}

export const personalizationStore = new PersonalizationStore();
