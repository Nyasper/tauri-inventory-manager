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
  import Button from "$lib/components/ui/Button.svelte";
  import { Plus, Truck, Pencil, Trash } from "@lucide/svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  suppliersService.load();
  const targetId = $derived(page.url.searchParams.get("id"));
  const archive = new ArchivedList<Supplier>((archived) =>
    getSuppliers(archived),
  );
  const visible = $derived(
    (archive.active ? archive.items : suppliersService.items).filter(
      (supplier) => !targetId || supplier.id === Number(targetId),
    ),
  );
  async function restore(supplier: Supplier) {
    await suppliersService.restore(supplier.id);
    await archive.reload();
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
      return visible;
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
      `¿Archivar el proveedor "${supplier.name}"?`,
      "Archivar proveedor",
    );
    if (ok) await suppliersService.remove(supplier.id);
  }
</script>

<div class="toolbar">
  {#if targetId}<Button variant="secondary" onclick={()=>goto("/suppliers")}>Ver todos</Button>{/if}
  <label><input type="checkbox" bind:checked={archive.active} /> Archivados</label>
  <div class="spacer"></div>
  <Button icon={Plus} onclick={openNew}>Nuevo proveedor</Button>
</div>

{#if suppliersService.error || archive.error}<p role="alert" class="error">{suppliersService.error || archive.error}</p><Button onclick={()=>archive.active?archive.reload():suppliersService.load()}>Reintentar</Button>
{:else if suppliersService.loading || archive.loading}<p role="status">Cargando proveedores…</p>
{:else if visible.length === 0}
  <EmptyState
    icon={Truck}
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
      {#if supplier.archived}<Button size="sm" onclick={()=>restore(supplier)}>Restaurar</Button>{:else}
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
          onclick={() => removeSupplier(supplier)}
        >
          <Trash size={16} />
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
