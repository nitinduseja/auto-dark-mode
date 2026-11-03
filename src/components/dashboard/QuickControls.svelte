<script lang="ts">
  import {
    SlidersHorizontal,
    Sunrise,
    Image as ImageIcon,
    AppWindow,
    ArrowRight,
  } from '@lucide/svelte';
  import Toggle from '../ui/Toggle.svelte';
  import { dashboard } from '../../stores/dashboard.svelte';
  import type { ControlState } from '../../types/dashboard';
  import type { AppPage } from '../layout/Sidebar.svelte';

  interface Props {
    onnavigate?: (page: AppPage) => void;
  }

  let { onnavigate }: Props = $props();

  const icons = {
    automatic_switching: SlidersHorizontal,
    follow_sun: Sunrise,
    wallpaper: ImageIcon,
    app_themes: AppWindow,
  };

  const iconColors = {
    automatic_switching: 'text-primary-container',
    follow_sun: 'text-secondary',
    wallpaper: 'text-tertiary',
    app_themes: 'text-outline',
  };

  function handleToggle(control: ControlState) {
    dashboard.toggleControl(control);
  }
</script>

<div class="flex flex-col gap-space-sm select-none">
  <!-- Section Title -->
  <div class="flex items-center justify-between">
    <div class="flex flex-col">
      <h2 class="font-title-sm text-title-sm text-on-surface font-semibold">
        Quick controls
      </h2>
      <span class="font-caption-sm text-caption-sm text-on-surface-variant">
        Frequently accessed automation toggles
      </span>
    </div>
    {#if onnavigate}
      <button
        class="font-caption-sm text-caption-sm text-primary-container hover:underline flex items-center gap-0.5 bg-transparent border-none cursor-pointer p-0"
        onclick={() => onnavigate('conditions')}
      >
        <span>Configure all conditions</span>
        <ArrowRight size={14} />
      </button>
    {/if}
  </div>

  <!-- Bento 2x2 Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-space-sm">
    {#each dashboard.state?.quick_controls || [] as control (control.id)}
      {@const Icon = icons[control.id as keyof typeof icons] || SlidersHorizontal}
      {@const iconColor = iconColors[control.id as keyof typeof iconColors] || 'text-outline'}

      <div
        class="bg-surface-container rounded-lg p-space-md flex items-center justify-between shadow-[var(--card-shadow)] transition-colors border border-[var(--card-border)] {control.interactive
          ? 'hover:bg-surface-container-high'
          : 'opacity-65'}"
      >
        <div class="flex items-center gap-space-md min-w-0 pr-2">
          <!-- Control Icon -->
          <div
            class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center {iconColor} flex-shrink-0"
          >
            <Icon size={18} />
          </div>

          <!-- Titles and Descriptions -->
          <div class="flex flex-col min-w-0">
            <div class="flex items-center gap-1.5 truncate">
              <span
                class="font-label-md text-label-md text-on-surface font-semibold truncate"
              >
                {control.title}
              </span>
              {#if control.badge}
                <span
                  class="px-1.5 py-0.2 rounded text-[9px] font-bold tracking-wider leading-tight {control.badge ===
                  'DEMO'
                    ? 'bg-secondary/20 text-secondary'
                    : 'bg-outline-variant/40 text-outline'}"
                >
                  {control.badge}
                </span>
              {/if}
            </div>
            <span
              class="font-caption-sm text-caption-sm text-on-surface-variant truncate"
              title={control.description}
            >
              {control.description}
            </span>
          </div>
        </div>

        <!-- Toggle Switch -->
        <Toggle
          checked={control.enabled}
          disabled={!control.interactive}
          label={control.title}
          onchange={() => handleToggle(control)}
        />
      </div>
    {/each}
  </div>
</div>
