<script lang="ts">
  import { MapPin, Sunrise, Sunset, Sliders, Crosshair } from '@lucide/svelte';
  import { scheduleStore } from '../../stores/schedule.svelte';
  import { POPULAR_CITIES, type CityPreset } from '../../types/schedule';

  function handleCitySelect(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    if (val === 'custom') return;
    const city = POPULAR_CITIES.find((c) => c.name === val);
    if (city) {
      scheduleStore.setCityPreset(city);
    }
  }

  function handleSunriseOffsetChange(e: Event) {
    const val = parseInt((e.target as HTMLInputElement).value, 10);
    if (!isNaN(val)) scheduleStore.updateField('sunrise_offset_minutes', val);
  }

  function handleSunsetOffsetChange(e: Event) {
    const val = parseInt((e.target as HTMLInputElement).value, 10);
    if (!isNaN(val)) scheduleStore.updateField('sunset_offset_minutes', val);
  }

  function handleLatChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    if (!isNaN(val) && val >= -90 && val <= 90) {
      scheduleStore.updateField('latitude', val);
    }
  }

  function handleLonChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    if (!isNaN(val) && val >= -180 && val <= 180) {
      scheduleStore.updateField('longitude', val);
    }
  }

  function formatOffset(offset: number, event: 'sunrise' | 'sunset'): string {
    if (offset === 0) return `Exactly at ${event}`;
    if (offset < 0) return `${Math.abs(offset)}m before ${event}`;
    return `${offset}m after ${event}`;
  }

  let selectedCityValue = $derived.by(() => {
    const matched = POPULAR_CITIES.find(
      (c) =>
        Math.abs(c.latitude - scheduleStore.config.latitude) < 0.01 &&
        Math.abs(c.longitude - scheduleStore.config.longitude) < 0.01
    );
    return matched ? matched.name : 'custom';
  });
</script>

<div
  class="bg-surface-container rounded-lg p-space-lg flex flex-col gap-space-md shadow-[var(--card-shadow)] border border-[var(--card-border)]"
>
  <div class="flex items-center justify-between">
    <div class="flex flex-col">
      <h3 class="font-title-sm text-title-sm text-on-surface font-semibold">
        Astronomical Location & Offsets
      </h3>
      <span class="font-caption-sm text-caption-sm text-on-surface-variant">
        Calculate precise solar twilight transitions based on latitude and longitude
      </span>
    </div>

    <!-- Live Astronomical Summary -->
    <div
      class="px-space-sm py-1 rounded bg-surface-container-high border border-[var(--titlebar-border)] flex items-center gap-space-md font-caption-sm text-caption-sm text-on-surface-variant"
    >
      <div class="flex items-center gap-1.5">
        <Sunrise size={14} class="text-secondary" />
        <span>Sunrise: <strong class="text-on-surface">{scheduleStore.preview?.sunrise_time_formatted ?? '6:42 AM'}</strong></span>
      </div>
      <div class="flex items-center gap-1.5">
        <Sunset size={14} class="text-secondary-fixed-dim" />
        <span>Sunset: <strong class="text-on-surface">{scheduleStore.preview?.sunset_time_formatted ?? '5:48 PM'}</strong></span>
      </div>
    </div>
  </div>

  <!-- City & Coordinates Configuration -->
  <div class="grid grid-cols-1 md:grid-cols-12 gap-space-md">
    <!-- City Preset Dropdown -->
    <div class="md:col-span-5 flex flex-col gap-1.5">
      <div class="flex items-center justify-between">
        <label for="city-select" class="font-label-md text-label-md text-on-surface font-medium flex items-center gap-1.5">
          <MapPin size={14} class="text-primary-container" />
          <span>Location Preset</span>
        </label>
        <button
          type="button"
          class="text-caption-sm text-primary hover:underline flex items-center gap-1 cursor-pointer font-medium"
          onclick={() => scheduleStore.autoDetectLocation()}
          title="Auto-detect coordinates from Windows system timezone"
        >
          <Crosshair size={12} />
          <span>Auto-detect</span>
        </button>
      </div>
      <select
        id="city-select"
        value={selectedCityValue}
        onchange={handleCitySelect}
        class="bg-surface-container-highest text-on-surface rounded px-3 py-2 font-label-md text-label-md border border-outline-variant/40 focus:outline-none focus:border-primary-container cursor-pointer"
      >
        <optgroup label="Popular Cities">
          {#each POPULAR_CITIES as city}
            <option value={city.name}>{city.name}</option>
          {/each}
        </optgroup>
        <option value="custom">Custom Coordinates</option>
      </select>
    </div>

    <!-- Latitude Input -->
    <div class="md:col-span-3 flex flex-col gap-1.5">
      <label for="latitude-input" class="font-label-md text-label-md text-on-surface font-medium">
        Latitude (° N/S)
      </label>
      <input
        id="latitude-input"
        type="number"
        step="0.0001"
        min="-90"
        max="90"
        value={scheduleStore.config.latitude}
        oninput={handleLatChange}
        class="bg-surface-container-highest text-on-surface rounded px-3 py-2 font-label-md text-label-md border border-outline-variant/40 focus:outline-none focus:border-primary-container"
      />
    </div>

    <!-- Longitude Input -->
    <div class="md:col-span-4 flex flex-col gap-1.5">
      <label for="longitude-input" class="font-label-md text-label-md text-on-surface font-medium">
        Longitude (° E/W)
      </label>
      <input
        id="longitude-input"
        type="number"
        step="0.0001"
        min="-180"
        max="180"
        value={scheduleStore.config.longitude}
        oninput={handleLonChange}
        class="bg-surface-container-highest text-on-surface rounded px-3 py-2 font-label-md text-label-md border border-outline-variant/40 focus:outline-none focus:border-primary-container"
      />
    </div>
  </div>

  <!-- Twilight Offsets Sliders -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-space-md pt-2 border-t border-[var(--titlebar-border)]">
    <!-- Sunrise Offset Slider -->
    <div class="p-space-md rounded bg-surface-container-low border border-[var(--card-border)] flex flex-col gap-2">
      <div class="flex items-center justify-between">
        <span class="font-label-md text-label-md text-on-surface font-semibold flex items-center gap-1.5">
          <Sunrise size={16} class="text-secondary" />
          <span>Sunrise Offset</span>
        </span>
        <span class="font-caption-sm text-caption-sm text-secondary font-medium">
          {formatOffset(scheduleStore.config.sunrise_offset_minutes, 'sunrise')}
        </span>
      </div>

      <div class="flex items-center gap-3">
        <input
          type="range"
          min="-120"
          max="120"
          step="5"
          value={scheduleStore.config.sunrise_offset_minutes}
          oninput={handleSunriseOffsetChange}
          aria-label="Sunrise offset slider"
          class="flex-1 accent-primary-container cursor-pointer"
        />
        <span class="w-16 text-right font-caption-sm text-caption-sm text-outline font-mono">
          {scheduleStore.config.sunrise_offset_minutes > 0 ? `+${scheduleStore.config.sunrise_offset_minutes}` : scheduleStore.config.sunrise_offset_minutes} min
        </span>
      </div>
      <p class="font-caption-sm text-caption-sm text-outline">
        Switch to Light mode earlier or later than exact astronomical sunrise
      </p>
    </div>

    <!-- Sunset Offset Slider -->
    <div class="p-space-md rounded bg-surface-container-low border border-[var(--card-border)] flex flex-col gap-2">
      <div class="flex items-center justify-between">
        <span class="font-label-md text-label-md text-on-surface font-semibold flex items-center gap-1.5">
          <Sunset size={16} class="text-secondary-fixed-dim" />
          <span>Sunset Offset</span>
        </span>
        <span class="font-caption-sm text-caption-sm text-secondary-fixed-dim font-medium">
          {formatOffset(scheduleStore.config.sunset_offset_minutes, 'sunset')}
        </span>
      </div>

      <div class="flex items-center gap-3">
        <input
          type="range"
          min="-120"
          max="120"
          step="5"
          value={scheduleStore.config.sunset_offset_minutes}
          oninput={handleSunsetOffsetChange}
          aria-label="Sunset offset slider"
          class="flex-1 accent-primary-container cursor-pointer"
        />
        <span class="w-16 text-right font-caption-sm text-caption-sm text-outline font-mono">
          {scheduleStore.config.sunset_offset_minutes > 0 ? `+${scheduleStore.config.sunset_offset_minutes}` : scheduleStore.config.sunset_offset_minutes} min
        </span>
      </div>
      <p class="font-caption-sm text-caption-sm text-outline">
        Switch to Dark mode earlier or later than exact astronomical sunset
      </p>
    </div>
  </div>
</div>
