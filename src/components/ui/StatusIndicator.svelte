<script lang="ts">
  interface Props {
    active: boolean;
    label: string;
    pulse?: boolean;
    variant?: 'tertiary' | 'primary' | 'secondary' | 'neutral';
  }

  let {
    active,
    label,
    pulse = false,
    variant = 'tertiary',
  }: Props = $props();

  const colorClasses = {
    tertiary: 'bg-tertiary shadow-[0_0_8px_rgba(16,124,16,0.6)]',
    primary: 'bg-primary-container shadow-[0_0_8px_rgba(0,103,192,0.6)]',
    secondary: 'bg-secondary shadow-[0_0_8px_rgba(178,107,0,0.6)]',
    neutral: 'bg-outline',
  };
</script>

<div
  class="h-7 px-space-md rounded-full bg-surface-container flex items-center gap-space-xs shadow-sm border border-[var(--titlebar-border)] select-none"
>
  <span
    class="w-1.5 h-1.5 rounded-full {active
      ? colorClasses[variant]
      : colorClasses.neutral} {pulse && active ? 'animate-pulse' : ''}"
  ></span>
  <span class="font-label-md text-label-md text-on-surface font-semibold">
    {label}
  </span>
</div>
