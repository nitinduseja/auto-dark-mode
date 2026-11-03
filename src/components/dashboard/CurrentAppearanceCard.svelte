<script lang="ts">
  import { History, Loader2, AlertCircle } from '@lucide/svelte';
  import SegmentedControl from '../ui/SegmentedControl.svelte';
  import { dashboard } from '../../stores/dashboard.svelte';
  import type { ThemeMode } from '../../types/theme';

  function handleThemeChange(target: ThemeMode) {
    dashboard.setTheme(target);
  }
</script>

<div
  class="lg:col-span-7 bg-surface-container rounded-lg p-space-lg flex flex-col justify-between shadow-[var(--card-shadow)] relative overflow-hidden group hover:bg-surface-container-high transition-colors border border-[var(--card-border)]"
>
  <!-- Background Glow Effect -->
  <div
    class="absolute -right-10 -bottom-10 w-44 h-44 rounded-full bg-primary/10 blur-2xl pointer-events-none"
  ></div>

  <div class="flex flex-col gap-space-md">
    <!-- Card Top Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-space-sm">
        <span
          class="font-caption-sm text-caption-sm text-outline tracking-wider font-semibold uppercase"
        >
          Current Appearance
        </span>
        <div
          class="px-space-xs py-0.5 rounded bg-tertiary/15 text-tertiary flex items-center gap-1 font-caption-sm text-caption-sm font-semibold"
        >
          <span class="w-1.5 h-1.5 rounded-full bg-tertiary"></span>
          <span class="leading-none">Active</span>
        </div>
      </div>

      <!-- Segmented Theme Switcher -->
      <div class="flex items-center gap-2">
        {#if dashboard.isSwitchingTheme}
          <Loader2 size={14} class="animate-spin text-primary-container" />
        {/if}
        <SegmentedControl
          value={dashboard.state?.current_theme || 'dark'}
          disabled={dashboard.isSwitchingTheme}
          onchange={handleThemeChange}
        />
      </div>
    </div>

    <!-- Main Appearance Title & Explanation -->
    <div class="flex flex-col gap-0.5">
      <div
        class="font-headline-md text-headline-md text-on-surface font-semibold flex items-center gap-2 capitalize"
      >
        {dashboard.state?.current_theme || 'Dark'} mode
      </div>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Windows is currently using your {dashboard.state?.current_theme || 'dark'} appearance system-wide.
      </p>
    </div>

    <!-- Error message display if operation failed -->
    {#if dashboard.errorMessage}
      <div
        class="p-2.5 rounded bg-error/10 border border-error/20 text-error flex items-center gap-2 font-caption-sm text-caption-sm"
      >
        <AlertCircle size={14} class="flex-shrink-0" />
        <span>{dashboard.errorMessage}</span>
      </div>
    {/if}
  </div>

  <!-- Bottom History Information -->
  <div class="pt-space-lg flex items-center gap-space-xs text-outline font-caption-sm text-caption-sm">
    <History size={14} />
    <span>{dashboard.state?.last_switched_formatted || 'Detected from Windows system settings'}</span>
  </div>
</div>
