<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { personsService } from "$lib/services/persons.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";

  let { onCreated }: { onCreated?: (id: number) => void } = $props();

  let name = $state("");
  let isDefault = $state(personsService.items.length === 0);
  let saving = $state(false);
  let error = $state("");

  const validation = $derived(!name.trim() ? "El nombre es obligatorio." : "");
  const canSave = $derived(validation === "" && !saving);

  async function save() {
    if (!canSave) return;
    saving = true;
    error = "";
    try {
      const id = await personsService.create(name.trim(), isDefault);
      onCreated?.(id);
      modalService.close();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title="Nuevo responsable"
  onClose={() => { if (!saving) modalService.close(); }}
  width="sm"
>
  <div class="form">
    <Input
      label="Nombre"
      bind:value={name}
      placeholder="Nombre del responsable"
    />
    <label class="check">
      <input type="checkbox" bind:checked={isDefault} />
      Responsable por defecto
    </label>
    {#if validation && !saving}
      <p class="validation" role="status">{validation}</p>
    {/if}
    {#if error}
      <p class="error" role="alert">{error}</p>
    {/if}
  </div>

  {#snippet footer()}
    <Button
      variant="secondary"
      disabled={saving}
      onclick={() => modalService.close()}
    >
      Cancelar
    </Button>
    <Button disabled={!canSave} onclick={save}>
      {saving ? "Guardando…" : "Crear"}
    </Button>
  {/snippet}
</Modal>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
  }

  .check {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    cursor: pointer;
  }

  .check input {
    accent-color: var(--primary);
  }

  .validation {
    color: var(--warning);
    font-size: 0.8125rem;
  }

  .error {
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>