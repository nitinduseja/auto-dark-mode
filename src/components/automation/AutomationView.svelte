<script lang="ts">
  import { onMount } from 'svelte';
  import {
    SlidersHorizontal,
    Power,
    Bell,
    Download,
    Cpu,
    Check,
    AlertCircle,
    RefreshCw,
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { automationStore } from '../../stores/automation.svelte';

  onMount(() => {
    automationStore.init();
  });
</script>

<div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto select-none">
  <!-- Top Section Title Area -->
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-0.5">
      <h1 class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold">
        Automation
      </h1>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Configure Windows startup integration, background daemon persistence, and notifications.
      </p>
    </div>
    <div class="flex items-center gap-space-sm">
      <Button
        variant="primary"
        size="md"
        loading={automationStore.isSaving}
        onclick={() => automationStore.save()}
      >
        Save Changes
      </Button>
    </div>
  </div>

  {#if automationStore.successMessage}
    <div
      class="p-space-sm px-space-md rounded bg-tertiary/15 border border-tertiary/30 text-tertiary font-label-md text-label-md flex items-center gap-2"
    >
      <Check class="w-4 h-4" />
      <span>{automationStore.successMessage}</span>
    </div>
  {/if}

  {#if automationStore.errorMessage}
    <div
      class="p-space-sm px-space-md rounded bg-error/15 border border-error/30 text-error font-label-md text-label-md flex items-center gap-2"
    >
      <AlertCircle class="w-4 h-4" />
      <span>{automationStore.errorMessage}</span>
    </div>
  {/if}

  <!-- Section 1: Windows Startup & Tray -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Windows Boot & Lifecycle"
      subtitle="Ensure AutoDarkMode starts automatically with Windows 11"
      icon={Power}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Start with Windows
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Registers AutoDarkMode into HKCU\Software\Microsoft\Windows\CurrentVersion\Run for automatic launch on log-in.
          </span>
        </div>
        <Toggle
          checked={automationStore.config.launch_on_startup}
          onchange={(val) => {
            automationStore.config.launch_on_startup = val;
            automationStore.save();
          }}
        />
      </div>

      <div class="flex items-center justify-between pt-space-xs border-t border-[var(--card-border)]">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Start Minimized in Background
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Silently launch in the background without opening the dashboard window on Windows boot.
          </span>
        </div>
        <Toggle
          checked={automationStore.config.start_minimized}
          onchange={(val) => {
            automationStore.config.start_minimized = val;
            automationStore.save();
          }}
        />
      </div>
    </Card>
  </div>

  <!-- Section 2: Service Architecture -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Background Execution Daemon"
      subtitle="Ultra-low memory resident background scheduler"
      icon={Cpu}
    />
    <Card class="flex items-center justify-between">
      <div class="flex items-center gap-space-md min-w-0 pr-2">
        <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-tertiary flex-shrink-0">
          <Cpu class="w-5 h-5" />
        </div>
        <div class="flex flex-col min-w-0">
          <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
            Asynchronous Rust Service Daemon
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
            Active memory footprint: ~18MB • Evaluates schedule every 2 seconds with zero CPU consumption
          </span>
        </div>
      </div>
      <div class="px-space-sm py-1 rounded bg-tertiary/15 text-tertiary font-label-md text-label-md font-semibold flex items-center gap-1.5">
        <span class="w-1.5 h-1.5 rounded-full bg-tertiary shadow-[0_0_6px_currentColor]"></span>
        <span>Active</span>
      </div>
    </Card>
  </div>

  <!-- Section 3: Windows Toast Notifications -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Windows Notification Center"
      subtitle="Receive subtle alerts when appearance transitions occur"
      icon={Bell}
    />
    <Card class="flex items-center justify-between">
      <div class="flex flex-col gap-0.5 max-w-[70%]">
        <span class="font-label-md text-label-md text-on-surface font-semibold">
          Show Notifications on Theme Transition
        </span>
        <span class="font-caption-sm text-caption-sm text-on-surface-variant">
          Displays a brief Windows toast notification when switching between Light Mode and Dark Mode.
        </span>
      </div>
      <Toggle
        checked={automationStore.config.notify_on_switch}
        onchange={(val) => {
          automationStore.config.notify_on_switch = val;
          automationStore.save();
        }}
      />
    </Card>
  </div>

  <!-- Section 4: Automatic Software Updates -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Software Maintenance & Releases"
      subtitle="Keep AutoDarkMode updated with security and compatibility patches"
      icon={Download}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Automatic Update Verification
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Periodically checks GitHub releases for newer production builds.
          </span>
        </div>
        <Toggle
          checked={automationStore.config.check_updates}
          onchange={(val) => {
            automationStore.config.check_updates = val;
            automationStore.save();
          }}
        />
      </div>

      <div class="flex items-center justify-between pt-space-xs border-t border-[var(--card-border)]">
        <div class="flex flex-col gap-0.5">
          <span class="font-caption-sm text-caption-sm text-outline">
            {automationStore.updateStatus || 'Latest check: Today at startup'}
          </span>
        </div>
        <Button
          variant="secondary"
          size="sm"
          icon={RefreshCw}
          loading={automationStore.isCheckingUpdate}
          onclick={() => automationStore.checkUpdates()}
        >
          Check for Updates
        </Button>
      </div>
    </Card>
  </div>
</div>
