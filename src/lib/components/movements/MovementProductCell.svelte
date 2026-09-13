<script lang="ts">
  import type { Movement } from "$lib/types";

  let { movement }: { movement: Movement } = $props();
</script>

<span class="name" title={movement.product_name ?? "—"}>
  {movement.product_name ?? "—"}
</span>
{#if movement.notes}
  <span class="notes" title={movement.notes}>{movement.notes}</span>
{/if}

<style>
  .name,
  .notes {
    display: block;
  }

  .name {
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .notes {
    font-size: 0.75rem;
    color: var(--text-muted);
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>