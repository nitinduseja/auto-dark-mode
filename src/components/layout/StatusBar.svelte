<script lang="ts">
  import { dashboard } from '../../stores/dashboard.svelte';

  let appInfo = $derived(dashboard.state?.app_info);
  let isAutomationActive = $derived(dashboard.state?.automation_enabled ?? true);
</script>

<footer
  class="mt-space-xs py-space-sm px-space-md rounded bg-surface-container-lowest flex flex-wrap items-center justify-between text-outline font-caption-sm text-caption-sm gap-space-sm border border-[var(--titlebar-border)] select-none transition-colors shadow-sm"
>
  <div class="flex items-center gap-2">
    <span
      class="w-2 h-2 rounded-full {isAutomationActive
        ? 'bg-tertiary shadow-[0_0_6px_rgba(16,124,16,0.6)] animate-pulse'
        : 'bg-outline'}"
    ></span>
    <span class="text-on-surface-variant font-medium">
      {isAutomationActive
        ? 'AutoDarkMode background automation active'
        : 'AutoDarkMode automation paused'}
    </span>
  </div>

  <div class="flex items-center gap-4">
    <span class="text-outline">
      {appInfo?.app_name || 'AutoDarkMode'}
      {appInfo?.version || 'v1.1.0'} ({appInfo?.target_platform || 'Windows 11 x64'}) • {appInfo?.webview_runtime ||
        'WebView2 Runtime 122.0'}
    </span>
  </div>
</footer>
