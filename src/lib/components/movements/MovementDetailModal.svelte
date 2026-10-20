<script lang="ts">
  import { goto } from "$app/navigation";
  import type { Movement } from "$lib/types";
  import { productsService } from "$lib/services/products.service.svelte";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { errorMessage } from "$lib/utils/errors";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";
  import Badge from "$lib/components/ui/Badge.svelte";
  import MovementTypeCell from "$lib/components/movements/MovementTypeCell.svelte";
  import CancelMovementModal from "$lib/components/movements/CancelMovementModal.svelte";
  import EditMovementSupplierModal from "$lib/components/movements/EditMovementSupplierModal.svelte";
  import { formatDateTime, formatNumber } from "$lib/utils/format";
  import { ArrowLeftRight, Undo2, Pencil, Check, X } from "@lucide/svelte";

  let { movement }: { movement: Movement } = $props();

  const supplierId = $derived(movement.supplier_id);
  const supplierName = $derived(movement.supplier_name);
  let isEditingSupplier = $state(false);
  let selectedSupplierId = $state<string>("");
  let isSavingSupplier = $state(false);
  let supplierError = $state("");

  const supplierOptions = $derived([
    { value: "", label: "— Sin proveedor —" },
    ...suppliersService.items.map((s) => ({ value: String(s.id), label: s.name })),
  ]);

  function startEditSupplier() {
    suppliersService.load();
    selectedSupplierId = supplierId !== null ? String(supplierId) : "";
    supplierError = "";
    isEditingSupplier = true;
  }

  function cancelEditSupplier() {
    isEditingSupplier = false;
    supplierError = "";
  }

  async function handleSaveSupplier() {
    isSavingSupplier = true;
    supplierError = "";
    try {
      const nextId = selectedSupplierId ? Number(selectedSupplierId) : null;
      await movementsService.updateSupplier(movement.id, nextId);
      movement.supplier_id = nextId;
      if (nextId === null) {
        movement.supplier_name = null;
      } else {
        const found = suppliersService.items.find((s) => s.id === nextId);
        movement.supplier_name = found ? found.name : movement.supplier_name;
      }
      isEditingSupplier = false;
    } catch (e) {
      supplierError = errorMessage(e);
    } finally {
      isSavingSupplier = false;
    }
  }

  const isCancelled = $derived(movement.cancelled_at !== null);
  const isAdjustment = $derived(movement.kind === "adjustment");
  const statusLabel = $derived(isCancelled ? "Anulado" : isAdjustment ? "Ajuste" : "Vigente");
  const statusStyle = $derived(
    isCancelled
      ? { background: "var(--danger-soft)", textColor: "var(--danger)" }
      : isAdjustment
        ? { background: "var(--warning-soft)", textColor: "var(--warning)" }
        : { background: "var(--surface-3)", textColor: "var(--text-muted)" },
  );
  const basePresentationName = $derived(
    movement.product_id !== null
      ? (productsService.items.find((product) => product.id === movement.product_id)
          ?.base_presentation_name ?? "")
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

  function openEditSupplierModal() {
    modalService.close();
    modalService.open(EditMovementSupplierModal, { movement });
  }
</script>

<Modal title="Detalle del movimiento" onClose={() => modalService.close()} width="lg">
  <div class="detail">
    <div class="badges">
      <MovementTypeCell type={movement.type} />
      <Badge background={statusStyle.background} textColor={statusStyle.textColor}
        >{statusLabel}</Badge
      >
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
          {basePresentationName ? `${basePresentationName} base` : "unidades base"}
        </span>
      </dd>

      <dt>Proveedor</dt>
      <dd>
        {#if isEditingSupplier}
          <div class="supplier-edit-box">
            <div class="supplier-edit-row">
              <div class="supplier-select-wrap">
                <SearchableSelect
                  bind:value={selectedSupplierId}
                  options={supplierOptions}
                  disabled={isSavingSupplier}
                  placeholder="Seleccionar proveedor"
                  searchPlaceholder="Buscar proveedor..."
                  ariaLabel="Seleccionar proveedor"
                />
              </div>
              <button
                type="button"
                class="action-btn save"
                title="Guardar proveedor"
                aria-label="Guardar proveedor"
                disabled={isSavingSupplier}
                onclick={handleSaveSupplier}
              >
                <Check size={14} />
              </button>
              <button
                type="button"
                class="action-btn cancel"
                title="Cancelar edición"
                aria-label="Cancelar edición"
                disabled={isSavingSupplier}
                onclick={cancelEditSupplier}
              >
                <X size={14} />
              </button>
            </div>
            {#if supplierError}
              <span class="supplier-error">{supplierError}</span>
            {/if}
          </div>
        {:else}
          <div class="supplier-view-row">
            <span>
              {#if supplierName}
                {supplierName}
                {#if supplierId === null}
                  <span class="deleted">Proveedor eliminado</span>
                {/if}
              {:else}
                —
              {/if}
            </span>
            {#if movement.type === "in" && !isCancelled}
              <button
                type="button"
                class="edit-supplier-btn"
                title={supplierId !== null ? "Editar proveedor" : "Agregar proveedor"}
                aria-label={supplierId !== null ? "Editar proveedor" : "Agregar proveedor"}
                onclick={startEditSupplier}
              >
                <Pencil size={12} />
                <span>{supplierId !== null ? "Editar" : "Agregar"}</span>
              </button>
            {/if}
          </div>
        {/if}
      </dd>

      <dt>Fecha</dt>
      <dd>{formatDateTime(movement.created_at)}</dd>

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
        <dd>{formatDateTime(movement.cancelled_at)}</dd>

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
    {#if movement.type === "in" && !isCancelled}
      <Button variant="secondary" icon={Pencil} onclick={openEditSupplierModal}>
        Editar proveedor
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

  .supplier-view-row {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .edit-supplier-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.125rem 0.375rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    color: var(--text-muted);
    font-size: 0.75rem;
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  .edit-supplier-btn:hover {
    background: var(--surface-3);
    color: var(--text);
  }

  .supplier-edit-box {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .supplier-edit-row {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    max-width: 320px;
  }

  .supplier-select-wrap {
    flex: 1;
    min-width: 0;
  }

  .action-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    color: var(--text-muted);
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  .action-btn.save:hover:not(:disabled) {
    background: var(--primary);
    color: white;
    border-color: var(--primary);
  }

  .action-btn.cancel:hover:not(:disabled) {
    background: var(--surface-3);
    color: var(--text);
  }

  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .supplier-error {
    font-size: 0.75rem;
    color: var(--danger);
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
