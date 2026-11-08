import type { AutomationConfig } from '../types/automation';
import {
  fetchAutomationConfig,
  saveAutomationConfig,
  getDefaultAutomationConfig,
} from '../services/tauri';
import {
  checkForUpdates,
  downloadUpdate,
  installAndRestart,
  listenUpdateProgress,
  listenUpdateComplete,
  listenUpdateError,
  type UpdateCheckResult,
  type DownloadProgressPayload,
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

  // In-app update state
  isDownloadingUpdate = $state<boolean>(false);
  downloadProgress = $state<DownloadProgressPayload>({
    percentage: 0,
    downloaded_bytes: 0,
    total_bytes: 0,
  });
  isRestarting = $state<boolean>(false);
  showUpdateNotification = $state<boolean>(false);
  updateErrorMessage = $state<string | null>(null);

  private unlistenProgress: (() => void) | null = null;
  private unlistenComplete: (() => void) | null = null;
  private unlistenError: (() => void) | null = null;

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
    this.updateErrorMessage = null;
    try {
      const result = await checkForUpdates(APP_VERSION);
      this.updateResult = result;
      this.updateStatus = result.message;
      this.lastCheckedTime = result.checkedAt;
      if (result.status === 'update-available') {
        this.showUpdateNotification = true;
      }
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
      if (result.status === 'update-available') {
        this.showUpdateNotification = true;
      }
    } catch {
      // Fail silently in background
    }
  }

  dismissUpdateNotification(): void {
    this.showUpdateNotification = false;
  }

  async startUpdateDownload(targetUrl?: string): Promise<void> {
    const url = targetUrl || this.updateResult?.downloadUrl;
    if (!url) {
      this.updateErrorMessage = 'No valid download URL available for update.';
      return;
    }

    this.isDownloadingUpdate = true;
    this.isRestarting = false;
    this.updateErrorMessage = null;
    this.downloadProgress = { percentage: 0, downloaded_bytes: 0, total_bytes: 0 };

    try {
      // Clean up any existing listeners before attaching new ones
      if (this.unlistenProgress) this.unlistenProgress();
      if (this.unlistenComplete) this.unlistenComplete();
      if (this.unlistenError) this.unlistenError();

      this.unlistenProgress = await listenUpdateProgress((payload) => {
        this.downloadProgress = payload;
      });

      this.unlistenComplete = await listenUpdateComplete(async () => {
        this.isDownloadingUpdate = false;
        this.isRestarting = true;
        // Brief delay so user sees 100% completion before app smoothly restarts
        setTimeout(async () => {
          try {
            await installAndRestart();
          } catch (e: unknown) {
            this.isRestarting = false;
            this.updateErrorMessage = e instanceof Error ? e.message : String(e);
          }
        }, 1200);
      });

      this.unlistenError = await listenUpdateError((err) => {
        this.isDownloadingUpdate = false;
        this.isRestarting = false;
        this.updateErrorMessage = err;
      });

      await downloadUpdate(url);
    } catch (err: unknown) {
      this.isDownloadingUpdate = false;
      this.isRestarting = false;
      this.updateErrorMessage =
        err instanceof Error ? err.message : typeof err === 'string' ? err : 'Failed to download update.';
    }
  }
}

export const automationStore = new AutomationStore();
