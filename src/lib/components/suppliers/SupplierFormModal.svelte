<script lang="ts">
  import DuplicateHint from "$lib/components/ui/DuplicateHint.svelte";
  import { untrack } from "svelte";
  import type { Supplier } from "$lib/types";
  import { suppliersStore } from "$lib/stores/suppliers.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Button from "$lib/components/ui/Button.svelte";

  let { supplier = null }: { supplier?: Supplier | null } = $props();

  const initial = untrack(() => supplier);

  let name = $state(initial?.name ?? "");
  let contact = $state(initial?.contact ?? "");
  let notes = $state(initial?.notes ?? "");
  let saving = $state(false);
  let error = $state("");

  async function save() {
    if (saving) return;
    if (!name.trim()) {
      error = "El nombre es obligatorio.";
      return;
    }

    saving = true;
    error = "";
    try {
      const data = {
        name: name.trim(),
        contact: contact.trim() || null,
        notes: notes.trim() || null,
      };
      if (initial) {
        await suppliersStore.update({ ...data, id: initial.id });
      } else {
        await suppliersStore.create(data);
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
  onClose={() => { if (!saving) modalService.close(); }}
  width="md"
>
  <div class="form">
    <Input label="Nombre" bind:value={name} placeholder="Nombre del proveedor" />
    <Input
      label="Contacto"
      bind:value={contact}
      placeholder="Teléfono, email..."
    />
    <Textarea label="Notas" bind:value={notes} placeholder="Notas adicionales" />
    {#if error}
      <p class="error" role="alert">{error}</p>
      <DuplicateHint {name} {error} kind="supplier" currentId={initial?.id} />
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.close()}>
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
    gap: 0.875rem;
  }

  .error {
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>
