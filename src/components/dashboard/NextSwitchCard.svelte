<script lang="ts">
  import { Sunrise, Sunset, MapPin } from '@lucide/svelte';
  import { dashboard } from '../../stores/dashboard.svelte';

  let next = $derived(dashboard.state?.next_transition);
</script>

<div
  class="lg:col-span-5 bg-surface-container rounded-lg p-space-lg flex flex-col justify-between shadow-[var(--card-shadow)] relative overflow-hidden hover:bg-surface-container-high transition-colors border border-[var(--card-border)]"
>
  <!-- Background Glow Effect -->
  <div
    class="absolute -right-8 -top-8 w-36 h-36 rounded-full bg-secondary/10 blur-xl pointer-events-none"
  ></div>

  <div class="flex flex-col gap-space-sm">
    <!-- Header with Badges -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span
          class="font-caption-sm text-caption-sm text-outline tracking-wider font-semibold uppercase"
        >
          Next Automatic Switch
        </span>
      </div>
      <span
        class="px-space-xs py-0.5 rounded bg-surface-variant text-secondary font-caption-sm text-caption-sm font-semibold"
      >
        {next?.countdown_formatted || 'Calculating...'}
      </span>
    </div>

    <!-- Switch Target and Event Time -->
    <div class="flex flex-col">
      <span class="font-body-md text-body-md text-on-surface-variant">
        Switch to {next?.target_theme === 'dark' ? 'Dark' : 'Light'} mode
      </span>
      <div class="flex items-baseline gap-space-sm mt-1">
        <span
          class="font-display text-display tracking-tight text-on-surface font-semibold leading-none"
        >
          {next?.time_formatted || '--:--'}
        </span>
        {#if next?.target_theme === 'light'}
          <Sunrise size={24} class="text-secondary" />
        {:else}
          <Sunset size={24} class="text-secondary-fixed-dim" />
        {/if}
      </div>
    </div>
  </div>

  <!-- Astronomical / Location Metadata -->
  <div class="pt-space-md flex items-center gap-1.5 font-caption-sm text-caption-sm text-outline">
    <MapPin size={14} class="flex-shrink-0" />
    <span class="truncate">
      {next?.location_note || 'Configured schedule'}
    </span>
  </div>
</div>
