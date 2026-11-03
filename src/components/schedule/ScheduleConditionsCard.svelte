<script lang="ts">
  import { BatteryCharging, Gamepad2, UserCheck, ShieldCheck } from '@lucide/svelte';
  import Toggle from '../ui/Toggle.svelte';
  import { scheduleStore } from '../../stores/schedule.svelte';

  function handleIdleChange(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value, 10);
    if (!isNaN(val)) scheduleStore.updateField('idle_delay_minutes', val);
  }

  function toggleBattery() {
    scheduleStore.updateField('pause_on_battery', !scheduleStore.config.pause_on_battery);
  }

  function toggleGaming() {
    scheduleStore.updateField('pause_on_gaming', !scheduleStore.config.pause_on_gaming);
  }
</script>

<div
  class="bg-surface-container rounded-lg p-space-lg flex flex-col gap-space-md shadow-[var(--card-shadow)] border border-[var(--card-border)]"
>
  <div class="flex items-center justify-between">
    <div class="flex flex-col">
      <h3 class="font-title-sm text-title-sm text-on-surface font-semibold flex items-center gap-2">
        <ShieldCheck size={18} class="text-tertiary" />
        <span>Switching Conditions & Exceptions</span>
      </h3>
      <span class="font-caption-sm text-caption-sm text-on-surface-variant">
        Prevent abrupt theme changes during active work, presentations, or low-battery states
      </span>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-space-md">
    <!-- Idle Delay Control -->
    <div
      class="p-space-md rounded bg-surface-container-low border border-[var(--card-border)] flex flex-col justify-between gap-3"
    >
      <div class="flex items-start gap-space-sm">
        <div
          class="w-8 h-8 rounded bg-surface-container-highest flex items-center justify-center text-primary-container flex-shrink-0"
        >
          <UserCheck size={18} />
        </div>
        <div class="flex flex-col">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            User Idle Delay
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Wait for mouse/keyboard inactivity before switching
          </span>
        </div>
      </div>

      <select
        value={scheduleStore.config.idle_delay_minutes}
        onchange={handleIdleChange}
        aria-label="User idle delay"
        class="bg-surface-container-highest text-on-surface rounded px-3 py-1.5 font-caption-sm text-caption-sm border border-outline-variant/40 focus:outline-none focus:border-primary-container cursor-pointer"
      >
        <option value={0}>Switch immediately (0 min delay)</option>
        <option value={5}>Wait for 5 minutes idle</option>
        <option value={10}>Wait for 10 minutes idle</option>
        <option value={15}>Wait for 15 minutes idle</option>
      </select>
    </div>

    <!-- Battery Exception Control -->
    <div
      class="p-space-md rounded bg-surface-container-low border border-[var(--card-border)] flex items-center justify-between gap-space-sm"
    >
      <div class="flex items-center gap-space-sm min-w-0 pr-2">
        <div
          class="w-8 h-8 rounded bg-surface-container-highest flex items-center justify-center text-secondary flex-shrink-0"
        >
          <BatteryCharging size={18} />
        </div>
        <div class="flex flex-col min-w-0">
          <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
            Pause on Battery
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
            Don't switch when running on battery power
          </span>
        </div>
      </div>

      <Toggle
        checked={scheduleStore.config.pause_on_battery}
        label="Pause on Battery"
        onchange={toggleBattery}
      />
    </div>

    <!-- Gaming / Full Screen Exception Control -->
    <div
      class="p-space-md rounded bg-surface-container-low border border-[var(--card-border)] flex items-center justify-between gap-space-sm"
    >
      <div class="flex items-center gap-space-sm min-w-0 pr-2">
        <div
          class="w-8 h-8 rounded bg-surface-container-highest flex items-center justify-center text-tertiary flex-shrink-0"
        >
          <Gamepad2 size={18} />
        </div>
        <div class="flex flex-col min-w-0">
          <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
            Full Screen & Gaming
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
            Defer switches while games or media are playing
          </span>
        </div>
      </div>

      <Toggle
        checked={scheduleStore.config.pause_on_gaming}
        label="Pause during Gaming"
        onchange={toggleGaming}
      />
    </div>
  </div>
</div>
