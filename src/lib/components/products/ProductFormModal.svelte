<script lang="ts">
  import DuplicateHint from "#lib/components/ui/DuplicateHint.svelte";
  import { untrack } from "svelte";
  import type { Product } from "#lib/types.js";
  import {
    getPresentations,
    type PresentationInput,
    type ProductInput,
  } from "#lib/services/db.service.js";
  import { isDuplicateError, toAppError, type AppError } from "#lib/utils/errors.js";
  import { productsService } from "#lib/services/products.service.svelte.js";
  import { categoriesService } from "#lib/services/categories.service.svelte.js";
  import { suppliersService } from "#lib/services/suppliers.service.svelte.js";
  import { settingsService } from "#lib/services/settings.service.svelte.js";
  import CategoryFormModal from "#lib/components/categories/CategoryFormModal.svelte";
  import { modalService } from "#lib/services/modal.service.svelte.js";
  import Modal from "#lib/components/ui/Modal.svelte";
  import Input from "#lib/components/ui/Input.svelte";
  import NumberStepper from "#lib/components/ui/NumberStepper.svelte";
  import Textarea from "#lib/components/ui/Textarea.svelte";
  import SearchableSelect from "#lib/components/ui/SearchableSelect.svelte";
  import Button from "#lib/components/ui/Button.svelte";
  import { Plus, Trash } from "@lucide/svelte";

  let { product = null }: { product?: Product | null } = $props();

  const initialProduct = untrack(() => product);

  interface DraftPresentation extends PresentationInput {
    key: number;
  }

  let nextPresentationKey = 0;
  function newPresentation(name = "", factor = 1): DraftPresentation {
    return { key: nextPresentationKey++, name, factor };
  }

  suppliersService.load();

  let name = $state(initialProduct?.name ?? "");
  let description = $state(initialProduct?.description ?? "");
  let categoryId = $state(
    initialProduct?.category_id != null ? String(initialProduct.category_id) : "",
  );
  let defaultSupplierId = $state(
    initialProduct?.default_supplier_id != null ? String(initialProduct.default_supplier_id) : "",
  );
  let presentations = $state<DraftPresentation[]>([newPresentation("Unidad", 1)]);
  const hasCustomThresholds =
    initialProduct?.stock_low_threshold != null && initialProduct?.stock_medium_threshold != null;
  let useCustomThresholds = $state(hasCustomThresholds);
  let customLowThreshold = $state(
    initialProduct?.stock_low_threshold != null
      ? String(initialProduct.stock_low_threshold)
      : String(settingsService.settings.stock_low_threshold),
  );
  let customMediumThreshold = $state(
    initialProduct?.stock_medium_threshold != null
      ? String(initialProduct.stock_medium_threshold)
      : String(settingsService.settings.stock_medium_threshold),
  );
  let ignoreStockAlerts = $state(Boolean(initialProduct?.ignore_stock_alerts));
  let saving = $state(false);
  let loading = $state(!!initialProduct?.id);
  let loadFailed = $state(false);
  let error = $state<AppError | null>(null);

  // El error de duplicado deja de ser válido en cuanto cambia el nombre.
  $effect(() => {
    void name;
    untrack(() => {
      if (error && isDuplicateError(error)) error = null;
    });
  });

  const formSnapshot = () =>
    JSON.stringify({
      name,
      description,
      categoryId,
      defaultSupplierId,
      presentations,
      useCustomThresholds,
      customLowThreshold: useCustomThresholds ? customLowThreshold : null,
      customMediumThreshold: useCustomThresholds ? customMediumThreshold : null,
      ignoreStockAlerts,
    });
  let initialSnapshot = $state<string | null>(initialProduct?.id ? null : formSnapshot());
  const dirty = $derived(initialSnapshot !== null && formSnapshot() !== initialSnapshot);

  if (initialProduct?.id) {
    getPresentations(initialProduct.id)
      .then((rows) => {
        presentations = rows.length
          ? rows.map((r) => ({ ...newPresentation(r.name, r.factor), id: r.id }))
          : [newPresentation("Unidad", 1)];
        initialSnapshot = formSnapshot();
      })
      .catch((e) => {
        loadFailed = true;
        error = toAppError(e);
        initialSnapshot = formSnapshot();
      })
      .finally(() => {
        loading = false;
      });
  }

  const categoryOptions = $derived([
    { value: "", label: "Sin categoría" },
    ...categoriesService.items.map((c) => ({
      value: String(c.id),
      label: c.name,
    })),
  ]);

  const supplierOptions = $derived([
    { value: "", label: "(Sin proveedor por defecto)" },
    ...suppliersService.items.map((s) => ({
      value: String(s.id),
      label: s.name,
    })),
  ]);

  function openNewCategory() {
    modalService.open(CategoryFormModal, {
      onCreated: (id: number) => {
        categoryId = String(id);
      },
    });
  }

  const validation = $derived.by(() => {
    if (loading || loadFailed) return "";
    if (!name.trim()) return "El nombre es obligatorio.";
    if (
      presentations.some(
        (p) => !p.name.trim() || !Number.isFinite(Number(p.factor)) || Number(p.factor) <= 0,
      )
    ) {
      return "Cada presentación necesita nombre y factor positivo.";
    }
    if (presentations.filter((p) => Number(p.factor) === 1).length !== 1) {
      return "Debe haber exactamente una presentación base con factor 1.";
    }
    if (useCustomThresholds) {
      const lowNum = Number(customLowThreshold);
      const medNum = Number(customMediumThreshold);
      if (
        !Number.isFinite(lowNum) ||
        !Number.isFinite(medNum) ||
        lowNum < 0 ||
        lowNum >= medNum ||
        medNum > 1_000_000_000
      ) {
        return "Los límites de stock deben cumplir: 0 ≤ bajo < medio ≤ 1.000.000.000.";
      }
    }
    return "";
  });

  const canSave = $derived(validation === "" && !saving && !loading && !loadFailed);

  function addPresentation() {
    presentations.push(newPresentation("", 10));
  }

  function removePresentation(index: number) {
    if (presentations.length <= 1) return;
    presentations = presentations.filter((_, i) => i !== index);
  }

  async function save() {
    if (!canSave) return;

    const input: ProductInput = {
      id: initialProduct?.id ?? null,
      name: name.trim(),
      description: description.trim() || null,
      category_id: categoryId ? Number(categoryId) : null,
      default_supplier_id: defaultSupplierId ? Number(defaultSupplierId) : null,
      stock_low_threshold: useCustomThresholds ? Number(customLowThreshold) : null,
      stock_medium_threshold: useCustomThresholds ? Number(customMediumThreshold) : null,
      ignore_stock_alerts: ignoreStockAlerts,
      presentations: presentations.map(({ id, name: presentationName, factor }) => ({
        id,
        name: presentationName.trim(),
        factor: Number(factor),
      })),
    };

    saving = true;
    error = null;
    try {
      await productsService.save(input);
      modalService.close();
    } catch (e) {
      error = toAppError(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title={initialProduct ? "Editar producto" : "Nuevo producto"}
  onClose={() => modalService.close()}
  hasUnsavedChanges={dirty}
  isBusy={saving}
  width="lg"
>
  <div class="form">
    {#if loading}<p role="status">Cargando presentaciones…</p>{/if}
    <div class="grid">
      <div class="span-2">
        <Input
          label="Nombre"
          bind:value={name}
          placeholder="Nombre del producto"
          disabled={loading}
        />
      </div>
      <div class="span-2">
        <div class="category-row">
          <div class="category-select">
            <SearchableSelect
              label="Categoría"
              bind:value={categoryId}
              options={categoryOptions}
              disabled={loading}
              placeholder="Selecciona una categoría"
              searchPlaceholder="Buscar categoría..."
            />
          </div>
          <Button variant="secondary" icon={Plus} onclick={openNewCategory} disabled={loading}>
            Nueva
          </Button>
        </div>
      </div>
      <div class="span-2">
        <SearchableSelect
          label="Proveedor por defecto (opcional)"
          bind:value={defaultSupplierId}
          options={supplierOptions}
          disabled={loading}
          placeholder="Selecciona un proveedor por defecto"
          searchPlaceholder="Buscar proveedor..."
          clearable
        />
      </div>
      <div class="span-2">
        <Textarea
          label="Observaciones"
          bind:value={description}
          placeholder="Notas asociadas a este producto"
          disabled={loading}
        />
      </div>
    </div>

    <div class="thresholds-group">
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={useCustomThresholds} disabled={loading} />
        <span>Personalizar límites de stock para este producto</span>
      </label>
      {#if useCustomThresholds}
        <div class="grid thresholds-grid">
          <div>
            <Input
              label="Límite bajo"
              type="number"
              bind:value={customLowThreshold}
              placeholder={`Por defecto global: ${settingsService.settings.stock_low_threshold}`}
              disabled={loading}
            />
          </div>
          <div>
            <Input
              label="Límite medio"
              type="number"
              bind:value={customMediumThreshold}
              placeholder={`Por defecto global: ${settingsService.settings.stock_medium_threshold}`}
              disabled={loading}
            />
          </div>
        </div>
        <p class="hint">
          Sobrescribe los límites globales ({settingsService.settings.stock_low_threshold} y {settingsService
            .settings.stock_medium_threshold}) para la clasificación de este producto.
        </p>
      {:else}
        <p class="hint">
          Usa los límites globales por defecto: Bajo &le; {settingsService.settings
            .stock_low_threshold}, Medio &le; {settingsService.settings.stock_medium_threshold}.
        </p>
      {/if}

      <div class="threshold-divider"></div>

      <label class="checkbox-label">
        <input type="checkbox" bind:checked={ignoreStockAlerts} disabled={loading} />
        <span>Ignorar alertas de stock para este producto</span>
      </label>
      {#if ignoreStockAlerts}
        <p class="hint">
          El producto no se considerará en estado crítico ni aparecerá en el panel de stock bajo del
          Dashboard.
        </p>
      {/if}
    </div>

    <section class="presentations">
      <div class="section-head">
        <h3>Presentaciones</h3>
        <Button
          size="sm"
          variant="secondary"
          icon={Plus}
          onclick={addPresentation}
          disabled={loading}
        >
          Añadir
        </Button>
      </div>
      <p class="hint">
        La presentación con factor 1 es la unidad base en la que se cuenta el stock. Se admiten
        hasta 6 decimales.
      </p>

      {#each presentations as presentation, index (presentation.key)}
        <div class="pres-row">
          <div class="pres-name">
            <Input
              bind:value={presentation.name}
              placeholder="Ej. Caja, Botella"
              disabled={loading}
            />
          </div>
          <div class="pres-factor">
            <NumberStepper
              label="Factor"
              bind:value={presentation.factor}
              min={1}
              disabled={loading}
            />
          </div>
          <div class="pres-base">
            {#if presentation.factor === 1}
              <span class="base-tag">Base</span>
            {:else}
              <span class="base-tag muted">= {presentation.factor || 0} base</span>
            {/if}
          </div>
          <button
            class="icon-btn"
            type="button"
            title="Quitar presentación"
            aria-label="Quitar presentación"
            disabled={loading || presentations.length <= 1}
            onclick={() => removePresentation(index)}
          >
            <Trash size={16} />
          </button>
        </div>
      {/each}
    </section>

    {#if validation && !saving}
      <p class="validation" role="status">{validation}</p>
    {/if}

    {#if error}
      <p class="error" role="alert">{error.message}</p>
      <DuplicateHint {name} {error} recordType="product" currentId={initialProduct?.id} />
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.requestClose()}>
      Cancelar
    </Button>
    <Button disabled={!canSave} onclick={save}>
      {saving ? "Guardando…" : initialProduct ? "Guardar" : "Crear"}
    </Button>
  {/snippet}
</Modal>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.875rem;
  }

  .span-2 {
    grid-column: span 2;
  }

  .category-row {
    display: flex;
    align-items: end;
    gap: 0.5rem;
  }

  .category-select {
    flex: 1;
    min-width: 0;
  }

  .presentations {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border);
  }

  .section-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  h3 {
    font-size: 0.9375rem;
  }

  .hint {
    color: var(--text-muted);
    font-size: 0.8125rem;
  }

  .pres-row {
    display: grid;
    grid-template-columns: 1fr 150px 90px 32px;
    gap: 0.625rem;
    align-items: end;
  }

  .pres-base {
    display: flex;
    align-items: center;
    height: 38px;
  }

  .base-tag {
    font-size: 0.7rem;
    color: var(--primary);
    background: var(--primary-soft);
    padding: 0.0625rem 0.375rem;
    border-radius: 999px;
  }

  .base-tag.muted {
    color: var(--text-muted);
    background: var(--surface-2);
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 38px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .icon-btn:hover:not(:disabled) {
    background: var(--danger-soft);
    color: var(--danger);
  }

  .icon-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .error {
    color: var(--danger);
    font-size: 0.8125rem;
  }

  .thresholds-group {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    padding: 0.875rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
  }

  .checkbox-label {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text);
    cursor: pointer;
    user-select: none;
  }

  .thresholds-grid {
    margin-top: 0.25rem;
  }

  .threshold-divider {
    height: 1px;
    background: var(--border);
    margin: 0.25rem 0;
  }

  .validation {
    color: var(--warning);
    font-size: 0.8125rem;
  }
</style>
