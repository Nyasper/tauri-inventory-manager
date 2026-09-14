<script lang="ts">
  import type { Movement } from "$lib/types";

  let { movement }: { movement: Movement } = $props();
</script>

{#if movement.supplier_name}
  <span class="name" title={movement.supplier_name}>{movement.supplier_name}</span>
  {#if movement.supplier_id === null}
    <span class="deleted">Proveedor eliminado</span>
  {/if}
{:else}
  <span class="empty">—</span>
{/if}

<style>
  .name {
    display: block;
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

  .empty {
    color: var(--text-muted);
  }
</style>
