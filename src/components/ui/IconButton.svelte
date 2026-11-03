<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title?: string;
    disabled?: boolean;
    size?: 'sm' | 'md' | 'caption';
    variant?: 'subtle' | 'close';
    onclick?: () => void;
    children?: Snippet;
    class?: string;
  }

  let {
    title = '',
    disabled = false,
    size = 'md',
    variant = 'subtle',
    onclick,
    children,
    class: className = '',
  }: Props = $props();

  const sizeClasses = {
    sm: 'w-7 h-7 text-[14px]',
    md: 'w-8 h-8 text-[16px]',
    caption: 'h-10 w-11 text-[14px]',
  };

  const variantClasses = {
    subtle:
      'text-on-surface-variant hover:bg-surface-variant hover:text-on-surface',
    close: 'text-on-surface-variant hover:bg-error hover:text-on-error',
  };
</script>

<button
  type="button"
  {title}
  {disabled}
  {onclick}
  class="flex items-center justify-center rounded transition-colors disabled:opacity-40 disabled:cursor-not-allowed select-none {sizeClasses[
    size
  ]} {variantClasses[variant]} {className}"
>
  {@render children?.()}
</button>
