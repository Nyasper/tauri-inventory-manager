<script lang="ts">
  import { untrack } from "svelte";
  import type { Presentation, MovementType } from "$lib/types";
  import { getPresentations } from "$lib/services/db.service";
  import { errorMessage } from "$lib/utils/errors";
  import { productsService } from "$lib/services/products.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import NumberStepper from "$lib/components/ui/NumberStepper.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import type { SelectOption } from "$lib/components/ui/select-option";
  import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import SupplierFormModal from "$lib/components/suppliers/SupplierFormModal.svelte";
  import { Plus } from "@lucide/svelte";

  let {
    initialProductId = "",
    initialType = "in",
    initialSupplierId = "",
  }: {
    initialProductId?: string;
    initialType?: MovementType;
    initialSupplierId?: string;
  } = $props();
  let productId = $state(untrack(() => initialProductId));
  let adjustment = $state(false);
  let reason = $state("");
  let presentations = $state<Presentation[]>([]);
  let loading = $state(false);
  let presentationId = $state("");
  let movementType = $state<MovementType>(untrack(() => initialType));
  let rawQuantity = $state(1);
  let supplierId = $state(untrack(() => initialSupplierId));
  let notes = $state("");
  let saving = $state(false);
  let error = $state("");

  const selectedProduct = $derived(productsService.items.find((p) => p.id === Number(productId)));
  const selectedPresentation = $derived(presentations.find((p) => p.id === Number(presentationId)));

  const availableUnits = $derived(
    selectedProduct && selectedPresentation && selectedPresentation.factor > 0
      ? Math.floor((selectedProduct.stock + 1e-9) / selectedPresentation.factor)
      : null,
  );
  const maxQuantity = $derived(
    movementType === "out" && availableUnits !== null ? availableUnits : undefined,
  );
  const clampedQuantity = $derived(Math.min(rawQuantity, maxQuantity ?? Infinity));

  function setQuantity(next: number) {
    rawQuantity = Number.isFinite(next) ? Math.min(next, maxQuantity ?? Infinity) : 0;
  }

  const formSnapshot = () =>
    JSON.stringify({
      productId,
      presentationId,
      movementType,
      quantity: clampedQuantity,
      supplierId,
      notes,
      adjustment,
      reason,
    });
  let initialSnapshot = $state<string | null>(
    untrack(() => initialProductId) ? null : formSnapshot(),
  );
  let initialSnapshotSet = !untrack(() => initialProductId);
  const dirty = $derived(initialSnapshot !== null && formSnapshot() !== initialSnapshot);

  const baseQuantity = $derived(
    Math.round(
      (Number.isFinite(clampedQuantity) ? clampedQuantity : 0) *
        (selectedPresentation?.factor ?? 0) *
        1e6,
    ) / 1e6,
  );
  const projectedStock = $derived(
    Math.round(
      ((selectedProduct?.stock ?? 0) + (movementType === "in" ? baseQuantity : -baseQuantity)) *
        1e6,
    ) / 1e6,
  );

  const validation = $derived.by(() => {
    if (loading) return "";
    if (!productId) return "Selecciona un producto.";
    if (!presentationId) return "Selecciona una presentación.";
    if (!Number.isFinite(clampedQuantity) || clampedQuantity <= 0)
      return "La cantidad debe ser mayor que cero.";
    if (movementType === "out") {
      if (availableUnits !== null && availableUnits < 1)
        return `Stock insuficiente: no alcanza para 1 ${selectedPresentation?.name ?? "unidad"}.`;
      if (projectedStock < 0)
        return `Stock insuficiente: disponible ${selectedProduct?.stock ?? 0} ${selectedProduct?.base_presentation_name ?? ""}.`;
    }
    if (adjustment && !reason.trim()) return "Indica el motivo del ajuste.";
    return "";
  });

  const canSave = $derived(validation === "" && !saving);

  let presentationOptions = $derived(
    presentations.map((presentation) => ({
      value: String(presentation.id),
      label: `${presentation.name} (×${presentation.factor})`,
    })),
  );

  const productOptions = $derived(
    productsService.items.map((p) => ({
      value: String(p.id),
      label: p.name,
    })),
  );
  const supplierOptions = $derived([
    { value: "", label: "Sin proveedor" },
    ...suppliersService.items.map((s) => ({
      value: String(s.id),
      label: s.name,
    })),
  ]);
  const typeOptions: SelectOption[] = [
    { value: "in", label: "Entrada" },
    { value: "out", label: "Salida" },
  ];

  $effect(() => {
    const pid = productId ? Number(productId) : null;
    if (!pid) {
      presentationId = "";
      presentations = [];
      return;
    }

    const product = productsService.items.find((p) => p.id === pid);
    if (movementType === "in") {
      if (product?.default_supplier_id) {
        supplierId = String(product.default_supplier_id);
      } else if (!initialSupplierId || pid !== Number(initialProductId)) {
        supplierId = "";
      }
    }

    let cancelled = false;
    loading = true;
    presentationId = "";
    presentations = [];
    getPresentations(pid)
      .then((rows) => {
        if (cancelled) return;
        presentations = rows;
        const base = rows.find((r) => r.factor === 1) ?? rows[0];
        presentationId = base ? String(base.id) : "";
        if (!initialSnapshotSet) {
          initialSnapshotSet = true;
          initialSnapshot = formSnapshot();
        }
      })
      .catch((e) => {
        if (cancelled) return;
        error = errorMessage(e);
        if (!initialSnapshotSet) {
          initialSnapshotSet = true;
          initialSnapshot = formSnapshot();
        }
      })
      .finally(() => {
        if (!cancelled) loading = false;
      });

    return () => {
      cancelled = true;
    };
  });

  $effect(() => {
    if (movementType === "in" && !supplierId && selectedProduct?.default_supplier_id) {
      supplierId = String(selectedProduct.default_supplier_id);
    }
  });

  function openNewSupplier() {
    modalService.open(SupplierFormModal, {
      onCreated: (id: number) => {
        supplierId = String(id);
      },
    });
  }

  async function save() {
    if (saving || loading || validation) return;
    const pid = productId ? Number(productId) : null;
    const presId = presentationId ? Number(presentationId) : null;
    const qty = clampedQuantity;

    if (!pid || presId === null) return;
    if (!Number.isFinite(qty) || qty <= 0) return;

    saving = true;
    error = "";
    try {
      await movementsService.create({
        product_id: pid,
        type: movementType,
        quantity: qty,
        presentation_id: presId,
        supplier_id: movementType === "in" && supplierId ? Number(supplierId) : null,
        notes: notes.trim() || null,
        adjustment,
        reason: adjustment ? reason.trim() : null,
      });
      await productsService.load();
      modalService.close();
    } catch (e) {
      error = errorMessage(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title="Nuevo movimiento"
  onClose={() => modalService.close()}
  hasUnsavedChanges={dirty}
  isBusy={saving}
  width="md"
>
  <div class="form">
    <SearchableSelect
      label="Producto"
      bind:value={productId}
      options={productOptions}
      placeholder="Selecciona un producto"
      searchPlaceholder="Buscar producto..."
    />

    {#if loading}<p role="status">Cargando presentaciones…</p>{/if}

    <Select
      label="Presentación"
      bind:value={presentationId}
      options={presentationOptions}
      placeholder="Selecciona una presentación"
    />

    <NumberStepper
      label="Cantidad"
      bind:value={() => clampedQuantity, setQuantity}
      min={1}
      max={maxQuantity}
      showBoundaryButtons
    />

    {#if selectedProduct && selectedPresentation}
      <div class="stock-preview" aria-live="polite">
        <p>
          Actual: <strong>{selectedProduct.stock} {selectedProduct.base_presentation_name}</strong>
        </p>
        <p>
          {Number.isFinite(clampedQuantity) ? clampedQuantity : 0} × {selectedPresentation.factor} = {Number.isFinite(
            baseQuantity,
          )
            ? baseQuantity
            : 0}
          {selectedProduct.base_presentation_name}
        </p>
        <p>
          Resultante: <strong class={{ error: projectedStock < 0 }}
            >{Number.isFinite(projectedStock) ? projectedStock : "—"}
            {selectedProduct.base_presentation_name}</strong
          >
        </p>
      </div>
    {/if}

    {#if validation && !saving}
      <p class="validation" role="alert">{validation}</p>
    {/if}

    <Select label="Tipo" bind:value={movementType} options={typeOptions} />

    <label><input type="checkbox" bind:checked={adjustment} /> Ajuste de inventario</label>
    {#if adjustment}<Textarea label="Motivo del ajuste" bind:value={reason} />{/if}

    {#if movementType === "in"}
      <div class="field-row">
        <div class="field-select">
          <SearchableSelect
            label="Proveedor"
            bind:value={supplierId}
            options={supplierOptions}
            placeholder="Selecciona un proveedor"
            searchPlaceholder="Buscar proveedor..."
          />
        </div>
        <Button variant="secondary" icon={Plus} onclick={openNewSupplier}>Nuevo</Button>
      </div>
    {/if}

    <Textarea label="Notas" bind:value={notes} placeholder="Observaciones del movimiento" />

    {#if error}
      <p class="error" role="alert">{error}</p>
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.requestClose()}>
      Cancelar
    </Button>
    <Button disabled={!canSave} onclick={save}>Registrar</Button>
  {/snippet}
</Modal>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
  }

  .field-row {
    display: flex;
    align-items: end;
    gap: 0.5rem;
  }

  .field-select {
    flex: 1;
    min-width: 0;
  }

  .error {
    color: var(--danger);
    font-size: 0.8125rem;
  }

  .validation {
    color: var(--warning);
    font-size: 0.8125rem;
  }
</style>
