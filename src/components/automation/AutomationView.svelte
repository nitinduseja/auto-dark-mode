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
    ExternalLink,
    Sparkles,
    CheckCircle2,
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { automationStore } from '../../stores/automation.svelte';
  import { openExternalUrl } from '../../services/tauri';

  onMount(() => {
    automationStore.init();
  });

  function formatBytes(bytes: number): string {
    if (!bytes || bytes <= 0) return '0 MB';
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(1)} MB`;
  }
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

      {#if automationStore.updateResult?.status === 'update-available'}
        <div
          class="p-space-md rounded-lg bg-primary/10 border border-primary/30 flex flex-col gap-space-md"
        >
          <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-space-md">
            <div class="flex items-start gap-3">
              <div
                class="w-9 h-9 rounded-md bg-primary/20 text-primary flex items-center justify-center flex-shrink-0 mt-0.5"
              >
                <Sparkles class="w-5 h-5" />
              </div>
              <div class="flex flex-col gap-0.5">
                <span class="font-label-md text-label-md text-on-surface font-semibold">
                  AutoDarkMode {automationStore.updateResult.latestVersion} is available!
                </span>
                <span class="font-caption-sm text-caption-sm text-on-surface-variant">
                  You are currently running v{automationStore.updateResult.currentVersion}. {automationStore.updateResult.releaseName ? `Release: ${automationStore.updateResult.releaseName}` : ''}
                </span>
              </div>
            </div>
            <div class="flex items-center gap-space-sm flex-shrink-0">
              {#if automationStore.updateResult.releaseUrl}
                <button
                  type="button"
                  onclick={() => openExternalUrl(automationStore.updateResult!.releaseUrl!)}
                  class="h-8 px-space-md rounded bg-surface-container-high hover:bg-surface-variant text-on-surface text-caption-sm font-medium flex items-center gap-1.5 transition-colors cursor-pointer border border-[var(--card-border)]"
                >
                  <span>Release Notes</span>
                  <ExternalLink class="w-3.5 h-3.5" />
                </button>
              {/if}
              {#if !automationStore.isDownloadingUpdate && !automationStore.isRestarting}
                <button
                  type="button"
                  onclick={() => automationStore.startUpdateDownload()}
                  class="h-8 px-space-md rounded bg-primary text-on-primary hover:bg-primary/90 text-caption-sm font-medium flex items-center gap-1.5 transition-colors cursor-pointer shadow-sm"
                >
                  <Download class="w-3.5 h-3.5" />
                  <span>Update Now</span>
                </button>
              {/if}
            </div>
          </div>

          <!-- Active Download / Restarting Progress -->
          {#if automationStore.isRestarting}
            <div class="flex items-center gap-2 p-2.5 rounded bg-tertiary/15 text-tertiary font-label-md text-label-md">
              <CheckCircle2 class="w-4 h-4 animate-pulse" />
              <span>Download complete! Replacing files and restarting application...</span>
            </div>
          {:else if automationStore.isDownloadingUpdate}
            <div class="flex flex-col gap-2 p-3 rounded-md bg-surface-container-lowest/80 border border-[var(--card-border)]">
              <div class="flex items-center justify-between text-caption-sm">
                <span class="text-on-surface-variant font-medium flex items-center gap-1.5">
                  <Download class="w-3.5 h-3.5 animate-bounce text-primary" />
                  Downloading update package in-app...
                </span>
                <span class="font-mono font-bold text-primary">
                  {automationStore.downloadProgress.percentage >= 0
                    ? `${automationStore.downloadProgress.percentage}%`
                    : 'Downloading...'}
                </span>
              </div>
              <div class="w-full h-2 rounded-full bg-surface-container-high overflow-hidden">
                <div
                  class="h-full bg-gradient-to-r from-primary to-tertiary transition-all duration-300 rounded-full"
                  style="width: {Math.max(0, Math.min(100, automationStore.downloadProgress.percentage))}%"
                ></div>
              </div>
              <div class="flex items-center justify-between text-[11px] text-outline font-mono">
                <span>
                  {formatBytes(automationStore.downloadProgress.downloaded_bytes)}
                  {#if automationStore.downloadProgress.total_bytes > 0}
                    / {formatBytes(automationStore.downloadProgress.total_bytes)}
                  {/if}
                </span>
                <span>Automatic file replacement & smooth restart upon completion</span>
              </div>
            </div>
          {/if}

          {#if automationStore.updateErrorMessage}
            <div class="p-2.5 rounded bg-error/15 border border-error/30 text-error text-caption-sm flex items-center gap-2">
              <AlertCircle class="w-4 h-4 flex-shrink-0" />
              <span>{automationStore.updateErrorMessage}</span>
            </div>
          {/if}
        </div>
      {/if}

      <div class="flex items-center justify-between pt-space-xs border-t border-[var(--card-border)]">
        <div class="flex flex-col gap-0.5">
          <span class="font-caption-sm text-caption-sm text-outline">
            {automationStore.updateStatus || (automationStore.lastCheckedTime ? `Last checked: Today at ${automationStore.lastCheckedTime}` : 'Latest check: Today at startup')}
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
