<script lang="ts">
  import { goto } from "$app/navigation";
  import type { Movement, Product } from "$lib/types";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { movementsStore } from "$lib/stores/movements.store.svelte";
  import { suppliersStore } from "$lib/stores/suppliers.store.svelte";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import { getStockStatus } from "$lib/utils/stock";
  import { formatNumber, formatDate } from "$lib/utils/format";
  import ProductFormModal from "$lib/components/products/ProductFormModal.svelte";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import SupplierFormModal from "$lib/components/suppliers/SupplierFormModal.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import Badge from "$lib/components/ui/Badge.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import type { IconName } from "$lib/components/ui/Icon.svelte";

  productsStore.load();
  movementsStore.load();
  suppliersStore.load();

  const lowProducts = $derived(
    productsStore.items
      .filter((p) => {
        const status = getStockStatus(
          p.stock,
          settingsStore.settings.stock_low_threshold,
          settingsStore.settings.stock_medium_threshold,
        );
        return status === "low" || status === "out";
      })
      .slice(0, 5),
  );

  const recentMovements = $derived(movementsStore.items.slice(0, 5));

  const stats = $derived([
    {
      label: "Productos",
      value: formatNumber(productsStore.items.length),
      icon: "box" as IconName,
      tone: "primary",
      href: "/products",
    },
    {
      label: "Stock bajo / agotado",
      value: formatNumber(
        productsStore.items.filter((p) => {
          const status = getStockStatus(
            p.stock,
            settingsStore.settings.stock_low_threshold,
            settingsStore.settings.stock_medium_threshold,
          );
          return status === "low" || status === "out";
        }).length,
      ),
      icon: "alertTriangle" as IconName,
      tone: "danger",
      href: "/products",
    },
    {
      label: "Movimientos",
      value: formatNumber(movementsStore.items.length),
      icon: "activity" as IconName,
      tone: "primary",
      href: "/movements",
    },
    {
      label: "Proveedores",
      value: formatNumber(suppliersStore.items.length),
      icon: "truck" as IconName,
      tone: "primary",
      href: "/suppliers",
    },
  ]);

  function openNewProduct() {
    modalService.open(ProductFormModal);
  }

  function openNewMovement() {
    modalService.open(MovementFormModal);
  }

  function openNewSupplier() {
    modalService.open(SupplierFormModal);
  }
</script>

<div class="dashboard">
  <section class="stats">
    {#each stats as stat (stat.label)}
      <button class="stat-card tone-{stat.tone}" type="button" onclick={() => goto(stat.href)}>
        <span class="stat-icon"><Icon name={stat.icon} size={20} /></span>
        <span class="stat-body">
          <span class="stat-value">{stat.value}</span>
          <span class="stat-label">{stat.label}</span>
        </span>
      </button>
    {/each}
  </section>

  <div class="columns">
    <section class="panel">
      <div class="panel-head">
        <h2>Stock por agotarse</h2>
        <button class="link" type="button" onclick={() => goto("/products")}>
          Ver todos
        </button>
      </div>

      {#if lowProducts.length === 0}
        <p class="empty-note">No hay productos con stock bajo.</p>
      {:else}
        <div class="rows">
          {#each lowProducts as product (product.id)}
            <button class="row" type="button" onclick={() => goto("/products")}>
              <span class="row-name" title={product.name}>{product.name}</span>
              <span class="row-meta">
                <span class="row-stock">{formatNumber(product.stock)}</span>
                <StockBadge
                  status={getStockStatus(
                    product.stock,
                    settingsStore.settings.stock_low_threshold,
                    settingsStore.settings.stock_medium_threshold,
                  )}
                />
              </span>
            </button>
          {/each}
        </div>
      {/if}
    </section>

    <section class="panel">
      <div class="panel-head">
        <h2>Movimientos recientes</h2>
        <button class="link" type="button" onclick={() => goto("/movements")}>
          Ver todos
        </button>
      </div>

      {#if recentMovements.length === 0}
        <p class="empty-note">No hay movimientos registrados.</p>
      {:else}
        <div class="rows">
          {#each recentMovements as movement (movement.id)}
            <button class="row" type="button" onclick={() => goto("/movements")}>
              {#if movement.type === "in"}
                <Badge bg="var(--success)" color="#ffffff">
                  <Icon name="arrowDown" size={13} /> Entrada
                </Badge>
              {:else}
                <Badge bg="var(--danger)" color="#ffffff">
                  <Icon name="arrowUp" size={13} /> Salida
                </Badge>
              {/if}
              <span class="row-name" title={movement.product_name ?? "—"}>
                {movement.product_name ?? "—"}
              </span>
              <span class="row-meta">
                {#if movement.person_name}
                  <span class="row-person" title={movement.person_name}>
                    {movement.person_name}
                  </span>
                {/if}
                <span class="row-stock">{formatNumber(movement.quantity)}</span>
                <span class="row-date">{formatDate(movement.created_at)}</span>
              </span>
            </button>
          {/each}
        </div>
      {/if}
    </section>
  </div>

  <section class="actions">
    <button class="action" type="button" onclick={openNewProduct}>
      <Icon name="plus" size={16} />
      <span>Nuevo producto</span>
    </button>
    <button class="action" type="button" onclick={openNewMovement}>
      <Icon name="swap" size={16} />
      <span>Registrar movimiento</span>
    </button>
    <button class="action" type="button" onclick={openNewSupplier}>
      <Icon name="truck" size={16} />
      <span>Nuevo proveedor</span>
    </button>
  </section>
</div>

<style>
  .dashboard {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 0.75rem;
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--surface);
    box-shadow: var(--shadow-sm);
    cursor: pointer;
    text-align: left;
    transition:
      border-color 0.15s,
      transform 0.15s;
  }

  .stat-card:hover {
    border-color: var(--border-strong);
    transform: translateY(-1px);
  }

  .stat-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: var(--radius-sm);
    background: var(--primary-soft);
    color: var(--primary);
    flex-shrink: 0;
  }

  .tone-danger .stat-icon {
    background: var(--danger-soft);
    color: var(--danger);
  }

  .stat-body {
    display: flex;
    flex-direction: column;
  }

  .stat-value {
    font-size: 1.25rem;
    font-weight: 600;
    line-height: 1.2;
  }

  .stat-label {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .columns {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .panel {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 1rem;
  }

  .panel-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .panel-head h2 {
    font-size: 0.9375rem;
  }

  .link {
    border: none;
    background: transparent;
    color: var(--primary);
    font-size: 0.8125rem;
    cursor: pointer;
    padding: 0;
  }

  .link:hover {
    text-decoration: underline;
  }

  .rows {
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0;
    border: none;
    border-bottom: 1px solid var(--border);
    background: transparent;
    color: inherit;
    font: inherit;
    width: 100%;
    text-align: left;
    cursor: pointer;
  }

  .row:last-child {
    border-bottom: none;
  }

  .row:hover .row-name {
    color: var(--primary);
  }

  .row-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .row-meta {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    flex-shrink: 0;
  }

  .row-stock {
    font-weight: 600;
  }

  .row-person {
    font-size: 0.75rem;
    color: var(--text-muted);
    max-width: 140px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .row-date {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .empty-note {
    color: var(--text-muted);
    font-size: 0.8125rem;
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.625rem;
  }

  .action {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.875rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    font-weight: 500;
    cursor: pointer;
    transition:
      background-color 0.15s,
      border-color 0.15s;
  }

  .action:hover {
    background: var(--surface-2);
    border-color: var(--border-strong);
  }
</style>