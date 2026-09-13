<script lang="ts">
  import { goto } from "$app/navigation";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import { page } from "$app/state";
  import { renderComponent } from "@tanstack/svelte-table";
  import type { SortingState } from "@tanstack/svelte-table";
  import type { Product } from "$lib/types";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { categoriesStore } from "$lib/stores/categories.store.svelte";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import { productService } from "$lib/services/product.service";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import { createAppTable, createAppColumnHelper } from "$lib/components/ui/table";
  import { ArchivedList } from "$lib/utils/archived.svelte";
  import DataTable from "$lib/components/ui/DataTable.svelte";
  import ProductFormModal from "$lib/components/products/ProductFormModal.svelte";
  import ProductNameCell from "$lib/components/products/ProductNameCell.svelte";
  import ProductCategoryCell from "$lib/components/products/ProductCategoryCell.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import { getStockStatus } from "$lib/utils/stock";
  import { formatNumber } from "$lib/utils/format";
  import Select from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import {
    Search,
    Plus,
    History,
    Pencil,
    Trash,
  } from "@lucide/svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  productsStore.load();

  const columnHelper = createAppColumnHelper<Product>();

  let search = $state("");
  const archive = new ArchivedList<Product>((archived) =>
    productService.list(archived),
  );
  const visible = $derived(archive.active ? archive.items : productsStore.items);
  let stockFilter = $state("");
  $effect(()=>{stockFilter=page.url.searchParams.get("stock")??"";});
  const targetId = $derived(page.url.searchParams.get("id"));
  let categoryFilter = $state("");

  function statusOf(product: Product) {
    return getStockStatus(
      product.stock,
      settingsStore.settings.stock_low_threshold,
      settingsStore.settings.stock_medium_threshold,
    );
  }

  function statusRank(product: Product): number {
    switch (statusOf(product)) {
      case "out":
        return 0;
      case "low":
        return 1;
      case "medium":
        return 2;
      case "high":
        return 3;
    }
  }

  const columns = columnHelper.columns([
    columnHelper.accessor("name", {
      header: () => "Nombre",
      meta: { width: "34%" },
      cell: (info) =>
        renderComponent(ProductNameCell, { product: info.row.original }),
    }),
    columnHelper.accessor("category_name", {
      header: () => "Categoría",
      meta: { width: "22%" },
      cell: (info) =>
        renderComponent(ProductCategoryCell, { product: info.row.original }),
    }),
    columnHelper.accessor("stock", {
      header: () => "Stock",
      meta: { align: "center", width: "14%" },
      cell: (info) => `${formatNumber(info.getValue<number>())} ${info.row.original.base_unit ?? ""}`,
    }),
    columnHelper.accessor(statusRank, {
      id: "status",
      header: () => "Estado",
      enableGlobalFilter: false,
      meta: { align: "center", width: "14%" },
      cell: (info) =>
        renderComponent(StockBadge, { status: statusOf(info.row.original) }),
    }),
  ]);

  const filtered = $derived.by(() => {
    const q = search.trim().toLowerCase();
    const cat = categoryFilter ? Number(categoryFilter) : null;
    return visible.filter((p) => {
      if (targetId && p.id !== Number(targetId)) return false;
      if (stockFilter === "low" && !["low","out"].includes(statusOf(p))) return false;
      if (cat !== null && p.category_id !== cat) return false;
      if (!q) return true;
      return (
        p.name.toLowerCase().includes(q) ||
        (p.description ?? "").toLowerCase().includes(q)
      );
    });
  });

  const initialSort: SortingState =
    page.url.searchParams.get("sort") === "status"
      ? [{ id: "status", desc: false }]
      : [];

  const table = createAppTable({
    columns,
    get data() {
      return filtered;
    },
    initialState: { sorting: initialSort },
  });

  const categoryOptions = $derived([
    { value: "", label: "Todas las categorías" },
    ...categoriesStore.items.map((c) => ({
      value: String(c.id),
      label: c.name,
    })),
  ]);

  function openNew() {
    modalService.open(ProductFormModal);
  }

  function openEdit(product: Product) {
    if (product.archived) {
      productService
        .restore(product.id)
        .then(async () => {
          await archive.reload();
          await productsStore.load();
        });
      return;
    }
    modalService.open(ProductFormModal, { product });
  }

  async function removeProduct(product: Product) {
    const ok = await modalService.confirm(
      `¿Archivar el producto "${product.name}"? Se conservará el historial. El stock debe ser cero.`,
      "Archivar producto",
    );
    if (ok) {
      await productService.remove(product.id);
      await productsStore.load();
    }
  }
</script>

<div class="toolbar">
  <label><input type="checkbox" bind:checked={archive.active} /> Archivados</label>
  <label><input type="checkbox" checked={stockFilter==="low"} onchange={(e)=>stockFilter=e.currentTarget.checked?"low":""} /> Stock bajo / agotado</label>
  {#if targetId}<Button variant="secondary" onclick={()=>goto("/products")}>Ver todos</Button>{/if}
  <div class="search">
    <Search size={16} />
    <input
      class="search-input"
      placeholder="Buscar productos..."
      aria-label="Buscar productos"
      bind:value={search}
      autocomplete="off"
    />
  </div>

  <div class="filter">
    <Select ariaLabel="Categoría" bind:value={categoryFilter} options={categoryOptions} />
  </div>

  <div class="spacer"></div>

  <Button icon={Plus} onclick={openNew}>Nuevo producto</Button>
</div>

<div class="products">
  {#if productsStore.error || archive.error}<p class="error" role="alert">{productsStore.error || archive.error}</p><Button onclick={()=>archive.active?archive.reload():productsStore.load()}>Reintentar</Button>
  {:else if productsStore.loading || archive.loading}<p role="status">Cargando productos…</p>
  {:else if visible.length === 0}
    <EmptyState
      title="No hay productos"
      description="Crea tu primer producto para empezar a gestionar el inventario."
      actionLabel="Nuevo producto"
      onAction={openNew}
    />
  {:else if filtered.length === 0}
    <EmptyState
      icon={Search}
      title="Sin resultados"
      description="Ningún producto coincide con la búsqueda o el filtro seleccionado."
    />
  {:else}
    <DataTable
      {table}
      fixed
      onRowClick={openEdit}
      showSearch={false}
      searchPlaceholder="Buscar productos..."
    >
      {#snippet actions(product)}
      {#if product.archived}
        <Button size="sm" onclick={() => openEdit(product)}>Restaurar</Button>
      {:else}
        <div class="actions">
          <button
            class="icon-btn"
            type="button"
            title="Registrar movimiento"
            aria-label={`Registrar movimiento de ${product.name}`}
            onclick={(e) => {
              e.stopPropagation();
              modalService.open(MovementFormModal, {
                initialProductId: String(product.id),
              });
            }}
          >
            <Plus size={16} />
          </button>
          <button
            class="icon-btn"
            type="button"
            title="Ver historial"
            aria-label={`Ver historial de ${product.name}`}
            onclick={(e) => {
              e.stopPropagation();
              goto(`/movements?product=${product.id}`);
            }}
          >
            <History size={16} />
          </button>
          <button
            class="icon-btn"
            type="button"
            title="Editar"
            aria-label="Editar"
            onclick={(e) => {
              e.stopPropagation();
              openEdit(product);
            }}
          >
            <Pencil size={16} />
          </button>
          <span
            class="archive-wrap"
            title={product.stock > 0
              ? "El stock debe ser 0. Registra un ajuste antes de archivarlo."
              : "Archivar"}
          >
            <button
              class="icon-btn danger"
              type="button"
              aria-label="Archivar"
              disabled={product.stock > 0}
              onclick={(e) => {
                e.stopPropagation();
                removeProduct(product);
              }}
            >
              <Trash size={16} />
            </button>
          </span>
        </div>
      {/if}
      {/snippet}
    </DataTable>
  {/if}
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1.25rem;
  }

  .search {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    width: 260px;
  }

  .search-input {
    flex: 1;
    border: none;
    outline: none;
    background: transparent;
    padding: 0.5rem 0;
    color: var(--text);
  }

  .filter {
    width: 200px;
  }

  .spacer {
    flex: 1;
  }

  .products {
    display: flex;
    flex-direction: column;
  }

  .actions {
    display: flex;
    gap: 0.125rem;
  }

  .archive-wrap {
    display: inline-flex;
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
