<script lang="ts">
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import IconPicker from "$lib/components/ui/IconPicker.svelte";
  import { categoriesService } from "$lib/services/categories.service.svelte";
  import { CATEGORY_COLORS, type CategoryDraft } from "$lib/categories";

  let {
    draft,
    onSaved,
    onCancel,
  }: {
    draft: CategoryDraft;
    onSaved?: (id: number) => void;
    onCancel: () => void;
  } = $props();

  let saving = $state(false);
  let error = $state("");

  const canSave = $derived(draft.name.trim() !== "" && !saving);

  async function save() {
    if (!canSave) return;
    saving = true;
    error = "";
    try {
      const name = draft.name.trim();
      let id = draft.id;
      if (id === null) {
        id = await categoriesService.create(name, draft.color, draft.icon);
      } else {
        await categoriesService.update(id, name, draft.color, draft.icon);
      }
      onSaved?.(id);
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<div class="editor">
  <Input
    label="Nombre"
    placeholder="Nombre de la categoría"
    bind:value={draft.name}
  />
  <span class="field-label">Color</span>
  <div class="cat-colors">
    {#each CATEGORY_COLORS as color (color)}
      <button
        type="button"
        class="cat-color"
        class:active={draft.color === color}
        style="--swatch: {color}"
        aria-label={color}
        onclick={() => (draft.color = color)}
      ></button>
    {/each}
  </div>
  <IconPicker bind:value={draft.icon} />
  {#if error}
    <p class="error" role="alert">{error}</p>
  {:else if !canSave && !saving}
    <p class="validation" role="status">El nombre es obligatorio.</p>
  {/if}
  <div class="actions">
    <Button variant="secondary" size="sm" disabled={saving} onclick={onCancel}>
      Cancelar
    </Button>
    <Button size="sm" disabled={!canSave} onclick={save}>
      {saving ? "Guardando…" : draft.id === null ? "Crear" : "Guardar"}
    </Button>
  </div>
</div>

<style>
  .editor {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .field-label {
    display: block;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .cat-colors {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .cat-color {
    width: 26px;
    height: 26px;
    padding: 0;
    border-radius: 50%;
    border: 2px solid var(--surface);
    background: var(--swatch);
    box-shadow: 0 0 0 1px var(--border);
    cursor: pointer;
    transition:
      transform 0.12s,
      box-shadow 0.12s;
  }

  .cat-color:hover {
    transform: scale(1.1);
  }

  .cat-color.active {
    box-shadow: 0 0 0 2px var(--primary);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
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