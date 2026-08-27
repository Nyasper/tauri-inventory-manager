<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import { personsStore } from "$lib/stores/persons.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";

  let low = $state("");
  let medium = $state("");
  let newPerson = $state("");
  let editingId = $state<number | null>(null);
  let editName = $state("");

  $effect(() => {
    low = String(settingsStore.settings.stock_low_threshold);
    medium = String(settingsStore.settings.stock_medium_threshold);
  });

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
    if (person) await personsStore.update(id, person.name, true);
  }

  function startEdit(id: number, name: string) {
    editingId = id;
    editName = name;
  }

  async function saveEdit() {
    if (editingId === null) return;
    const person = personsStore.items.find((p) => p.id === editingId);
    if (person) {
      await personsStore.update(editingId, editName.trim() || person.name, person.is_default);
    }
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
