import type { DashboardState, ControlState } from '../types/dashboard';
import type { ThemeMode } from '../types/theme';
import {
  fetchDashboardState,
  executeSetTheme,
  executeSetAutomation,
  executeSetFollowSun,
  executeSetQuickControl,
  executePostponeSwitch,
  executeResumeAutomation,
  syncThemeState,
  listenToSchedulerEvents,
} from '../services/tauri';

function applyDomTheme(theme: ThemeMode) {
  if (typeof document === 'undefined') return;
  const root = document.documentElement;
  if (theme === 'dark') {
    root.classList.add('dark');
    root.classList.remove('light');
  } else {
    root.classList.remove('dark');
    root.classList.add('light');
  }
}

export class DashboardStore {
  state = $state<DashboardState | null>(null);
  isLoading = $state<boolean>(true);
  isSwitchingTheme = $state<boolean>(false);
  errorMessage = $state<string | null>(null);

  private syncIntervalId: number | null = null;
  private mediaQueryListener: ((e: MediaQueryListEvent) => void) | null = null;
  private unlistenEvents: (() => void) | null = null;

  async init(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;

    // Detect system preferred theme initially if available
    if (typeof window !== 'undefined') {
      const isSystemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      applyDomTheme(isSystemDark ? 'dark' : 'light');
    }

    try {
      this.state = await fetchDashboardState();
      if (this.state) {
        applyDomTheme(this.state.current_theme);
      }
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to initialize dashboard';
    } finally {
      this.isLoading = false;
    }

    // Set up continuous system theme synchronization and real-time backend events
    this.setupSystemSync();
  }

  private setupSystemSync(): void {
    if (typeof window === 'undefined') return;

    // 1. Listen for real-time background scheduler transitions
    listenToSchedulerEvents(
      (detected) => {
        if (this.state) {
          this.state.current_theme = detected.effective_theme;
          this.state.detected_theme = detected;
          applyDomTheme(detected.effective_theme);
        }
      },
      (state) => {
        this.state = state;
        applyDomTheme(state.current_theme);
      }
    ).then((unlisten) => {
      this.unlistenEvents = unlisten;
    });

    // 2. Browser/OS media query change listener
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    this.mediaQueryListener = () => {
      this.refreshSilent();
    };
    mediaQuery.addEventListener('change', this.mediaQueryListener);

    // 3. Periodic fallback sync with Windows Registry (60s heartbeat; real-time events handle changes immediately)
    if (!this.syncIntervalId) {
      this.syncIntervalId = window.setInterval(() => {
        this.refreshSilent();
      }, 60000);
    }
  }

  async postpone(minutes: number = 60): Promise<void> {
    this.errorMessage = null;
    try {
      await executePostponeSwitch(minutes);
      await this.refresh();
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : `Failed to postpone switch`;
    }
  }

  async resumeAutomation(): Promise<void> {
    this.errorMessage = null;
    try {
      await executeResumeAutomation();
      await this.refresh();
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : `Failed to resume schedule`;
    }
  }

  async setTheme(target: ThemeMode): Promise<void> {
    if (this.isSwitchingTheme || !this.state) return;
    if (this.state.current_theme === target) return;

    this.isSwitchingTheme = true;
    this.errorMessage = null;

    try {
      // 1. Execute theme switch on Windows native backend
      const detected = await executeSetTheme(target);

      // 2. Apply theme immediately across the entire UI
      this.state.current_theme = detected.effective_theme;
      this.state.detected_theme = detected;
      applyDomTheme(detected.effective_theme);

      // 3. Refresh full state from backend for consistency
      const freshState = await fetchDashboardState();
      this.state = freshState;
      applyDomTheme(freshState.current_theme);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to switch Windows theme';
    } finally {
      this.isSwitchingTheme = false;
    }
  }

  async toggleControl(control: ControlState): Promise<void> {
    if (!control.interactive || !this.state) return;

    const nextState = !control.enabled;
    this.errorMessage = null;

    try {
      const result = await executeSetQuickControl(control.id, nextState);
      control.enabled = result;
      if (control.id === 'automatic_switching') {
        this.state.automation_enabled = result;
      }
      await this.refreshSilent();
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : `Failed to toggle ${control.title}`;
    }
  }

  async refresh(): Promise<void> {
    this.isLoading = true;
    this.errorMessage = null;
    try {
      const fresh = await syncThemeState();
      this.state = fresh;
      applyDomTheme(fresh.current_theme);
    } catch (err: unknown) {
      this.errorMessage = err instanceof Error ? err.message : 'Failed to sync with Windows settings';
    } finally {
      this.isLoading = false;
    }
  }

  // Silent sync without showing loading spinner
  async refreshSilent(): Promise<void> {
    if (this.isSwitchingTheme) return;
    try {
      const fresh = await syncThemeState();
      if (fresh) {
        if (!this.state || this.state.current_theme !== fresh.current_theme) {
          applyDomTheme(fresh.current_theme);
        }
        this.state = fresh;
      }
    } catch {
      // Ignore background sync errors
    }
  }

  destroy(): void {
    if (this.syncIntervalId && typeof window !== 'undefined') {
      window.clearInterval(this.syncIntervalId);
      this.syncIntervalId = null;
    }
    if (this.mediaQueryListener && typeof window !== 'undefined') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      mediaQuery.removeEventListener('change', this.mediaQueryListener);
      this.mediaQueryListener = null;
    }
  }
}

export const dashboard = new DashboardStore();
