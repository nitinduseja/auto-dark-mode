<script lang="ts">
  import { SunMedium, RefreshCw, AlertTriangle, CheckCircle2, ShieldCheck } from '@lucide/svelte';
  import { scheduleStore } from '../../stores/schedule.svelte';
  import Button from '../ui/Button.svelte';

  function getLightingCategory(lux: number): string {
    if (lux < 10) return 'Very Dark / Low Night Light';
    if (lux < 40) return 'Dim Room / Soft Ambient';
    if (lux < 150) return 'Normal Living Room';
    if (lux < 500) return 'Bright Office / Studio';
    return 'Direct Daylight / Window Light';
  }

  function handleDarkThresholdChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value) || 0;
    const clamped = Math.max(0, Math.min(val, scheduleStore.config.ambient_light_light_threshold_lux));
    scheduleStore.updateField('ambient_light_dark_threshold_lux', clamped);
  }

  function handleLightThresholdChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value) || 0;
    const clamped = Math.max(scheduleStore.config.ambient_light_dark_threshold_lux, Math.min(val, 2000));
    scheduleStore.updateField('ambient_light_light_threshold_lux', clamped);
  }
</script>

<div class="bg-surface-container rounded-lg p-space-lg flex flex-col gap-space-lg shadow-[var(--card-shadow)] border border-[var(--card-border)]">
  <!-- Card Header -->
  <div class="flex items-center justify-between flex-wrap gap-2">
    <div class="flex items-center gap-space-sm">
      <div class="w-10 h-10 rounded-lg bg-amber-500/10 text-amber-500 flex items-center justify-center">
        <SunMedium size={22} />
      </div>
      <div class="flex flex-col">
        <h3 class="font-title-sm text-title-sm text-on-surface font-semibold">
          Ambient Light Sensor (ALS) Configuration
        </h3>
        <p class="font-caption-sm text-caption-sm text-on-surface-variant">
          Continuously monitors ambient room illumination using your Windows light sensor
        </p>
      </div>
    </div>

    <Button
      variant="ghost"
      size="sm"
      icon={RefreshCw}
      loading={scheduleStore.isQueryingSensor}
      onclick={() => scheduleStore.querySensor()}
    >
      Rescan Sensor
    </Button>
  </div>

  <!-- Live Sensor Status Section -->
  <div class="p-space-md rounded-lg bg-surface-container-high border border-[var(--card-border)] flex flex-col gap-space-sm">
    <div class="flex items-center justify-between flex-wrap gap-2">
      <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase tracking-wider">
        Hardware Sensor Diagnostics
      </span>

      {#if scheduleStore.ambientLightStatus?.is_supported && scheduleStore.ambientLightStatus?.current_lux !== null}
        <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold bg-tertiary/15 text-tertiary border border-tertiary/30">
          <CheckCircle2 size={13} />
          <span>Sensor Online & Calibrated</span>
        </span>
      {:else}
        <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold bg-amber-500/15 text-amber-500 border border-amber-500/30">
          <AlertTriangle size={13} />
          <span>No Hardware Sensor Found</span>
        </span>
      {/if}
    </div>

    {#if scheduleStore.ambientLightStatus?.is_supported && scheduleStore.ambientLightStatus?.current_lux !== null}
      <div class="flex items-baseline gap-3 pt-1">
        <span class="font-display-md text-display-md text-on-surface font-bold tracking-tight">
          {scheduleStore.ambientLightStatus.current_lux.toFixed(1)}
        </span>
        <span class="text-sm font-semibold text-on-surface-variant uppercase tracking-wider">
          Lux
        </span>
        <span class="text-xs px-2 py-0.5 rounded bg-surface-container-highest text-on-surface-variant font-medium">
          {getLightingCategory(scheduleStore.ambientLightStatus.current_lux)}
        </span>
      </div>
    {:else}
      <p class="text-caption-sm text-on-surface-variant leading-relaxed">
        {scheduleStore.ambientLightStatus?.error_message || 'No physical ambient light sensor was detected on this device. AutoDarkMode will safely fall back to astronomical sunrise and sunset schedules when ambient light mode is active.'}
      </p>
    {/if}
  </div>

  <!-- Threshold Sliders & Settings -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-space-lg pt-space-xs border-t border-[var(--card-border)]">
    <!-- Dark Mode Threshold -->
    <div class="flex flex-col gap-space-sm bg-surface-container-low p-space-md rounded border border-[var(--card-border)]">
      <div class="flex items-center justify-between">
        <span class="font-label-md text-label-md text-on-surface font-semibold flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-primary-container"></span>
          Dark Mode Threshold
        </span>
        <span class="font-mono text-sm font-bold text-on-surface bg-surface-container px-2 py-0.5 rounded border border-[var(--card-border)]">
          {scheduleStore.config.ambient_light_dark_threshold_lux} Lux
        </span>
      </div>
      <p class="font-caption-sm text-caption-sm text-on-surface-variant">
        Switch to Dark Mode when room illumination drops to or below this level.
      </p>
      <input
        type="range"
        min="0"
        max="300"
        step="5"
        value={scheduleStore.config.ambient_light_dark_threshold_lux}
        oninput={handleDarkThresholdChange}
        class="w-full accent-primary h-1.5 bg-surface-container-highest rounded-lg cursor-pointer mt-1"
      />
      <div class="flex justify-between text-[11px] text-outline font-mono">
        <span>0 Lux (Pitch Dark)</span>
        <span>150 Lux</span>
        <span>300 Lux (Moderate)</span>
      </div>
    </div>

    <!-- Light Mode Threshold -->
    <div class="flex flex-col gap-space-sm bg-surface-container-low p-space-md rounded border border-[var(--card-border)]">
      <div class="flex items-center justify-between">
        <span class="font-label-md text-label-md text-on-surface font-semibold flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-secondary"></span>
          Light Mode Threshold
        </span>
        <span class="font-mono text-sm font-bold text-on-surface bg-surface-container px-2 py-0.5 rounded border border-[var(--card-border)]">
          {scheduleStore.config.ambient_light_light_threshold_lux} Lux
        </span>
      </div>
      <p class="font-caption-sm text-caption-sm text-on-surface-variant">
        Switch to Light Mode when room illumination rises to or above this level.
      </p>
      <input
        type="range"
        min="20"
        max="600"
        step="5"
        value={scheduleStore.config.ambient_light_light_threshold_lux}
        oninput={handleLightThresholdChange}
        class="w-full accent-secondary h-1.5 bg-surface-container-highest rounded-lg cursor-pointer mt-1"
      />
      <div class="flex justify-between text-[11px] text-outline font-mono">
        <span>20 Lux</span>
        <span>300 Lux</span>
        <span>600 Lux (Bright Daylight)</span>
      </div>
    </div>
  </div>

  <!-- Anti-Flicker Hysteresis Notice -->
  <div class="flex items-start gap-space-sm p-space-sm px-space-md rounded bg-surface-container-low border border-[var(--card-border)] text-on-surface-variant">
    <ShieldCheck size={18} class="text-tertiary flex-shrink-0 mt-0.5" />
    <div class="flex flex-col gap-0.5">
      <span class="font-label-md text-label-md font-semibold text-on-surface">
        Intelligent Hysteresis Deadband Protection
      </span>
      <span class="font-caption-sm text-caption-sm leading-relaxed">
        The difference between the Dark threshold ({scheduleStore.config.ambient_light_dark_threshold_lux} Lux) and Light threshold ({scheduleStore.config.ambient_light_light_threshold_lux} Lux) creates an anti-flickering buffer. When ambient light fluctuates between these values, the active theme remains stable.
      </span>
    </div>
  </div>
</div>
