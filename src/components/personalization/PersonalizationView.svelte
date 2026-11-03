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
  } from '@lucide/svelte';
  import Card from '../ui/Card.svelte';
  import Button from '../ui/Button.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import SectionHeading from '../ui/SectionHeading.svelte';
  import { personalizationStore } from '../../stores/personalization.svelte';
  import { themeSelectionStore } from '../../stores/themeSelection.svelte';
  import type { WallpaperFit } from '../../types/personalization';

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
            <label for="light-wallpaper-input" class="sr-only">Light Mode Wallpaper Path</label>
            <input
              id="light-wallpaper-input"
              type="text"
              placeholder="C:\Users\Name\Pictures\wallpaper-light.jpg"
              bind:value={personalizationStore.config.light_wallpaper_path}
              class="h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono"
            />
            <span class="font-caption-sm text-caption-sm text-outline">
              Specify full image file path (.jpg, .png, .jpeg, .bmp).
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
            <label for="dark-wallpaper-input" class="sr-only">Dark Mode Wallpaper Path</label>
            <input
              id="dark-wallpaper-input"
              type="text"
              placeholder="C:\Users\Name\Pictures\wallpaper-dark.jpg"
              bind:value={personalizationStore.config.dark_wallpaper_path}
              class="h-8 px-space-sm rounded bg-surface-container-lowest border border-[var(--card-border)] text-on-surface text-body-md focus:border-primary outline-none font-mono"
            />
            <span class="font-caption-sm text-caption-sm text-outline">
              Specify full image file path (.jpg, .png, .jpeg, .bmp).
            </span>
          </div>
        </div>

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
          </div>
        </div>
      {/if}
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
    </div>
  </div>
</div>
