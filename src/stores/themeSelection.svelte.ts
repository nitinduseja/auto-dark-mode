import type { InstalledTheme, ThemeConfig } from '../types/theme';
import {
  fetchInstalledThemes,
  saveModeThemes,
  applySpecificTheme,
  fetchDashboardState,
  fetchFullConfiguration,
} from '../services/tauri';

export class ThemeSelectionStore {
  installedThemes = $state<InstalledTheme[]>([]);
  useThemeFiles = $state<boolean>(false);
  lightThemePath = $state<string>('C:\\Windows\\Resources\\Themes\\aero.theme');
  lightThemeName = $state<string>('Windows (light)');
  darkThemePath = $state<string>('C:\\Windows\\Resources\\Themes\\dark.theme');
  darkThemeName = $state<string>('Windows (dark)');

  isLoading = $state<boolean>(false);
  isSaving = $state<boolean>(false);
  isTesting = $state<boolean>(false);
  errorMessage = $state<string | null>(null);
  successMessage = $state<string | null>(null);

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;

    try {
      // 1. Discover all installed Windows themes
      this.installedThemes = await fetchInstalledThemes();

      // 2. Fetch current config from backend
      const fullConfig = await fetchFullConfiguration();
      if (fullConfig && fullConfig.theme) {
        const t = fullConfig.theme;
        this.useThemeFiles = t.use_theme_files ?? false;
        if (t.light_theme_path) this.lightThemePath = t.light_theme_path;
        if (t.light_theme_name) this.lightThemeName = t.light_theme_name;
        if (t.dark_theme_path) this.darkThemePath = t.dark_theme_path;
        if (t.dark_theme_name) this.darkThemeName = t.dark_theme_name;
      }
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to discover installed themes';
    } finally {
      this.isLoading = false;
    }
  }

  async setLightTheme(path: string): Promise<void> {
    this.lightThemePath = path;
    const found = this.installedThemes.find((t) => t.path === path);
    if (found) {
      this.lightThemeName = found.name;
    }
    await this.save();
  }

  async setDarkTheme(path: string): Promise<void> {
    this.darkThemePath = path;
    const found = this.installedThemes.find((t) => t.path === path);
    if (found) {
      this.darkThemeName = found.name;
    }
    await this.save();
  }

  async toggleUseThemeFiles(enabled: boolean): Promise<void> {
    this.useThemeFiles = enabled;
    await this.save();
  }

  async save(): Promise<void> {
    this.isSaving = true;
    this.errorMessage = null;
    this.successMessage = null;

    try {
      await saveModeThemes({
        use_theme_files: this.useThemeFiles,
        light_theme_path: this.lightThemePath,
        light_theme_name: this.lightThemeName,
        dark_theme_path: this.darkThemePath,
        dark_theme_name: this.darkThemeName,
      });
      this.successMessage = 'Windows theme assignments saved successfully';
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage =
        err instanceof Error ? err.message : 'Failed to save theme assignments';
    } finally {
      this.isSaving = false;
    }
  }

  async testTheme(path: string, modeName: string): Promise<void> {
    this.isTesting = true;
    this.errorMessage = null;
    try {
      await applySpecificTheme(path);
      this.successMessage = `Applied ${modeName} successfully!`;
      setTimeout(() => {
        this.successMessage = null;
      }, 3000);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to apply test theme';
    } finally {
      this.isTesting = false;
    }
  }
}

export const themeSelectionStore = new ThemeSelectionStore();
