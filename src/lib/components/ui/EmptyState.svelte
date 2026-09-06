<script lang="ts">
  import type { Snippet } from "svelte";
  import type { Component } from "svelte";
  import type { LucideProps } from "@lucide/svelte";
  import { Box, Plus } from "@lucide/svelte";
  import Button from "./Button.svelte";

let {
    icon = Box,
    title,
    description,
    actionLabel,
    onAction,
  }: {
    icon?: Component<LucideProps>;
    title: string;
    description?: string;
    actionLabel?: string;
    onAction?: () => void;
  } = $props();

  const Icon = $derived(icon);
</script>

<div class="empty">
  <div class="empty-icon">
    <Icon size={28} />
  </div>
  <h3>{title}</h3>
  {#if description}
    <p>{description}</p>
  {/if}
  {#if actionLabel && onAction}
    <div class="empty-action">
      <Button icon={Plus} onclick={onAction}>{actionLabel}</Button>
    </div>
  {/if}
</div>

<style>
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 3rem 1rem;
    text-align: center;
  }

  .empty-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 60px;
    height: 60px;
    border-radius: 999px;
    background: linear-gradient(135deg, var(--primary-soft), var(--surface-2));
    color: var(--primary);
    box-shadow: inset 0 0 0 1px var(--border);
    margin-bottom: 0.25rem;
  }

  h3 {
    font-size: 1rem;
  }

  p {
    color: var(--text-muted);
    max-width: 340px;
  }

  .empty-action {
    margin-top: 0.5rem;
  }
</style>
