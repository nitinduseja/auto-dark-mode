<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Settings,
    FileCode,
    RotateCcw,
    FolderOpen,
    Copy,
    Check,
    AlertCircle,
    Info,
    Laptop,
    Globe,
    Upload,
    Download,
    ExternalLink,
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { settingsStore } from '../../stores/settings.svelte';
  import { openExternalUrl } from '../../services/tauri';

  let showImportModal = $state(false);
  let importJsonText = $state('');
  let showResetModal = $state(false);
  let copyFeedback = $state(false);

  onMount(() => {
    settingsStore.init();
  });

  async function handleExport() {
    try {
      const json = await settingsStore.exportConfig();
      if (typeof navigator !== 'undefined' && navigator.clipboard) {
        await navigator.clipboard.writeText(json);
        copyFeedback = true;
        setTimeout(() => {
          copyFeedback = false;
        }, 2500);
      }
    } catch {
      // Handled in store
    }
  }

  async function handleImportSubmit() {
    if (!importJsonText.trim()) return;
    await settingsStore.importConfig(importJsonText);
    showImportModal = false;
    importJsonText = '';
  }

  async function handleConfirmReset() {
    await settingsStore.resetDefaults();
    showResetModal = false;
  }

  async function handleCopyDiagnostics() {
    if (!settingsStore.diagnostics) return;
    const info = `AutoDarkMode Diagnostics
OS: ${settingsStore.diagnostics.os_name} (Build ${settingsStore.diagnostics.os_build})
Arch: ${settingsStore.diagnostics.arch}
Tauri: ${settingsStore.diagnostics.tauri_version}
WebView2: ${settingsStore.diagnostics.webview_version}
Config Path: ${settingsStore.diagnostics.config_path}`;

    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      await navigator.clipboard.writeText(info);
      copyFeedback = true;
      setTimeout(() => {
        copyFeedback = false;
      }, 2500);
    }
  }
</script>

<div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto select-none">
  <!-- Top Section Title Area -->
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-0.5">
      <h1 class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold">
        Settings
      </h1>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Manage AutoDarkMode preferences, backup configurations, diagnostics, and application details.
      </p>
    </div>
    <div class="flex items-center gap-space-sm">
      <Button
        variant="primary"
        size="md"
        loading={settingsStore.isSaving}
        onclick={() => settingsStore.save()}
      >
        Save Changes
      </Button>
    </div>
  </div>

  {#if settingsStore.successMessage}
    <div
      class="p-space-sm px-space-md rounded bg-tertiary/15 border border-tertiary/30 text-tertiary font-label-md text-label-md flex items-center gap-2"
    >
      <Check class="w-4 h-4" />
      <span>{settingsStore.successMessage}</span>
    </div>
  {/if}

  {#if settingsStore.errorMessage}
    <div
      class="p-space-sm px-space-md rounded bg-error/15 border border-error/30 text-error font-label-md text-label-md flex items-center gap-2"
    >
      <AlertCircle class="w-4 h-4" />
      <span>{settingsStore.errorMessage}</span>
    </div>
  {/if}

  <!-- Section 1: General Preferences -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Application Preferences"
      subtitle="Theme mode and internationalization settings for AutoDarkMode itself"
      icon={Settings}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-space-md">
        <div class="flex flex-col gap-1">
          <label for="app-ui-theme-select" class="font-label-md text-label-md text-on-surface font-semibold">
            AutoDarkMode Interface Theme
          </label>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Choose whether this configuration app window follows the system or uses an override.
          </span>
          <select
            id="app-ui-theme-select"
            bind:value={settingsStore.config.app_ui_theme}
            class="h-8 mt-1 max-w-xs px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none"
            onchange={() => settingsStore.save()}
          >
            <option value="system">Follow Windows System</option>
            <option value="dark">Always Dark Mode</option>
            <option value="light">Always Light Mode</option>
          </select>
        </div>

        <div class="flex flex-col gap-1">
          <label for="app-language-select" class="font-label-md text-label-md text-on-surface font-semibold">
            Language & Region
          </label>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Select application display language.
          </span>
          <select
            id="app-language-select"
            bind:value={settingsStore.config.language}
            class="h-8 mt-1 max-w-xs px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none"
            onchange={() => settingsStore.save()}
          >
            <option value="en-US">English (United States)</option>
            <option value="system">System Default</option>
          </select>
        </div>
      </div>
    </Card>
  </div>

  <!-- Section 2: Backup & Configuration Storage -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Data & Configuration Storage"
      subtitle="Export, import, or reset your schedules and customized rules"
      icon={FileCode}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex flex-wrap items-center justify-between gap-space-md">
        <div class="flex flex-col gap-0.5">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Configuration Management
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            JSON backups contain your active schedule, process rules, and condition preferences.
          </span>
        </div>
        <div class="flex flex-wrap items-center gap-space-sm">
          <Button variant="secondary" size="sm" icon={FolderOpen} onclick={() => settingsStore.openFolder()}>
            Open AppData Folder
          </Button>
          <Button variant="secondary" size="sm" icon={Download} onclick={handleExport}>
            {copyFeedback ? 'Copied to Clipboard!' : 'Export Config'}
          </Button>
          <Button variant="secondary" size="sm" icon={Upload} onclick={() => (showImportModal = true)}>
            Import Config
          </Button>
          <Button variant="ghost" size="sm" icon={RotateCcw} onclick={() => (showResetModal = true)}>
            Reset Defaults
          </Button>
        </div>
      </div>
    </Card>
  </div>

  <!-- Section 3: System Diagnostics -->
  <div class="flex flex-col gap-space-sm">
    <div class="flex items-center justify-between">
      <SectionHeading
        title="Environment & Diagnostics"
        subtitle="Platform runtime and device specifications"
        icon={Laptop}
      />
      <Button variant="ghost" size="sm" icon={Copy} onclick={handleCopyDiagnostics}>
        {copyFeedback ? 'Copied!' : 'Copy Diagnostics'}
      </Button>
    </div>

    {#if settingsStore.diagnostics}
      <div class="grid grid-cols-2 md:grid-cols-4 gap-space-sm">
        <div class="bg-surface-container-low p-space-sm rounded flex flex-col gap-0.5">
          <span class="font-caption-sm text-caption-sm text-outline">Operating System</span>
          <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
            {settingsStore.diagnostics.os_name}
          </span>
          <span class="font-caption-sm text-caption-sm text-outline truncate">
            Build {settingsStore.diagnostics.os_build}
          </span>
        </div>

        <div class="bg-surface-container-low p-space-sm rounded flex flex-col gap-0.5">
          <span class="font-caption-sm text-caption-sm text-outline">Target Architecture</span>
          <span class="font-label-md text-label-md text-on-surface font-semibold truncate uppercase">
            {settingsStore.diagnostics.arch}
          </span>
          <span class="font-caption-sm text-caption-sm text-outline">
            Native Windows binary
          </span>
        </div>

        <div class="bg-surface-container-low p-space-sm rounded flex flex-col gap-0.5">
          <span class="font-caption-sm text-caption-sm text-outline">Tauri Core Runtime</span>
          <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
            v{settingsStore.diagnostics.tauri_version}
          </span>
          <span class="font-caption-sm text-caption-sm text-outline">
            Rust 1.91+ toolchain
          </span>
        </div>

        <div class="bg-surface-container-low p-space-sm rounded flex flex-col gap-0.5">
          <span class="font-caption-sm text-caption-sm text-outline">WebView2 Engine</span>
          <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
            Chromium Evergreen
          </span>
          <span class="font-caption-sm text-caption-sm text-outline truncate">
            {settingsStore.diagnostics.webview_version}
          </span>
        </div>
      </div>
    {/if}
  </div>

  <!-- Section 4: About AutoDarkMode -->
  <Card class="flex items-center justify-between p-space-lg">
    <div class="flex items-center gap-space-md">
      <div class="w-12 h-12 rounded bg-surface-container-high flex items-center justify-center border border-[var(--card-border)] overflow-hidden shadow-sm">
        <img src="/favicon.png" alt="AutoDarkMode" class="w-8 h-8 rounded object-contain" />
      </div>
      <div class="flex flex-col">
        <span class="font-title-sm text-title-sm font-semibold text-on-surface">
          AutoDarkMode for Windows 11
        </span>
        <span class="font-body-md text-body-md text-on-surface-variant">
          Version 1.1.0 • Built with Rust + Tauri 2 + Svelte 5
        </span>
        <span class="font-caption-sm text-caption-sm text-outline pt-0.5">
          Open source desktop application licensed under MIT.
        </span>
      </div>
    </div>
    <div class="flex items-center gap-space-sm">
      <button
        type="button"
        onclick={() => openExternalUrl('https://github.com/nitinduseja/auto-dark-mode')}
        class="h-8 px-space-md rounded bg-surface-container-high hover:bg-surface-variant text-on-surface text-caption-sm flex items-center gap-1.5 transition-colors cursor-pointer"
      >
        <span>GitHub Project</span>
        <ExternalLink class="w-3.5 h-3.5" />
      </button>
    </div>
  </Card>
</div>

<!-- Import Configuration Modal -->
{#if showImportModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
    <div class="w-full max-w-lg bg-surface-container rounded-lg border border-[var(--card-border)] shadow-xl p-space-lg flex flex-col gap-space-md">
      <h3 class="font-title-sm text-title-sm text-on-surface font-semibold">
        Import Configuration JSON
      </h3>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Paste your AutoDarkMode configuration JSON below to restore all settings and rules.
      </p>
      <textarea
        bind:value={importJsonText}
        placeholder="Paste JSON configuration here..."
        rows="8"
        class="w-full p-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface font-mono text-caption-sm focus:border-primary outline-none"
      ></textarea>
      <div class="flex justify-end gap-space-sm">
        <Button variant="ghost" size="sm" onclick={() => (showImportModal = false)}>
          Cancel
        </Button>
        <Button variant="primary" size="sm" onclick={handleImportSubmit}>
          Restore Configuration
        </Button>
      </div>
    </div>
  </div>
{/if}

<!-- Reset Confirmation Modal -->
{#if showResetModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
    <div class="w-full max-w-md bg-surface-container rounded-lg border border-[var(--card-border)] shadow-xl p-space-lg flex flex-col gap-space-md">
      <h3 class="font-title-sm text-title-sm text-on-surface font-semibold flex items-center gap-2 text-error">
        <AlertCircle class="w-5 h-5" />
        Reset to Factory Defaults?
      </h3>
      <p class="font-body-md text-body-md text-on-surface-variant">
        This will wipe all customized sunrise/sunset times, process rules, and conditions, restoring the application to its initial out-of-the-box defaults.
      </p>
      <div class="flex justify-end gap-space-sm pt-2">
        <Button variant="ghost" size="sm" onclick={() => (showResetModal = false)}>
          Cancel
        </Button>
        <Button variant="primary" size="sm" onclick={handleConfirmReset}>
          Confirm Reset
        </Button>
      </div>
    </div>
  </div>
{/if}
