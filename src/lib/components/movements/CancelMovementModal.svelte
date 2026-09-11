<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import { movementsStore } from "$lib/stores/movements.store.svelte";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import type { Movement } from "$lib/types";
  let { movement }: { movement: Movement } = $props();
  let reason = $state(""); let saving = $state(false); let error = $state("");
  const canSave = $derived(reason.trim() !== "" && !saving);
  async function save() {
    if (!canSave) return;
    saving = true;
    try { await movementsStore.remove(movement.id, reason); await productsStore.load(); modalService.close(); }
    catch(e) { error = String(e); } finally { saving = false; }
  }
</script>
<Modal title="Anular movimiento" onClose={() => { if (!saving) modalService.close(); }}>
  <p>Se revertirá el stock de {movement.product_name}. El movimiento y el motivo permanecerán en el historial.</p>
  <Textarea label="Motivo obligatorio" bind:value={reason} />
  {#if error}<p role="alert" class="error">{error}</p>{/if}
  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.close()}>Volver</Button>
    <Button disabled={!canSave} onclick={save}>{saving ? "Anulando…" : "Anular movimiento"}</Button>
  {/snippet}
</Modal>
