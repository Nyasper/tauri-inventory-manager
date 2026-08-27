<script lang="ts">
  import type { Movement } from "$lib/types";
  import { movementsStore } from "$lib/stores/movements.store.svelte";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import MovementList from "$lib/components/movements/MovementList.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  movementsStore.load();
  productsStore.load();

  function openNew() {
    modalService.open(MovementFormModal);
  }

  async function removeMovement(movement: Movement) {
    const ok = await modalService.confirm(
      "¿Eliminar este movimiento? El stock del producto se revertirá.",
      "Eliminar movimiento",
    );
    if (ok) {
      await movementsStore.remove(movement.id);
      await productsStore.load();
    }
  }
</script>

<div class="toolbar">
  <div class="spacer"></div>
  <Button icon="plus" onclick={openNew}>Nuevo movimiento</Button>
</div>

{#if movementsStore.items.length === 0}
  <EmptyState
    icon="swap"
    title="No hay movimientos"
    description="Registra entradas y salidas de stock para llevar el control del inventario."
    actionLabel="Nuevo movimiento"
    onAction={openNew}
  />
{:else}
  <MovementList movements={movementsStore.items} onDelete={removeMovement} />
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
