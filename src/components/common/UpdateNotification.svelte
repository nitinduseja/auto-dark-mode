<script lang="ts">
  import {
    Sparkles,
    Download,
    RefreshCw,
    CheckCircle2,
    X,
    ExternalLink,
    AlertCircle,
  } from '@lucide/svelte';
  import { automationStore } from '../../stores/automation.svelte';
  import { openExternalUrl } from '../../services/tauri';

  function formatBytes(bytes: number): string {
    if (!bytes || bytes <= 0) return '0 MB';
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(1)} MB`;
  }
</script>

{#if automationStore.showUpdateNotification && automationStore.updateResult?.status === 'update-available'}
  <div
    class="fixed bottom-6 right-6 z-50 w-[380px] max-w-[calc(100vw-3rem)] rounded-2xl bg-surface-container-high/95 dark:bg-surface-container-high/90 backdrop-blur-xl border border-[var(--card-border)] shadow-2xl p-space-md flex flex-col gap-3 transition-all duration-300 animate-in fade-in slide-in-from-bottom-4 select-none"
    role="alert"
    aria-live="polite"
  >
    <!-- Header Row -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div
          class="w-7 h-7 rounded-lg bg-primary/20 text-primary flex items-center justify-center flex-shrink-0"
        >
          <Sparkles class="w-4 h-4" />
        </div>
        <div class="flex items-center gap-2">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Update Available
          </span>
          <span
            class="px-1.5 py-0.5 rounded text-[10px] font-mono font-semibold bg-primary text-on-primary"
          >
            {automationStore.updateResult.latestVersion}
          </span>
        </div>
      </div>
      {#if !automationStore.isDownloadingUpdate && !automationStore.isRestarting}
        <button
          type="button"
          onclick={() => automationStore.dismissUpdateNotification()}
          title="Dismiss update notification"
          class="w-6 h-6 rounded-md hover:bg-surface-container-highest text-on-surface-variant hover:text-on-surface flex items-center justify-center transition-colors cursor-pointer"
        >
          <X class="w-3.5 h-3.5" />
        </button>
      {/if}
    </div>

    <!-- Error State -->
    {#if automationStore.updateErrorMessage}
      <div
        class="p-2.5 rounded-lg bg-error/15 border border-error/30 text-error text-caption-sm flex items-start gap-2"
      >
        <AlertCircle class="w-4 h-4 flex-shrink-0 mt-0.5" />
        <span class="break-words">{automationStore.updateErrorMessage}</span>
      </div>
    {/if}

    <!-- Content / Progress View -->
    {#if automationStore.isRestarting}
      <!-- Restarting / Finalizing State -->
      <div class="flex flex-col gap-2 py-1">
        <div class="flex items-center gap-2 text-tertiary">
          <CheckCircle2 class="w-5 h-5 flex-shrink-0 animate-pulse" />
          <span class="font-label-md text-label-md font-semibold text-on-surface">
            Update downloaded successfully!
          </span>
        </div>
        <div class="flex items-center gap-2 text-on-surface-variant text-caption-sm">
          <RefreshCw class="w-3.5 h-3.5 animate-spin" />
          <span>Replacing executable and restarting application...</span>
        </div>
      </div>
    {:else if automationStore.isDownloadingUpdate}
      <!-- Active In-App Downloading State -->
      <div class="flex flex-col gap-2 py-1">
        <div class="flex items-center justify-between text-caption-sm">
          <span class="text-on-surface-variant font-medium flex items-center gap-1.5">
            <Download class="w-3.5 h-3.5 animate-bounce" />
            Downloading update package...
          </span>
          <span class="font-mono font-bold text-primary">
            {automationStore.downloadProgress.percentage >= 0
              ? `${automationStore.downloadProgress.percentage}%`
              : 'Downloading...'}
          </span>
        </div>

        <!-- Smooth Progress Track -->
        <div class="w-full h-2 rounded-full bg-surface-container-lowest overflow-hidden">
          <div
            class="h-full bg-gradient-to-r from-primary to-tertiary transition-all duration-300 rounded-full shadow-[0_0_8px_rgba(0,103,192,0.6)]"
            style="width: {Math.max(0, Math.min(100, automationStore.downloadProgress.percentage))}%"
          ></div>
        </div>

        <!-- Download Size Metrics -->
        <div class="flex items-center justify-between text-[11px] text-outline font-mono">
          <span>
            {formatBytes(automationStore.downloadProgress.downloaded_bytes)}
            {#if automationStore.downloadProgress.total_bytes > 0}
              / {formatBytes(automationStore.downloadProgress.total_bytes)}
            {/if}
          </span>
          <span>In-app background download</span>
        </div>
      </div>
    {:else}
      <!-- Idle State: Prompt to Update -->
      <p class="font-caption-sm text-caption-sm text-on-surface-variant leading-relaxed">
        AutoDarkMode {automationStore.updateResult.latestVersion} is available. Download and replace directly in-app without leaving the application.
      </p>

      <div class="flex items-center justify-between gap-2 pt-1 border-t border-[var(--card-border)]">
        {#if automationStore.updateResult.releaseUrl}
          <button
            type="button"
            onclick={() => openExternalUrl(automationStore.updateResult!.releaseUrl!)}
            class="h-7 px-2.5 rounded text-caption-sm text-on-surface-variant hover:text-on-surface hover:bg-surface-container-highest flex items-center gap-1 transition-colors cursor-pointer"
          >
            <span>Notes</span>
            <ExternalLink class="w-3 h-3" />
          </button>
        {:else}
          <div></div>
        {/if}

        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={() => automationStore.dismissUpdateNotification()}
            class="h-7 px-3 rounded text-caption-sm text-on-surface-variant hover:text-on-surface hover:bg-surface-container-highest transition-colors cursor-pointer"
          >
            Later
          </button>
          <button
            type="button"
            onclick={() => automationStore.startUpdateDownload()}
            class="h-7 px-3.5 rounded bg-primary text-on-primary hover:bg-primary/90 text-caption-sm font-medium flex items-center gap-1.5 transition-colors shadow-sm cursor-pointer"
          >
            <Download class="w-3.5 h-3.5" />
            <span>Update Now</span>
          </button>
        </div>
      </div>
    {/if}
  </div>
{/if}
