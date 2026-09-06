<script lang="ts">
  import { renderComponent } from "@tanstack/svelte-table";
  import type { Movement, MovementType } from "$lib/types";
  import { movementsStore } from "$lib/stores/movements.store.svelte";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import { createAppTable, createAppColumnHelper } from "$lib/components/ui/table";
  import DataTable from "$lib/components/ui/DataTable.svelte";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import MovementTypeCell from "$lib/components/movements/MovementTypeCell.svelte";
  import MovementProductCell from "$lib/components/movements/MovementProductCell.svelte";
  import { formatNumber, formatDate } from "$lib/utils/format";
  import Button from "$lib/components/ui/Button.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  movementsStore.load();
  productsStore.load();

  const columnHelper = createAppColumnHelper<Movement>();

  let typeFilter = $state<"all" | MovementType>("all");

  const columns = columnHelper.columns([
    columnHelper.accessor((m) => (m.type === "in" ? "Entrada" : "Salida"), {
      id: "type",
      header: () => "Tipo",
      cell: (info) =>
        renderComponent(MovementTypeCell, { type: info.row.original.type }),
    }),
    columnHelper.accessor("product_name", {
      header: () => "Producto",
      cell: (info) =>
        renderComponent(MovementProductCell, { movement: info.row.original }),
    }),
    columnHelper.accessor("presentation_name", {
      header: () => "Presentación",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
    columnHelper.accessor("quantity", {
      header: () => "Cantidad",
      meta: { align: "right" },
      cell: (info) => formatNumber(info.getValue<number>()),
    }),
    columnHelper.accessor("supplier_name", {
      header: () => "Proveedor",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
    columnHelper.accessor("person_name", {
      header: () => "Responsable",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
    columnHelper.accessor("created_at", {
      header: () => "Fecha",
      cell: (info) => formatDate(info.getValue<string>()),
    }),
  ]);

  const data = $derived(
    typeFilter === "all"
      ? movementsStore.items
      : movementsStore.items.filter((m) => m.type === typeFilter),
  );

  const table = createAppTable({
    columns,
    get data() {
      return data;
    },
  });

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

  const filters: { value: "all" | MovementType; label: string }[] = [
    { value: "all", label: "Todos" },
    { value: "in", label: "Entradas" },
    { value: "out", label: "Salidas" },
  ];
</script>

<div class="toolbar">
  <div class="type-filter">
    {#each filters as filter (filter.value)}
      <button
        class="filter-btn"
        class:active={typeFilter === filter.value}
        type="button"
        onclick={() => (typeFilter = filter.value)}
      >
        {filter.label}
      </button>
    {/each}
  </div>

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
  <DataTable
    {table}
    searchPlaceholder="Buscar movimientos..."
    emptyText="No hay movimientos que coincidan con el filtro."
  >
    {#snippet actions(movement)}
      <button
        class="icon-btn danger"
        type="button"
        aria-label="Eliminar"
        onclick={() => removeMovement(movement)}
      >
        <Icon name="trash" size={16} />
      </button>
    {/snippet}
  </DataTable>
{/if}

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1.25rem;
  }

  .type-filter {
    display: flex;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
    background: var(--surface);
  }

  .filter-btn {
    border: none;
    background: transparent;
    color: var(--text-muted);
    padding: 0.5rem 0.875rem;
    font-weight: 500;
    cursor: pointer;
  }

  .filter-btn + .filter-btn {
    border-left: 1px solid var(--border);
  }

  .filter-btn.active {
    background: var(--primary-soft);
    color: var(--primary);
  }

  .spacer {
    flex: 1;
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