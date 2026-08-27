<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    children,
    bg,
    color,
  }: {
    children: Snippet;
    bg?: string;
    color?: string;
  } = $props();
</script>

<span class="badge" style:background={bg ?? null} style:color={color ?? null}>
  {@render children()}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.125rem 0.5rem;
    border-radius: 999px;
    font-size: 0.75rem;
    font-weight: 500;
    line-height: 1.4;
    white-space: nowrap;
  }
</style>
