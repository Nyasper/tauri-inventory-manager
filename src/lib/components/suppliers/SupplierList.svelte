<script lang="ts">
  import type { Supplier } from "$lib/types";
  import Icon from "$lib/components/ui/Icon.svelte";

  let {
    suppliers,
    onEdit,
    onDelete,
  }: {
    suppliers: Supplier[];
    onEdit: (supplier: Supplier) => void;
    onDelete: (supplier: Supplier) => void;
  } = $props();
</script>

<table class="table">
  <thead>
    <tr>
      <th>Nombre</th>
      <th>Contacto</th>
      <th>Notas</th>
      <th class="actions-col"></th>
    </tr>
  </thead>
  <tbody>
    {#each suppliers as supplier (supplier.id)}
      <tr>
        <td class="strong">{supplier.name}</td>
        <td class="muted">{supplier.contact ?? "—"}</td>
        <td class="muted">{supplier.notes ?? "—"}</td>
        <td class="actions-col">
          <div class="actions">
            <button
              class="icon-btn"
              type="button"
              aria-label="Editar"
              onclick={() => onEdit(supplier)}
            >
              <Icon name="edit" size={16} />
            </button>
            <button
              class="icon-btn danger"
              type="button"
              aria-label="Eliminar"
              onclick={() => onDelete(supplier)}
            >
              <Icon name="trash" size={16} />
            </button>
          </div>
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

  .strong {
    font-weight: 600;
  }

  .muted {
    color: var(--text-muted);
  }

  .actions-col {
    width: 80px;
  }

  .actions {
    display: flex;
    gap: 0.125rem;
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

  .icon-btn:hover {
    background: var(--surface-3);
    color: var(--text);
  }

  .icon-btn.danger:hover {
    background: var(--danger-soft);
    color: var(--danger);
  }
</style>
