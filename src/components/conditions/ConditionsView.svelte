<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    ListChecks,
    Battery,
    Gamepad2,
    Clock,
    Wifi,
    Check,
    AlertCircle,
    Zap,
    ShieldAlert,
    ShieldCheck,
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { conditionsStore } from '../../stores/conditions.svelte';

  onMount(() => {
    conditionsStore.init();
  });

  onDestroy(() => {
    conditionsStore.destroy();
  });
</script>

<div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto select-none">
  <!-- Top Section Title Area -->
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-0.5">
      <h1 class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold">
        Conditions
      </h1>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Prevent theme interruptions during battery operation, fullscreen gaming, and active computer use.
      </p>
    </div>
    <div class="flex items-center gap-space-sm">
      <Button
        variant="primary"
        size="md"
        loading={conditionsStore.isSaving}
        onclick={() => conditionsStore.save()}
      >
        Save Changes
      </Button>
    </div>
  </div>

  {#if conditionsStore.successMessage}
    <div
      class="p-space-sm px-space-md rounded bg-tertiary/15 border border-tertiary/30 text-tertiary font-label-md text-label-md flex items-center gap-2"
    >
      <Check class="w-4 h-4" />
      <span>{conditionsStore.successMessage}</span>
    </div>
  {/if}

  {#if conditionsStore.errorMessage}
    <div
      class="p-space-sm px-space-md rounded bg-error/15 border border-error/30 text-error font-label-md text-label-md flex items-center gap-2"
    >
      <AlertCircle class="w-4 h-4" />
      <span>{conditionsStore.errorMessage}</span>
    </div>
  {/if}

  <!-- Live Hardware Safety Status Ribbon -->
  {#if conditionsStore.systemStatus}
    <div class="py-space-sm px-space-md rounded bg-surface-container-high/60 border border-[var(--card-border)] flex items-center justify-between gap-space-md">
      <div class="flex items-center gap-space-sm">
        {#if conditionsStore.systemStatus.is_switch_allowed}
          <div class="w-2.5 h-2.5 rounded-full bg-tertiary shadow-[0_0_8px_rgba(147,244,130,0.6)]"></div>
          <span class="font-label-md text-label-md text-on-surface font-medium">
            Transitions Permitted
          </span>
        {:else}
          <div class="w-2.5 h-2.5 rounded-full bg-secondary shadow-[0_0_8px_rgba(255,219,156,0.6)] animate-pulse"></div>
          <span class="font-label-md text-label-md text-secondary font-medium">
            Transitions Guarded: {conditionsStore.systemStatus.restriction_reason || 'Safety Condition Active'}
          </span>
        {/if}
      </div>

      <div class="flex items-center gap-space-lg text-outline font-caption-sm text-caption-sm">
        <div class="flex items-center gap-1.5">
          <Zap class="w-3.5 h-3.5 {conditionsStore.systemStatus.power.is_on_battery ? 'text-secondary' : 'text-tertiary'}" />
          <span>
            {conditionsStore.systemStatus.power.is_on_battery ? 'Battery Power' : 'AC Connected'}
            ({conditionsStore.systemStatus.power.battery_percent}%)
          </span>
        </div>
        <div class="flex items-center gap-1.5">
          <Gamepad2 class="w-3.5 h-3.5 {conditionsStore.systemStatus.is_gaming_active ? 'text-secondary' : 'text-outline'}" />
          <span>{conditionsStore.systemStatus.is_gaming_active ? 'Fullscreen Active' : 'Desktop'}</span>
        </div>
      </div>
    </div>
  {/if}

  <!-- Section 1: Power & Battery Conditions -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Battery & Power Source"
      subtitle="Protect laptop battery longevity and prevent backlight shifts unplugged"
      icon={Battery}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Pause Switching When on Battery Power
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Delays scheduled light/dark theme switches until the laptop is plugged back into AC power.
          </span>
        </div>
        <Toggle
          checked={conditionsStore.config.pause_on_battery}
          onchange={(val) => {
            conditionsStore.config.pause_on_battery = val;
            conditionsStore.save();
          }}
        />
      </div>

      <div class="flex items-center justify-between pt-space-xs border-t border-[var(--card-border)]">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Force Dark Mode on Windows Battery Saver
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Immediately trigger Dark mode when Windows Battery Saver engages to conserve power on OLED displays.
          </span>
        </div>
        <Toggle
          checked={conditionsStore.config.force_dark_on_battery_saver}
          onchange={(val) => {
            conditionsStore.config.force_dark_on_battery_saver = val;
            conditionsStore.save();
          }}
        />
      </div>
    </Card>
  </div>

  <!-- Section 2: Fullscreen & Gaming Interruption Guard -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Gaming & Fullscreen Interruption Prevention"
      subtitle="Avoid frame drops and visual stutters during immersion"
      icon={Gamepad2}
    />
    <Card class="flex items-center justify-between">
      <div class="flex flex-col gap-0.5 max-w-[70%]">
        <span class="font-label-md text-label-md text-on-surface font-semibold">
          Don't Switch While Playing Games or in Fullscreen Apps
        </span>
        <span class="font-caption-sm text-caption-sm text-on-surface-variant">
          Postpones theme changes whenever a fullscreen game, video player, or presentation is occupying the primary display.
        </span>
      </div>
      <Toggle
        checked={conditionsStore.config.pause_on_gaming}
        onchange={(val) => {
          conditionsStore.config.pause_on_gaming = val;
          conditionsStore.save();
        }}
      />
    </Card>
  </div>

  <!-- Section 3: Inactivity & Idle Detection -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="User Inactivity Window"
      subtitle="Defer changes until you step away from the keyboard"
      icon={Clock}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Wait for Idle Period Before Switching
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Avoids sudden visual shifts while you are actively typing or moving the mouse.
          </span>
        </div>
        <Toggle
          checked={conditionsStore.config.idle_delay_minutes > 0}
          onchange={(val) => {
            conditionsStore.config.idle_delay_minutes = val ? 5 : 0;
            conditionsStore.save();
          }}
        />
      </div>

      {#if conditionsStore.config.idle_delay_minutes > 0}
        <div class="flex flex-col gap-space-xs pt-space-xs border-t border-[var(--card-border)]">
          <div class="flex justify-between font-caption-sm text-caption-sm">
            <label for="idle-delay-slider" class="text-outline">Required Idle Time</label>
            <span class="text-primary-container font-mono font-semibold">
              {conditionsStore.config.idle_delay_minutes} minutes
            </span>
          </div>
          <input
            id="idle-delay-slider"
            type="range"
            min="1"
            max="30"
            step="1"
            bind:value={conditionsStore.config.idle_delay_minutes}
            class="w-full accent-primary-container cursor-pointer"
            onchange={() => conditionsStore.save()}
          />
        </div>
      {/if}
    </Card>
  </div>

  <!-- Section 4: Metered Connection Condition -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Network Restraints"
      subtitle="Conserve bandwidth when connected to mobile hotspots"
      icon={Wifi}
    />
    <Card class="flex items-center justify-between">
      <div class="flex flex-col gap-0.5 max-w-[70%]">
        <span class="font-label-md text-label-md text-on-surface font-semibold">
          Pause Geolocation Lookups on Metered Networks
        </span>
        <span class="font-caption-sm text-caption-sm text-on-surface-variant">
          Uses cached astronomical coordinates when connected to a metered mobile hotspot.
        </span>
      </div>
      <Toggle
        checked={conditionsStore.config.pause_on_metered}
        onchange={(val) => {
          conditionsStore.config.pause_on_metered = val;
          conditionsStore.save();
        }}
      />
    </Card>
  </div>
</div>
