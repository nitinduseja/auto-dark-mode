<script lang="ts">
  import {
    Calendar,
    Sun,
    Moon,
    Sunrise,
    Sunset,
    Hourglass,
    Crosshair,
  } from '@lucide/svelte';
  import type { ScheduleTimeline as ScheduleTimelineData } from '../../types/dashboard';
  import { dashboard } from '../../stores/dashboard.svelte';

  interface Props {
    timeline?: ScheduleTimelineData;
  }

  let { timeline }: Props = $props();

  let night1 = $derived(timeline?.night1_percentage ?? 27.9);
  let day = $derived(timeline?.day_percentage ?? 46.3);
  let night2 = $derived(timeline?.night2_percentage ?? 25.8);
  let needle = $derived(timeline?.current_needle_percentage ?? 50.0);
  let sunriseStr = $derived(timeline?.sunrise_time_formatted ?? '6:42 AM');
  let sunsetStr = $derived(timeline?.sunset_time_formatted ?? '5:48 PM');
  let currentTimeStr = $derived(timeline?.current_time_formatted ?? '12:00 PM');
  let daylightStr = $derived(timeline?.daylight_window_formatted ?? '11h 06m');
</script>

<div
  class="bg-surface-container rounded-lg p-space-lg flex flex-col gap-space-md shadow-[var(--card-shadow)] border border-[var(--card-border)] transition-colors"
>
  <!-- Timeline Card Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-space-sm">
      <Calendar size={18} class="text-primary-container" />
      <h2 class="font-title-sm text-title-sm text-on-surface font-semibold">
        Today's schedule
      </h2>
    </div>
    <div
      class="flex items-center gap-space-md font-caption-sm text-caption-sm text-on-surface-variant"
    >
      <div class="flex items-center gap-1.5">
        <span class="w-2 h-2 rounded-full bg-secondary"></span>
        <span>Sunrise: {sunriseStr}</span>
      </div>
      <div class="flex items-center gap-1.5">
        <span class="w-2 h-2 rounded-full bg-secondary-fixed-dim"></span>
        <span>Sunset: {sunsetStr}</span>
      </div>
    </div>
  </div>

  <!-- 24-Hour Dynamic Horizon Bar -->
  <div class="flex flex-col gap-2 pt-1 select-none">
    <div
      class="relative w-full h-8 bg-surface-container-lowest rounded-md overflow-hidden flex items-center p-1 border border-[var(--titlebar-border)] shadow-inner"
    >
      <!-- Night Segment 1 (00:00 to Sunrise) -->
      <div
        class="h-full rounded-l bg-surface-container-high/80 relative flex items-center justify-end px-2"
        style="width: {night1}%;"
      >
        <Moon size={13} class="text-outline" />
      </div>

      <!-- Sunrise Marker Pill -->
      <div
        class="h-full w-1 bg-secondary rounded-full relative z-10 -ml-0.5 shadow-[0_0_6px_currentColor] text-secondary flex-shrink-0"
      ></div>

      <!-- Day Segment (Sunrise to Sunset) -->
      <div
        class="h-full bg-surface-variant flex items-center justify-between px-3 relative"
        style="width: {day}%;"
      >
        <div
          class="flex items-center gap-1 text-secondary font-caption-sm text-caption-sm font-medium"
        >
          <Sun size={13} />
          <span class="hidden sm:inline">Light Mode Period</span>
        </div>
        <span class="font-caption-sm text-caption-sm text-on-surface-variant font-medium">
          {daylightStr} daylight
        </span>
      </div>

      <!-- Sunset Marker Pill -->
      <div
        class="h-full w-1 bg-secondary-fixed-dim rounded-full relative z-10 -ml-0.5 shadow-[0_0_6px_currentColor] text-secondary-fixed-dim flex-shrink-0"
      ></div>

      <!-- Night Segment 2 (Sunset to 24:00) -->
      <div
        class="h-full rounded-r bg-surface-container-high/80 flex items-center justify-start px-2 flex-grow"
      >
        <Moon size={13} class="text-outline" />
      </div>

      <!-- Dynamic Current Time Needle -->
      <div
        class="absolute top-0 bottom-0 z-20 flex flex-col items-center pointer-events-none -ml-[3px]"
        style="left: {Math.max(1, Math.min(99, needle))}%;"
      >
        <div
          class="w-1.5 h-full bg-primary-container rounded-full shadow-[0_0_8px_rgba(0,103,192,0.6)]"
        ></div>
        <div
          class="absolute -top-6 px-1.5 py-0.5 rounded bg-surface-container-highest text-primary font-caption-sm text-caption-sm font-semibold tracking-tight whitespace-nowrap shadow-md border border-[var(--titlebar-border)]"
        >
          {currentTimeStr}
        </div>
      </div>
    </div>

    <!-- Time Axis Markings -->
    <div
      class="flex justify-between font-caption-sm text-caption-sm text-outline px-1"
    >
      <span>00:00</span>
      <span>04:00</span>
      <span class="text-secondary font-medium">{sunriseStr}</span>
      <span>12:00</span>
      <span class="text-secondary-fixed-dim font-medium">{sunsetStr}</span>
      <span>20:00</span>
      <span>24:00</span>
    </div>
  </div>

  <!-- Schedule Stat Badges Grid -->
  <div class="grid grid-cols-2 md:grid-cols-4 gap-space-sm pt-space-xs">
    <!-- Stat 1: Dawn / Sunrise -->
    <div
      class="bg-surface-container-low p-space-sm rounded flex items-center gap-space-sm border border-[var(--titlebar-border)] shadow-sm"
    >
      <div
        class="w-8 h-8 rounded bg-secondary/15 flex items-center justify-center text-secondary flex-shrink-0"
      >
        <Sunrise size={16} />
      </div>
      <div class="flex flex-col min-w-0">
        <span class="font-caption-sm text-caption-sm text-outline"
          >Dawn / Sunrise</span
        >
        <span
          class="font-label-md text-label-md text-on-surface font-semibold truncate"
        >
          {sunriseStr}
        </span>
      </div>
    </div>

    <!-- Stat 2: Dusk / Sunset -->
    <div
      class="bg-surface-container-low p-space-sm rounded flex items-center gap-space-sm border border-[var(--titlebar-border)] shadow-sm"
    >
      <div
        class="w-8 h-8 rounded bg-secondary-fixed-dim/15 flex items-center justify-center text-secondary-fixed-dim flex-shrink-0"
      >
        <Sunset size={16} />
      </div>
      <div class="flex flex-col min-w-0">
        <span class="font-caption-sm text-caption-sm text-outline"
          >Dusk / Sunset</span
        >
        <span
          class="font-label-md text-label-md text-on-surface font-semibold truncate"
        >
          {sunsetStr}
        </span>
      </div>
    </div>

    <!-- Stat 3: Daylight window -->
    <div
      class="bg-surface-container-low p-space-sm rounded flex items-center gap-space-sm border border-[var(--titlebar-border)] shadow-sm"
    >
      <div
        class="w-8 h-8 rounded bg-surface-variant flex items-center justify-center text-on-surface-variant flex-shrink-0"
      >
        <Hourglass size={16} />
      </div>
      <div class="flex flex-col min-w-0">
        <span class="font-caption-sm text-caption-sm text-outline"
          >Daylight window</span
        >
        <span
          class="font-label-md text-label-md text-on-surface font-semibold truncate"
        >
          {daylightStr}
        </span>
      </div>
    </div>

    <!-- Stat 4: Coordinates -->
    <div
      class="bg-surface-container-low p-space-sm rounded flex items-center gap-space-sm border border-[var(--titlebar-border)] shadow-sm"
    >
      <div
        class="w-8 h-8 rounded bg-primary/10 flex items-center justify-center text-primary flex-shrink-0"
      >
        <Crosshair size={16} />
      </div>
      <div class="flex flex-col min-w-0">
        <span class="font-caption-sm text-caption-sm text-outline">Schedule Source</span>
        <span
          class="font-label-md text-label-md text-on-surface font-semibold truncate"
        >
          {dashboard.state?.next_transition?.location_note || 'Configured schedule'}
        </span>
      </div>
    </div>
  </div>
</div>
