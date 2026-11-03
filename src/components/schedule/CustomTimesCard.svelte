<script lang="ts">
  import { Sun, Moon, Sparkles } from '@lucide/svelte';
  import { scheduleStore } from '../../stores/schedule.svelte';

  const presets = [
    { label: 'Standard (7 AM - 7 PM)', lightH: 7, lightM: 0, darkH: 19, darkM: 0 },
    { label: 'Early Bird (6 AM - 6 PM)', lightH: 6, lightM: 0, darkH: 18, darkM: 0 },
    { label: 'Night Owl (8:30 AM - 10 PM)', lightH: 8, lightM: 30, darkH: 22, darkM: 0 },
  ];

  function applyPreset(p: (typeof presets)[0]) {
    scheduleStore.updateField('light_start_hour', p.lightH);
    scheduleStore.updateField('light_start_minute', p.lightM);
    scheduleStore.updateField('dark_start_hour', p.darkH);
    scheduleStore.updateField('dark_start_minute', p.darkM);
  }

  function handleLightHourChange(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value, 10);
    if (!isNaN(val)) scheduleStore.updateField('light_start_hour', val);
  }

  function handleLightMinuteChange(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value, 10);
    if (!isNaN(val)) scheduleStore.updateField('light_start_minute', val);
  }

  function handleDarkHourChange(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value, 10);
    if (!isNaN(val)) scheduleStore.updateField('dark_start_hour', val);
  }

  function handleDarkMinuteChange(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value, 10);
    if (!isNaN(val)) scheduleStore.updateField('dark_start_minute', val);
  }

  function formatTime(h: number, m: number): string {
    const period = h >= 12 ? 'PM' : 'AM';
    const displayH = h % 12 === 0 ? 12 : h % 12;
    return `${displayH}:${m.toString().padStart(2, '0')} ${period}`;
  }

  let daylightDuration = $derived.by(() => {
    const lightTotal = scheduleStore.config.light_start_hour * 60 + scheduleStore.config.light_start_minute;
    const darkTotal = scheduleStore.config.dark_start_hour * 60 + scheduleStore.config.dark_start_minute;
    const diff = darkTotal >= lightTotal ? darkTotal - lightTotal : 1440 - lightTotal + darkTotal;
    const h = Math.floor(diff / 60);
    const m = diff % 60;
    return `${h}h ${m.toString().padStart(2, '0')}m`;
  });
</script>

<div
  class="bg-surface-container rounded-lg p-space-lg flex flex-col gap-space-md shadow-[var(--card-shadow)] border border-[var(--card-border)]"
>
  <div class="flex items-center justify-between">
    <div class="flex flex-col">
      <h3 class="font-title-sm text-title-sm text-on-surface font-semibold">
        Custom Theme Switching Hours
      </h3>
      <span class="font-caption-sm text-caption-sm text-on-surface-variant">
        Configure the exact times when Windows should transition between light and dark modes
      </span>
    </div>

    <!-- Daylight Summary Badge -->
    <div
      class="px-space-sm py-1 rounded bg-surface-container-high border border-[var(--titlebar-border)] flex items-center gap-1.5 font-caption-sm text-caption-sm text-on-surface-variant"
    >
      <span class="w-1.5 h-1.5 rounded-full bg-secondary"></span>
      <span>Light mode duration: <strong class="text-on-surface">{daylightDuration}</strong></span>
    </div>
  </div>

  <!-- Time Pickers Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-space-lg">
    <!-- Light Mode Start Card -->
    <div
      class="p-space-md rounded-md bg-surface-container-low border border-[var(--card-border)] flex flex-col gap-space-sm"
    >
      <div class="flex items-center gap-2 text-secondary font-label-md text-label-md font-semibold">
        <Sun size={16} />
        <span>Light Mode Start Time</span>
      </div>

      <div class="flex items-center gap-2">
        <!-- Hour Select -->
        <select
          value={scheduleStore.config.light_start_hour}
          onchange={handleLightHourChange}
          aria-label="Light mode start hour"
          class="flex-1 bg-surface-container-highest text-on-surface rounded px-3 py-2 font-label-md text-label-md border border-outline-variant/40 focus:outline-none focus:border-primary-container cursor-pointer"
        >
          {#each Array.from({ length: 24 }, (_, i) => i) as h}
            <option value={h}>
              {h % 12 === 0 ? 12 : h % 12}:00 {h >= 12 ? 'PM' : 'AM'} ({h.toString().padStart(2, '0')}:00)
            </option>
          {/each}
        </select>

        <span class="text-outline font-bold">:</span>

        <!-- Minute Select -->
        <select
          value={scheduleStore.config.light_start_minute}
          onchange={handleLightMinuteChange}
          aria-label="Light mode start minute"
          class="w-24 bg-surface-container-highest text-on-surface rounded px-3 py-2 font-label-md text-label-md border border-outline-variant/40 focus:outline-none focus:border-primary-container cursor-pointer"
        >
          {#each [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55] as m}
            <option value={m}>{m.toString().padStart(2, '0')} min</option>
          {/each}
        </select>
      </div>

      <span class="font-caption-sm text-caption-sm text-outline">
        Active at {formatTime(scheduleStore.config.light_start_hour, scheduleStore.config.light_start_minute)} every morning
      </span>
    </div>

    <!-- Dark Mode Start Card -->
    <div
      class="p-space-md rounded-md bg-surface-container-low border border-[var(--card-border)] flex flex-col gap-space-sm"
    >
      <div class="flex items-center gap-2 text-secondary-fixed-dim font-label-md text-label-md font-semibold">
        <Moon size={16} />
        <span>Dark Mode Start Time</span>
      </div>

      <div class="flex items-center gap-2">
        <!-- Hour Select -->
        <select
          value={scheduleStore.config.dark_start_hour}
          onchange={handleDarkHourChange}
          aria-label="Dark mode start hour"
          class="flex-1 bg-surface-container-highest text-on-surface rounded px-3 py-2 font-label-md text-label-md border border-outline-variant/40 focus:outline-none focus:border-primary-container cursor-pointer"
        >
          {#each Array.from({ length: 24 }, (_, i) => i) as h}
            <option value={h}>
              {h % 12 === 0 ? 12 : h % 12}:00 {h >= 12 ? 'PM' : 'AM'} ({h.toString().padStart(2, '0')}:00)
            </option>
          {/each}
        </select>

        <span class="text-outline font-bold">:</span>

        <!-- Minute Select -->
        <select
          value={scheduleStore.config.dark_start_minute}
          onchange={handleDarkMinuteChange}
          aria-label="Dark mode start minute"
          class="w-24 bg-surface-container-highest text-on-surface rounded px-3 py-2 font-label-md text-label-md border border-outline-variant/40 focus:outline-none focus:border-primary-container cursor-pointer"
        >
          {#each [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55] as m}
            <option value={m}>{m.toString().padStart(2, '0')} min</option>
          {/each}
        </select>
      </div>

      <span class="font-caption-sm text-caption-sm text-outline">
        Active at {formatTime(scheduleStore.config.dark_start_hour, scheduleStore.config.dark_start_minute)} every evening
      </span>
    </div>
  </div>

  <!-- Quick Presets -->
  <div class="flex items-center gap-2 pt-2 border-t border-[var(--titlebar-border)]">
    <div class="flex items-center gap-1.5 text-outline font-caption-sm text-caption-sm mr-1">
      <Sparkles size={13} />
      <span>Quick presets:</span>
    </div>

    <div class="flex flex-wrap gap-2">
      {#each presets as preset}
        <button
          type="button"
          onclick={() => applyPreset(preset)}
          class="px-2.5 py-1 rounded bg-surface-container-high hover:bg-surface-variant text-on-surface-variant hover:text-on-surface font-caption-sm text-caption-sm transition-colors border border-[var(--titlebar-border)]"
        >
          {preset.label}
        </button>
      {/each}
    </div>
  </div>
</div>
