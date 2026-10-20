<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { getSuppliers } from "$lib/services/db.service";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import type { Supplier } from "$lib/types";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { createAppTable, createAppColumnHelper } from "$lib/components/ui/table";
  import { ArchivedList } from "$lib/utils/archived.svelte";
  import DataTable from "$lib/components/ui/DataTable.svelte";
  import SupplierFormModal from "$lib/components/suppliers/SupplierFormModal.svelte";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import DeleteRecordModal from "$lib/components/ui/DeleteRecordModal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { Plus, Truck, Pencil, Trash, Archive, Undo2 } from "@lucide/svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  suppliersService.load();
  const targetId = $derived(page.url.searchParams.get("id"));
  const archivedList = new ArchivedList<Supplier>(getSuppliers);
  const visibleSuppliers = $derived(
    (archivedList.showingArchived ? archivedList.items : suppliersService.items).filter(
      (supplier) => !targetId || supplier.id === Number(targetId),
    ),
  );
  async function restore(supplier: Supplier) {
    await suppliersService.restore(supplier.id);
    await archivedList.reload();
  }

  const columnHelper = createAppColumnHelper<Supplier>();

  const columns = columnHelper.columns([
    columnHelper.accessor("name", {
      header: () => "Nombre",
      cell: (info) => info.getValue<string>(),
    }),
    columnHelper.accessor((supplier) => supplier.contacts.join(" · "), {
      id: "contacts",
      header: () => "Contacto",
      cell: (info) => info.getValue<string>() || "—",
    }),
    columnHelper.accessor("notes", {
      header: () => "Notas",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
  ]);

  const table = createAppTable({
    columns,
    get data() {
      return visibleSuppliers;
    },
    manualPagination: true,
  });

  function openNew() {
    modalService.open(SupplierFormModal);
  }

  function openEdit(supplier: Supplier) {
    modalService.open(SupplierFormModal, { supplier });
  }

  async function archiveSupplier(supplier: Supplier) {
    const ok = await modalService.confirm(
      `¿Archivar el proveedor "${supplier.name}"?`,
      "Archivar proveedor",
    );
    if (ok) await suppliersService.archive(supplier.id);
  }

  function deletePermanently(supplier: Supplier) {
    modalService.open(DeleteRecordModal, {
      recordType: "supplier",
      name: supplier.name,
      onConfirm: async (password: string) => {
        await suppliersService.deletePermanently(supplier.id, password);
        await archivedList.reload();
      },
    });
  }
</script>

<div class="toolbar">
  {#if targetId}<Button variant="secondary" onclick={() => goto("/suppliers")}>Ver todos</Button
    >{/if}
  <label><input type="checkbox" bind:checked={archivedList.showingArchived} /> Archivados</label>
  <div class="spacer"></div>
  <Button icon={Plus} onclick={openNew}>Nuevo proveedor</Button>
</div>

{#if suppliersService.error || archivedList.error}<p role="alert" class="error">
    {suppliersService.error || archivedList.error}
  </p>
  <Button
    onclick={() => (archivedList.showingArchived ? archivedList.reload() : suppliersService.load())}
    >Reintentar</Button
  >
{:else if suppliersService.loading || archivedList.loading}<p role="status">
    Cargando proveedores…
  </p>
{:else if visibleSuppliers.length === 0}
  {#if archivedList.showingArchived}
    <EmptyState
      icon={Archive}
      title="No hay proveedores archivados"
      description="Los proveedores que archives aparecerán aquí."
    />
  {:else}
    <EmptyState
      icon={Truck}
      title="No hay proveedores"
      description="Añade proveedores para asociarlos a las entradas de stock."
      actionLabel="Nuevo proveedor"
      onAction={openNew}
    />
  {/if}
{:else}
  <DataTable
    {table}
    searchPlaceholder="Buscar proveedores..."
    onRowClick={openEdit}
    rowClickable={(supplier) => !supplier.isArchived}
    rowActionLabel={(supplier) => `Editar ${supplier.name}`}
  >
    {#snippet actions(supplier)}
      {#if supplier.isArchived}
        <div class="actions">
          <button
            class="icon-btn"
            type="button"
            title="Restaurar"
            aria-label={`Restaurar ${supplier.name}`}
            onclick={() => restore(supplier)}
          >
            <Undo2 size={16} />
          </button>
          <button
            class="icon-btn danger"
            type="button"
            title="Eliminar definitivamente"
            aria-label={`Eliminar definitivamente ${supplier.name}`}
            onclick={() => deletePermanently(supplier)}
          >
            <Trash size={16} />
          </button>
        </div>
      {:else}
        <div class="actions">
          <button
            class="icon-btn"
            type="button"
            title="Registrar entrada"
            aria-label={`Registrar entrada de ${supplier.name}`}
            onclick={() =>
              modalService.open(MovementFormModal, {
                initialSupplierId: String(supplier.id),
              })}
          >
            <Plus size={16} />
          </button>
          <button
            class="icon-btn"
            type="button"
            title="Editar"
            aria-label="Editar"
            onclick={() => openEdit(supplier)}
          >
            <Pencil size={16} />
          </button>
          <button
            class="icon-btn danger"
            type="button"
            title="Archivar"
            aria-label="Archivar"
            onclick={() => archiveSupplier(supplier)}
          >
            <Archive size={16} />
          </button>
        </div>
      {/if}
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
