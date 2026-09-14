<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select, { type SelectOption } from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { Trash } from "@lucide/svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { feedback } from "$lib/services/feedback.service.svelte";
  import type { MovementDeleteMode } from "$lib/services/db.service";

  let mode = $state<MovementDeleteMode>("recent");
  let count = $state("1");
  let password = $state("");
  let deleting = $state(false);
  let deleteError = $state("");

  const modeOptions: SelectOption[] = [
    { value: "recent", label: "Más recientes" },
    { value: "oldest", label: "Más antiguos" },
  ];

  const countValue = $derived(Number(count));
  const countValid = $derived(Number.isInteger(countValue) && countValue >= 1);
  const canDelete = $derived(countValid && password.trim() !== "" && !deleting);

  async function remove() {
    if (!canDelete) return;
    const ok = await modalService.confirm(
      `Se eliminarán hasta ${countValue} movimientos ${
        mode === "recent" ? "más recientes" : "más antiguos"
      } del historial. Esta acción no se puede deshacer.`,
      "Eliminar movimientos",
    );
    if (!ok) return;
    deleting = true;
    deleteError = "";
    try {
      const deleted = await movementsService.deleteBatch(mode, countValue, password);
      feedback.success(
        `Se eliminaron ${deleted} movimiento${deleted === 1 ? "" : "s"}.`,
      );
      modalService.close();
    } catch (e) {
      deleteError = String(e);
    } finally {
      deleting = false;
    }
  }
</script>

<Modal
  title="Eliminar movimientos"
  onClose={() => {
    if (!deleting) modalService.close();
  }}
>
  <section>
    <h3>Eliminar movimientos</h3>
    <p class="hint">
      Borra movimientos del historial en lote, empezando por los más recientes
      o por los más antiguos. El stock actual de los productos no cambia.
    </p>
    <div class="grid">
      <Select label="Desde" bind:value={mode} options={modeOptions} />
      <Input label="Cantidad" type="number" bind:value={count} />
    </div>
    <div class="password">
      <Input label="Contraseña" type="password" bind:value={password} />
    </div>
    {#if !countValid}
      <p class="validation" role="status">
        Indica una cantidad entera mayor o igual a 1.
      </p>
    {/if}
    {#if deleteError}<p class="error" role="alert">{deleteError}</p>{/if}
    <div class="section-action">
      <Button
        variant="danger"
        icon={Trash}
        disabled={!canDelete}
        onclick={remove}
      >
        {deleting ? "Eliminando…" : "Eliminar movimientos"}
      </Button>
    </div>
  </section>
</Modal>

<style>
  h3 {
    font-size: 0.9375rem;
    margin-bottom: 0.25rem;
  }

  .hint {
    color: var(--text-muted);
    font-size: 0.8125rem;
    margin-bottom: 0.75rem;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .password {
    margin-top: 0.75rem;
  }

  .section-action {
    margin-top: 0.75rem;
    display: flex;
    justify-content: flex-end;
  }

  .validation {
    margin-top: 0.5rem;
    color: var(--warning);
    font-size: 0.8125rem;
  }

  .error {
    margin-top: 0.5rem;
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>
