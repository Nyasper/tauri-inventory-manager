<script lang="ts">
  import { untrack } from "svelte";
  import type { Product } from "$lib/types";
  import {
    productService,
    type PresentationInput,
    type ProductInput,
  } from "$lib/services/product.service";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { categoriesStore } from "$lib/stores/categories.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import ImagePicker from "$lib/components/ui/ImagePicker.svelte";

  let { product = null }: { product?: Product | null } = $props();

  const initial = untrack(() => product);

  let name = $state(initial?.name ?? "");
  let description = $state(initial?.description ?? "");
  let image = $state<string | null>(initial?.image ?? null);
  let categoryId = $state(
    initial?.category_id != null ? String(initial.category_id) : "",
  );
  let presentations = $state<PresentationInput[]>([
    { name: "Unidad", factor: 1 },
  ]);
  let saving = $state(false);
  let error = $state("");

  const baseId = $props.id();

  if (initial?.id) {
    productService.presentations(initial.id).then((rows) => {
      presentations = rows.length
        ? rows.map((r) => ({ name: r.name, factor: r.factor }))
        : [{ name: "Unidad", factor: 1 }];
    });
  }

  const categoryOptions = $derived([
    { value: "", label: "Sin categoría" },
    ...categoriesStore.items.map((c) => ({
      value: String(c.id),
      label: c.name,
    })),
  ]);

  function addPresentation() {
    presentations.push({ name: "", factor: 1 });
  }

  function removePresentation(index: number) {
    if (presentations.length <= 1) return;
    presentations = presentations.filter((_, i) => i !== index);
  }

  async function save() {
    if (!name.trim()) {
      error = "El nombre es obligatorio.";
      return;
    }

    const valid = presentations
      .filter((p) => p.name.trim())
      .map((p) => ({
        name: p.name.trim(),
        factor: Number(p.factor) > 0 ? Number(p.factor) : 1,
      }));

    if (valid.length === 0) {
      error = "Añade al menos una presentación.";
      return;
    }

    if (!valid.some((p) => p.factor === 1)) {
      valid[0].factor = 1;
    }

    const input: ProductInput = {
      id: initial?.id ?? null,
      name: name.trim(),
      description: description.trim() || null,
      image,
      category_id: categoryId ? Number(categoryId) : null,
      presentations: valid,
    };

    saving = true;
    error = "";
    try {
      await productService.save(input);
      await productsStore.load();
      modalService.close();
    } catch {
      error = "No se pudo guardar el producto.";
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title={initial ? "Editar producto" : "Nuevo producto"}
  onClose={() => modalService.close()}
  width="lg"
>
  <div class="form">
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
      <div class="span-2">
        <ImagePicker bind:value={image} />
      </div>
    </div>

    <section class="presentations">
      <div class="section-head">
        <h3>Presentaciones</h3>
        <Button size="sm" variant="secondary" icon="plus" onclick={addPresentation}>
          Añadir
        </Button>
      </div>
      <p class="hint">
        La presentación con factor 1 es la unidad base en la que se cuenta el
        stock.
      </p>

      {#each presentations as presentation, index (index)}
        <div class="pres-row">
          <div class="pres-name">
            <Input bind:value={presentation.name} placeholder="Ej. Caja, Botella" />
          </div>
          <div class="pres-factor">
            <label class="field-label" for="{baseId}-factor-{index}">Factor</label>
            <input
              id="{baseId}-factor-{index}"
              type="number"
              min="0"
              step="any"
              class="input"
              bind:value={presentation.factor}
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
            aria-label="Quitar"
            disabled={presentations.length <= 1}
            onclick={() => removePresentation(index)}
          >
            <Icon name="trash" size={16} />
          </button>
        </div>
      {/each}
    </section>

    {#if error}
      <p class="error">{error}</p>
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" onclick={() => modalService.close()}>
      Cancelar
    </Button>
    <Button disabled={saving} onclick={save}>
      {initial ? "Guardar" : "Crear"}
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
    grid-template-columns: 1fr 110px 90px 32px;
    gap: 0.625rem;
    align-items: end;
  }

  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    outline: none;
  }

  .input:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
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
</style>
