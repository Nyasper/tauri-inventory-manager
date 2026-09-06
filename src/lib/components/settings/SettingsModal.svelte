<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select, { type SelectOption } from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import { personsStore } from "$lib/stores/persons.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import type { DefaultView } from "$lib/types";
  import {
    accentService,
    ACCENTS,
  } from "$lib/stores/accent.service.svelte";

  let low = $state("");
  let medium = $state("");
  let newPerson = $state("");
  let editingId = $state<number | null>(null);
  let editName = $state("");
  let defaultView = $state("dashboard");

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
    if (!Number.isFinite(lowValue) || !Number.isFinite(mediumValue)) return;
    await settingsStore.saveThresholds(lowValue, mediumValue);
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
    const ok = await modalService.confirm(
      `¿Guardar el nombre "${newName}"?`,
      "Editar responsable",
    );
    if (!ok) return;
    await personsStore.update(editingId, newName, person.is_default);
    editingId = null;
  }

  async function removePerson(id: number) {
    const ok = await modalService.confirm(
      "¿Eliminar esta persona responsable?",
      "Eliminar persona",
    );
    if (ok) await personsStore.remove(id);
  }
</script>

<Modal title="Configuración" onClose={() => modalService.close()} width="md">
  <div class="settings">
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
      <h3>Responsables</h3>
      <p class="hint">
        Personas que se pueden asociar a los movimientos de stock.
      </p>

      <div class="add-row">
        <Input placeholder="Nombre de la persona" bind:value={newPerson} />
        <Button size="sm" icon="plus" onclick={addPerson}>Añadir</Button>
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
                  <Icon name="edit" size={16} />
                </button>
                <button
                  class="icon-btn"
                  type="button"
                  aria-label="Eliminar"
                  onclick={() => removePerson(person.id)}
                >
                  <Icon name="trash" size={16} />
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

  .person-actions {
    display: flex;
    gap: 0.125rem;
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
