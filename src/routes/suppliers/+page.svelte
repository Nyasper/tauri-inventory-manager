<script lang="ts">
  import type { Supplier } from "$lib/types";
  import { suppliersStore } from "$lib/stores/suppliers.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import { createAppTable, createAppColumnHelper } from "$lib/components/ui/table";
  import DataTable from "$lib/components/ui/DataTable.svelte";
  import SupplierFormModal from "$lib/components/suppliers/SupplierFormModal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  suppliersStore.load();

  const columnHelper = createAppColumnHelper<Supplier>();

  const columns = columnHelper.columns([
    columnHelper.accessor("name", {
      header: () => "Nombre",
      cell: (info) => info.getValue<string>(),
    }),
    columnHelper.accessor("contact", {
      header: () => "Contacto",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
    columnHelper.accessor("notes", {
      header: () => "Notas",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
  ]);

  const table = createAppTable({
    columns,
    get data() {
      return suppliersStore.items;
    },
  });

  function openNew() {
    modalService.open(SupplierFormModal);
  }

  function openEdit(supplier: Supplier) {
    modalService.open(SupplierFormModal, { supplier });
  }

  async function removeSupplier(supplier: Supplier) {
    const ok = await modalService.confirm(
      `¿Eliminar el proveedor "${supplier.name}"?`,
      "Eliminar proveedor",
    );
    if (ok) await suppliersStore.remove(supplier.id);
  }
</script>

<div class="toolbar">
  <div class="spacer"></div>
  <Button icon="plus" onclick={openNew}>Nuevo proveedor</Button>
</div>

{#if suppliersStore.items.length === 0}
  <EmptyState
    icon="truck"
    title="No hay proveedores"
    description="Añade proveedores para asociarlos a las entradas de stock."
    actionLabel="Nuevo proveedor"
    onAction={openNew}
  />
{:else}
  <DataTable
    {table}
    searchPlaceholder="Buscar proveedores..."
  >
    {#snippet actions(supplier)}
      <div class="actions">
        <button
          class="icon-btn"
          type="button"
          aria-label="Editar"
          onclick={() => openEdit(supplier)}
        >
          <Icon name="edit" size={16} />
        </button>
        <button
          class="icon-btn danger"
          type="button"
          aria-label="Eliminar"
          onclick={() => removeSupplier(supplier)}
        >
          <Icon name="trash" size={16} />
        </button>
      </div>
    {/snippet}
  </DataTable>
{/if}

<style>
  .toolbar {
    display: flex;
    align-items: center;
    margin-bottom: 1.25rem;
  }

  .spacer {
    flex: 1;
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
