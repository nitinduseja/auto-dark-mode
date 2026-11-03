<script lang="ts">
  import { Sun, Moon, Calendar, Eye } from '@lucide/svelte';
  import { scheduleStore } from '../../stores/schedule.svelte';

  // Compute live preview percentages based on current reactive config
  let metrics = $derived.by(() => {
    let sunriseMin = 402;
    let sunsetMin = 1068;

    if (scheduleStore.config.type === 'custom_times') {
      sunriseMin =
        scheduleStore.config.light_start_hour * 60 + scheduleStore.config.light_start_minute;
      sunsetMin =
        scheduleStore.config.dark_start_hour * 60 + scheduleStore.config.dark_start_minute;
    } else if (scheduleStore.config.type === 'night_light') {
      sunriseMin = 360; // 6:00 AM
      sunsetMin = 1200; // 8:00 PM
    } else {
      sunriseMin = scheduleStore.preview?.sunrise_minute ?? 402;
      sunsetMin = scheduleStore.preview?.sunset_minute ?? 1068;
    }

    // Ensure ordering for the 24h bar
    const validSunrise = Math.min(1440, Math.max(0, sunriseMin));
    const validSunset = Math.min(1440, Math.max(validSunrise, sunsetMin));

    const total = 1440.0;
    const night1 = (validSunrise / total) * 100.0;
    const day = ((validSunset - validSunrise) / total) * 100.0;
    const night2 = ((total - validSunset) / total) * 100.0;

    const now = new Date();
    const currentMin = now.getHours() * 60 + now.getMinutes();
    const needle = (currentMin / total) * 100.0;

    const daylightMins = validSunset - validSunrise;
    const dh = Math.floor(daylightMins / 60);
    const dm = daylightMins % 60;

    const format12 = (min: number) => {
      const h = Math.floor(min / 60) % 24;
      const m = min % 60;
      const p = h >= 12 ? 'PM' : 'AM';
      const dh12 = h % 12 === 0 ? 12 : h % 12;
      return `${dh12}:${m.toString().padStart(2, '0')} ${p}`;
    };

    return {
      night1,
      day,
      night2,
      needle,
      sunriseFormatted: format12(validSunrise),
      sunsetFormatted: format12(validSunset),
      daylightFormatted: `${dh}h ${dm.toString().padStart(2, '0')}m`,
    };
  });
</script>

<div
  class="bg-surface-container rounded-lg p-space-lg flex flex-col gap-space-md shadow-[var(--card-shadow)] border border-[var(--card-border)]"
>
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <Eye size={18} class="text-primary-container" />
      <h3 class="font-title-sm text-title-sm text-on-surface font-semibold">
        24-Hour Horizon Visualizer
      </h3>
    </div>

    <div
      class="flex items-center gap-space-md font-caption-sm text-caption-sm text-on-surface-variant"
    >
      <div class="flex items-center gap-1.5">
        <span class="w-2 h-2 rounded-full bg-secondary"></span>
        <span>Light Mode begins: <strong class="text-on-surface">{metrics.sunriseFormatted}</strong></span>
      </div>
      <div class="flex items-center gap-1.5">
        <span class="w-2 h-2 rounded-full bg-secondary-fixed-dim"></span>
        <span>Dark Mode begins: <strong class="text-on-surface">{metrics.sunsetFormatted}</strong></span>
      </div>
    </div>
  </div>

  <!-- 24-Hour Bar -->
  <div class="flex flex-col gap-2 pt-1 select-none">
    <div
      class="relative w-full h-8 bg-surface-container-lowest rounded-md overflow-hidden flex items-center p-1 border border-[var(--titlebar-border)] shadow-inner"
    >
      <!-- Night Segment 1 (00:00 to Sunrise) -->
      <div
        class="h-full rounded-l bg-surface-container-high/80 relative flex items-center justify-end px-2"
        style="width: {metrics.night1}%;"
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
        style="width: {metrics.day}%;"
      >
        <div
          class="flex items-center gap-1 text-secondary font-caption-sm text-caption-sm font-medium"
        >
          <Sun size={13} />
          <span class="hidden sm:inline">Light Mode Period</span>
        </div>
        <span class="font-caption-sm text-caption-sm text-on-surface-variant font-medium">
          {metrics.daylightFormatted} daylight
        </span>
      </div>

      <!-- Sunset Marker Pill -->
      <div
        class="h-full w-1 bg-secondary-fixed-dim rounded-full relative z-10 -ml-0.5 shadow-[0_0_6px_currentColor] text-secondary-fixed-dim flex-shrink-0"
      ></div>

      <!-- Night Segment 2 (Sunset to 24:00) -->
      <div
        class="h-full rounded-r bg-surface-container-high/80 flex items-center justify-start px-2 flex-grow"
        style="width: {metrics.night2}%;"
      >
        <Moon size={13} class="text-outline" />
      </div>

      <!-- Current Time Indicator Needle -->
      <div
        class="absolute top-0 bottom-0 z-20 flex flex-col items-center pointer-events-none"
        style="left: {metrics.needle}%;"
      >
        <div
          class="w-1.5 h-full bg-primary-container rounded-full shadow-[0_0_8px_rgba(96,205,255,0.8)]"
        ></div>
      </div>
    </div>

    <!-- Time Axis Markings -->
    <div class="flex justify-between font-caption-sm text-caption-sm text-outline px-1">
      <span>00:00</span>
      <span>04:00</span>
      <span class="text-secondary font-medium">{metrics.sunriseFormatted}</span>
      <span>12:00</span>
      <span class="text-secondary-fixed-dim font-medium">{metrics.sunsetFormatted}</span>
      <span>20:00</span>
      <span>24:00</span>
    </div>
  </div>
</div>
