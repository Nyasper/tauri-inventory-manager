<script lang="ts">
  import DuplicateHint from "$lib/components/ui/DuplicateHint.svelte";
  import { untrack } from "svelte";
  import type { Product } from "$lib/types";
  import {
    getPresentations,
    type PresentationInput,
    type ProductInput,
  } from "$lib/services/db.service";
  import { productsService } from "$lib/services/products.service.svelte";
  import { categoriesService } from "$lib/services/categories.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import NumberStepper from "$lib/components/ui/NumberStepper.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { Plus, Trash } from "@lucide/svelte";

  let { product = null }: { product?: Product | null } = $props();

  const initial = untrack(() => product);

  let name = $state(initial?.name ?? "");
  let description = $state(initial?.description ?? "");
  let categoryId = $state(
    initial?.category_id != null ? String(initial.category_id) : "",
  );
  let presentations = $state<PresentationInput[]>([
    { name: "Unidad", factor: 1 },
  ]);
  let saving = $state(false);
  let loading = $state(!!initial?.id);
  let loadFailed = $state(false);
  let error = $state("");

  if (initial?.id) {
    getPresentations(initial.id).then((rows) => {
      presentations = rows.length
        ? rows.map((r) => ({ id: r.id, name: r.name, factor: r.factor }))
        : [{ name: "Unidad", factor: 1 }];
    }).catch((e) => { loadFailed = true; error = String(e); }).finally(() => { loading = false; });
  }

  const categoryOptions = $derived([
    { value: "", label: "Sin categoría" },
    ...categoriesService.items.map((c) => ({
      value: String(c.id),
      label: c.name,
    })),
  ]);

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
    return "";
  });

  const canSave = $derived(validation === "" && !saving && !loading && !loadFailed);

  function addPresentation() {
    presentations.push({ name: "", factor: 1 });
  }

  function removePresentation(index: number) {
    if (presentations.length <= 1) return;
    presentations = presentations.filter((_, i) => i !== index);
  }

  async function save() {
    if (saving || loading || loadFailed) return;
    if (!name.trim()) {
      error = "El nombre es obligatorio.";
      return;
    }

    const valid = presentations.map((p) => ({ ...p, name: p.name.trim(), factor: Number(p.factor) }));
    if (valid.some((p) => !p.name || !Number.isFinite(p.factor) || p.factor <= 0)) {
      error = "Cada presentación necesita nombre y factor positivo.";
      return;
    }
    if (valid.filter((p) => p.factor === 1).length !== 1) {
      error = "Debe haber exactamente una presentación base con factor 1.";
      return;
    }
    const input: ProductInput = {
      id: initial?.id ?? null,
      name: name.trim(),
      description: description.trim() || null,
      category_id: categoryId ? Number(categoryId) : null,
      presentations: valid,
    };

    saving = true;
    error = "";
    try {
      await productsService.save(input);
      modalService.close();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title={initial ? "Editar producto" : "Nuevo producto"}
  onClose={() => { if (!saving) modalService.close(); }}
  width="lg"
>
  <div class="form">
    {#if loading}<p role="status">Cargando presentaciones…</p>{/if}
    <div class="grid">
      <div class="span-2">
        <Input label="Nombre" bind:value={name} placeholder="Nombre del producto" />
      </div>
      <div class="span-2">
        <Select
          label="Categoría"
          bind:value={categoryId}
          options={categoryOptions}
        />
      </div>
      <div class="span-2">
        <Textarea
          label="Observaciones"
          bind:value={description}
          placeholder="Notas asociadas a este producto"
        />
      </div>
    </div>

    <section class="presentations">
      <div class="section-head">
        <h3>Presentaciones</h3>
        <Button size="sm" variant="secondary" icon={Plus} onclick={addPresentation}>
          Añadir
        </Button>
      </div>
      <p class="hint">
        La presentación con factor 1 es la unidad base en la que se cuenta el
        stock. Se admiten hasta 6 decimales.
      </p>

      {#each presentations as presentation, index (index)}
        <div class="pres-row">
          <div class="pres-name">
            <Input bind:value={presentation.name} placeholder="Ej. Caja, Botella" />
          </div>
          <div class="pres-factor">
            <NumberStepper label="Factor" bind:value={presentation.factor} min={1} />
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
            aria-label="Quitar"
            disabled={presentations.length <= 1}
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
      <p class="error" role="alert">{error}</p>
      <DuplicateHint {name} {error} kind="product" currentId={initial?.id} />
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.close()}>
      Cancelar
    </Button>
    <Button disabled={!canSave} onclick={save}>
      {saving ? "Guardando…" : initial ? "Guardar" : "Crear"}
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

  .validation {
    color: var(--warning);
    font-size: 0.8125rem;
  }
</style>
