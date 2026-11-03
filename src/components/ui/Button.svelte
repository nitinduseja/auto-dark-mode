<script lang="ts">
  import type { Snippet, Component } from 'svelte';
  import { Loader2 } from '@lucide/svelte';

  interface Props {
    variant?: 'primary' | 'standard' | 'subtle' | 'secondary' | 'ghost';
    size?: 'sm' | 'md' | 'lg';
    loading?: boolean;
    icon?: Component<any>;
    disabled?: boolean;
    onclick?: () => void;
    children?: Snippet;
    class?: string;
  }

  let {
    variant = 'standard',
    size = 'md',
    loading = false,
    icon: Icon,
    disabled = false,
    onclick,
    children,
    class: className = '',
  }: Props = $props();

  const variantClasses = {
    primary:
      'bg-primary-container text-on-primary-container font-semibold hover:bg-primary hover:text-on-primary',
    standard:
      'bg-surface-container-high text-on-surface border border-white/[0.08] hover:bg-surface-variant hover:text-on-surface',
    secondary:
      'bg-surface-container-high text-on-surface border border-white/[0.08] hover:bg-surface-variant hover:text-on-surface font-medium',
    subtle:
      'text-on-surface-variant hover:bg-surface-container-high hover:text-on-surface',
    ghost:
      'text-on-surface-variant hover:bg-surface-container hover:text-on-surface',
  };

  const sizeClasses = {
    sm: 'h-7 px-2.5 text-caption-sm rounded',
    md: 'h-8 px-3 text-label-md rounded',
    lg: 'h-9 px-4 text-body-md rounded',
  };
</script>

<button
  type="button"
  disabled={disabled || loading}
  {onclick}
  class="flex items-center justify-center gap-1.5 transition-colors disabled:opacity-40 disabled:cursor-not-allowed select-none cursor-pointer {variantClasses[
    variant
  ]} {sizeClasses[size]} {className}"
>
  {#if loading}
    <Loader2 class="w-3.5 h-3.5 animate-spin" />
  {:else if Icon}
    <Icon class="w-3.5 h-3.5" />
  {/if}
  {@render children?.()}
</button>
