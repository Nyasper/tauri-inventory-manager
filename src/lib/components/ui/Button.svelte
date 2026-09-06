<script lang="ts">
  import type { Snippet } from "svelte";
  import type { Component } from "svelte";
  import type { LucideProps } from "@lucide/svelte";

  let {
    children,
    variant = "primary",
    size = "md",
    type = "button",
    icon,
    disabled = false,
    onclick,
  }: {
    children?: Snippet;
    variant?: "primary" | "secondary" | "ghost" | "danger";
    size?: "sm" | "md";
    type?: "button" | "submit";
    icon?: Component<LucideProps>;
    disabled?: boolean;
    onclick?: (event: MouseEvent) => void;
  } = $props();
</script>

<button
  {type}
  {disabled}
  class="btn btn-{variant} btn-{size}"
  class:icon-only={!children}
  {onclick}
>
  {#if icon}
    {@const Icon = icon}
    <Icon size={size === "sm" ? 15 : 16} />
  {/if}
  {#if children}
    <span>{@render children()}</span>
  {/if}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    border: 1px solid transparent;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    transition:
      background-color 0.15s,
      border-color 0.15s,
      color 0.15s;
    white-space: nowrap;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-md {
    padding: 0.5rem 0.875rem;
    font-size: 0.875rem;
  }

  .btn-sm {
    padding: 0.375rem 0.625rem;
    font-size: 0.8125rem;
  }

  .btn.icon-only {
    padding: 0.5rem;
  }

  .btn-primary {
    background: var(--primary);
    color: #fff;
  }
  .btn-primary:hover:not(:disabled) {
    background: var(--primary-hover);
  }

  .btn-secondary {
    background: var(--surface);
    color: var(--text);
    border-color: var(--border);
  }
  .btn-secondary:hover:not(:disabled) {
    background: var(--surface-2);
  }

  .btn-ghost {
    background: transparent;
    color: var(--text-muted);
  }
  .btn-ghost:hover:not(:disabled) {
    background: var(--surface-2);
    color: var(--text);
  }

  .btn-danger {
    background: var(--danger);
    color: #fff;
  }
  .btn-danger:hover:not(:disabled) {
    filter: brightness(0.92);
  }
</style>
