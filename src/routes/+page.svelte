<script lang="ts">
  import { goto } from "$app/navigation";
  import type { Component } from "svelte";
  import type { LucideProps } from "@lucide/svelte";
  import type { Product } from "$lib/types";
  import { productsService } from "$lib/services/products.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { settingsService } from "$lib/services/settings.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { getStockStatus } from "$lib/utils/stock";
  import { formatNumber, formatDate } from "$lib/utils/format";
  import ProductFormModal from "$lib/components/products/ProductFormModal.svelte";
  import MovementFormModal from "$lib/components/movements/MovementFormModal.svelte";
  import MovementDetailModal from "$lib/components/movements/MovementDetailModal.svelte";
  import SupplierFormModal from "$lib/components/suppliers/SupplierFormModal.svelte";
  import StockBadge from "$lib/components/ui/StockBadge.svelte";
  import Badge from "$lib/components/ui/Badge.svelte";
  import ProductIcon from "$lib/components/ui/ProductIcon.svelte";
  import {
    Box,
    TriangleAlert,
    Activity,
    Truck,
    Plus,
    ArrowLeftRight,
    ArrowDownToLine,
    ArrowUpFromLine,
  } from "@lucide/svelte";

  productsService.load();
  movementsService.load({});
  suppliersService.load();

  const PANEL_SIZE = 10;

  function statusOf(product: Product) {
    return getStockStatus(
      product.stock,
      settingsService.settings.stock_low_threshold,
      settingsService.settings.stock_medium_threshold,
    );
  }

  const lowStockProducts = $derived(
    productsService.items
      .filter((product) => {
        const status = statusOf(product);
        return status === "low" || status === "out";
      })
      .sort((a, b) => a.stock - b.stock),
  );

  const lowProducts = $derived(lowStockProducts.slice(0, PANEL_SIZE));

  const recentMovements = $derived(movementsService.items.slice(0, PANEL_SIZE));

  const stats = $derived([
    {
      label: "Productos",
      value: formatNumber(productsService.items.length),
      icon: Box as Component<LucideProps>,
      tone: "primary",
      href: "/products",
    },
    {
      label: "Stock bajo / agotado",
      value: formatNumber(lowStockProducts.length),
      icon: TriangleAlert as Component<LucideProps>,
      tone: "danger",
      href: "/products?stock=low",
    },
    {
      label: "Movimientos",
      value: formatNumber(movementsService.total),
      icon: Activity as Component<LucideProps>,
      tone: "primary",
      href: "/movements",
    },
    {
      label: "Proveedores",
      value: formatNumber(suppliersService.items.length),
      icon: Truck as Component<LucideProps>,
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
  {#if productsService.loading || movementsService.loading || suppliersService.loading}<p
      role="status"
    >
      Cargando inventario…
    </p>{/if}
  {#if productsService.error || movementsService.error || suppliersService.error}<p
      class="error"
      role="alert"
    >
      No se pudo cargar el inventario.
    </p>
    <button
      onclick={() => {
        productsService.load();
        movementsService.load({});
        suppliersService.load();
      }}>Reintentar</button
    >{/if}
  <section class="stats">
    {#each stats as stat (stat.label)}
      {const Icon = stat.icon}
      <button class="stat-card tone-{stat.tone}" type="button" onclick={() => goto(stat.href)}>
        <span class="stat-icon">
          <Icon size={20} />
        </span>
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
        <button class="link" type="button" onclick={() => goto("/products?stock=low")}>
          Ver todos
        </button>
      </div>

      {#if lowProducts.length === 0 && !productsService.loading && !productsService.error}
        <p class="empty-note">No hay productos con stock bajo.</p>
      {:else}
        <div class="rows">
          {#each lowProducts as product (product.id)}
            <button class="row" type="button" onclick={() => goto("/products")}>
              <span class="row-icon" title={product.category_name ?? "Sin categoría"}>
                <ProductIcon
                  icon={product.category_icon}
                  color={product.category_color}
                  size={28}
                />
              </span>
              <span class="row-name" title={product.name}>{product.name}</span>
              <span class="row-meta">
                <span class="row-stock">{formatNumber(product.stock)} {product.base_unit}</span>
                <StockBadge
                  status={getStockStatus(
                    product.stock,
                    settingsService.settings.stock_low_threshold,
                    settingsService.settings.stock_medium_threshold,
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
        <button class="link" type="button" onclick={() => goto("/movements")}> Ver todos </button>
      </div>

      {#if recentMovements.length === 0 && !movementsService.loading && !movementsService.error}
        <p class="empty-note">No hay movimientos registrados.</p>
      {:else}
        <div class="rows">
          {#each recentMovements as movement (movement.id)}
            <button
              class="row"
              type="button"
              onclick={() => modalService.open(MovementDetailModal, { movement })}
            >
              {#if movement.cancelled_at}
                <Badge uniform bg="var(--surface-3)" color="var(--text-muted)">Anulado</Badge>
              {/if}
              {#if movement.type === "in"}
                <Badge uniform bg="var(--success)" color="#ffffff">
                  <ArrowDownToLine size={13} /> Entrada
                </Badge>
              {:else}
                <Badge uniform bg="var(--danger)" color="#ffffff">
                  <ArrowUpFromLine size={13} /> Salida
                </Badge>
              {/if}
              <span class="row-name" title={movement.product_name ?? "—"}>
                {movement.product_name ?? "—"}
              </span>
              <span class="row-meta">
                <span class="row-stock"
                  >{formatNumber(movement.quantity)} {movement.presentation_name}</span
                >
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
      <Plus size={16} />
      <span>Nuevo producto</span>
    </button>
    <button class="action" type="button" onclick={openNewMovement}>
      <ArrowLeftRight size={16} />
      <span>Registrar movimiento</span>
    </button>
    <button class="action" type="button" onclick={openNewSupplier}>
      <Truck size={16} />
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
    background: linear-gradient(135deg, var(--primary-soft), transparent);
    color: var(--primary);
    box-shadow: inset 0 0 0 1px var(--border);
    flex-shrink: 0;
  }

  .tone-danger .stat-icon {
    background: linear-gradient(135deg, var(--danger-soft), transparent);
    color: var(--danger);
    box-shadow: inset 0 0 0 1px var(--danger-soft);
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

  .row-icon {
    display: inline-flex;
    flex-shrink: 0;
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
