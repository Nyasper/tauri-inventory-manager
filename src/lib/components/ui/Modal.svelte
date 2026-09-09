<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import type { Snippet } from "svelte";
  import { X } from "@lucide/svelte";

  let {
    title,
    children,
    footer,
    onClose,
    width = "md",
  }: {
    title: string;
    children: Snippet;
    footer?: Snippet;
    onClose?: () => void;
    width?: "sm" | "md" | "lg";
  } = $props();

  let dialog: HTMLDivElement;
  const titleId = $props.id();
  onMount(() => {
    const previous = document.activeElement as HTMLElement | null;
    (dialog.querySelector<HTMLElement>("input,select,textarea,button") ?? dialog).focus();
    return () => previous?.focus();
  });
  function handleKeydown(event: KeyboardEvent) {
    const dialogs = document.querySelectorAll('[role="dialog"]');
    if (dialogs[dialogs.length-1] !== dialog) return;
    if (event.key === "Escape") { event.preventDefault(); onClose?.(); }
    if (event.key === "Tab") {
      const items = Array.from(dialog.querySelectorAll<HTMLElement>('button:not(:disabled), input:not(:disabled), select:not(:disabled), textarea:not(:disabled), a[href], [tabindex="0"]')).filter(el => el.getClientRects().length);
      const first = items[0], last = items[items.length-1];
      if (!first) { event.preventDefault(); dialog.focus(); return; }
      if (event.shiftKey && (document.activeElement === first || !dialog.contains(document.activeElement))) { event.preventDefault(); last.focus(); }
      else if (!event.shiftKey && (document.activeElement === last || !dialog.contains(document.activeElement))) { event.preventDefault(); first.focus(); }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="overlay"
  role="presentation"
  transition:fade={{ duration: 160 }}
  onclick={(event) => {
    if (event.target === event.currentTarget) onClose?.();
  }}
>
  <div bind:this={dialog} class="modal modal-{width}" role="dialog" aria-modal="true" aria-labelledby={titleId} tabindex="-1">
    <header class="header">
      <h2 id={titleId}>{title}</h2>
      <button class="icon-btn" type="button" aria-label="Cerrar" onclick={onClose}>
        <X size={18} />
      </button>
    </header>
    <div class="body">
      {@render children()}
    </div>
    {#if footer}
      <footer class="footer">
        {@render footer()}
      </footer>
    {/if}
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: 8vh 1rem 1rem;
    z-index: 1000;
    overflow-y: auto;
  }

  .modal {
    width: 100%;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    display: flex;
    flex-direction: column;
    max-height: 84vh;
  }

  .modal-sm {
    max-width: 420px;
  }
  .modal-md {
    max-width: 560px;
  }
  .modal-lg {
    max-width: 720px;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--border);
  }

  .header h2 {
    font-size: 1.05rem;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .icon-btn:hover {
    background: var(--surface-2);
    color: var(--text);
  }

  .body {
    padding: 1.25rem;
    overflow-y: auto;
  }

  .footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 1rem 1.25rem;
    border-top: 1px solid var(--border);
  }
</style>
