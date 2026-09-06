<script lang="ts">
  import type { Product } from "$lib/types";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { categoriesStore } from "$lib/stores/categories.store.svelte";
  import { productService } from "$lib/services/product.service";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import ProductList from "$lib/components/products/ProductList.svelte";
  import ProductCard from "$lib/components/products/ProductCard.svelte";
  import ProductFormModal from "$lib/components/products/ProductFormModal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  productsStore.load();

  let search = $state(productsStore.search);
  let categoryFilter = $state("");

  $effect(() => {
    productsStore.search = search;
  });

  $effect(() => {
    productsStore.categoryFilter = categoryFilter
      ? Number(categoryFilter)
      : null;
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
  {:else if productsStore.filtered.length === 0}
    <EmptyState
      icon="search"
      title="Sin resultados"
      description="Ningún producto coincide con la búsqueda o el filtro seleccionado."
    />
  {:else if productsStore.view === "grid"}
    <div class="grid">
      {#each productsStore.filtered as product (product.id)}
        <ProductCard
          {product}
          onEdit={openEdit}
          onDelete={removeProduct}
        />
      {/each}
    </div>
  {:else}
    <ProductList
      products={productsStore.filtered}
      onEdit={openEdit}
      onDelete={removeProduct}
    />
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
</style>
