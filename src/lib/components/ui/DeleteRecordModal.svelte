<script lang="ts">
  import Modal from "#lib/components/ui/Modal.svelte";
  import Input from "#lib/components/ui/Input.svelte";
  import Button from "#lib/components/ui/Button.svelte";
  import { Trash } from "@lucide/svelte";
  import { modalService } from "#lib/services/modal.service.svelte.js";
  import { errorMessage } from "#lib/utils/errors.js";

  let {
    recordType,
    name,
    onConfirm,
  }: {
    recordType: "product" | "supplier";
    name: string;
    onConfirm: (password: string) => Promise<void>;
  } = $props();

  const label = $derived(recordType === "product" ? "producto" : "proveedor");

  let password = $state("");
  let deleting = $state(false);
  let error = $state("");

  const canDelete = $derived(password.trim() !== "" && !deleting);
  const dirty = $derived(password !== "");

  async function confirmDeletion() {
    if (!canDelete) return;
    deleting = true;
    error = "";
    try {
      await onConfirm(password);
      modalService.close();
    } catch (e) {
      error = errorMessage(e);
    } finally {
      deleting = false;
    }
  }
</script>

<Modal
  title="Eliminar definitivamente"
  onClose={() => modalService.close()}
  hasUnsavedChanges={dirty}
  isBusy={deleting}
  width="sm"
>
  <div class="delete">
    <p class="message">
      ¿Eliminar definitivamente el {label} <strong>{name}</strong>? Esta acción no se puede
      deshacer.
    </p>
    <p class="hint">
      Sus movimientos se conservarán en el historial marcados como "{label}
      eliminado".
    </p>
    <Input label="Contraseña de administración" type="password" bind:value={password} />
    {#if error}<p class="error" role="alert">{error}</p>{/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={deleting} onclick={() => modalService.requestClose()}>
      Cancelar
    </Button>
    <Button variant="danger" icon={Trash} disabled={!canDelete} onclick={confirmDeletion}>
      {deleting ? "Eliminando…" : "Eliminar definitivamente"}
    </Button>
  {/snippet}
</Modal>

<style>
  .delete {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .message {
    font-size: 0.875rem;
    line-height: 1.5;
  }

  .hint {
    color: var(--text-muted);
    font-size: 0.8125rem;
    line-height: 1.4;
  }

  .error {
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>
