<script lang="ts">
  import { goto } from "$app/navigation";
  import type { Movement } from "$lib/types";
  import { productsService } from "$lib/services/products.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Badge from "$lib/components/ui/Badge.svelte";
  import MovementTypeCell from "$lib/components/movements/MovementTypeCell.svelte";
  import CancelMovementModal from "$lib/components/movements/CancelMovementModal.svelte";
  import { formatDate, formatNumber } from "$lib/utils/format";
  import { ArrowLeftRight, Undo2 } from "@lucide/svelte";

  let { movement }: { movement: Movement } = $props();

  const isCancelled = $derived(movement.cancelled_at !== null);
  const isAdjustment = $derived(movement.kind === "adjustment");
  const stateLabel = $derived(isCancelled ? "Anulado" : isAdjustment ? "Ajuste" : "Vigente");
  const stateStyle = $derived(
    isCancelled
      ? { bg: "var(--danger-soft)", color: "var(--danger)" }
      : isAdjustment
        ? { bg: "var(--warning-soft)", color: "var(--warning)" }
        : { bg: "var(--surface-3)", color: "var(--text-muted)" },
  );
  const baseUnit = $derived(
    movement.product_id !== null
      ? (productsService.items.find((product) => product.id === movement.product_id)?.base_unit ??
          "")
      : "",
  );

  function viewProductMovements() {
    if (movement.product_id === null) return;
    modalService.close();
    void goto(`/movements?product=${movement.product_id}`);
  }

  function cancelMovement() {
    modalService.close();
    modalService.open(CancelMovementModal, { movement });
  }
</script>

<Modal title="Detalle del movimiento" onClose={() => modalService.close()} width="lg">
  <div class="detail">
    <div class="badges">
      <MovementTypeCell type={movement.type} />
      <Badge bg={stateStyle.bg} color={stateStyle.color}>{stateLabel}</Badge>
      <span class="id">Movimiento #{movement.id}</span>
    </div>

    <dl class="fields">
      <dt>Producto</dt>
      <dd>
        {movement.product_name ?? "—"}
        {#if movement.product_id === null}
          <span class="deleted">Producto eliminado</span>
        {/if}
      </dd>

      <dt>Presentación</dt>
      <dd>{movement.presentation_name} (×{formatNumber(movement.factor)})</dd>

      <dt>Cantidad</dt>
      <dd>
        {formatNumber(movement.quantity)}
        {movement.presentation_name}
        <span class="muted">
          = {formatNumber(movement.base_quantity)}
          {baseUnit ? `${baseUnit} base` : "unidades base"}
        </span>
      </dd>

      <dt>Proveedor</dt>
      <dd>
        {#if movement.supplier_name}
          {movement.supplier_name}
          {#if movement.supplier_id === null}
            <span class="deleted">Proveedor eliminado</span>
          {/if}
        {:else}
          —
        {/if}
      </dd>

      <dt>Fecha</dt>
      <dd>{formatDate(movement.created_at)}</dd>

      <dt>Tipo de registro</dt>
      <dd>{isAdjustment ? "Ajuste de inventario" : "Movimiento regular"}</dd>

      {#if movement.reason}
        <dt>{isAdjustment ? "Motivo del ajuste" : "Motivo"}</dt>
        <dd class="pre-wrap">{movement.reason}</dd>
      {/if}

      <dt>Notas</dt>
      <dd class="pre-wrap">{movement.notes ?? "—"}</dd>

      {#if isCancelled}
        <dt>Anulado el</dt>
        <dd>{formatDate(movement.cancelled_at)}</dd>

        <dt>Motivo de anulación</dt>
        <dd class="pre-wrap">{movement.cancellation_reason ?? "—"}</dd>
      {/if}
    </dl>
  </div>

  {#snippet footer()}
    {#if movement.product_id !== null}
      <Button variant="secondary" icon={ArrowLeftRight} onclick={viewProductMovements}>
        Ver movimientos del producto
      </Button>
    {/if}
    {#if !isCancelled}
      <Button variant="danger" icon={Undo2} onclick={cancelMovement}>Anular movimiento</Button>
    {/if}
    <Button variant="secondary" onclick={() => modalService.close()}>Cerrar</Button>
  {/snippet}
</Modal>

<style>
  .detail {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .badges {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .id {
    margin-left: auto;
    color: var(--text-muted);
    font-size: 0.75rem;
  }

  .fields {
    display: grid;
    grid-template-columns: 170px minmax(0, 1fr);
    gap: 0.5rem 1rem;
    margin: 0;
  }

  .fields dt {
    color: var(--text-muted);
    font-size: 0.8125rem;
    font-weight: 500;
  }

  .fields dd {
    margin: 0;
    min-width: 0;
    font-size: 0.875rem;
  }

  .pre-wrap {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }

  .muted {
    color: var(--text-muted);
  }

  .deleted {
    display: inline-block;
    margin-left: 0.375rem;
    padding: 0 0.375rem;
    border-radius: 999px;
    background: var(--danger-soft);
    color: var(--danger);
    font-size: 0.6875rem;
    font-weight: 500;
  }

  @media (max-width: 520px) {
    .fields {
      grid-template-columns: minmax(0, 1fr);
      gap: 0.25rem;
    }

    .fields dd {
      margin-bottom: 0.5rem;
    }
  }
</style>
