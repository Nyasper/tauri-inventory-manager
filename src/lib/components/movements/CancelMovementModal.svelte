<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { productsService } from "$lib/services/products.service.svelte";
  import type { Movement } from "$lib/types";

  let { movement }: { movement: Movement } = $props();

  let reason = $state("");
  let saving = $state(false);
  let error = $state("");

  const canSave = $derived(reason.trim() !== "" && !saving);
  const dirty = $derived(reason !== "");

  async function save() {
    if (!canSave) return;
    saving = true;
    error = "";
    try {
      await movementsService.remove(movement.id, reason);
      await productsService.load();
      modalService.close();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal title="Anular movimiento" onClose={() => modalService.close()} {dirty} busy={saving}>
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
    <Button disabled={!canSave} onclick={save}>
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
