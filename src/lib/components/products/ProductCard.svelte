<script lang="ts">
  import type { Product } from "$lib/types";
  import { getStockStatus } from "$lib/utils/stock";
  import { formatNumber } from "$lib/utils/format";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import ProductIcon from "$lib/components/ui/ProductIcon.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import { History, Pencil, Plus, Trash } from "@lucide/svelte";
  import { receive, send } from "$lib/components/ui/transitions";

  let {
    product,
    onEdit,
    onDelete,
    onMovement,
    onHistory,
  }: {
    product: Product;
    onEdit: (product: Product) => void;
    onDelete: (product: Product) => void;
    onMovement: (product: Product) => void;
    onHistory: (product: Product) => void;
  } = $props();

  const status = $derived(
    getStockStatus(
      product.stock,
      settingsStore.settings.stock_low_threshold,
      settingsStore.settings.stock_medium_threshold,
    ),
  );

  function handleKeydown(event: KeyboardEvent) {
    if (event.target !== event.currentTarget) return;
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onEdit(product);
    }
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
<article
  class="card"
  role="button"
  tabindex="0"
  in:receive={{ key: product.id }}
  out:send={{ key: product.id }}
  onclick={() => onEdit(product)}
  onkeydown={handleKeydown}
>
  <div class="card-image">
    <ProductIcon icon={product.category_icon} color={product.category_color} size={48} />
  </div>

  <div class="card-body">
    <h3 class="name" title={product.name}>{product.name}</h3>
    {#if product.archived}<span>Archivado · Pulsa para restaurar</span>{/if}
    {#if product.category_name}
      <span class="category" title={product.category_name}>
        {product.category_name}
      </span>
    {/if}
  </div>

  <div class="card-footer">
    <div class="stock">
      <span class="stock-qty">{formatNumber(product.stock)} {product.base_unit}</span>
      <StockBadge {status} />
    </div>
    <div class="actions">
      {#if !product.archived}
        <button
          class="icon-btn"
          type="button"
          title="Registrar movimiento"
          aria-label={`Registrar movimiento de ${product.name}`}
          onclick={(e) => {
            e.stopPropagation();
            onMovement(product);
          }}
        >
          <Plus size={16} />
        </button>
      {/if}
      <button
        class="icon-btn"
        type="button"
        title="Ver historial"
        aria-label={`Ver historial de ${product.name}`}
        onclick={(e) => {
          e.stopPropagation();
          onHistory(product);
        }}
      >
        <History size={16} />
      </button>
      <button
        class="icon-btn"
        type="button"
        title={product.archived ? "Restaurar" : "Editar"}
        aria-label={product.archived ? "Restaurar" : "Editar"}
        onclick={(e) => {
          e.stopPropagation();
          onEdit(product);
        }}
      >
        <Pencil size={16} />
      </button>
      <button
        class="icon-btn danger"
        type="button"
        title="Archivar"
        aria-label="Archivar"
        disabled={!!product.archived}
        onclick={(e) => {
          e.stopPropagation();
          onDelete(product);
        }}
      >
        <Trash size={16} />
      </button>
    </div>
  </div>
</article>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: var(--shadow-sm);
    cursor: pointer;
    transition:
      border-color 0.15s,
      box-shadow 0.15s,
      transform 0.15s;
  }

  .card:hover {
    border-color: var(--border-strong);
    box-shadow: var(--shadow);
    transform: translateY(-1px);
  }

  .card-image {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 96px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
  }

  .card-body {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-height: 40px;
  }

  .name {
    font-size: 0.9375rem;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .category {
    font-size: 0.75rem;
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-top: 1px solid var(--border);
    padding-top: 0.625rem;
  }

  .stock {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .stock-qty {
    font-weight: 600;
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
    background: var(--surface-2);
    color: var(--text);
  }

  .icon-btn.danger:hover {
    background: var(--danger-soft);
    color: var(--danger);
  }
</style>
