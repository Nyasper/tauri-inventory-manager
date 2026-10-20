<script lang="ts">
  import { goto } from "$app/navigation";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import { page } from "$app/state";
  import { renderComponent } from "@tanstack/svelte-table";
  import type { SortingState } from "@tanstack/svelte-table";
  import type { Product } from "$lib/types";
  import { getProducts } from "$lib/services/db.service";
  import { productsService } from "$lib/services/products.service.svelte";
  import { categoriesService } from "$lib/services/categories.service.svelte";
  import { settingsService } from "$lib/services/settings.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { createAppTable, createAppColumnHelper } from "$lib/components/ui/table";
  import { ArchivedList } from "$lib/utils/archived.svelte";
  import DataTable from "$lib/components/ui/DataTable.svelte";
  import ProductFormModal from "$lib/components/products/ProductFormModal.svelte";
  import DeleteRecordModal from "$lib/components/ui/DeleteRecordModal.svelte";
  import ProductNameCell from "$lib/components/products/ProductNameCell.svelte";
  import ProductCategoryCell from "$lib/components/products/ProductCategoryCell.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import { resolveProductStockStatus } from "$lib/utils/stock";
  import { formatNumber } from "$lib/utils/format";
  import Select from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { Search, Plus, Minus, History, Pencil, Trash, Archive, Undo2 } from "@lucide/svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  productsService.load();

  const columnHelper = createAppColumnHelper<Product>();

  let search = $state("");
  const archivedList = new ArchivedList<Product>(getProducts);
  const sourceItems = $derived(
    archivedList.showingArchived ? archivedList.items : productsService.items,
  );
  let stockFilter = $derived(page.url.searchParams.get("stock") ?? "");
  const targetId = $derived(page.url.searchParams.get("id"));
  let categoryFilter = $state("");

  $effect(() => {
    if (categoryFilter === "" || categoryFilter === "none") return;
    const exists = categoriesService.items.some(
      (category) => String(category.id) === categoryFilter,
    );
    if (!exists) categoryFilter = "";
  });

  function getProductStockStatus(product: Product) {
    return resolveProductStockStatus(
      product,
      settingsService.settings.stock_low_threshold,
      settingsService.settings.stock_medium_threshold,
    );
  }

  function thresholdTitle(product: Product): string {
    if (product.ignore_stock_alerts) {
      return `Alertas desactivadas para este producto (Stock actual: ${formatNumber(product.stock)} ${product.base_presentation_name ?? ""})`;
    }
    if (product.stock_low_threshold != null && product.stock_medium_threshold != null) {
      return `Límites personalizados: Bajo ≤ ${product.stock_low_threshold}, Medio ≤ ${product.stock_medium_threshold}`;
    }
    return `Límites globales: Bajo ≤ ${settingsService.settings.stock_low_threshold}, Medio ≤ ${settingsService.settings.stock_medium_threshold}`;
  }

  function statusRank(product: Product): number {
    if (product.ignore_stock_alerts) {
      return 4;
    }
    switch (getProductStockStatus(product)) {
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
      cell: (info) => renderComponent(ProductNameCell, { product: info.row.original }),
    }),
    columnHelper.accessor("category_name", {
      header: () => "Categoría",
      meta: { width: "22%" },
      cell: (info) => renderComponent(ProductCategoryCell, { product: info.row.original }),
    }),
    columnHelper.accessor("stock", {
      header: () => "Stock",
      meta: { align: "center", width: "14%" },
      cell: (info) =>
        `${formatNumber(info.getValue<number>())} ${info.row.original.base_presentation_name ?? ""}`,
    }),
    columnHelper.accessor(statusRank, {
      id: "status",
      header: () => "Estado",
      enableGlobalFilter: false,
      meta: { align: "center", width: "14%" },
      cell: (info) =>
        renderComponent(StockBadge, {
          status: getProductStockStatus(info.row.original),
          title: thresholdTitle(info.row.original),
          ignored: Boolean(info.row.original.ignore_stock_alerts),
        }),
    }),
  ]);

  const filteredProducts = $derived.by(() => {
    const q = search.trim().toLowerCase();
    const cat = categoryFilter && categoryFilter !== "none" ? Number(categoryFilter) : null;
    return sourceItems.filter((p) => {
      if (targetId && p.id !== Number(targetId)) return false;
      if (
        stockFilter === "low" &&
        (p.ignore_stock_alerts || !["low", "out"].includes(getProductStockStatus(p)))
      )
        return false;
      if (categoryFilter === "none") {
        if (p.category_id !== null) return false;
      } else if (cat !== null && p.category_id !== cat) return false;
      if (!q) return true;
      return p.name.toLowerCase().includes(q) || (p.description ?? "").toLowerCase().includes(q);
    });
  });

  const initialSort: SortingState =
    page.url.searchParams.get("sort") === "status" ? [{ id: "status", desc: false }] : [];

  const table = createAppTable({
    columns,
    get data() {
      return filteredProducts;
    },
    initialState: { sorting: initialSort, pagination: { pageIndex: 0, pageSize: 50 } },
  });

  const categoryOptions = $derived([
    { value: "", label: "Todas las categorías" },
    { value: "none", label: "Sin categoría" },
    ...categoriesService.items.map((c) => ({
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

  function restoreArchived(product: Product) {
    void productsService.restore(product.id).then(() => archivedList.reload());
  }

  async function archiveProduct(product: Product) {
    const ok = await modalService.confirm(
      `¿Archivar el producto "${product.name}"? Se conservará el historial. El stock debe ser cero.`,
      "Archivar producto",
    );
    if (ok) {
      await productsService.archive(product.id);
    }
  }

  function deletePermanently(product: Product) {
    modalService.open(DeleteRecordModal, {
      recordType: "product",
      name: product.name,
      onConfirm: async (password: string) => {
        await productsService.deletePermanently(product.id, password);
        await archivedList.reload();
      },
    });
  }
</script>

<div class="toolbar">
  <label><input type="checkbox" bind:checked={archivedList.showingArchived} /> Archivados</label>
  <label
    ><input
      type="checkbox"
      checked={stockFilter === "low"}
      onchange={(e) => (stockFilter = e.currentTarget.checked ? "low" : "")}
    /> Stock bajo / agotado</label
  >
  {#if targetId}<Button variant="secondary" onclick={() => goto("/products")}>Ver todos</Button
    >{/if}
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
  {#if productsService.error || archivedList.error}<p class="error" role="alert">
      {productsService.error || archivedList.error}
    </p>
    <Button
      onclick={() =>
        archivedList.showingArchived ? archivedList.reload() : productsService.load()}
      >Reintentar</Button
    >
  {:else if productsService.loading || archivedList.loading}<p role="status">Cargando productos…</p>
  {:else if sourceItems.length === 0}
    {#if archivedList.showingArchived}
      <EmptyState
        icon={Archive}
        title="No hay productos archivados"
        description="Los productos que archives aparecerán aquí."
      />
    {:else}
      <EmptyState
        title="No hay productos"
        description="Crea tu primer producto para empezar a gestionar el inventario."
        actionLabel="Nuevo producto"
        onAction={openNew}
      />
    {/if}
  {:else if filteredProducts.length === 0}
    <EmptyState
      icon={Search}
      title="Sin resultados"
      description="Ningún producto coincide con la búsqueda o el filtro seleccionado."
    />
  {:else}
    <DataTable
      {table}
      fixedLayout
      showPagination
      onRowClick={openEdit}
      rowClickable={(product) => !product.isArchived}
      rowActionLabel={(product) => `Editar ${product.name}`}
      showSearch={false}
    >
      {#snippet actions(product)}
        {#if product.isArchived}
          <div class="actions">
            <button
              class="icon-btn"
              type="button"
              title="Restaurar"
              aria-label={`Restaurar ${product.name}`}
              onclick={(e) => {
                e.stopPropagation();
                restoreArchived(product);
              }}
            >
              <Undo2 size={16} />
            </button>
            <button
              class="icon-btn danger"
              type="button"
              title="Eliminar definitivamente"
              aria-label={`Eliminar definitivamente ${product.name}`}
              onclick={(e) => {
                e.stopPropagation();
                deletePermanently(product);
              }}
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
              aria-label={`Registrar entrada de ${product.name}`}
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
              title="Registrar salida"
              aria-label={`Registrar salida de ${product.name}`}
              onclick={(e) => {
                e.stopPropagation();
                modalService.open(MovementFormModal, {
                  initialProductId: String(product.id),
                  initialType: "out",
                });
              }}
            >
              <Minus size={16} />
            </button>
            <button
              class="icon-btn"
              type="button"
              title="Ver movimientos"
              aria-label={`Ver movimientos de ${product.name}`}
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
              title={product.stock > 0 ? "Archivar (el stock debe ser 0)." : "Archivar"}
            >
              <button
                class="icon-btn danger"
                type="button"
                aria-label="Archivar"
                disabled={product.stock > 0}
                onclick={(e) => {
                  e.stopPropagation();
                  archiveProduct(product);
                }}
              >
                <Archive size={16} />
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
