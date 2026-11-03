<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    children?: Snippet;
    class?: string;
    hoverable?: boolean;
  }

  let { children, class: className = '', hoverable = false }: Props = $props();
</script>

<div
  class="bg-surface-container rounded-lg p-space-lg shadow-[var(--card-shadow)] border border-[var(--card-border)] relative overflow-hidden transition-colors {hoverable
    ? 'hover:bg-surface-container-high'
    : ''} {className}"
>
  {@render children?.()}
</div>
