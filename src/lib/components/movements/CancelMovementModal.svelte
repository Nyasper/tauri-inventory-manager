<script lang="ts">
  import Modal from "#lib/components/ui/Modal.svelte";
  import Textarea from "#lib/components/ui/Textarea.svelte";
  import Button from "#lib/components/ui/Button.svelte";
  import { modalService } from "#lib/services/modal.service.svelte.js";
  import { movementsService } from "#lib/services/movements.service.svelte.js";
  import { productsService } from "#lib/services/products.service.svelte.js";
  import { errorMessage } from "#lib/utils/errors.js";
  import type { Movement } from "#lib/types.js";

  let { movement }: { movement: Movement } = $props();

  let reason = $state("");
  let saving = $state(false);
  let error = $state("");

  const canSave = $derived(reason.trim() !== "" && !saving);
  const dirty = $derived(reason !== "");

  async function cancelMovement() {
    if (!canSave) return;
    saving = true;
    error = "";
    try {
      await movementsService.cancel(movement.id, reason);
      await productsService.load();
      modalService.close();
    } catch (e) {
      error = errorMessage(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title="Anular movimiento"
  onClose={() => modalService.close()}
  hasUnsavedChanges={dirty}
  isBusy={saving}
>
  <p>
    Se revertirá el stock de {movement.product_name}. El movimiento y el motivo permanecerán en el
    historial.
  </p>
  <Textarea label="Motivo obligatorio" bind:value={reason} />
  {#if error}<p role="alert" class="error">{error}</p>{/if}
  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.requestClose()}>
      Volver
    </Button>
    <Button disabled={!canSave} onclick={cancelMovement}>
      {saving ? "Anulando…" : "Anular movimiento"}
    </Button>
  {/snippet}
</Modal>

<style>
  .error {
    margin-top: 0.5rem;
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>
