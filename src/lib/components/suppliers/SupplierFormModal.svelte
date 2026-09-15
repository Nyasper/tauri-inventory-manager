<script lang="ts">
  import DuplicateHint from "$lib/components/ui/DuplicateHint.svelte";
  import { untrack } from "svelte";
  import { Plus, Trash } from "@lucide/svelte";
  import type { Supplier } from "$lib/types";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Button from "$lib/components/ui/Button.svelte";

  let {
    supplier = null,
    onCreated,
  }: {
    supplier?: Supplier | null;
    onCreated?: (id: number) => void;
  } = $props();

  const initial = untrack(() => supplier);

  let name = $state(initial?.name ?? "");
  let contacts = $state<string[]>(
    initial?.contacts.length ? [...initial.contacts] : [""],
  );
  let notes = $state(initial?.notes ?? "");
  let saving = $state(false);
  let error = $state("");

  const snapshot = () => JSON.stringify({ name, contacts, notes });
  const baseline = $state(snapshot());
  const dirty = $derived(snapshot() !== baseline);

  const validation = $derived(!name.trim() ? "El nombre es obligatorio." : "");
  const canSave = $derived(validation === "" && !saving);

  async function save() {
    if (!canSave) return;

    saving = true;
    error = "";
    try {
      const data = {
        name: name.trim(),
        contacts,
        notes: notes.trim() || null,
      };
      if (initial) {
        await suppliersService.update({ ...data, id: initial.id });
      } else {
        const id = await suppliersService.create(data);
        onCreated?.(id);
      }
      modalService.close();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title={initial ? "Editar proveedor" : "Nuevo proveedor"}
  onClose={() => modalService.close()}
  dirty={dirty}
  busy={saving}
  width="md"
>
  <div class="form">
    <Input label="Nombre" bind:value={name} placeholder="Nombre del proveedor" />
    <section class="contacts">
      <div class="section-head">
        <span class="section-label">Contactos</span>
        <Button
          size="sm"
          variant="secondary"
          icon={Plus}
          onclick={() => (contacts = [...contacts, ""])}
        >
          Añadir
        </Button>
      </div>
      {#each contacts as _, index (index)}
        <div class="contact-row">
          <div class="contact-field">
            <Input
              label={index === 0 ? "Contacto principal" : `Contacto ${index + 1}`}
              bind:value={contacts[index]}
              placeholder="Teléfono, email, WhatsApp..."
            />
          </div>
          <button
            class="remove-contact"
            type="button"
            title="Quitar contacto"
            aria-label={`Quitar contacto ${index + 1}`}
            disabled={contacts.length === 1}
            onclick={() =>
              (contacts = contacts.filter((_, contactIndex) => contactIndex !== index))}
          >
            <Trash size={16} />
          </button>
        </div>
      {/each}
    </section>
    <Textarea label="Notas" bind:value={notes} placeholder="Notas adicionales" />
    {#if validation && !saving}
      <p class="validation" role="status">{validation}</p>
    {/if}
    {#if error}
      <p class="error" role="alert">{error}</p>
      <DuplicateHint {name} {error} kind="supplier" currentId={initial?.id} />
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.requestClose()}>
      Cancelar
    </Button>
    <Button disabled={!canSave} onclick={save}>
      {initial ? "Guardar" : "Crear"}
    </Button>
  {/snippet}
</Modal>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
  }

  .error {
    color: var(--danger);
    font-size: 0.8125rem;
  }

  .validation {
    color: var(--warning);
    font-size: 0.8125rem;
  }

  .contacts {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
  }

  .section-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .section-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .contact-row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 34px;
    gap: 0.5rem;
    align-items: end;
  }

  .contact-field {
    min-width: 0;
  }

  .remove-contact {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 38px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .remove-contact:hover:not(:disabled) {
    background: var(--danger-soft);
    color: var(--danger);
  }
</style>
