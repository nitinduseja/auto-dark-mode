<script lang="ts">
  import type { Snippet, Component } from 'svelte';

  interface Props {
    title: string;
    description?: string;
    subtitle?: string;
    icon?: Component<any>;
    actions?: Snippet;
  }

  let { title, description, subtitle, icon: Icon, actions }: Props = $props();
  let descText = $derived(description || subtitle);
</script>

<div class="flex items-center justify-between select-none py-1">
  <div class="flex flex-col gap-0.5">
    <div class="flex items-center gap-2">
      {#if Icon}
        <Icon class="w-4 h-4 text-primary-container" />
      {/if}
      <h2 class="font-title-sm text-title-sm text-on-surface font-semibold">
        {title}
      </h2>
    </div>
    {#if descText}
      <p class="font-caption-sm text-caption-sm text-on-surface-variant">
        {descText}
      </p>
    {/if}
  </div>
  {#if actions}
    <div class="flex items-center gap-space-sm">
      {@render actions()}
    </div>
  {/if}
</div>
