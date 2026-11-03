<script lang="ts">
  import { Minus, Square, X } from '@lucide/svelte';
  import {
    minimizeWindow,
    toggleMaximizeWindow,
    closeWindow,
  } from '../../services/tauri';

  function handleMinimize() {
    minimizeWindow();
  }

  function handleMaximize() {
    toggleMaximizeWindow();
  }

  function handleClose() {
    closeWindow();
  }
</script>

<div
  data-tauri-drag-region
  class="fixed top-0 left-0 right-0 h-10 z-50 flex items-center justify-between pl-space-lg bg-surface-container-lowest/80 backdrop-blur-xl select-none border-b border-[var(--titlebar-border)] transition-colors"
>
  <!-- Window Title and Icon (Draggable) -->
  <div data-tauri-drag-region class="flex items-center gap-space-sm cursor-default">
    <img src="/favicon.png" alt="AutoDarkMode" class="w-4 h-4 rounded object-contain" />
    <span class="font-caption-sm text-caption-sm text-on-surface-variant font-medium tracking-tight">
      AutoDarkMode
    </span>
  </div>

  <!-- Windows Native Caption Control Buttons -->
  <div class="flex items-center h-full">
    <!-- Minimize -->
    <button
      type="button"
      title="Minimize"
      onclick={handleMinimize}
      class="h-10 w-11 flex items-center justify-center text-on-surface-variant hover:bg-surface-variant hover:text-on-surface transition-colors cursor-pointer"
    >
      <Minus size={14} />
    </button>

    <!-- Maximize / Restore -->
    <button
      type="button"
      title="Maximize"
      onclick={handleMaximize}
      class="h-10 w-11 flex items-center justify-center text-on-surface-variant hover:bg-surface-variant hover:text-on-surface transition-colors cursor-pointer"
    >
      <Square size={12} />
    </button>

    <!-- Close -->
    <button
      type="button"
      title="Close"
      onclick={handleClose}
      class="h-10 w-11 flex items-center justify-center text-on-surface-variant hover:bg-error hover:text-on-error transition-colors cursor-pointer"
    >
      <X size={14} />
    </button>
  </div>
</div>
