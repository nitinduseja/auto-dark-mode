<script lang="ts">
  import {
    Home,
    Clock,
    LayoutGrid,
    Palette,
    ListChecks,
    SlidersHorizontal,
    Keyboard,
    Settings,
  } from '@lucide/svelte';
  import NavigationItem from '../ui/NavigationItem.svelte';
  import { dashboard } from '../../stores/dashboard.svelte';
  import { APP_VERSION } from '../../services/updater';

  export type AppPage =
    | 'overview'
    | 'schedule'
    | 'apps'
    | 'personalization'
    | 'conditions'
    | 'automation'
    | 'shortcuts'
    | 'settings';

  interface Props {
    currentPage?: AppPage;
    onnavigate?: (page: AppPage) => void;
  }

  let { currentPage = 'overview', onnavigate }: Props = $props();

  let version = $derived(dashboard.state?.app_info.version || `v${APP_VERSION}`);

  function handleNavigate(page: AppPage) {
    if (onnavigate) {
      onnavigate(page);
    }
  }
</script>

<aside
  class="fixed left-0 top-10 bottom-0 w-[250px] bg-surface-container-lowest/80 backdrop-blur-xl flex flex-col justify-between p-space-sm z-40 border-r border-[var(--titlebar-border)] select-none transition-colors"
>
  <div class="flex flex-col gap-space-md">
    <!-- App Logo & Branding Header -->
    <div class="px-space-sm py-space-xs flex items-center gap-space-sm">
      <div
        class="w-7 h-7 rounded bg-surface-container-high flex items-center justify-center border border-[var(--card-border)] shadow-sm overflow-hidden"
      >
        <img src="/favicon.png" alt="AutoDarkMode" class="w-5 h-5 rounded object-contain" />
      </div>
      <div class="flex flex-col">
        <span class="font-title-sm text-[15px] font-semibold leading-tight text-on-surface">
          AutoDarkMode
        </span>
        <span class="font-caption-sm text-caption-sm text-outline leading-tight">
          {version}
        </span>
      </div>
    </div>

    <!-- General Navigation Section -->
    <div class="flex flex-col gap-space-xs">
      <span
        class="px-space-sm font-caption-sm text-caption-sm font-semibold tracking-wider text-outline uppercase"
      >
        General
      </span>
      <nav class="flex flex-col gap-[2px]">
        <NavigationItem
          title="Overview"
          icon={Home}
          active={currentPage === 'overview'}
          disabled={false}
          onclick={() => handleNavigate('overview')}
        />
        <NavigationItem
          title="Schedule"
          icon={Clock}
          active={currentPage === 'schedule'}
          disabled={false}
          onclick={() => handleNavigate('schedule')}
        />
        <NavigationItem
          title="Apps"
          icon={LayoutGrid}
          active={currentPage === 'apps'}
          disabled={false}
          onclick={() => handleNavigate('apps')}
        />
        <NavigationItem
          title="Personalization"
          icon={Palette}
          active={currentPage === 'personalization'}
          disabled={false}
          onclick={() => handleNavigate('personalization')}
        />
      </nav>
    </div>

    <!-- Automation Navigation Section -->
    <div class="flex flex-col gap-space-xs">
      <span
        class="px-space-sm font-caption-sm text-caption-sm font-semibold tracking-wider text-outline uppercase"
      >
        Automation
      </span>
      <nav class="flex flex-col gap-[2px]">
        <NavigationItem
          title="Conditions"
          icon={ListChecks}
          active={currentPage === 'conditions'}
          disabled={false}
          onclick={() => handleNavigate('conditions')}
        />
        <NavigationItem
          title="Automation"
          icon={SlidersHorizontal}
          active={currentPage === 'automation'}
          disabled={false}
          onclick={() => handleNavigate('automation')}
        />
        <NavigationItem
          title="Shortcuts"
          icon={Keyboard}
          active={currentPage === 'shortcuts'}
          disabled={false}
          onclick={() => handleNavigate('shortcuts')}
        />
      </nav>
    </div>
  </div>

  <!-- Bottom Settings Section -->
  <div class="pt-space-xs border-t border-[var(--titlebar-border)]">
    <nav class="flex flex-col">
      <NavigationItem
        title="Settings"
        icon={Settings}
        active={currentPage === 'settings'}
        disabled={false}
        onclick={() => handleNavigate('settings')}
      />
    </nav>
  </div>
</aside>
