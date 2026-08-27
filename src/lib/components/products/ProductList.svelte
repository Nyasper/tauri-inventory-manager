<script lang="ts">
  import type { Product } from "$lib/types";
  import { getStockStatus } from "$lib/utils/stock";
  import { formatNumber } from "$lib/utils/format";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import ResolvedImage from "$lib/components/ui/ResolvedImage.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";

  let {
    products,
    onEdit,
    onDelete,
  }: {
    products: Product[];
    onEdit: (product: Product) => void;
    onDelete: (product: Product) => void;
  } = $props();

  function statusOf(product: Product) {
    return getStockStatus(
      product.stock,
      settingsStore.settings.stock_low_threshold,
      settingsStore.settings.stock_medium_threshold,
    );
  }
</script>

<table class="table">
  <thead>
    <tr>
      <th></th>
      <th>Nombre</th>
      <th>Código</th>
      <th>Categoría</th>
      <th class="num">Stock</th>
      <th>Estado</th>
      <th class="actions-col"></th>
    </tr>
  </thead>
  <tbody>
    {#each products as product (product.id)}
      <tr onclick={() => onEdit(product)}>
        <td class="img-cell">
          <ResolvedImage src={product.image} size={32} />
        </td>
        <td class="name-cell">
          <span class="name" title={product.name}>{product.name}</span>
          {#if product.description}
            <span class="desc" title={product.description}>
              {product.description}
            </span>
          {/if}
        </td>
        <td class="muted">{product.code ?? "—"}</td>
        <td class="muted">{product.category_name ?? "—"}</td>
        <td class="num stock-qty">{formatNumber(product.stock)}</td>
        <td><StockBadge status={statusOf(product)} /></td>
        <td class="actions-col">
          <div class="actions">
            <button
              class="icon-btn"
              type="button"
              aria-label="Editar"
              onclick={(e) => {
                e.stopPropagation();
                onEdit(product);
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
                onDelete(product);
              }}
            >
              <Icon name="trash" size={16} />
            </button>
          </div>
        </td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .table {
    width: 100%;
    border-collapse: collapse;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }

  th {
    text-align: left;
    padding: 0.625rem 1rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
    background: var(--surface-2);
  }

  td {
    padding: 0.625rem 1rem;
    border-bottom: 1px solid var(--border);
    font-size: 0.875rem;
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  tbody tr {
    cursor: pointer;
    transition: background-color 0.12s;
  }

  tbody tr:hover {
    background: var(--surface-2);
  }

  .num {
    text-align: right;
  }

  .stock-qty {
    font-weight: 600;
  }

  .muted {
    color: var(--text-muted);
  }

  .img-cell {
    width: 40px;
  }

  .name-cell {
    display: flex;
    flex-direction: column;
  }

  .name {
    font-weight: 600;
  }

  .desc {
    font-size: 0.75rem;
    color: var(--text-muted);
    max-width: 320px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .actions-col {
    width: 80px;
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
