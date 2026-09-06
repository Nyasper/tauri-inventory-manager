<script lang="ts">
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
  import DataTable from "$lib/components/ui/DataTable.svelte";
  import ProductCard from "$lib/components/products/ProductCard.svelte";
  import ProductFormModal from "$lib/components/products/ProductFormModal.svelte";
  import ProductNameCell from "$lib/components/products/ProductNameCell.svelte";
  import ResolvedImage from "$lib/components/ui/ResolvedImage.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import { getStockStatus } from "$lib/utils/stock";
  import { formatNumber } from "$lib/utils/format";
  import Select from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  productsStore.load();

  const columnHelper = createAppColumnHelper<Product>();

  let search = $state("");
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
    columnHelper.accessor((p) => p.image, {
      id: "image",
      header: "",
      enableSorting: false,
      cell: (info) =>
        renderComponent(ResolvedImage, {
          src: info.row.original.image,
          size: 32,
        }),
    }),
    columnHelper.accessor("name", {
      header: () => "Nombre",
      cell: (info) =>
        renderComponent(ProductNameCell, { product: info.row.original }),
    }),
    columnHelper.accessor("category_name", {
      header: () => "Categoría",
      cell: (info) => info.getValue<string>() ?? "—",
    }),
    columnHelper.accessor("stock", {
      header: () => "Stock",
      meta: { align: "right" },
      cell: (info) => formatNumber(info.getValue<number>()),
    }),
    columnHelper.accessor(statusRank, {
      id: "status",
      header: () => "Estado",
      enableGlobalFilter: false,
      cell: (info) =>
        renderComponent(StockBadge, { status: statusOf(info.row.original) }),
    }),
  ]);

  const filtered = $derived.by(() => {
    const q = search.trim().toLowerCase();
    const cat = categoryFilter ? Number(categoryFilter) : null;
    return productsStore.items.filter((p) => {
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
    modalService.open(ProductFormModal, { product });
  }

  async function removeProduct(product: Product) {
    const ok = await modalService.confirm(
      `¿Eliminar el producto "${product.name}"?`,
      "Eliminar producto",
    );
    if (ok) {
      await productService.remove(product.id);
      await productsStore.load();
    }
  }
</script>

<div class="toolbar">
  <div class="search">
    <Icon name="search" size={16} />
    <input
      class="search-input"
      placeholder="Buscar productos..."
      bind:value={search}
      autocomplete="off"
    />
  </div>

  <div class="filter">
    <Select bind:value={categoryFilter} options={categoryOptions} />
  </div>

  <div class="spacer"></div>

  <div class="view-toggle">
    <button
      class="toggle-btn"
      class:active={productsStore.view === "list"}
      type="button"
      aria-label="Vista lista"
      onclick={() => productsStore.setView("list")}
    >
      <Icon name="list" size={16} />
    </button>
    <button
      class="toggle-btn"
      class:active={productsStore.view === "grid"}
      type="button"
      aria-label="Vista cuadrícula"
      onclick={() => productsStore.setView("grid")}
    >
      <Icon name="grid" size={16} />
    </button>
  </div>

  <Button icon="plus" onclick={openNew}>Nuevo producto</Button>
</div>

<div class="products">
  {#if productsStore.items.length === 0 && !productsStore.loading}
    <EmptyState
      title="No hay productos"
      description="Crea tu primer producto para empezar a gestionar el inventario."
      actionLabel="Nuevo producto"
      onAction={openNew}
    />
  {:else if filtered.length === 0}
    <EmptyState
      icon="search"
      title="Sin resultados"
      description="Ningún producto coincide con la búsqueda o el filtro seleccionado."
    />
  {:else if productsStore.view === "grid"}
    <div class="grid">
      {#each filtered as product (product.id)}
        <ProductCard
          {product}
          onEdit={openEdit}
          onDelete={removeProduct}
        />
      {/each}
    </div>
  {:else}
    <DataTable
      {table}
      onRowClick={openEdit}
      showSearch={false}
      searchPlaceholder="Buscar productos..."
    >
      {#snippet actions(product)}
        <div class="actions">
          <button
            class="icon-btn"
            type="button"
            aria-label="Editar"
            onclick={(e) => {
              e.stopPropagation();
              openEdit(product);
            }}
          >
            <Icon name="edit" size={16} />
          </button>
          <button
            class="icon-btn danger"
            type="button"
            aria-label="Eliminar"
            onclick={(e) => {
              e.stopPropagation();
              removeProduct(product);
            }}
          >
            <Icon name="trash" size={16} />
          </button>
        </div>
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

  .view-toggle {
    display: flex;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
    background: var(--surface);
  }

  .toggle-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 34px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .toggle-btn.active {
    background: var(--primary-soft);
    color: var(--primary);
  }

  .products {
    display: flex;
    flex-direction: column;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
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