<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    children,
    background,
    textColor,
    fixedWidth = false,
    title,
  }: {
    children: Snippet;
    background?: string;
    textColor?: string;
    fixedWidth?: boolean;
    title?: string;
  } = $props();
</script>

<span
  class={["badge", { "fixed-width": fixedWidth }]}
  style:background={background ?? null}
  style:color={textColor ?? null}
  {title}
>
  {@render children()}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    padding: 0.125rem 0.5rem;
    border-radius: 999px;
    font-size: 0.75rem;
    font-weight: 500;
    line-height: 1.4;
    white-space: nowrap;
  }

  .badge.fixed-width {
    min-width: 5rem;
  }
</style>
