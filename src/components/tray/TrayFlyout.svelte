<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Sun,
    Moon,
    ExternalLink,
    X,
    RotateCw,
    Clock,
    Monitor,
    SlidersHorizontal,
    Sunrise,
    Power,
    Check,
  } from '@lucide/svelte';
  import { dashboard } from '../../stores/dashboard.svelte';
  import { themeSelectionStore } from '../../stores/themeSelection.svelte';
  import {
    openMainWindow,
    closeTrayPopup,
    exitApp,
    executeSetTheme,
    executeSetAutomation,
    executeSetFollowSun,
  } from '../../services/tauri';
  import type { ThemeMode } from '../../types/theme';

  let isSwitching = $state(false);
  let followSunControl = $derived(dashboard.state?.quick_controls.find((c) => c.id === 'follow_sun'));

  onMount(() => {
    dashboard.init();
    themeSelectionStore.init();
  });

  async function handleSetTheme(mode: ThemeMode) {
    if (isSwitching || dashboard.state?.current_theme === mode) return;
    isSwitching = true;
    try {
      await dashboard.setTheme(mode);
    } finally {
      isSwitching = false;
    }
  }

  async function handleToggleAutomation() {
    if (!dashboard.state) return;
    const next = !dashboard.state.automation_enabled;
    await executeSetAutomation(next);
    dashboard.state.automation_enabled = next;
    dashboard.refreshSilent();
  }

  async function handleToggleFollowSun() {
    if (!dashboard.state) return;
    const currentControl = dashboard.state.quick_controls.find((c) => c.id === 'follow_sun');
    const next = !(currentControl?.enabled ?? true);
    await executeSetFollowSun(next);
    if (currentControl) currentControl.enabled = next;
    dashboard.refreshSilent();
  }

  async function handleOpenPage(page: 'overview' | 'settings') {
    await openMainWindow(page);
  }
</script>

<div
  class="w-full h-full bg-surface-container/95 backdrop-blur-2xl text-on-surface border border-[var(--titlebar-border)] flex flex-col p-3.5 select-none rounded-xl overflow-hidden shadow-2xl font-body-md text-body-md"
>
  <!-- Top Header Bar -->
  <div class="flex items-center justify-between pb-2 border-b border-[var(--card-border)]">
    <div class="flex items-center gap-2">
      <div
        class="w-6 h-6 rounded bg-surface-container-high flex items-center justify-center border border-[var(--card-border)] overflow-hidden shadow-sm"
      >
        <img src="/favicon.png" alt="AutoDarkMode" class="w-4 h-4 rounded object-contain" />
      </div>
      <div class="flex flex-col">
        <span class="font-title-sm text-[13px] font-semibold leading-tight text-on-surface">
          AutoDarkMode
        </span>
        <span class="font-caption-sm text-[10px] text-outline leading-tight">
          {dashboard.state?.app_info?.version || 'v1.1.0'}
        </span>
      </div>
    </div>

    <div class="flex items-center gap-1">
      <button
        type="button"
        title="Open Full Dashboard"
        onclick={() => handleOpenPage('overview')}
        class="h-7 px-2 rounded flex items-center gap-1 bg-surface-container-high hover:bg-surface-variant text-on-surface text-caption-sm font-medium transition-colors cursor-pointer"
      >
        <span>Dashboard</span>
        <ExternalLink class="w-3 h-3" />
      </button>

      <button
        type="button"
        title="Close Flyout"
        onclick={closeTrayPopup}
        class="w-7 h-7 rounded flex items-center justify-center text-outline hover:bg-surface-variant hover:text-on-surface transition-colors"
      >
        <X class="w-3.5 h-3.5" />
      </button>
    </div>
  </div>

  <!-- Status & Quick Segmented Theme Switch -->
  <div class="flex flex-col gap-2.5 pt-2.5">
    <!-- Active Status Card -->
    <div class="p-2.5 rounded-lg bg-surface-container-low border border-[var(--card-border)] flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span
          class="w-2 h-2 rounded-full {dashboard.state?.automation_enabled ?? true
            ? 'bg-tertiary shadow-[0_0_6px_rgba(16,124,16,0.7)] animate-pulse'
            : 'bg-outline'}"
        ></span>
        <div class="flex flex-col">
          <span class="font-label-md text-[12px] font-semibold text-on-surface leading-tight">
            {dashboard.state?.current_theme === 'dark' ? 'Dark Mode Active' : 'Light Mode Active'}
          </span>
          <span class="font-caption-sm text-[10.5px] text-on-surface-variant leading-tight">
            {dashboard.state?.automation_enabled ? 'Automatic switching running' : 'Automation paused'}
          </span>
        </div>
      </div>

      <!-- Segmented Mode Buttons -->
      <div class="bg-surface-container-lowest p-0.5 rounded-md flex items-center border border-[var(--titlebar-border)]">
        <button
          type="button"
          onclick={() => handleSetTheme('light')}
          disabled={isSwitching}
          class="h-6 px-2 rounded flex items-center gap-1 text-caption-sm font-medium transition-all {dashboard.state?.current_theme === 'light'
            ? 'bg-surface-container text-on-surface shadow-sm font-semibold'
            : 'text-outline hover:text-on-surface'}"
        >
          <Sun class="w-3 h-3 {dashboard.state?.current_theme === 'light' ? 'text-secondary' : ''}" />
          <span>Light</span>
        </button>

        <button
          type="button"
          onclick={() => handleSetTheme('dark')}
          disabled={isSwitching}
          class="h-6 px-2 rounded flex items-center gap-1 text-caption-sm font-medium transition-all {dashboard.state?.current_theme === 'dark'
            ? 'bg-surface-container text-on-surface shadow-sm font-semibold'
            : 'text-outline hover:text-on-surface'}"
        >
          <Moon class="w-3 h-3 {dashboard.state?.current_theme === 'dark' ? 'text-primary-container' : ''}" />
          <span>Dark</span>
        </button>
      </div>
    </div>

    <!-- Next Transition Card -->
    {#if dashboard.state?.next_transition}
      <div class="p-2 rounded bg-surface-container-low/70 border border-[var(--card-border)] flex items-center justify-between text-caption-sm">
        <div class="flex items-center gap-1.5 text-on-surface-variant">
          <Clock class="w-3.5 h-3.5 text-secondary" />
          <span>Next: {dashboard.state.next_transition.event_name}</span>
        </div>
        <span class="font-mono text-primary-container font-semibold">
          {dashboard.state.next_transition.time_formatted} ({dashboard.state.next_transition.countdown_formatted})
        </span>
      </div>
    {/if}

    <!-- Installed Windows Theme Quick Selectors -->
    <div class="flex flex-col gap-1.5 p-2.5 rounded-lg bg-surface-container-low border border-[var(--card-border)]">
      <span class="font-caption-sm text-[11px] font-semibold uppercase text-outline flex items-center gap-1">
        <Monitor class="w-3 h-3 text-primary-container" />
        Windows Themes
      </span>

      <!-- Light Theme Dropdown -->
      <div class="flex items-center justify-between gap-2">
        <span class="text-caption-sm text-outline w-12 flex-shrink-0">Light:</span>
        <select
          bind:value={themeSelectionStore.lightThemePath}
          onchange={(e) => themeSelectionStore.setLightTheme(e.currentTarget.value)}
          class="h-7 flex-1 px-1.5 rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-caption-sm outline-none cursor-pointer truncate"
        >
          {#each themeSelectionStore.installedThemes as theme}
            <option value={theme.path}>{theme.name}</option>
          {/each}
        </select>
      </div>

      <!-- Dark Theme Dropdown -->
      <div class="flex items-center justify-between gap-2">
        <span class="text-caption-sm text-outline w-12 flex-shrink-0">Dark:</span>
        <select
          bind:value={themeSelectionStore.darkThemePath}
          onchange={(e) => themeSelectionStore.setDarkTheme(e.currentTarget.value)}
          class="h-7 flex-1 px-1.5 rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-caption-sm outline-none cursor-pointer truncate"
        >
          {#each themeSelectionStore.installedThemes as theme}
            <option value={theme.path}>{theme.name}</option>
          {/each}
        </select>
      </div>
    </div>

    <!-- Quick Automation Toggles -->
    <div class="flex flex-col gap-1.5">
      <!-- Toggle 1: Automatic Switching -->
      <div class="flex items-center justify-between px-2 py-1 rounded hover:bg-surface-container-low transition-colors">
        <div class="flex items-center gap-2">
          <SlidersHorizontal class="w-3.5 h-3.5 text-primary-container" />
          <span class="text-caption-sm text-on-surface">Automatic switching</span>
        </div>
        <button
          type="button"
          role="switch"
          aria-label="Toggle automatic switching"
          aria-checked={dashboard.state?.automation_enabled ?? true}
          onclick={handleToggleAutomation}
          class="w-8 h-4 rounded-full relative transition-colors p-0.5 flex items-center cursor-pointer {dashboard.state?.automation_enabled ?? true
            ? 'bg-primary-container justify-end'
            : 'bg-surface-variant justify-start'}"
        >
          <span
            class="w-3 h-3 rounded-full shadow-sm {dashboard.state?.automation_enabled ?? true
              ? 'bg-surface-container-lowest'
              : 'bg-outline'}"
          ></span>
        </button>
      </div>

      <!-- Toggle 2: Follow Sun -->
      <div class="flex items-center justify-between px-2 py-1 rounded hover:bg-surface-container-low transition-colors">
        <div class="flex items-center gap-2">
          <Sunrise class="w-3.5 h-3.5 text-secondary" />
          <span class="text-caption-sm text-on-surface">Follow sunrise/sunset</span>
        </div>
        <button
          type="button"
          role="switch"
          aria-label="Toggle follow sunrise and sunset"
          aria-checked={followSunControl?.enabled ?? true}
          onclick={handleToggleFollowSun}
          class="w-8 h-4 rounded-full relative transition-colors p-0.5 flex items-center cursor-pointer {followSunControl?.enabled ?? true
            ? 'bg-primary-container justify-end'
            : 'bg-surface-variant justify-start'}"
        >
          <span
            class="w-3 h-3 rounded-full shadow-sm {followSunControl?.enabled ?? true
              ? 'bg-surface-container-lowest'
              : 'bg-outline'}"
          ></span>
        </button>
      </div>
    </div>
  </div>

  <!-- Bottom Navigation & Controls Footer -->
  <div class="mt-auto pt-2 border-t border-[var(--card-border)] flex items-center justify-between text-caption-sm">
    <button
      type="button"
      onclick={() => dashboard.refresh()}
      class="flex items-center gap-1 text-outline hover:text-on-surface transition-colors"
    >
      <RotateCw class="w-3 h-3 {dashboard.isLoading ? 'animate-spin' : ''}" />
      <span>Sync</span>
    </button>

    <div class="flex items-center gap-3">
      <button
        type="button"
        title="Open Application Settings"
        onclick={() => handleOpenPage('settings')}
        class="text-outline hover:text-primary transition-colors cursor-pointer"
      >
        Settings
      </button>

      <button
        type="button"
        onclick={exitApp}
        class="flex items-center gap-1 text-outline hover:text-error transition-colors"
      >
        <Power class="w-3 h-3" />
        <span>Exit</span>
      </button>
    </div>
  </div>
</div>
