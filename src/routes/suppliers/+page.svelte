<script lang="ts">
  import type { Supplier } from "$lib/types";
  import { suppliersStore } from "$lib/stores/suppliers.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import SupplierList from "$lib/components/suppliers/SupplierList.svelte";
  import SupplierFormModal from "$lib/components/suppliers/SupplierFormModal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  suppliersStore.load();

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
  <SupplierList
    suppliers={suppliersStore.items}
    onEdit={openEdit}
    onDelete={removeSupplier}
  />
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
</style>
