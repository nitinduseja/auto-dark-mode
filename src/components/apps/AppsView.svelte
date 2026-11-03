<script lang="ts">
  import { onMount } from 'svelte';
  import {
    LayoutGrid,
    Laptop,
    Globe,
    FileText,
    Plus,
    Trash2,
    Check,
    AlertCircle,
    Info,
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import SegmentedControl from '../ui/SegmentedControl.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { appsStore } from '../../stores/apps.svelte';
  import type { ThemeMode } from '../../types/theme';

  let newAppName = $state('');
  let newAppProcess = $state('');
  let newAppTheme = $state<'system' | 'dark' | 'light'>('system');
  let isAddingRule = $state(false);

  onMount(() => {
    appsStore.init();
  });

  const themeOptions: { value: ThemeMode; label: string }[] = [
    { value: 'light', label: 'Light' },
    { value: 'dark', label: 'Dark' },
  ];

  async function handleAddRule() {
    if (!newAppName.trim() || !newAppProcess.trim()) return;

    let processName = newAppProcess.trim().toLowerCase();
    if (!processName.endsWith('.exe')) {
      processName += '.exe';
    }

    await appsStore.addRule({
      id: `rule_${Date.now()}`,
      name: newAppName.trim(),
      process_name: processName,
      target_theme: newAppTheme,
      enabled: true,
    });

    newAppName = '';
    newAppProcess = '';
    newAppTheme = 'system';
    isAddingRule = false;
  }
</script>

<div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto select-none">
  <!-- Top Section Title Area -->
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-0.5">
      <h1 class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold">
        App Themes
      </h1>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Customize theme behavior for Windows components, desktop software, and browsers.
      </p>
    </div>
    <div class="flex items-center gap-space-sm">
      <Button
        variant="primary"
        size="md"
        loading={appsStore.isSaving}
        onclick={() => appsStore.save()}
      >
        Save Changes
      </Button>
    </div>
  </div>

  {#if appsStore.successMessage}
    <div
      class="p-space-sm px-space-md rounded bg-tertiary/15 border border-tertiary/30 text-tertiary font-label-md text-label-md flex items-center gap-2"
    >
      <Check class="w-4 h-4" />
      <span>{appsStore.successMessage}</span>
    </div>
  {/if}

  {#if appsStore.errorMessage}
    <div
      class="p-space-sm px-space-md rounded bg-error/15 border border-error/30 text-error font-label-md text-label-md flex items-center gap-2"
    >
      <AlertCircle class="w-4 h-4" />
      <span>{appsStore.errorMessage}</span>
    </div>
  {/if}

  <!-- Section 1: System and Apps Independent Modes -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Windows Subsystem Separation"
      subtitle="Decouple taskbar and system UI from window applications"
      icon={Laptop}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Independent Windows & Apps Themes
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Allow Windows Taskbar/Start Menu to use a different appearance than File Explorer and application windows.
          </span>
        </div>
        <Toggle
          checked={appsStore.config.apps_separate_from_system}
          onchange={(val) => {
            appsStore.config.apps_separate_from_system = val;
            appsStore.save();
          }}
        />
      </div>

      {#if appsStore.config.apps_separate_from_system}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-space-md pt-space-xs border-t border-[var(--card-border)]">
          <div class="flex flex-col gap-space-xs">
            <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase">
              Windows System Surface (Taskbar, Shell)
            </span>
            <SegmentedControl
              options={themeOptions}
              value={appsStore.config.system_theme}
              onchange={(val) => {
                appsStore.config.system_theme = val as ThemeMode;
                appsStore.save();
              }}
            />
          </div>
          <div class="flex flex-col gap-space-xs">
            <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase">
              App Windows & Native Controls
            </span>
            <SegmentedControl
              options={themeOptions}
              value={appsStore.config.apps_theme}
              onchange={(val) => {
                appsStore.config.apps_theme = val as ThemeMode;
                appsStore.save();
              }}
            />
          </div>
        </div>
      {/if}
    </Card>
  </div>

  <!-- Section 2: Supported Suites Integration -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Productivity Suites & Web Browsers"
      subtitle="Synchronize theme mode across web browsers and Office applications"
      icon={Globe}
    />
    <div class="grid grid-cols-1 md:grid-cols-2 gap-space-md">
      <Card class="flex items-center justify-between">
        <div class="flex items-center gap-space-md min-w-0 pr-2">
          <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-primary-container flex-shrink-0">
            <FileText class="w-5 h-5" />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
              Microsoft Office Suite
            </span>
            <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
              Synchronize Word, Excel, PowerPoint, and Outlook
            </span>
          </div>
        </div>
        <Toggle
          checked={appsStore.config.sync_office}
          onchange={(val) => {
            appsStore.config.sync_office = val;
            appsStore.save();
          }}
        />
      </Card>

      <Card class="flex items-center justify-between">
        <div class="flex items-center gap-space-md min-w-0 pr-2">
          <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-secondary flex-shrink-0">
            <Globe class="w-5 h-5" />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
              Web Browsers (Edge, Chrome, Firefox)
            </span>
            <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
              Enforce system dark/light mode on web page contents
            </span>
          </div>
        </div>
        <Toggle
          checked={appsStore.config.sync_browsers}
          onchange={(val) => {
            appsStore.config.sync_browsers = val;
            appsStore.save();
          }}
        />
      </Card>
    </div>
  </div>

  <!-- Section 3: Custom Application Rules -->
  <div class="flex flex-col gap-space-sm">
    <div class="flex items-center justify-between">
      <SectionHeading
        title="Custom Application Process Rules"
        subtitle="Force explicit theme behavior on designated executables"
        icon={LayoutGrid}
      />
      <Button
        variant="secondary"
        size="sm"
        icon={Plus}
        onclick={() => (isAddingRule = !isAddingRule)}
      >
        Add Application Rule
      </Button>
    </div>

    {#if isAddingRule}
      <Card class="flex flex-col gap-space-md border border-primary/30 bg-surface-container-high/60">
        <span class="font-label-md text-label-md text-primary font-semibold">
          New Application Theme Override
        </span>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-space-md">
          <div class="flex flex-col gap-1">
            <label for="new-app-name-input" class="font-caption-sm text-caption-sm text-outline">Application Label</label>
            <input
              id="new-app-name-input"
              type="text"
              placeholder="e.g. Visual Studio Code"
              bind:value={newAppName}
              class="h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none"
            />
          </div>
          <div class="flex flex-col gap-1">
            <label for="new-process-name-input" class="font-caption-sm text-caption-sm text-outline">Executable Process (.exe)</label>
            <input
              id="new-process-name-input"
              type="text"
              placeholder="e.g. code.exe"
              bind:value={newAppProcess}
              class="h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono"
            />
          </div>
          <div class="flex flex-col gap-1">
            <label for="new-app-theme-select" class="font-caption-sm text-caption-sm text-outline">Forced Appearance</label>
            <select
              id="new-app-theme-select"
              bind:value={newAppTheme}
              class="h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none"
            >
              <option value="system">Follow Windows System</option>
              <option value="dark">Always Dark Mode</option>
              <option value="light">Always Light Mode</option>
            </select>
          </div>
        </div>
        <div class="flex justify-end gap-space-sm">
          <Button variant="ghost" size="sm" onclick={() => (isAddingRule = false)}>
            Cancel
          </Button>
          <Button variant="primary" size="sm" onclick={handleAddRule}>
            Save Rule
          </Button>
        </div>
      </Card>
    {/if}

    <div class="flex flex-col gap-space-xs">
      {#each appsStore.config.custom_rules as rule (rule.id)}
        <Card class="flex items-center justify-between p-space-sm">
          <div class="flex items-center gap-space-md min-w-0">
            <div class="w-8 h-8 rounded bg-surface-container-highest flex items-center justify-center text-outline">
              <Laptop class="w-4 h-4" />
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
                {rule.name}
              </span>
              <span class="font-caption-sm text-caption-sm text-outline font-mono truncate">
                {rule.process_name} • {rule.target_theme.toUpperCase()}
              </span>
            </div>
          </div>
          <div class="flex items-center gap-space-md">
            <Toggle
              checked={rule.enabled}
              onchange={(val) => appsStore.toggleRule(rule.id, val)}
            />
            <button
              class="w-7 h-7 rounded flex items-center justify-center text-outline hover:text-error hover:bg-error/10 transition-colors"
              title="Delete rule"
              onclick={() => appsStore.removeRule(rule.id)}
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </Card>
      {/each}
    </div>
  </div>
</div>
