<script lang="ts">
  import { renderComponent } from "@tanstack/svelte-table";
  import type { Movement, MovementType } from "$lib/types";
  import { movementsStore } from "$lib/stores/movements.store.svelte";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import { createAppTable, createAppColumnHelper } from "$lib/components/ui/table";
  import DataTable from "$lib/components/ui/DataTable.svelte";
  import CancelMovementModal from "$lib/components/movements/CancelMovementModal.svelte";
  import { page } from "$app/state";
  import { getDb } from "$lib/db";
  import Select from "$lib/components/ui/Select.svelte";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import MovementTypeCell from "$lib/components/movements/MovementTypeCell.svelte";
  import MovementProductCell from "$lib/components/movements/MovementProductCell.svelte";
  import { formatNumber, formatDate } from "$lib/utils/format";
  import Button from "$lib/components/ui/Button.svelte";
  import { Plus, ArrowLeftRight, Trash } from "@lucide/svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";


  productsStore.load();

  const columnHelper = createAppColumnHelper<Movement>();

  let typeFilter = $state<"all" | MovementType>("all");
  let search = $state(""); let productFilter = $state(page.url.searchParams.get("product") ?? "");
  let supplierFilter = $state(""); let personFilter = $state(""); let from = $state(""); let to = $state("");
  let status = $state("all"); let currentPage = $state(0);
  let products = $state<{value:string;label:string}[]>([]), suppliers = $state<{value:string;label:string}[]>([]), persons = $state<{value:string;label:string}[]>([]);
  async function loadOptions() {
    const db = await getDb();
    const options = async (table:string) => (await db.select<{id:number;name:string}[]>(`SELECT id,name FROM ${table} ORDER BY name`)).map(r=>({value:String(r.id),label:r.name}));
    [products,suppliers,persons] = await Promise.all([options("products"),options("suppliers"),options("responsible_persons")]);
  }
  loadOptions();
  const query = $derived({search, product:productFilter, supplier:supplierFilter, person:personFilter, from,to,status,type:typeFilter});
  $effect(() => { query; currentPage=0; });
  $effect(() => { const filters={...query,page:currentPage}; const timer=setTimeout(()=>movementsStore.load(filters),150); return ()=>clearTimeout(timer); });


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
    columnHelper.accessor((m) => m.cancelled_at ? "Anulado" : m.kind === "adjustment" ? "Ajuste" : "Vigente", {
      id: "state", header: () => "Estado",
    }),
    columnHelper.accessor((m) => m.cancellation_reason ?? m.reason ?? m.notes ?? "—", { id:"reason", header:()=>"Motivo / notas" }),
    columnHelper.accessor("created_at", {
      header: () => "Fecha",
      cell: (info) => formatDate(info.getValue<string>()),
    }),
  ]);

  const data = $derived(movementsStore.items);

  const table = createAppTable({
    columns,
    defaultColumn: { enableSorting: false },
    get data() {
      return data;
    },
  });

  function openNew() {
    modalService.open(MovementFormModal);
  }

  function removeMovement(movement: Movement) { modalService.open(CancelMovementModal, { movement }); }

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

  <Button icon={Plus} onclick={openNew}>Nuevo movimiento</Button>
</div>

<div class="filter-grid">
  <label>Buscar<input class="field" bind:value={search} placeholder="Producto, proveedor, notas…" /></label>
  <div><Select label="Producto" bind:value={productFilter} options={[{value:"",label:"Todos"},...products]} /></div>
  <div><Select label="Proveedor" bind:value={supplierFilter} options={[{value:"",label:"Todos"},...suppliers]} /></div>
  <div><Select label="Responsable" bind:value={personFilter} options={[{value:"",label:"Todos"},...persons]} /></div>
  <label>Desde<input class="field" type="date" bind:value={from} /></label>
  <label>Hasta<input class="field" type="date" bind:value={to} /></label>
  <div><Select label="Estado" bind:value={status} options={[{value:"all",label:"Todos"},{value:"active",label:"Vigentes"},{value:"cancelled",label:"Anulados"}]} /></div>
</div>
{#if movementsStore.error}
  <p role="alert" class="error">{movementsStore.error}</p><Button onclick={()=>movementsStore.load()}>Reintentar</Button>
{:else if movementsStore.loading}<p role="status">Cargando movimientos…</p>
{:else if movementsStore.items.length === 0}
  <EmptyState
    icon={ArrowLeftRight}
    title="No hay movimientos para estos filtros"
    description="Registra entradas y salidas de stock para llevar el control del inventario."
    actionLabel="Nuevo movimiento"
    onAction={openNew}
  />
{:else}
  <DataTable
    {table}
    showSearch={false}
    searchPlaceholder="Buscar movimientos..."
    emptyText="No hay movimientos que coincidan con el filtro."
  >
    {#snippet actions(movement)}
      <button
        class="icon-btn danger"
        type="button"
        aria-label="Anular movimiento"
        disabled={!!movement.cancelled_at}
        onclick={() => removeMovement(movement)}
      >
        <Trash size={16} />
      </button>
    {/snippet}
  </DataTable>
{/if}

<div class="pagination">
  <Button variant="secondary" disabled={currentPage===0 || movementsStore.loading} onclick={()=>currentPage--}>Anterior</Button>
  <span>Página {currentPage+1} · {movementsStore.total} movimientos</span>
  <Button variant="secondary" disabled={(currentPage+1)*25>=movementsStore.total || movementsStore.loading} onclick={()=>currentPage++}>Siguiente</Button>
</div>
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