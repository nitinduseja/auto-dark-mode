<script lang="ts">
  interface Props {
    checked: boolean;
    disabled?: boolean;
    onchange?: (checked: boolean) => void;
    label?: string;
  }

  let { checked, disabled = false, onchange, label = '' }: Props = $props();

  function handleClick() {
    if (disabled) return;
    onchange?.(!checked);
  }
</script>

<button
  type="button"
  role="switch"
  aria-checked={checked}
  aria-label={label}
  {disabled}
  onclick={handleClick}
  class="w-10 h-5 rounded-full relative flex-shrink-0 transition-colors p-0.5 flex items-center select-none {checked
    ? 'bg-primary-container justify-end'
    : 'bg-surface-variant border border-outline-variant justify-start'} {disabled
    ? 'opacity-40 cursor-not-allowed'
    : 'cursor-pointer'}"
>
  <span
    class="w-4 h-4 rounded-full transition-transform duration-150 {checked
      ? 'bg-on-primary shadow-sm'
      : 'bg-outline'}"
  ></span>
</button>
