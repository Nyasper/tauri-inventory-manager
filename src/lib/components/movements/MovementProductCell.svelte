<script lang="ts">
  import type { Movement } from "$lib/types";

  let { movement }: { movement: Movement } = $props();
</script>

<span class="name" title={movement.product_name ?? "—"}>
  {movement.product_name ?? "—"}
</span>
{#if movement.product_id === null}
  <span class="deleted">Producto eliminado</span>
{/if}
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

  .deleted {
    display: inline-block;
    margin-top: 0.125rem;
    padding: 0 0.375rem;
    border-radius: 999px;
    background: var(--danger-soft);
    color: var(--danger);
    font-size: 0.6875rem;
    font-weight: 500;
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
