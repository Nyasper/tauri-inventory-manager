<script lang="ts">
  import type { Movement } from "$lib/types";
  import { formatNumber, formatDate } from "$lib/utils/format";
  import Badge from "$lib/components/ui/Badge.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";

  let {
    movements,
    onDelete,
  }: {
    movements: Movement[];
    onDelete: (movement: Movement) => void;
  } = $props();
</script>

<table class="table">
  <thead>
    <tr>
      <th>Tipo</th>
      <th>Producto</th>
      <th>Presentación</th>
      <th class="num">Cantidad</th>
      <th>Proveedor</th>
      <th>Responsable</th>
      <th>Fecha</th>
      <th class="actions-col"></th>
    </tr>
  </thead>
  <tbody>
    {#each movements as movement (movement.id)}
      <tr>
        <td>
          {#if movement.type === "in"}
            <Badge bg="var(--success)" color="#ffffff">
              <Icon name="arrowDown" size={13} /> Entrada
            </Badge>
          {:else}
            <Badge bg="var(--danger)" color="#ffffff">
              <Icon name="arrowUp" size={13} /> Salida
            </Badge>
          {/if}
        </td>
        <td>
          <span class="strong">{movement.product_name ?? "—"}</span>
          {#if movement.notes}
            <span class="sub" title={movement.notes}>{movement.notes}</span>
          {/if}
        </td>
        <td class="muted">{movement.presentation_name ?? "—"}</td>
        <td class="num strong">{formatNumber(movement.quantity)}</td>
        <td class="muted">{movement.supplier_name ?? "—"}</td>
        <td class="muted">{movement.person_name ?? "—"}</td>
        <td class="muted">{formatDate(movement.created_at)}</td>
        <td class="actions-col">
          <button
            class="icon-btn danger"
            type="button"
            aria-label="Eliminar"
            onclick={() => onDelete(movement)}
          >
            <Icon name="trash" size={16} />
          </button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .table {
    width: 100%;
    border-collapse: collapse;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }

  th {
    text-align: left;
    padding: 0.625rem 1rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
    background: var(--surface-2);
  }

  td {
    padding: 0.625rem 1rem;
    border-bottom: 1px solid var(--border);
    font-size: 0.875rem;
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  tbody tr:hover {
    background: var(--surface-2);
  }

  .num {
    text-align: right;
  }

  .strong {
    font-weight: 600;
  }

  .muted {
    color: var(--text-muted);
  }

  td > .strong,
  td > .sub {
    display: block;
  }

  .sub {
    font-size: 0.75rem;
    color: var(--text-muted);
    max-width: 220px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .actions-col {
    width: 48px;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .icon-btn.danger:hover {
    background: var(--danger-soft);
    color: var(--danger);
  }
</style>
