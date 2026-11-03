<script lang="ts">
  import { Sunrise, Clock, Sparkles } from '@lucide/svelte';
  import type { ScheduleType } from '../../types/schedule';
  import { scheduleStore } from '../../stores/schedule.svelte';

  const modes: Array<{
    type: ScheduleType;
    title: string;
    description: string;
    icon: typeof Sunrise;
    iconColor: string;
    badge?: string;
  }> = [
    {
      type: 'sunrise_sunset',
      title: 'Sunrise to sunset',
      description: 'Follow astronomical daylight calculated from your geographical location',
      icon: Sunrise,
      iconColor: 'text-secondary',
    },
    {
      type: 'custom_times',
      title: 'Set custom hours',
      description: 'Define specific start times for light and dark mode appearances',
      icon: Clock,
      iconColor: 'text-primary-container',
    },
    {
      type: 'night_light',
      title: 'Follow Windows Night Light',
      description: 'Synchronize theme transitions with the Windows Night Light schedule',
      icon: Sparkles,
      iconColor: 'text-tertiary',
      badge: 'System Sync',
    },
  ];

  function selectMode(type: ScheduleType) {
    scheduleStore.setScheduleType(type);
  }
</script>

<div class="flex flex-col gap-space-sm">
  <div class="flex flex-col">
    <h2 class="font-title-sm text-title-sm text-on-surface font-semibold">
      Schedule Mode
    </h2>
    <span class="font-caption-sm text-caption-sm text-on-surface-variant">
      Select how AutoDarkMode determines when to toggle theme appearances
    </span>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-space-md">
    {#each modes as mode (mode.type)}
      {@const isSelected = scheduleStore.config.type === mode.type}
      {@const Icon = mode.icon}

      <button
        type="button"
        onclick={() => selectMode(mode.type)}
        class="relative flex flex-col items-start text-left p-space-md rounded-lg bg-surface-container border transition-all duration-200 outline-none select-none {isSelected
          ? 'border-primary-container bg-surface-container-high shadow-md ring-1 ring-primary-container/40'
          : 'border-[var(--card-border)] hover:bg-surface-container-high hover:border-outline-variant/60'}"
      >
        <!-- Selection Radio Indicator -->
        <div class="w-full flex items-center justify-between mb-3">
          <div
            class="w-8 h-8 rounded bg-surface-container-highest flex items-center justify-center {mode.iconColor}"
          >
            <Icon size={18} />
          </div>

          <div class="flex items-center gap-1.5">
            {#if mode.badge}
              <span
                class="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-surface-variant text-on-surface-variant"
              >
                {mode.badge}
              </span>
            {/if}
            <div
              class="w-4 h-4 rounded-full border flex items-center justify-center transition-colors {isSelected
                ? 'border-primary-container bg-primary-container text-on-primary-container'
                : 'border-outline-variant bg-transparent'}"
            >
              {#if isSelected}
                <div class="w-1.5 h-1.5 rounded-full bg-surface-container-lowest"></div>
              {/if}
            </div>
          </div>
        </div>

        <!-- Title & Description -->
        <span class="font-label-md text-label-md text-on-surface font-semibold">
          {mode.title}
        </span>
        <p class="font-caption-sm text-caption-sm text-on-surface-variant mt-1 leading-relaxed">
          {mode.description}
        </p>
      </button>
    {/each}
  </div>
</div>
