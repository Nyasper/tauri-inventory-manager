<script lang="ts">
  import { renderComponent } from "@tanstack/svelte-table";
  import type { Movement, MovementType } from "#lib/types.js";
  import { movementsService } from "#lib/services/movements.service.svelte.js";
  import { settingsService } from "#lib/services/settings.service.svelte.js";
  import { productsService } from "#lib/services/products.service.svelte.js";
  import { suppliersService } from "#lib/services/suppliers.service.svelte.js";
  import { modalService } from "#lib/services/modal.service.svelte.js";
  import { createAppTable, createAppColumnHelper } from "#lib/components/ui/table.js";
  import DataTable from "#lib/components/ui/DataTable.svelte";
  import CancelMovementModal from "#lib/components/movements/CancelMovementModal.svelte";
  import DeleteMovementsModal from "#lib/components/movements/DeleteMovementsModal.svelte";
  import { page } from "$app/state";
  import Select from "#lib/components/ui/Select.svelte";
  import MovementFormModal from "#lib/components/movements/MovementFormModal.svelte";
  import MovementDetailModal from "#lib/components/movements/MovementDetailModal.svelte";
  import EditMovementSupplierModal from "#lib/components/movements/EditMovementSupplierModal.svelte";
  import MovementTypeCell from "#lib/components/movements/MovementTypeCell.svelte";
  import MovementProductCell from "#lib/components/movements/MovementProductCell.svelte";
  import MovementSupplierCell from "#lib/components/movements/MovementSupplierCell.svelte";
  import { formatNumber, formatDateTime } from "#lib/utils/format.js";
  import Button from "#lib/components/ui/Button.svelte";
  import {
    Plus,
    Pencil,
    ArrowLeftRight,
    Trash,
    Undo2,
    ChevronFirst,
    ChevronLeft,
    ChevronRight,
    ChevronLast,
  } from "@lucide/svelte";
  import EmptyState from "#lib/components/ui/EmptyState.svelte";

  productsService.load();

  const columnHelper = createAppColumnHelper<Movement>();

  let typeFilter = $state<"all" | MovementType>("all");
  let searchInput = $state("");
  let search = $state("");
  let productFilter = $derived(page.url.searchParams.get("product") ?? "");
  let supplierFilter = $state("");
  let fromDate = $state("");
  let toDate = $state("");
  let statusFilter = $state("all");

  const productOptions = $derived(
    productsService.items.map((p) => ({ value: String(p.id), label: p.name })),
  );
  const supplierOptions = $derived(
    suppliersService.items.map((s) => ({ value: String(s.id), label: s.name })),
  );

  $effect(() => {
    const value = searchInput;
    const timer = setTimeout(() => {
      search = value;
    }, 200);
    return () => clearTimeout(timer);
  });

  const pageSize = $derived(settingsService.settings.movements_page_size);

  const filters = $derived({
    search,
    product: productFilter,
    supplier: supplierFilter,
    from: fromDate,
    to: toDate,
    status: statusFilter,
    type: typeFilter,
    pageSize,
  });
  const queryKey = $derived(JSON.stringify(filters));
  let paginationState = $state<{ key: string; page: number } | null>(null);
  const currentPage = $derived(
    paginationState !== null && paginationState.key === queryKey ? paginationState.page : 0,
  );

  const lastPage = $derived(Math.max(0, Math.ceil(movementsService.total / pageSize) - 1));

  function goToPage(targetPage: number) {
    paginationState = { key: queryKey, page: targetPage };
  }

  function reload() {
    void movementsService.load({ ...filters, page: currentPage });
  }

  $effect(() => {
    movementsService.load({ ...filters, page: currentPage });
  });

  $effect(() => {
    if (movementsService.loading || movementsService.total === 0) return;
    if (currentPage > lastPage) goToPage(lastPage);
  });

  const columns = columnHelper.columns([
    columnHelper.accessor((m) => (m.type === "in" ? "Entrada" : "Salida"), {
      id: "type",
      header: () => "Tipo",
      meta: { align: "center" },
      cell: (info) => renderComponent(MovementTypeCell, { type: info.row.original.type }),
    }),
    columnHelper.accessor("product_name", {
      header: () => "Producto",
      cell: (info) => renderComponent(MovementProductCell, { movement: info.row.original }),
    }),
    columnHelper.accessor("presentation_name", {
      header: () => "Presentación",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
    columnHelper.accessor("quantity", {
      header: () => "Cantidad",
      meta: { align: "center" },
      cell: (info) => formatNumber(info.getValue<number>()),
    }),
    columnHelper.accessor("supplier_name", {
      header: () => "Proveedor",
      cell: (info) => renderComponent(MovementSupplierCell, { movement: info.row.original }),
    }),
    columnHelper.accessor(
      (m) => (m.cancelled_at ? "Anulado" : m.kind === "adjustment" ? "Ajuste" : "Vigente"),
      {
        id: "status",
        header: () => "Estado",
        meta: { align: "center" },
      },
    ),
    columnHelper.accessor((m) => m.cancellation_reason ?? m.reason ?? m.notes ?? "—", {
      id: "reason",
      header: () => "Motivo / notas",
    }),
    columnHelper.accessor("created_at", {
      header: () => "Fecha",
      meta: { align: "center" },
      cell: (info) => formatDateTime(info.getValue<string>()),
    }),
  ]);

  const table = createAppTable({
    columns,
    defaultColumn: { enableSorting: false },
    get data() {
      return movementsService.items;
    },
    manualPagination: true,
  });

  function openNew() {
    modalService.open(MovementFormModal);
  }

  function cancelMovement(movement: Movement) {
    modalService.open(CancelMovementModal, { movement });
  }

  function editSupplier(movement: Movement) {
    modalService.open(EditMovementSupplierModal, { movement });
  }

  const typeFilterOptions: { value: "all" | MovementType; label: string }[] = [
    { value: "all", label: "Todos" },
    { value: "in", label: "Entradas" },
    { value: "out", label: "Salidas" },
  ];
</script>

<div class="page">
  <div class="toolbar">
    <div class="type-filter">
      {#each typeFilterOptions as filter (filter.value)}
        <button
          class={["filter-btn", { active: typeFilter === filter.value }]}
          type="button"
          onclick={() => (typeFilter = filter.value)}
        >
          {filter.label}
        </button>
      {/each}
    </div>

    <div class="spacer"></div>

    <Button icon={Plus} onclick={openNew}>Nuevo movimiento</Button>
  </div>

  <div class="filter-grid">
    <label>
      Buscar
      <input
        class="field"
        bind:value={searchInput}
        placeholder="Producto, proveedor, notas…"
        aria-label="Buscar movimientos"
      />
    </label>
    <div>
      <Select
        label="Producto"
        bind:value={productFilter}
        options={[{ value: "", label: "Todos" }, ...productOptions]}
      />
    </div>
    <div>
      <Select
        label="Proveedor"
        bind:value={supplierFilter}
        options={[{ value: "", label: "Todos" }, ...supplierOptions]}
      />
    </div>
    <label>Desde<input class="field" type="date" bind:value={fromDate} /></label>
    <label>Hasta<input class="field" type="date" bind:value={toDate} /></label>
    <div>
      <Select
        label="Estado"
        bind:value={statusFilter}
        options={[
          { value: "all", label: "Todos" },
          { value: "active", label: "Vigentes" },
          { value: "cancelled", label: "Anulados" },
        ]}
      />
    </div>
  </div>

  <div class={["table-state", { updating: movementsService.loading }]}>
    {#if movementsService.loading && movementsService.items.length > 0}
      <span class="updating-label" role="status">Actualizando…</span>
    {/if}
    {#if movementsService.error}
      <div class="state">
        <p role="alert" class="error">{movementsService.error}</p>
        <Button onclick={reload}>Reintentar</Button>
      </div>
    {:else if movementsService.loading && movementsService.items.length === 0}
      <p role="status">Cargando movimientos…</p>
    {:else if movementsService.items.length === 0}
      <EmptyState
        icon={ArrowLeftRight}
        title="No hay movimientos para estos filtros"
        description="Registra entradas y salidas de stock para llevar el control del inventario."
        actionLabel="Nuevo movimiento"
        onAction={openNew}
      />
    {:else}
      <DataTable
        fillHeight
        {table}
        showSearch={false}
        emptyText="No hay movimientos que coincidan con el filtro."
        onRowClick={(movement) => modalService.open(MovementDetailModal, { movement })}
        rowActionLabel={(movement) => `Ver detalle del movimiento #${movement.id}`}
      >
        {#snippet actions(movement)}
          <div class="actions">
            {#if movement.type === "in" && !movement.cancelled_at}
              <button
                class="icon-btn"
                type="button"
                title="Editar proveedor"
                aria-label={`Editar proveedor del movimiento #${movement.id}`}
                onclick={(e) => {
                  e.stopPropagation();
                  editSupplier(movement);
                }}
              >
                <Pencil size={16} />
              </button>
            {/if}
            <button
              class="icon-btn danger"
              type="button"
              title={movement.cancelled_at ? "Movimiento anulado" : "Anular movimiento"}
              aria-label="Anular movimiento"
              disabled={!!movement.cancelled_at}
              onclick={(e) => {
                e.stopPropagation();
                cancelMovement(movement);
              }}
            >
              <Undo2 size={16} />
            </button>
          </div>
        {/snippet}
      </DataTable>
    {/if}
  </div>

  <div class="pagination">
    <Button
      variant="secondary"
      icon={Trash}
      onclick={() => modalService.open(DeleteMovementsModal)}
    >
      Eliminar movimientos
    </Button>
    <div class="spacer"></div>
    <div class="pager">
      <Button
        variant="secondary"
        icon={ChevronFirst}
        ariaLabel="Primera página"
        title="Primera página"
        disabled={currentPage === 0 || movementsService.loading}
        onclick={() => goToPage(0)}
      />
      <Button
        variant="secondary"
        icon={ChevronLeft}
        ariaLabel="Página anterior"
        title="Página anterior"
        disabled={currentPage === 0 || movementsService.loading}
        onclick={() => goToPage(currentPage - 1)}
      />
      <span>Página {currentPage + 1} de {lastPage + 1} · {movementsService.total} movimientos</span>
      <Button
        variant="secondary"
        icon={ChevronRight}
        ariaLabel="Página siguiente"
        title="Página siguiente"
        disabled={currentPage >= lastPage || movementsService.loading}
        onclick={() => goToPage(currentPage + 1)}
      />
      <Button
        variant="secondary"
        icon={ChevronLast}
        ariaLabel="Última página"
        title="Última página"
        disabled={currentPage >= lastPage || movementsService.loading}
        onclick={() => goToPage(lastPage)}
      />
    </div>
  </div>
</div>

<style>
  .page {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
  }

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

  .pager {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .actions {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
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

  .icon-btn:hover:not(:disabled) {
    background: var(--surface-2);
    color: var(--text);
  }

  .icon-btn.danger:hover:not(:disabled) {
    background: var(--danger-soft);
    color: var(--danger);
  }

  .table-state {
    position: relative;
    flex: 1;
    min-height: 120px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }

  .table-state.updating {
    cursor: progress;
  }

  .updating-label {
    position: absolute;
    top: 0.5rem;
    right: 0.75rem;
    z-index: 2;
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--border);
    border-radius: 999px;
    background: var(--surface);
    color: var(--text-muted);
    box-shadow: var(--shadow-sm);
    font-size: 0.75rem;
  }
</style>
