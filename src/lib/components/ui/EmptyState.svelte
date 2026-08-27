<script lang="ts">
  import type { Snippet } from "svelte";
  import Icon from "./Icon.svelte";
  import type { IconName } from "./Icon.svelte";
  import Button from "./Button.svelte";

  let {
    icon = "box",
    title,
    description,
    actionLabel,
    onAction,
  }: {
    icon?: IconName;
    title: string;
    description?: string;
    actionLabel?: string;
    onAction?: () => void;
  } = $props();
</script>

<div class="empty">
  <div class="empty-icon">
    <Icon name={icon} size={28} />
  </div>
  <h3>{title}</h3>
  {#if description}
    <p>{description}</p>
  {/if}
  {#if actionLabel && onAction}
    <div class="empty-action">
      <Button icon="plus" onclick={onAction}>{actionLabel}</Button>
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
    background: var(--surface-2);
    color: var(--text-muted);
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
