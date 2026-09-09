<script lang="ts">
  import { personService } from "$lib/services/person.service";
  import type { ResponsiblePerson } from "$lib/types";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select, { type SelectOption } from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import IconPicker from "$lib/components/ui/IconPicker.svelte";
  import { ICONS, type IconName } from "$lib/icons";
  import { Plus, Pencil, Trash } from "@lucide/svelte";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import { categoriesStore } from "$lib/stores/categories.store.svelte";
  import { personsStore } from "$lib/stores/persons.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import type { DefaultView } from "$lib/types";
  import {
    accentService,
    ACCENTS,
  } from "$lib/stores/accent.service.svelte";

  const CATEGORY_COLORS = [
    "#3b82f6",
    "#22c55e",
    "#8b5cf6",
    "#14b8a6",
    "#64748b",
    "#f59e0b",
    "#ef4444",
    "#e11d48",
  ];

  interface CatDraft {
    id: number | null;
    name: string;
    color: string;
    icon: IconName | null;
  }

  let archivedPersons=$state<ResponsiblePerson[]>([]);
  let showArchivedPersons=$state(false);
  async function loadArchivedPersons() { archivedPersons=await personService.list(true); }
  $effect(()=>{if(showArchivedPersons) loadArchivedPersons();});
  async function restorePerson(id:number) { await personService.restore(id); await personsStore.load(); await loadArchivedPersons(); }
  let low = $state("");
  let medium = $state("");
  let newPerson = $state("");
  let editingId = $state<number | null>(null);
  let editName = $state("");
  let defaultView = $state("dashboard");
  let editingCat = $state<CatDraft | null>(null);

  const viewOptions: SelectOption[] = [
    { value: "dashboard", label: "Dashboard" },
    { value: "products", label: "Productos" },
    { value: "movements", label: "Movimientos" },
    { value: "suppliers", label: "Proveedores" },
  ];

  const selectedAccentLabel = $derived(
    ACCENTS.find((a) => a.id === accentService.accent)?.label ?? "",
  );

  $effect(() => {
    low = String(settingsStore.settings.stock_low_threshold);
    medium = String(settingsStore.settings.stock_medium_threshold);
  });

  $effect(() => {
    defaultView = settingsStore.settings.default_view;
  });

  async function saveDefaultView() {
    await settingsStore.saveDefaultView(defaultView as DefaultView);
  }

  async function saveThresholds() {
    const lowValue = Number(low);
    const mediumValue = Number(medium);
    if (!Number.isFinite(lowValue) || !Number.isFinite(mediumValue) || lowValue < 0 || mediumValue <= lowValue) {
      throw new Error("Los umbrales deben cumplir: 0 ≤ bajo < medio.");
    }
    await settingsStore.saveThresholds(lowValue, mediumValue);
  }

  function startNewCategory() {
    editingCat = { id: null, name: "", color: CATEGORY_COLORS[0], icon: "box" };
  }

  function startEditCategory(id: number, name: string, color: string | null, icon: string | null) {
    editingCat = {
      id,
      name,
      color: color ?? CATEGORY_COLORS[0],
      icon: icon && icon in ICONS ? (icon as IconName) : null,
    };
  }

  async function saveCategory() {
    if (!editingCat) return;
    const name = editingCat.name.trim();
    if (!name) {
      await modalService.confirm("El nombre de la categoría es obligatorio.", "Categoría");
      return;
    }
    if (editingCat.id === null) {
      await categoriesStore.create(name, editingCat.color, editingCat.icon);
    } else {
      await categoriesStore.update(editingCat.id, name, editingCat.color, editingCat.icon);
    }
    editingCat = null;
  }

  async function removeCategory(id: number, name: string) {
    const ok = await modalService.confirm(
      `¿Eliminar la categoría "${name}"? Los productos asociados quedarán sin categoría.`,
      "Eliminar categoría",
    );
    if (ok) await categoriesStore.remove(id);
  }

  async function addPerson() {
    const name = newPerson.trim();
    if (!name) return;
    await personsStore.create(name, personsStore.items.length === 0);
    newPerson = "";
  }

  async function setDefault(id: number) {
    const person = personsStore.items.find((p) => p.id === id);
    if (!person) return;
    const ok = await modalService.confirm(
      `¿Establecer a "${person.name}" como responsable por defecto?`,
      "Responsable por defecto",
    );
    if (ok) await personsStore.update(id, person.name, true);
  }

  function startEdit(id: number, name: string) {
    editingId = id;
    editName = name;
  }

  async function saveEdit() {
    if (editingId === null) return;
    const person = personsStore.items.find((p) => p.id === editingId);
    if (!person) return;
    const newName = editName.trim() || person.name;
    await personsStore.update(editingId, newName, person.is_default);
    editingId = null;
  }

  async function removePerson(id: number) {
    const ok = await modalService.confirm(
      "¿Archivar esta persona responsable? Su historial se conservará.",
      "Archivar persona",
    );
    if (ok) await personsStore.remove(id);
  }
</script>

<Modal title="Configuración" onClose={() => modalService.close()} width="md">
  <div class="settings">
    <section><label><input type="checkbox" bind:checked={showArchivedPersons} /> Responsables archivados</label>
    {#if showArchivedPersons}{#each archivedPersons as person}<p>{person.name} <Button size="sm" variant="secondary" onclick={()=>restorePerson(person.id)}>Restaurar</Button></p>{/each}{#if !archivedPersons.length}<p>No hay responsables archivados.</p>{/if}{/if}</section>
    <section>
      <h3>Franjas de stock</h3>
      <p class="hint">
        Define los umbrales para clasificar el estado de stock de cada producto.
      </p>
      <div class="thresholds">
        <Input label="Umbral bajo" type="number" bind:value={low} />
        <Input label="Umbral medio" type="number" bind:value={medium} />
      </div>
      <div class="section-action">
        <Button size="sm" onclick={saveThresholds}>Guardar franjas</Button>
      </div>
    </section>

    <section>
      <h3>Color de acento</h3>
      <p class="hint">
        Personaliza el color de énfasis de toda la aplicación.
      </p>
      <div class="accents">
        {#each ACCENTS as accent (accent.id)}
          <button
            type="button"
            class="swatch"
            class:active={accentService.accent === accent.id}
            style="--swatch: {accent.light.primary}"
            aria-label={accent.label}
            title={accent.label}
            onclick={() => accentService.setAccent(accent.id)}
          ></button>
        {/each}
      </div>
      <p class="selected-accent">Seleccionado: {selectedAccentLabel}</p>
    </section>

    <section>
      <h3>Vista por defecto</h3>
      <p class="hint">
        Vista que se muestra al abrir la aplicación.
      </p>
      <Select
        bind:value={defaultView}
        options={viewOptions}
        onchange={saveDefaultView}
      />
    </section>

    <section>
      <h3>Categorías</h3>
      <p class="hint">
        Los productos muestran el icono y el color de su categoría.
      </p>

      {#if editingCat}
        {@const cat = editingCat}
        <div class="cat-editor">
          <Input
            label="Nombre"
            placeholder="Nombre de la categoría"
            bind:value={cat.name}
          />
          <span class="field-label">Color</span>
          <div class="cat-colors">
            {#each CATEGORY_COLORS as color (color)}
              <button
                type="button"
                class="cat-color"
                class:active={cat.color === color}
                style="--swatch: {color}"
                aria-label={color}
                onclick={() => (cat.color = color)}
              ></button>
            {/each}
          </div>
          <IconPicker bind:value={cat.icon} />
          <div class="cat-editor-actions">
            <Button variant="secondary" size="sm" onclick={() => (editingCat = null)}>
              Cancelar
            </Button>
            <Button size="sm" onclick={saveCategory}>
              {cat.id === null ? "Crear" : "Guardar"}
            </Button>
          </div>
        </div>
      {:else}
        <div class="section-action">
          <Button size="sm" variant="secondary" icon={Plus} onclick={startNewCategory}>
            Nueva categoría
          </Button>
        </div>
      {/if}

      {#if categoriesStore.items.length === 0}
        <p class="empty-note">Sin categorías configuradas.</p>
      {:else}
        <ul class="cats">
          {#each categoriesStore.items as category (category.id)}
            {@const Icon = category.icon && category.icon in ICONS ? ICONS[category.icon as IconName] : ICONS.box}
            <li class="cat">
              <span
                class="cat-tile"
                style:background={category.color ?? "var(--surface-2)"}
                style:color={category.color ? "#ffffff" : "var(--text-muted)"}
              >
                <Icon size={16} />
              </span>
              <span class="cat-name" title={category.name}>{category.name}</span>
              <div class="cat-actions">
                <button
                  class="icon-btn"
                  type="button"
                  aria-label="Editar"
                  onclick={() =>
                    startEditCategory(category.id, category.name, category.color, category.icon)
                  }
                >
                  <Pencil size={16} />
                </button>
                <button
                  class="icon-btn"
                  type="button"
                  aria-label="Eliminar"
                  onclick={() => removeCategory(category.id, category.name)}
                >
                  <Trash size={16} />
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </section>

    <section>
      <h3>Responsables</h3>
      <p class="hint">
        Personas que se pueden asociar a los movimientos de stock.
      </p>

      <div class="add-row">
        <Input placeholder="Nombre de la persona" bind:value={newPerson} />
        <Button size="sm" icon={Plus} onclick={addPerson}>Añadir</Button>
      </div>

      {#if personsStore.items.length === 0}
        <p class="empty-note">Sin responsables configurados.</p>
      {:else}
        <ul class="people">
          {#each personsStore.items as person (person.id)}
            <li class="person">
              {#if editingId === person.id}
                <Input bind:value={editName} />
                <Button size="sm" onclick={saveEdit}>Guardar</Button>
              {:else}
                <label class="person-main">
                  <input
                    type="radio"
                    name="default-person"
                    checked={person.is_default}
                    onchange={() => setDefault(person.id)}
                    autocomplete="off"
                  />
                  <span>{person.name}</span>
                  {#if person.is_default}
                    <span class="default-tag">Por defecto</span>
                  {/if}
                </label>
              {/if}
              <div class="person-actions">
                <button
                  class="icon-btn"
                  type="button"
                  aria-label="Editar"
                  onclick={() => startEdit(person.id, person.name)}
                >
                  <Pencil size={16} />
                </button>
                <button
                  class="icon-btn"
                  type="button"
                  aria-label="Eliminar"
                  onclick={() => removePerson(person.id)}
                >
                  <Trash size={16} />
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  </div>
</Modal>

<style>
  .settings {
    display: flex;
    flex-direction: column;
    gap: 1.75rem;
  }

  h3 {
    font-size: 0.9375rem;
    margin-bottom: 0.25rem;
  }

  .hint {
    color: var(--text-muted);
    font-size: 0.8125rem;
    margin-bottom: 0.75rem;
  }

  .thresholds {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .section-action {
    margin-top: 0.75rem;
    display: flex;
    justify-content: flex-end;
  }

  .accents {
    display: flex;
    flex-wrap: wrap;
    gap: 0.625rem;
  }

  .swatch {
    width: 32px;
    height: 32px;
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

  .swatch:hover {
    transform: scale(1.1);
  }

  .swatch.active {
    box-shadow: 0 0 0 2px var(--primary);
  }

  .selected-accent {
    margin-top: 0.5rem;
    color: var(--text-muted);
    font-size: 0.8125rem;
  }

  .cat-editor {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 0.875rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    margin-bottom: 0.75rem;
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

  .cat-editor-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
  }

  .cats {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .cat {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.375rem 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
  }

  .cat-tile {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border-radius: var(--radius-sm);
    flex-shrink: 0;
  }

  .cat-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: 500;
  }

  .cat-actions,
  .person-actions {
    display: flex;
    gap: 0.125rem;
  }

  .add-row {
    display: flex;
    gap: 0.5rem;
    align-items: flex-start;
    margin-bottom: 0.75rem;
  }

  .add-row :global(.input) {
    flex: 1;
  }

  .empty-note {
    color: var(--text-muted);
    font-size: 0.8125rem;
  }

  .people {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .person {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.375rem 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
  }

  .person-main {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    flex: 1;
  }

  .person-main input {
    accent-color: var(--primary);
  }

  .default-tag {
    font-size: 0.7rem;
    color: var(--primary);
    background: var(--primary-soft);
    padding: 0.0625rem 0.375rem;
    border-radius: 999px;
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
    background: var(--surface-2);
    color: var(--text);
  }
</style>