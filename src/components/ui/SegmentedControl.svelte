<script lang="ts">
  import { Sun, Moon } from '@lucide/svelte';
  import type { ThemeMode } from '../../types/theme';

  interface Option {
    value: string;
    label: string;
  }

  interface Props {
    value: string;
    options?: Option[];
    disabled?: boolean;
    onchange?: (value: any) => void;
  }

  let { value, options, disabled = false, onchange }: Props = $props();

  const defaultOptions: Option[] = [
    { value: 'light', label: 'Light' },
    { value: 'dark', label: 'Dark' },
  ];

  let displayOptions = $derived(options || defaultOptions);
</script>

<div
  class="bg-surface-container-low p-0.5 rounded-md flex items-center shadow-inner select-none border border-[var(--titlebar-border)]"
>
  {#each displayOptions as opt (opt.value)}
    <button
      type="button"
      {disabled}
      onclick={() => onchange?.(opt.value)}
      class="h-6 px-space-sm rounded flex items-center gap-1.5 font-label-md text-label-md transition-all relative {value ===
      opt.value
        ? 'bg-surface-container text-on-surface shadow-sm font-semibold'
        : 'text-on-surface-variant hover:text-on-surface'} {disabled
        ? 'opacity-50 cursor-not-allowed'
        : 'cursor-pointer'}"
    >
      {#if opt.value === 'light'}
        <Sun
          size={13}
          class={value === 'light' ? 'text-secondary font-bold' : 'text-on-surface-variant'}
        />
      {:else if opt.value === 'dark'}
        <Moon
          size={13}
          class={value === 'dark'
            ? 'text-primary-container font-bold'
            : 'text-on-surface-variant'}
        />
      {/if}
      <span>{opt.label}</span>
      {#if value === opt.value}
        <span
          class="absolute bottom-0 left-2 right-2 h-[2px] bg-primary-container rounded-full"
        ></span>
      {/if}
    </button>
  {/each}
</div>
