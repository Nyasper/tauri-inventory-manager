<script lang="ts">
  import DuplicateHint from "$lib/components/ui/DuplicateHint.svelte";
  import { untrack } from "svelte";
  import { Plus, Trash } from "@lucide/svelte";
  import type { Supplier } from "$lib/types";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { toAppError, type AppError } from "$lib/utils/errors";
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

  const initialSupplier = untrack(() => supplier);

  interface ContactDraft {
    key: number;
    value: string;
  }

  let nextContactKey = 0;
  function newContact(value = ""): ContactDraft {
    return { key: nextContactKey++, value };
  }

  let name = $state(initialSupplier?.name ?? "");
  let contactDrafts = $state<ContactDraft[]>(
    initialSupplier?.contacts.length
      ? initialSupplier.contacts.map((contact) => newContact(contact))
      : [newContact()],
  );
  let notes = $state(initialSupplier?.notes ?? "");
  let saving = $state(false);
  let error = $state<AppError | null>(null);

  const formSnapshot = () => JSON.stringify({ name, contactDrafts, notes });
  const initialSnapshot = $state(formSnapshot());
  const dirty = $derived(formSnapshot() !== initialSnapshot);

  const validation = $derived(!name.trim() ? "El nombre es obligatorio." : "");
  const canSave = $derived(validation === "" && !saving);

  async function save() {
    if (!canSave) return;

    saving = true;
    error = null;
    try {
      const data = {
        name: name.trim(),
        contacts: contactDrafts.map((contact) => contact.value),
        notes: notes.trim() || null,
      };
      if (initialSupplier) {
        await suppliersService.update({ ...data, id: initialSupplier.id });
      } else {
        const id = await suppliersService.create(data);
        onCreated?.(id);
      }
      modalService.close();
    } catch (e) {
      error = toAppError(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title={initialSupplier ? "Editar proveedor" : "Nuevo proveedor"}
  onClose={() => modalService.close()}
  hasUnsavedChanges={dirty}
  isBusy={saving}
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
          onclick={() => (contactDrafts = [...contactDrafts, newContact()])}
        >
          Añadir
        </Button>
      </div>
      {#each contactDrafts as contact, index (contact.key)}
        <div class="contact-row">
          <div class="contact-field">
            <Input
              label={index === 0 ? "Contacto principal" : `Contacto ${index + 1}`}
              bind:value={contact.value}
              placeholder="Teléfono, email, WhatsApp..."
            />
          </div>
          <button
            class="remove-contact"
            type="button"
            title="Quitar contacto"
            aria-label={`Quitar contacto ${index + 1}`}
            disabled={contactDrafts.length === 1}
            onclick={() =>
              (contactDrafts = contactDrafts.filter((item) => item.key !== contact.key))}
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
      <p class="error" role="alert">{error.message}</p>
      <DuplicateHint {name} {error} recordType="supplier" currentId={initialSupplier?.id} />
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.requestClose()}>
      Cancelar
    </Button>
    <Button disabled={!canSave} onclick={save}>
      {initialSupplier ? "Guardar" : "Crear"}
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
