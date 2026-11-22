<script lang="ts">
  import { onMount } from 'svelte';
  import {
    Palette,
    Image,
    MousePointer,
    Volume2,
    Check,
    AlertCircle,
    Eye,
    Sparkles,
    Monitor,
    RefreshCw,
    FolderOpen,
    Keyboard,
    AppWindow,
    Paintbrush,
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { personalizationStore } from '../../stores/personalization.svelte';
  import { themeSelectionStore } from '../../stores/themeSelection.svelte';
  import type { WallpaperFit } from '../../types/personalization';

  const lightSolidPresets = ['#FFFFFF', '#F3F3F3', '#E5E5E5', '#DDE3EA', '#E0E6ED', '#F5F0E6'];
  const darkSolidPresets = ['#000000', '#1E1E1E', '#121212', '#1A202C', '#181E24', '#2D3748'];

  onMount(() => {
    personalizationStore.init();
    themeSelectionStore.init();
  });

  const fitOptions: { value: WallpaperFit; label: string }[] = [
    { value: 'fill', label: 'Fill Screen' },
    { value: 'fit', label: 'Fit to Aspect' },
    { value: 'stretch', label: 'Stretch' },
    { value: 'center', label: 'Center' },
    { value: 'tile', label: 'Tile' },
    { value: 'span', label: 'Span Across Monitors' },
  ];

  const presetAccentColors = [
    '#60CDFF',
    '#0078D4',
    '#93F482',
    '#FFB900',
    '#FF60A8',
    '#A460FF',
    '#20B2AA',
    '#FF8C00',
  ];

  function getRelativeLuminance(hex: string): number {
    const clean = hex.replace('#', '');
    if (clean.length !== 6) return 0.5;
    const r = parseInt(clean.substring(0, 2), 16) / 255;
    const g = parseInt(clean.substring(2, 4), 16) / 255;
    const b = parseInt(clean.substring(4, 6), 16) / 255;

    const toLinear = (c: number) => (c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4));
    return 0.2126 * toLinear(r) + 0.7152 * toLinear(g) + 0.0722 * toLinear(b);
  }

  function getContrastValidation(hex: string, forDark: boolean): { ratio: number; label: string; ok: boolean } {
    const lum = getRelativeLuminance(hex);
    const bgLum = forDark ? 0.05 : 0.95;
    const l1 = Math.max(lum, bgLum);
    const l2 = Math.min(lum, bgLum);
    const ratio = (l1 + 0.05) / (l2 + 0.05);

    const rounded = Math.round(ratio * 10) / 10;
    if (ratio >= 3.0) {
      return { ratio: rounded, label: `Optimal Contrast (${rounded}:1)`, ok: true };
    } else {
      return { ratio: rounded, label: `Low Contrast (${rounded}:1)`, ok: false };
    }
  }

  let lightCheck = $derived(getContrastValidation(personalizationStore.config.light_accent_color, false));
  let darkCheck = $derived(getContrastValidation(personalizationStore.config.dark_accent_color, true));
</script>

<div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto select-none">
  <!-- Top Section Title Area -->
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-0.5">
      <h1 class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold">
        Personalization
      </h1>
      <p class="font-body-md text-body-md text-on-surface-variant">
        Select installed Windows themes, desktop wallpapers, accent colors, mouse cursors, and system sounds.
      </p>
    </div>
    <div class="flex items-center gap-space-sm">
      <Button
        variant="primary"
        size="md"
        loading={personalizationStore.isSaving || themeSelectionStore.isSaving}
        onclick={() => {
          personalizationStore.save();
          themeSelectionStore.save();
        }}
      >
        Save Changes
      </Button>
    </div>
  </div>

  {#if personalizationStore.successMessage || themeSelectionStore.successMessage}
    <div
      class="p-space-sm px-space-md rounded bg-tertiary/15 border border-tertiary/30 text-tertiary font-label-md text-label-md flex items-center gap-2"
    >
      <Check class="w-4 h-4" />
      <span>{personalizationStore.successMessage || themeSelectionStore.successMessage}</span>
    </div>
  {/if}

  {#if personalizationStore.errorMessage || themeSelectionStore.errorMessage}
    <div
      class="p-space-sm px-space-md rounded bg-error/15 border border-error/30 text-error font-label-md text-label-md flex items-center gap-2"
    >
      <AlertCircle class="w-4 h-4" />
      <span>{personalizationStore.errorMessage || themeSelectionStore.errorMessage}</span>
    </div>
  {/if}

  <!-- Section 1: Windows Installed Themes Selection -->
  <div class="flex flex-col gap-space-sm">
    <div class="flex items-center justify-between">
      <SectionHeading
        title="Installed Windows Themes (.theme switch)"
        subtitle="Automatically apply your preferred installed Windows theme for Light Mode and Dark Mode"
        icon={Monitor}
      />
      <Button
        variant="ghost"
        size="sm"
        icon={RefreshCw}
        loading={themeSelectionStore.isLoading}
        onclick={() => themeSelectionStore.init()}
      >
        Rescan Themes
      </Button>
    </div>
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Apply Windows .theme Files Automatically
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            When enabled, AutoDarkMode switches between your chosen Windows theme profiles, loading all their associated wallpapers, color palettes, and visual styles.
          </span>
        </div>
        <Toggle
          checked={themeSelectionStore.useThemeFiles}
          onchange={(val) => themeSelectionStore.toggleUseThemeFiles(val)}
        />
      </div>

      {#if themeSelectionStore.useThemeFiles}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-space-lg pt-space-xs border-t border-[var(--card-border)]">
          <!-- Light Mode Theme Selector -->
          <div class="flex flex-col gap-space-xs bg-surface-container-low p-space-md rounded">
            <div class="flex items-center justify-between">
              <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                <span class="w-2 h-2 rounded-full bg-secondary"></span>
                Light Mode Windows Theme
              </span>
              <Button
                variant="ghost"
                size="sm"
                icon={Eye}
                loading={themeSelectionStore.isTesting}
                onclick={() =>
                  themeSelectionStore.testTheme(
                    themeSelectionStore.lightThemePath,
                    themeSelectionStore.lightThemeName
                  )}
              >
                Apply Test
              </Button>
            </div>
            <label for="light-theme-select" class="sr-only">Select Light Mode Windows Theme</label>
            <select
              id="light-theme-select"
              bind:value={themeSelectionStore.lightThemePath}
              onchange={(e) => themeSelectionStore.setLightTheme(e.currentTarget.value)}
              class="h-9 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none cursor-pointer"
            >
              {#each themeSelectionStore.installedThemes as theme}
                <option value={theme.path}>
                  {theme.name} {theme.is_system ? '(Windows System)' : '(Custom Theme)'}
                </option>
              {/each}
            </select>
            <span class="font-caption-sm text-caption-sm text-outline truncate" title={themeSelectionStore.lightThemePath}>
              Path: {themeSelectionStore.lightThemePath}
            </span>
          </div>

          <!-- Dark Mode Theme Selector -->
          <div class="flex flex-col gap-space-xs bg-surface-container-low p-space-md rounded">
            <div class="flex items-center justify-between">
              <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                <span class="w-2 h-2 rounded-full bg-primary-container"></span>
                Dark Mode Windows Theme
              </span>
              <Button
                variant="ghost"
                size="sm"
                icon={Eye}
                loading={themeSelectionStore.isTesting}
                onclick={() =>
                  themeSelectionStore.testTheme(
                    themeSelectionStore.darkThemePath,
                    themeSelectionStore.darkThemeName
                  )}
              >
                Apply Test
              </Button>
            </div>
            <label for="dark-theme-select" class="sr-only">Select Dark Mode Windows Theme</label>
            <select
              id="dark-theme-select"
              bind:value={themeSelectionStore.darkThemePath}
              onchange={(e) => themeSelectionStore.setDarkTheme(e.currentTarget.value)}
              class="h-9 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none cursor-pointer"
            >
              {#each themeSelectionStore.installedThemes as theme}
                <option value={theme.path}>
                  {theme.name} {theme.is_system ? '(Windows System)' : '(Custom Theme)'}
                </option>
              {/each}
            </select>
            <span class="font-caption-sm text-caption-sm text-outline truncate" title={themeSelectionStore.darkThemePath}>
              Path: {themeSelectionStore.darkThemePath}
            </span>
          </div>
        </div>
      {/if}
    </Card>
  </div>

  <!-- Section 2: Desktop Wallpaper Switching -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Desktop Wallpaper Automation"
      subtitle="Automatically swap your Windows desktop background image between light and dark variants"
      icon={Image}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Enable Desktop Wallpaper Switching
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Updates your Windows desktop wallpaper whenever the active theme mode transitions.
          </span>
        </div>
        <Toggle
          checked={personalizationStore.config.wallpaper_enabled}
          onchange={(val) => {
            personalizationStore.config.wallpaper_enabled = val;
            personalizationStore.save();
          }}
        />
      </div>

      {#if personalizationStore.config.wallpaper_enabled}
        <!-- Wallpaper Source Type Selector -->
        <div class="flex flex-col gap-space-xs pt-space-xs border-t border-[var(--card-border)]">
          <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase tracking-wider">
            Wallpaper Source Type
          </span>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-space-sm">
            <button
              type="button"
              onclick={() => personalizationStore.setWallpaperMode('picture')}
              class="flex items-center gap-2.5 p-space-sm px-space-md rounded border text-left transition-all {personalizationStore.config.wallpaper_mode === 'picture' || !personalizationStore.config.wallpaper_mode
                ? 'bg-primary/10 border-primary text-primary font-semibold shadow-sm'
                : 'bg-surface-container-low border-[var(--card-border)] text-on-surface hover:bg-surface-container'}"
            >
              <Image class="w-4 h-4 flex-shrink-0" />
              <div class="flex flex-col">
                <span class="text-body-sm font-medium">Image Files</span>
                <span class="text-[11px] text-outline">Custom pictures or multi-monitor</span>
              </div>
            </button>

            <button
              type="button"
              onclick={() => personalizationStore.setWallpaperMode('solid_color')}
              class="flex items-center gap-2.5 p-space-sm px-space-md rounded border text-left transition-all {personalizationStore.config.wallpaper_mode === 'solid_color'
                ? 'bg-primary/10 border-primary text-primary font-semibold shadow-sm'
                : 'bg-surface-container-low border-[var(--card-border)] text-on-surface hover:bg-surface-container'}"
            >
              <Paintbrush class="w-4 h-4 flex-shrink-0" />
              <div class="flex flex-col">
                <span class="text-body-sm font-medium">Solid Color</span>
                <span class="text-[11px] text-outline">Distinct Light & Dark colors</span>
              </div>
            </button>

            <button
              type="button"
              onclick={() => personalizationStore.setWallpaperMode('spotlight')}
              class="flex items-center gap-2.5 p-space-sm px-space-md rounded border text-left transition-all {personalizationStore.config.wallpaper_mode === 'spotlight'
                ? 'bg-primary/10 border-primary text-primary font-semibold shadow-sm'
                : 'bg-surface-container-low border-[var(--card-border)] text-on-surface hover:bg-surface-container'}"
            >
              <Sparkles class="w-4 h-4 flex-shrink-0" />
              <div class="flex flex-col">
                <span class="text-body-sm font-medium">Windows Spotlight</span>
                <span class="text-[11px] text-outline">Curated Microsoft daily photography</span>
              </div>
            </button>
          </div>
        </div>

        <!-- Mode 1: Solid Color Configuration -->
        {#if personalizationStore.config.wallpaper_mode === 'solid_color'}
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-space-lg pt-space-xs border-t border-[var(--card-border)]">
            <!-- Light Mode Solid Color -->
            <div class="flex flex-col gap-space-xs bg-surface-container-low p-space-md rounded border border-[var(--card-border)]">
              <div class="flex items-center justify-between">
                <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                  <span class="w-2.5 h-2.5 rounded-full bg-secondary"></span>
                  Light Mode Solid Background
                </span>
                <Button
                  variant="ghost"
                  size="sm"
                  icon={Eye}
                  loading={personalizationStore.isTestingWallpaper}
                  onclick={() => personalizationStore.testSolidColor('light', personalizationStore.config.light_solid_color)}
                >
                  Apply Test
                </Button>
              </div>

              <div class="flex items-center gap-space-sm pt-1">
                <div
                  class="w-10 h-10 rounded border border-[var(--card-border)] shadow-sm flex-shrink-0"
                  style="background-color: {personalizationStore.config.light_solid_color};"
                ></div>
                <input
                  type="color"
                  bind:value={personalizationStore.config.light_solid_color}
                  onchange={() => personalizationStore.save()}
                  class="w-10 h-10 p-0 rounded border-0 cursor-pointer bg-transparent"
                  title="Choose Light Mode Color"
                />
                <input
                  type="text"
                  bind:value={personalizationStore.config.light_solid_color}
                  onchange={() => personalizationStore.save()}
                  placeholder="#F3F3F3"
                  class="flex-1 h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono uppercase"
                />
              </div>

              <!-- Quick Presets -->
              <div class="flex items-center gap-1.5 pt-2 flex-wrap">
                <span class="text-[11px] text-outline mr-1">Presets:</span>
                {#each lightSolidPresets as color}
                  <button
                    type="button"
                    onclick={() => {
                      personalizationStore.config.light_solid_color = color;
                      personalizationStore.save();
                    }}
                    class="w-6 h-6 rounded border border-[var(--card-border)] hover:scale-110 transition-transform cursor-pointer shadow-xs"
                    style="background-color: {color};"
                    title={color}
                  ></button>
                {/each}
              </div>
            </div>

            <!-- Dark Mode Solid Color -->
            <div class="flex flex-col gap-space-xs bg-surface-container-low p-space-md rounded border border-[var(--card-border)]">
              <div class="flex items-center justify-between">
                <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                  <span class="w-2.5 h-2.5 rounded-full bg-primary-container"></span>
                  Dark Mode Solid Background
                </span>
                <Button
                  variant="ghost"
                  size="sm"
                  icon={Eye}
                  loading={personalizationStore.isTestingWallpaper}
                  onclick={() => personalizationStore.testSolidColor('dark', personalizationStore.config.dark_solid_color)}
                >
                  Apply Test
                </Button>
              </div>

              <div class="flex items-center gap-space-sm pt-1">
                <div
                  class="w-10 h-10 rounded border border-[var(--card-border)] shadow-sm flex-shrink-0"
                  style="background-color: {personalizationStore.config.dark_solid_color};"
                ></div>
                <input
                  type="color"
                  bind:value={personalizationStore.config.dark_solid_color}
                  onchange={() => personalizationStore.save()}
                  class="w-10 h-10 p-0 rounded border-0 cursor-pointer bg-transparent"
                  title="Choose Dark Mode Color"
                />
                <input
                  type="text"
                  bind:value={personalizationStore.config.dark_solid_color}
                  onchange={() => personalizationStore.save()}
                  placeholder="#1E1E1E"
                  class="flex-1 h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono uppercase"
                />
              </div>

              <!-- Quick Presets -->
              <div class="flex items-center gap-1.5 pt-2 flex-wrap">
                <span class="text-[11px] text-outline mr-1">Presets:</span>
                {#each darkSolidPresets as color}
                  <button
                    type="button"
                    onclick={() => {
                      personalizationStore.config.dark_solid_color = color;
                      personalizationStore.save();
                    }}
                    class="w-6 h-6 rounded border border-[var(--card-border)] hover:scale-110 transition-transform cursor-pointer shadow-xs"
                    style="background-color: {color};"
                    title={color}
                  ></button>
                {/each}
              </div>
            </div>
          </div>

        <!-- Mode 2: Windows Spotlight Configuration -->
        {:else if personalizationStore.config.wallpaper_mode === 'spotlight'}
          <div class="flex flex-col gap-space-md pt-space-xs border-t border-[var(--card-border)]">
            <div class="p-space-md rounded-lg bg-surface-container-high border border-[var(--card-border)] flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div class="flex flex-col gap-1 max-w-xl">
                <div class="flex items-center gap-2">
                  <Sparkles class="w-4 h-4 text-tertiary" />
                  <span class="font-label-md text-label-md text-on-surface font-semibold">
                    Windows Spotlight Daily Synchronization
                  </span>
                </div>
                <p class="font-caption-sm text-caption-sm text-on-surface-variant">
                  AutoDarkMode continuously harvests the highest resolution landscape wallpapers provided by Microsoft Windows Spotlight on your machine, setting them dynamically.
                </p>
              </div>

              <div class="flex items-center gap-2">
                <Button
                  variant="ghost"
                  size="sm"
                  icon={RefreshCw}
                  loading={personalizationStore.isLoadingSpotlight}
                  onclick={() => personalizationStore.loadSpotlightImages()}
                >
                  Rescan Cache
                </Button>
                <Button
                  variant="secondary"
                  size="sm"
                  icon={Eye}
                  loading={personalizationStore.isTestingWallpaper}
                  onclick={() => personalizationStore.applySpotlight()}
                >
                  Apply Latest
                </Button>
              </div>
            </div>

            <!-- Spotlight Gallery Preview -->
            {#if personalizationStore.spotlightImages.length > 0}
              <div class="flex flex-col gap-space-xs">
                <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase tracking-wider">
                  Discovered Windows Spotlight Desktop Wallpapers ({personalizationStore.spotlightImages.length})
                </span>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-space-sm pt-1">
                  {#each personalizationStore.spotlightImages as spot}
                    <div class="flex flex-col gap-1 p-2.5 rounded bg-surface-container-low border border-[var(--card-border)]">
                      <div class="flex items-center justify-between text-xs">
                        <span class="font-mono text-outline font-medium">{spot.width}×{spot.height}</span>
                        <span class="text-tertiary text-[11px] font-semibold">Landscape</span>
                      </div>
                      <span class="font-mono text-[11px] text-on-surface truncate" title={spot.cached_path}>
                        {spot.file_name}
                      </span>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          </div>

        <!-- Mode 3: Image Files (with Multi-Monitor option) -->
        {:else}
          <!-- Multi-Monitor Option -->
          <div class="flex items-center justify-between pt-space-xs border-t border-[var(--card-border)]">
            <div class="flex flex-col gap-0.5 max-w-[70%]">
              <span class="font-label-md text-label-md text-on-surface font-semibold flex items-center gap-2">
                <Monitor class="w-4 h-4 text-primary" />
                Multi-Monitor Individual Wallpapers
              </span>
              <span class="font-caption-sm text-caption-sm text-on-surface-variant">
                Configure different light and dark wallpapers for each connected display independently.
              </span>
            </div>
            <Toggle
              checked={personalizationStore.config.multi_monitor_enabled}
              onchange={(val) => {
                personalizationStore.config.multi_monitor_enabled = val;
                personalizationStore.save();
              }}
            />
          </div>

          {#if personalizationStore.config.multi_monitor_enabled}
            <div class="flex flex-col gap-space-md pt-space-xs border-t border-[var(--card-border)]">
              <div class="flex items-center justify-between">
                <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase tracking-wider">
                  Detected Displays ({personalizationStore.connectedMonitors.length})
                </span>
                <Button
                  variant="ghost"
                  size="sm"
                  icon={RefreshCw}
                  loading={personalizationStore.isDetectingMonitors}
                  onclick={() => personalizationStore.loadMonitors()}
                >
                  Rescan Displays
                </Button>
              </div>

              {#if personalizationStore.connectedMonitors.length === 0}
                <div class="p-4 rounded bg-surface-container-low text-on-surface-variant text-body-sm text-center">
                  No displays detected. Click "Rescan Displays" to scan for attached monitors.
                </div>
              {:else}
                <div class="flex flex-col gap-space-md">
                  {#each personalizationStore.connectedMonitors as monitor (monitor.id)}
                    {@const monConfig = personalizationStore.getMonitorConfig(monitor.id)}
                    <div class="flex flex-col gap-space-sm bg-surface-container-low p-space-md rounded border border-[var(--card-border)]">
                      <div class="flex items-center justify-between">
                        <div class="flex items-center gap-2">
                          <Monitor class="w-4 h-4 text-primary" />
                          <span class="font-label-md text-label-md text-on-surface font-medium">
                            {monitor.name}
                          </span>
                          <span class="text-xs px-2 py-0.5 rounded-full bg-surface-container border border-[var(--card-border)] text-outline font-mono">
                            {monitor.width}×{monitor.height}
                          </span>
                          {#if monitor.is_primary}
                            <span class="text-xs px-2 py-0.5 rounded-full bg-primary/10 border border-primary/20 text-primary font-medium">
                              Primary Display
                            </span>
                          {/if}
                        </div>
                      </div>

                      <div class="grid grid-cols-1 lg:grid-cols-2 gap-space-md pt-space-xs">
                        <!-- Monitor Light Mode Wallpaper -->
                        <div class="flex flex-col gap-space-xs bg-surface-container-lowest p-space-sm rounded border border-[var(--card-border)]">
                          <div class="flex items-center justify-between">
                            <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                              <span class="w-2 h-2 rounded-full bg-secondary"></span>
                              Light Wallpaper
                            </span>
                            <Button
                              variant="ghost"
                              size="sm"
                              icon={Eye}
                              loading={personalizationStore.isTestingWallpaper}
                              onclick={() => personalizationStore.testWallpaper('light', monitor.id)}
                            >
                              Apply Test
                            </Button>
                          </div>
                          <div class="flex items-center gap-space-xs">
                            <input
                              type="text"
                              placeholder="Light wallpaper path..."
                              bind:value={monConfig.light_wallpaper_path}
                              class="flex-1 h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono"
                            />
                            <Button
                              variant="secondary"
                              size="sm"
                              icon={FolderOpen}
                              onclick={() => personalizationStore.browseMonitorWallpaper(monitor.id, 'light')}
                            >
                              Browse...
                            </Button>
                          </div>
                        </div>

                        <!-- Monitor Dark Mode Wallpaper -->
                        <div class="flex flex-col gap-space-xs bg-surface-container-lowest p-space-sm rounded border border-[var(--card-border)]">
                          <div class="flex items-center justify-between">
                            <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                              <span class="w-2 h-2 rounded-full bg-primary-container"></span>
                              Dark Wallpaper
                            </span>
                            <Button
                              variant="ghost"
                              size="sm"
                              icon={Eye}
                              loading={personalizationStore.isTestingWallpaper}
                              onclick={() => personalizationStore.testWallpaper('dark', monitor.id)}
                            >
                              Apply Test
                            </Button>
                          </div>
                          <div class="flex items-center gap-space-xs">
                            <input
                              type="text"
                              placeholder="Dark wallpaper path..."
                              bind:value={monConfig.dark_wallpaper_path}
                              class="flex-1 h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono"
                            />
                            <Button
                              variant="secondary"
                              size="sm"
                              icon={FolderOpen}
                              onclick={() => personalizationStore.browseMonitorWallpaper(monitor.id, 'dark')}
                            >
                              Browse...
                            </Button>
                          </div>
                        </div>
                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          {:else}
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-space-lg pt-space-xs border-t border-[var(--card-border)]">
              <!-- Light Wallpaper -->
              <div class="flex flex-col gap-space-xs bg-surface-container-low p-space-md rounded">
                <div class="flex items-center justify-between">
                  <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                    <span class="w-2 h-2 rounded-full bg-secondary"></span>
                    Light Mode Wallpaper
                  </span>
                  <Button
                    variant="ghost"
                    size="sm"
                    icon={Eye}
                    loading={personalizationStore.isTestingWallpaper}
                    onclick={() => personalizationStore.testWallpaper('light')}
                  >
                    Apply Test
                  </Button>
                </div>
                <div class="flex items-center gap-space-xs">
                  <label for="light-wallpaper-input" class="sr-only">Light Mode Wallpaper Path</label>
                  <input
                    id="light-wallpaper-input"
                    type="text"
                    placeholder="C:\Users\Name\Pictures\wallpaper-light.jpg"
                    bind:value={personalizationStore.config.light_wallpaper_path}
                    class="flex-1 h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono"
                  />
                  <Button
                    variant="secondary"
                    size="sm"
                    icon={FolderOpen}
                    onclick={() => personalizationStore.browseWallpaper('light')}
                  >
                    Browse...
                  </Button>
                </div>
                <span class="font-caption-sm text-caption-sm text-outline">
                  Specify or browse image file (.jpg, .png, .jpeg, .bmp, .webp).
                </span>
              </div>

              <!-- Dark Wallpaper -->
              <div class="flex flex-col gap-space-xs bg-surface-container-low p-space-md rounded">
                <div class="flex items-center justify-between">
                  <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase flex items-center gap-1.5">
                    <span class="w-2 h-2 rounded-full bg-primary-container"></span>
                    Dark Mode Wallpaper
                  </span>
                  <Button
                    variant="ghost"
                    size="sm"
                    icon={Eye}
                    loading={personalizationStore.isTestingWallpaper}
                    onclick={() => personalizationStore.testWallpaper('dark')}
                  >
                    Apply Test
                  </Button>
                </div>
                <div class="flex items-center gap-space-xs">
                  <label for="dark-wallpaper-input" class="sr-only">Dark Mode Wallpaper Path</label>
                  <input
                    id="dark-wallpaper-input"
                    type="text"
                    placeholder="C:\Users\Name\Pictures\wallpaper-dark.jpg"
                    bind:value={personalizationStore.config.dark_wallpaper_path}
                    class="flex-1 h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono"
                  />
                  <Button
                    variant="secondary"
                    size="sm"
                    icon={FolderOpen}
                    onclick={() => personalizationStore.browseWallpaper('dark')}
                  >
                    Browse...
                  </Button>
                </div>
                <span class="font-caption-sm text-caption-sm text-outline">
                  Specify or browse image file (.jpg, .png, .jpeg, .bmp, .webp).
                </span>
              </div>
            </div>
          {/if}

          <div class="flex flex-col gap-space-xs pt-space-xs border-t border-[var(--card-border)]">
            <label for="wallpaper-fit-select" class="font-caption-sm text-caption-sm text-outline font-semibold uppercase">
              Wallpaper Position & Fit Style
            </label>
            <select
              id="wallpaper-fit-select"
              bind:value={personalizationStore.config.wallpaper_fit}
              class="h-8 max-w-xs px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none cursor-pointer"
            >
              {#each fitOptions as opt}
                <option value={opt.value}>{opt.label}</option>
              {/each}
            </select>
          </div>
        {/if}
      {/if}
    </Card>
  </div>

  <!-- Section 3: Windows Accent Color Synchronization -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Accent Color Customization"
      subtitle="Shift Windows accent highlights to complement the current theme"
      icon={Palette}
    />
    <Card class="flex flex-col gap-space-md">
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Synchronize Windows Accent Colors
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Apply distinct accent tint highlights for light mode and dark mode.
          </span>
        </div>
        <Toggle
          checked={personalizationStore.config.sync_accent_color}
          onchange={(val) => {
            personalizationStore.config.sync_accent_color = val;
            personalizationStore.save();
          }}
        />
      </div>

      {#if personalizationStore.config.sync_accent_color}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-space-md pt-space-xs border-t border-[var(--card-border)]">
          <div class="flex flex-col gap-space-xs">
            <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase">
              Light Mode Accent Color
            </span>
            <div class="flex items-center gap-space-sm">
              <input
                type="color"
                bind:value={personalizationStore.config.light_accent_color}
                class="w-8 h-8 rounded border-none cursor-pointer bg-transparent"
              />
              <input
                type="text"
                bind:value={personalizationStore.config.light_accent_color}
                class="h-8 w-28 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface font-mono text-body-md uppercase"
              />
            </div>
            <div class="flex items-center gap-1.5 pt-1">
              {#each presetAccentColors as color}
                <button
                  type="button"
                  aria-label="Select accent color {color}"
                  class="w-5 h-5 rounded-full border border-white/20 transition-transform hover:scale-110"
                  style="background-color: {color};"
                  onclick={() => (personalizationStore.config.light_accent_color = color)}
                ></button>
              {/each}
            </div>
            <!-- Contrast Validation Indicator -->
            <div class="flex items-center gap-1.5 pt-1">
              <span
                class="px-2 py-0.5 rounded text-[11px] font-medium flex items-center gap-1.5 {lightCheck.ok
                  ? 'bg-tertiary/15 text-tertiary border border-tertiary/30'
                  : 'bg-secondary/15 text-secondary border border-secondary/30'}"
              >
                <span class="w-1.5 h-1.5 rounded-full {lightCheck.ok ? 'bg-tertiary' : 'bg-secondary'}"></span>
                {lightCheck.label}
              </span>
            </div>
          </div>

          <div class="flex flex-col gap-space-xs">
            <span class="font-caption-sm text-caption-sm text-outline font-semibold uppercase">
              Dark Mode Accent Color
            </span>
            <div class="flex items-center gap-space-sm">
              <input
                type="color"
                bind:value={personalizationStore.config.dark_accent_color}
                class="w-8 h-8 rounded border-none cursor-pointer bg-transparent"
              />
              <input
                type="text"
                bind:value={personalizationStore.config.dark_accent_color}
                class="h-8 w-28 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface font-mono text-body-md uppercase"
              />
            </div>
            <div class="flex items-center gap-1.5 pt-1">
              {#each presetAccentColors as color}
                <button
                  type="button"
                  aria-label="Select accent color {color}"
                  class="w-5 h-5 rounded-full border border-white/20 transition-transform hover:scale-110"
                  style="background-color: {color};"
                  onclick={() => (personalizationStore.config.dark_accent_color = color)}
                ></button>
              {/each}
            </div>
            <!-- Contrast Validation Indicator -->
            <div class="flex items-center gap-1.5 pt-1">
              <span
                class="px-2 py-0.5 rounded text-[11px] font-medium flex items-center gap-1.5 {darkCheck.ok
                  ? 'bg-tertiary/15 text-tertiary border border-tertiary/30'
                  : 'bg-secondary/15 text-secondary border border-secondary/30'}"
              >
                <span class="w-1.5 h-1.5 rounded-full {darkCheck.ok ? 'bg-tertiary' : 'bg-secondary'}"></span>
                {darkCheck.label}
              </span>
            </div>
          </div>
        </div>
      {/if}

      <div class="flex items-center justify-between pt-space-xs border-t border-[var(--card-border)]">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Show Accent Color on Start and Taskbar
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Paints the Windows 11 Taskbar, Start menu, and notification center surface with your chosen accent color.
          </span>
        </div>
        <Toggle
          checked={personalizationStore.config.colorize_taskbar}
          onchange={(val) => {
            personalizationStore.config.colorize_taskbar = val;
            personalizationStore.save();
          }}
        />
      </div>

      <div class="flex items-center justify-between pt-space-xs border-t border-[var(--card-border)]">
        <div class="flex flex-col gap-0.5 max-w-[70%]">
          <span class="font-label-md text-label-md text-on-surface font-semibold">
            Show Accent Color on Title Bars and Window Borders
          </span>
          <span class="font-caption-sm text-caption-sm text-on-surface-variant">
            Highlights active application window top title bars and borders with your chosen accent color.
          </span>
        </div>
        <Toggle
          checked={personalizationStore.config.colorize_titlebars}
          onchange={(val) => {
            personalizationStore.config.colorize_titlebars = val;
            personalizationStore.save();
          }}
        />
      </div>
    </Card>
  </div>

  <!-- Section 4: Cursor and Audio Schemes -->
  <div class="flex flex-col gap-space-sm">
    <SectionHeading
      title="Pointers & System Audio"
      subtitle="Alternate cursor visibility and sound themes based on lighting conditions"
      icon={Sparkles}
    />
    <div class="grid grid-cols-1 md:grid-cols-2 gap-space-md">
      <Card class="flex items-center justify-between">
        <div class="flex items-center gap-space-md min-w-0 pr-2">
          <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-primary-container flex-shrink-0">
            <MousePointer class="w-5 h-5" />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
              Adaptive Mouse Pointer
            </span>
            <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
              Switch cursor between Windows Light (Black outline) and Dark (Inverted)
            </span>
          </div>
        </div>
        <Toggle
          checked={personalizationStore.config.sync_cursor}
          onchange={(val) => {
            personalizationStore.config.sync_cursor = val;
            personalizationStore.save();
          }}
        />
      </Card>

      <Card class="flex items-center justify-between">
        <div class="flex items-center gap-space-md min-w-0 pr-2">
          <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-secondary flex-shrink-0">
            <Volume2 class="w-5 h-5" />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
              Windows Sound Scheme
            </span>
            <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
              Mute non-essential Windows UI chime sounds in Dark mode
            </span>
          </div>
        </div>
        <Toggle
          checked={personalizationStore.config.sync_sounds}
          onchange={(val) => {
            personalizationStore.config.sync_sounds = val;
            personalizationStore.save();
          }}
        />
      </Card>

      <Card class="flex items-center justify-between">
        <div class="flex items-center gap-space-md min-w-0 pr-2">
          <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-secondary flex-shrink-0">
            <Eye class="w-5 h-5" />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
              Windows Grayscale Color Filter
            </span>
            <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
              Automatically activate native Windows grayscale color filter in Dark Mode
            </span>
          </div>
        </div>
        <Toggle
          checked={personalizationStore.config.sync_color_filter}
          onchange={(val) => {
            personalizationStore.config.sync_color_filter = val;
            personalizationStore.save();
          }}
        />
      </Card>

      <Card class="flex items-center justify-between">
        <div class="flex items-center gap-space-md min-w-0 pr-2">
          <div class="w-9 h-9 rounded bg-surface-container-highest flex items-center justify-center text-secondary flex-shrink-0">
            <Keyboard class="w-5 h-5" />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-label-md text-label-md text-on-surface font-semibold truncate">
              Touch Keyboard Theme
            </span>
            <span class="font-caption-sm text-caption-sm text-on-surface-variant truncate">
              Synchronize Windows 10/11 on-screen touch keyboard appearance with system theme
            </span>
          </div>
        </div>
        <Toggle
          checked={personalizationStore.config.sync_touch_keyboard}
          onchange={(val) => {
            personalizationStore.config.sync_touch_keyboard = val;
            personalizationStore.save();
          }}
        />
      </Card>
    </div>
  </div>
</div>
