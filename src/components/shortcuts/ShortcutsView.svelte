<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Keyboard,
    Command,
    Sun,
    Moon,
    Clock,
    Check,
    AlertCircle,
    RotateCcw,
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { shortcutsStore } from '../../stores/shortcuts.svelte';

  onMount(() => {
    shortcutsStore.init();
  });

  function resetShortcuts() {
    shortcutsStore.config.toggle_shortcut = 'Ctrl+Alt+T';
    shortcutsStore.config.force_light_shortcut = 'Ctrl+Alt+L';
    shortcutsStore.config.force_dark_shortcut = 'Ctrl+Alt+D';
    shortcutsStore.config.postpone_shortcut = 'Ctrl+Alt+P';
    shortcutsStore.save();
  }
</script>

<div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto select-none">
  <!-- Top Section Title Area -->
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-0.5">
      <h1 class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold">
        Shortcuts
      </h1>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Define system-wide global hotkeys to switch or postpone themes from anywhere in Windows.
      </p>
    </div>
    <div class="flex items-center gap-space-sm">
      <Button
        variant="ghost"
        size="md"
        icon={RotateCcw}
        onclick={resetShortcuts}
      >
        Reset Defaults
      </Button>
      <Button
        variant="primary"
        size="md"
        loading={shortcutsStore.isSaving}
        onclick={() => shortcutsStore.save()}
      >
        Save Changes
      </Button>
    </div>
  </div>

  {#if shortcutsStore.successMessage}
    <div
      class="p-space-sm px-space-md rounded bg-tertiary/15 border border-tertiary/30 text-tertiary font-label-md text-label-md flex items-center gap-2"
    >
      <Check class="w-4 h-4" />
      <span>{shortcutsStore.successMessage}</span>
    </div>
  {/if}

  {#if shortcutsStore.errorMessage}
    <div
      class="p-space-sm px-space-md rounded bg-error/15 border border-error/30 text-error font-label-md text-label-md flex items-center gap-2"
    >
      <AlertCircle class="w-4 h-4" />
      <span>{shortcutsStore.errorMessage}</span>
    </div>
  {/if}

  <!-- Section 1: Shortcuts Master Toggle -->
  <Card class="flex items-center justify-between">
    <div class="flex flex-col gap-0.5 max-w-[70%]">
      <span class="font-label-md text-label-md text-on-surface font-semibold">
        Enable Global Keyboard Shortcuts
      </span>
      <span class="font-caption-sm text-caption-sm text-on-surface-variant">
        Listens for configured hotkeys globally across all Windows applications and games.
      </span>
    </div>
    <Toggle
      checked={shortcutsStore.config.shortcuts_enabled}
      onchange={(val) => {
        shortcutsStore.config.shortcuts_enabled = val;
        shortcutsStore.save();
      }}
    />
  </Card>

  <!-- Section 2: Hotkey Bindings List -->
  {#if shortcutsStore.config.shortcuts_enabled}
    <div class="flex flex-col gap-space-sm">
      <SectionHeading
        title="Configured Key Combinations"
        subtitle="Click any field to customize hotkey modifiers and keys"
        icon={Command}
      />
      <div class="grid grid-cols-1 md:grid-cols-2 gap-space-md">
        <!-- Toggle Shortcut -->
        <Card class="flex items-center justify-between">
          <div class="flex items-center gap-space-md min-w-0 pr-2">
            <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-primary-container flex-shrink-0">
              <Command class="w-5 h-5" />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
                Toggle Light / Dark Mode
              </span>
              <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
                Instantly invert the current system appearance
              </span>
            </div>
          </div>
          <input
            type="text"
            bind:value={shortcutsStore.config.toggle_shortcut}
            class="h-8 w-32 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-primary-container text-center font-mono font-semibold text-body-md focus:border-primary outline-none shadow-sm"
          />
        </Card>

        <!-- Force Light Shortcut -->
        <Card class="flex items-center justify-between">
          <div class="flex items-center gap-space-md min-w-0 pr-2">
            <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-secondary flex-shrink-0">
              <Sun class="w-5 h-5" />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
                Force Light Mode
              </span>
              <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
                Explicitly activate Windows Light appearance
              </span>
            </div>
          </div>
          <input
            type="text"
            bind:value={shortcutsStore.config.force_light_shortcut}
            class="h-8 w-32 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-secondary text-center font-mono font-semibold text-body-md focus:border-primary outline-none shadow-sm"
          />
        </Card>

        <!-- Force Dark Shortcut -->
        <Card class="flex items-center justify-between">
          <div class="flex items-center gap-space-md min-w-0 pr-2">
            <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-primary-container flex-shrink-0">
              <Moon class="w-5 h-5" />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
                Force Dark Mode
              </span>
              <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
                Explicitly activate Windows Dark appearance
              </span>
            </div>
          </div>
          <input
            type="text"
            bind:value={shortcutsStore.config.force_dark_shortcut}
            class="h-8 w-32 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-primary-container text-center font-mono font-semibold text-body-md focus:border-primary outline-none shadow-sm"
          />
        </Card>

        <!-- Postpone Transition Shortcut -->
        <Card class="flex items-center justify-between">
          <div class="flex items-center gap-space-md min-w-0 pr-2">
            <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-tertiary flex-shrink-0">
              <Clock class="w-5 h-5" />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
                Postpone Transition (+1 Hour)
              </span>
              <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
                Delay the upcoming automatic theme switch
              </span>
            </div>
          </div>
          <input
            type="text"
            bind:value={shortcutsStore.config.postpone_shortcut}
            class="h-8 w-32 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-tertiary text-center font-mono font-semibold text-body-md focus:border-primary outline-none shadow-sm"
          />
        </Card>
      </div>
    </div>
  {/if}
</div>
