<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Check,
    RotateCcw,
    Save,
    Loader2,
    AlertCircle,
    Info,
    Sparkles,
  } from '@lucide/svelte';
  import ScheduleModeSelector from './ScheduleModeSelector.svelte';
  import CustomTimesCard from './CustomTimesCard.svelte';
  import AstronomicalLocationCard from './AstronomicalLocationCard.svelte';
  import ScheduleConditionsCard from './ScheduleConditionsCard.svelte';
  import SchedulePreviewBar from './SchedulePreviewBar.svelte';
  import Button from '../ui/Button.svelte';
  import StatusIndicator from '../ui/StatusIndicator.svelte';
  import { scheduleStore } from '../../stores/schedule.svelte';

  onMount(() => {
    scheduleStore.init();
  });

  async function handleSave() {
    await scheduleStore.save();
  }

  function handleReset() {
    scheduleStore.reset();
  }
</script>

<div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto select-none">
  <!-- Top Section Title Area & Save Controls -->
  <div class="flex items-center justify-between flex-wrap gap-4">
    <div class="flex flex-col gap-0.5">
      <h1 class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold">
        Schedule
      </h1>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Choose when Windows should switch between light and dark themes.
      </p>
    </div>

    <!-- Top Action Buttons -->
    <div class="flex items-center gap-space-sm">
      <!-- Status Indicator -->
      {#if scheduleStore.saveSuccess}
        <div
          class="h-7 px-space-md rounded-full bg-tertiary/15 text-tertiary flex items-center gap-1.5 font-label-md text-label-md font-medium border border-tertiary/30 animate-fade-in"
        >
          <Check size={14} />
          <span>Saved to Windows</span>
        </div>
      {:else if scheduleStore.isDirty}
        <div
          class="h-7 px-space-md rounded-full bg-secondary/15 text-secondary flex items-center gap-1.5 font-label-md text-label-md font-medium border border-secondary/30"
        >
          <span class="w-1.5 h-1.5 rounded-full bg-secondary"></span>
          <span>Unsaved changes</span>
        </div>
      {/if}

      <!-- Reset Button -->
      <button
        type="button"
        disabled={!scheduleStore.isDirty || scheduleStore.isSaving}
        onclick={handleReset}
        title="Revert to last saved configuration"
        class="h-8 px-space-md rounded flex items-center gap-1.5 font-caption-sm text-caption-sm font-medium border transition-colors {scheduleStore.isDirty
          ? 'bg-surface-container-high border-[var(--titlebar-border)] text-on-surface hover:bg-surface-variant'
          : 'bg-surface-container-lowest/40 border-transparent text-on-surface-variant/40 cursor-not-allowed'}"
      >
        <RotateCcw size={13} />
        <span>Discard</span>
      </button>

      <!-- Save Button -->
      <button
        type="button"
        disabled={!scheduleStore.isDirty || scheduleStore.isSaving}
        onclick={handleSave}
        class="h-8 px-space-lg rounded flex items-center gap-1.5 font-label-md text-label-md font-semibold transition-all {scheduleStore.isDirty
          ? 'bg-primary-container text-on-primary-container shadow-md hover:brightness-110 active:scale-[0.98]'
          : 'bg-surface-container-high text-on-surface-variant/50 border border-[var(--titlebar-border)] cursor-not-allowed'}"
      >
        {#if scheduleStore.isSaving}
          <Loader2 size={14} class="animate-spin" />
          <span>Applying...</span>
        {:else}
          <Save size={14} />
          <span>Apply Changes</span>
        {/if}
      </button>
    </div>
  </div>

  <!-- Error Alert if Save Failed -->
  {#if scheduleStore.errorMessage}
    <div
      class="p-3 rounded-lg bg-error/15 border border-error/30 text-error flex items-center gap-2 font-body-md text-body-md"
    >
      <AlertCircle size={16} class="flex-shrink-0" />
      <span>{scheduleStore.errorMessage}</span>
    </div>
  {/if}

  <!-- 1. Schedule Mode Selection -->
  <ScheduleModeSelector />

  <!-- 2. Active Mode Details -->
  {#if scheduleStore.config.type === 'sunrise_sunset'}
    <AstronomicalLocationCard />
  {:else if scheduleStore.config.type === 'custom_times'}
    <CustomTimesCard />
  {:else if scheduleStore.config.type === 'night_light'}
    <div
      class="bg-surface-container rounded-lg p-space-lg flex flex-col gap-space-sm shadow-[var(--card-shadow)] border border-[var(--card-border)]"
    >
      <div class="flex items-center gap-2 text-tertiary">
        <Sparkles size={18} />
        <h3 class="font-title-sm text-title-sm text-on-surface font-semibold">
          Windows Night Light Integration
        </h3>
      </div>
      <p class="font-body-md text-body-md text-on-surface-variant">
        AutoDarkMode will listen to Windows 11 system events and automatically activate Dark Mode whenever Windows Night Light turns on, and Light Mode when Night Light turns off.
      </p>
      <div class="flex items-center gap-2 pt-2 text-outline font-caption-sm text-caption-sm">
        <Info size={14} />
        <span>Night Light schedule is configured in Windows Settings &gt; System &gt; Display &gt; Night light.</span>
      </div>
    </div>
  {/if}

  <!-- 3. Dynamic 24-Hour Horizon Visualizer -->
  <SchedulePreviewBar />

  <!-- 4. Conditions & Exceptions -->
  <ScheduleConditionsCard />
</div>
