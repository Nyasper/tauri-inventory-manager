<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { Trash } from "@lucide/svelte";
  import { modalService } from "$lib/services/modal.service.svelte";

  let {
    kind,
    name,
    onConfirm,
  }: {
    kind: "product" | "supplier";
    name: string;
    onConfirm: (password: string) => Promise<void>;
  } = $props();

  const label = $derived(kind === "product" ? "producto" : "proveedor");

  let password = $state("");
  let deleting = $state(false);
  let error = $state("");

  const canDelete = $derived(password.trim() !== "" && !deleting);

  async function submit() {
    if (!canDelete) return;
    deleting = true;
    error = "";
    try {
      await onConfirm(password);
      modalService.close();
    } catch (e) {
      error = String(e);
    } finally {
      deleting = false;
    }
  }
</script>

<Modal
  title="Eliminar definitivamente"
  onClose={() => {
    if (!deleting) modalService.close();
  }}
  width="sm"
>
  <div class="delete">
    <p class="message">
      ¿Eliminar definitivamente el {label} <strong>{name}</strong>? Esta acción
      no se puede deshacer.
    </p>
    <p class="hint">
      Sus movimientos se conservarán en el historial marcados como "{label}
      eliminado".
    </p>
    <Input label="Contraseña de administración" type="password" bind:value={password} />
    {#if error}<p class="error" role="alert">{error}</p>{/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={deleting} onclick={() => modalService.close()}>
      Cancelar
    </Button>
    <Button variant="danger" icon={Trash} disabled={!canDelete} onclick={submit}>
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
