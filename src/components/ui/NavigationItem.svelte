<script lang="ts">
  import type { Component } from 'svelte';

  interface Props {
    title: string;
    icon: Component<{ size?: number; class?: string }>;
    active?: boolean;
    disabled?: boolean;
    onclick?: () => void;
  }

  let { title, icon: Icon, active = false, disabled = true, onclick }: Props = $props();
</script>

{#if active}
  <div
    role="link"
    aria-current="page"
    class="relative h-9 px-space-sm flex items-center gap-space-md rounded transition-colors bg-surface-container-high text-on-surface font-semibold before:absolute before:left-0 before:top-1.5 before:bottom-1.5 before:w-[3px] before:rounded-full before:bg-primary-container cursor-default select-none"
  >
    <Icon size={18} class="text-primary-container" />
    <span class="font-body-md text-body-md">{title}</span>
  </div>
{:else if !disabled}
  <button
    type="button"
    role="link"
    {onclick}
    class="w-full relative h-9 px-space-sm flex items-center gap-space-md rounded text-on-surface-variant hover:bg-surface-container hover:text-on-surface transition-colors font-body-md text-body-md text-left cursor-pointer select-none group"
  >
    <Icon size={18} class="text-on-surface-variant group-hover:text-on-surface transition-colors" />
    <span class="font-body-md text-body-md">{title}</span>
  </button>
{:else}
  <div
    role="link"
    aria-disabled="true"
    title="{title} (Available in future updates)"
    class="relative h-9 px-space-sm flex items-center gap-space-md rounded text-on-surface-variant/40 font-body-md text-body-md cursor-not-allowed select-none group"
  >
    <Icon size={18} class="text-on-surface-variant/40" />
    <span class="font-body-md text-body-md">{title}</span>
  </div>
{/if}
