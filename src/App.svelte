<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { RotateCw } from '@lucide/svelte';
  import Header from './components/layout/Header.svelte';
  import Sidebar, { type AppPage } from './components/layout/Sidebar.svelte';
  import CurrentAppearanceCard from './components/dashboard/CurrentAppearanceCard.svelte';
  import NextSwitchCard from './components/dashboard/NextSwitchCard.svelte';
  import ScheduleTimeline from './components/dashboard/ScheduleTimeline.svelte';
  import QuickControls from './components/dashboard/QuickControls.svelte';
  import ScheduleView from './components/schedule/ScheduleView.svelte';
  import AppsView from './components/apps/AppsView.svelte';
  import PersonalizationView from './components/personalization/PersonalizationView.svelte';
  import ConditionsView from './components/conditions/ConditionsView.svelte';
  import AutomationView from './components/automation/AutomationView.svelte';
  import ShortcutsView from './components/shortcuts/ShortcutsView.svelte';
  import SettingsView from './components/settings/SettingsView.svelte';
  import TrayFlyout from './components/tray/TrayFlyout.svelte';
  import StatusBar from './components/layout/StatusBar.svelte';
  import StatusIndicator from './components/ui/StatusIndicator.svelte';
  import IconButton from './components/ui/IconButton.svelte';
  import { dashboard } from './stores/dashboard.svelte';

  const isTrayView =
    typeof window !== 'undefined' && window.location.search.includes('view=tray');

  let currentPage = $state<AppPage>('overview');
  let unlistenNavigate: (() => void) | null = null;

  function isValidPage(page: string): page is AppPage {
    return [
      'overview',
      'schedule',
      'apps',
      'personalization',
      'conditions',
      'automation',
      'shortcuts',
      'settings',
    ].includes(page);
  }

  onMount(async () => {
    dashboard.init();

    if (typeof window !== 'undefined') {
      const urlParams = new URLSearchParams(window.location.search);
      const requestedPage = urlParams.get('page');
      if (requestedPage && isValidPage(requestedPage)) {
        currentPage = requestedPage;
      }

      try {
        const { listen } = await import('@tauri-apps/api/event');
        unlistenNavigate = await listen<string>('navigate-page', (event) => {
          if (event.payload && isValidPage(event.payload)) {
            currentPage = event.payload;
          }
        });
      } catch (err) {
        console.warn('Could not attach navigate-page listener:', err);
      }
    }
  });

  onDestroy(() => {
    dashboard.destroy();
    if (unlistenNavigate) {
      unlistenNavigate();
      unlistenNavigate = null;
    }
  });

  function handleRefresh() {
    dashboard.refresh();
  }

  function handleNavigate(page: AppPage) {
    currentPage = page;
  }
</script>

{#if isTrayView}
  <!-- System Tray Compact Popup View (Anchored above taskbar) -->
  <div class="w-screen h-screen overflow-hidden bg-transparent p-0 m-0">
    <TrayFlyout />
  </div>
{:else}
  <!-- Main Desktop Application Window -->
  <div class="h-screen w-screen overflow-hidden text-on-surface select-none relative">
    <!-- Dynamic Atmospheric Fluent Wallpaper Background -->
    <div class="fluent-wallpaper" aria-hidden="true"></div>

    <!-- Custom Windows 11 Titlebar -->
    <Header />

    <!-- Fixed Left Sidebar -->
    <Sidebar {currentPage} onnavigate={handleNavigate} />

    <!-- Main Scrollable Content Area -->
    <div
      class="absolute left-[250px] right-0 top-10 bottom-0 overflow-y-auto overflow-x-hidden z-10 scroll-smooth"
    >
      <main class="w-full pt-6 px-space-xl pb-16 min-h-full">
        {#if currentPage === 'overview'}
          <div class="flex flex-col w-full gap-space-lg max-w-[1400px] mx-auto">
            <!-- Top Section Title Area -->
            <div class="flex items-center justify-between">
              <div class="flex flex-col gap-0.5">
                <h1
                  class="font-headline-lg text-headline-lg text-on-surface tracking-tight font-semibold"
                >
                  Overview
                </h1>
                <p class="font-body-md text-body-md text-on-surface-variant">
                  Manage your automatic Windows appearance.
                </p>
              </div>

              <div class="flex items-center gap-space-sm">
                <!-- Automation Status Badge -->
                <StatusIndicator
                  active={dashboard.state?.automation_enabled ?? true}
                  label={dashboard.state?.automation_enabled
                    ? 'Automation enabled'
                    : 'Automation disabled'}
                  variant="tertiary"
                />

                <!-- Refresh / Sync Button -->
                <IconButton
                  title="Synchronize appearance with Windows"
                  onclick={handleRefresh}
                  size="sm"
                >
                  <RotateCw size={14} class={dashboard.isLoading ? 'animate-spin' : ''} />
                </IconButton>
              </div>
            </div>

            <!-- Primary Status Cards: Asymmetrical 7/5 Grid -->
            <div class="grid grid-cols-1 lg:grid-cols-12 gap-space-md">
              <CurrentAppearanceCard />
              <NextSwitchCard />
            </div>

            <!-- Today's Schedule Timeline Section -->
            <ScheduleTimeline timeline={dashboard.state?.schedule_timeline} />

            <!-- Quick Controls Bento Section -->
            <QuickControls onnavigate={handleNavigate} />

            <!-- System Status Bar -->
            <StatusBar />
          </div>
        {:else if currentPage === 'schedule'}
          <ScheduleView />
        {:else if currentPage === 'apps'}
          <AppsView />
        {:else if currentPage === 'personalization'}
          <PersonalizationView />
        {:else if currentPage === 'conditions'}
          <ConditionsView />
        {:else if currentPage === 'automation'}
          <AutomationView />
        {:else if currentPage === 'shortcuts'}
          <ShortcutsView />
        {:else if currentPage === 'settings'}
          <SettingsView />
        {/if}
      </main>
    </div>
  </div>
{/if}
