<script lang="ts">
  import Modal from "#lib/components/ui/Modal.svelte";
  import Input from "#lib/components/ui/Input.svelte";
  import Select from "#lib/components/ui/Select.svelte";
  import type { SelectOption } from "#lib/components/ui/select-option.js";
  import Button from "#lib/components/ui/Button.svelte";
  import { Trash } from "@lucide/svelte";
  import { modalService } from "#lib/services/modal.service.svelte.js";
  import { movementsService } from "#lib/services/movements.service.svelte.js";
  import { feedbackService } from "#lib/services/feedback.service.svelte.js";
  import { errorMessage } from "#lib/utils/errors.js";
  import {
    countMovements,
    type MovementDeleteFilter,
    type MovementDeleteMode,
  } from "#lib/services/db.service.js";
  import { formatDay } from "#lib/utils/format.js";

  let mode = $state<MovementDeleteMode>("recent");
  let countInput = $state("1");
  let from = $state("");
  let to = $state("");
  let password = $state("");
  let deleting = $state(false);
  let deleteError = $state("");
  let countError = $state("");
  let matchedMovements = $state<number | null>(null);
  let matchedMovementsLoading = $state(false);

  const modeOptions: SelectOption[] = [
    { value: "recent", label: "Más recientes" },
    { value: "oldest", label: "Más antiguos" },
    { value: "from", label: "Desde una fecha en adelante" },
    { value: "until", label: "Hasta una fecha hacia atrás" },
    { value: "range", label: "Entre dos fechas" },
  ];

  const count = $derived(Number(countInput));
  const needsCount = $derived(mode === "recent" || mode === "oldest");
  const needsFrom = $derived(mode === "from" || mode === "range");
  const needsTo = $derived(mode === "until" || mode === "range");

  const filter = $derived<MovementDeleteFilter>({
    mode,
    count: needsCount ? count : undefined,
    from: needsFrom && from ? from : undefined,
    to: needsTo && to ? to : undefined,
  });

  const filterError = $derived.by(() => {
    if (needsCount && !(Number.isInteger(count) && count >= 1)) {
      return "Indica una cantidad entera mayor o igual a 1.";
    }
    if (needsFrom && !from) return "Indica la fecha de inicio.";
    if (needsTo && !to) return "Indica la fecha límite.";
    if (mode === "range" && from && to && from >= to) {
      return "La fecha inicial debe ser anterior a la final.";
    }
    return "";
  });

  const dirty = $derived(
    password !== "" || mode !== "recent" || Number(countInput) !== 1 || from !== "" || to !== "",
  );

  const canDelete = $derived(
    filterError === "" &&
      !countError &&
      matchedMovements !== null &&
      matchedMovements > 0 &&
      password.trim() !== "" &&
      !deleting,
  );

  // Vista previa con retardo para no consultar en cada tecla del filtro.
  $effect(() => {
    if (filterError) {
      countError = "";
      matchedMovements = null;
      matchedMovementsLoading = false;
      return;
    }
    const current = filter;
    let cancelled = false;
    countError = "";
    matchedMovements = null;
    matchedMovementsLoading = true;
    const timer = setTimeout(() => {
      countMovements(current)
        .then((total) => {
          if (!cancelled) {
            matchedMovements = total;
            countError = "";
          }
        })
        .catch((e) => {
          if (!cancelled) {
            matchedMovements = null;
            countError = errorMessage(e);
          }
        })
        .finally(() => {
          if (!cancelled) matchedMovementsLoading = false;
        });
    }, 250);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  });

  const matchedMovementsText = $derived.by(() => {
    if (filterError) return "Completa los datos del filtro.";
    if (countError) return countError;
    if (matchedMovementsLoading) return "Calculando coincidencias…";
    if (matchedMovements === null) return "No se pudo calcular la cantidad.";
    if (matchedMovements === 0) return "No hay movimientos que coincidan.";
    return `Se eliminarán ${matchedMovements} movimiento${matchedMovements === 1 ? "" : "s"}.`;
  });

  function describe(total: number): string {
    const label = total === 1 ? "1 movimiento" : `${total} movimientos`;
    switch (mode) {
      case "recent":
        return `Se eliminarán los ${label} más recientes del historial.`;
      case "oldest":
        return `Se eliminarán los ${label} más antiguos del historial.`;
      case "from":
        return `Se eliminarán los ${label} registrados desde el ${formatDay(from)} en adelante.`;
      case "until":
        return `Se eliminarán los ${label} registrados hasta el ${formatDay(to)} inclusive.`;
      case "range":
        return `Se eliminarán los ${label} registrados entre el ${formatDay(from)} y el ${formatDay(to)}.`;
      default:
        return `Se eliminarán ${label}.`;
    }
  }

  async function deleteMovements() {
    if (!canDelete || matchedMovements === null) return;
    const ok = await modalService.confirm(
      `${describe(matchedMovements)} El stock actual de los productos no cambia. Esta acción no se puede deshacer.`,
      "Eliminar movimientos",
    );
    if (!ok) return;
    deleting = true;
    deleteError = "";
    try {
      const deleted = await movementsService.deleteBatch(filter, password);
      feedbackService.success(`Se eliminaron ${deleted} movimiento${deleted === 1 ? "" : "s"}.`);
      modalService.close();
    } catch (e) {
      deleteError = errorMessage(e);
    } finally {
      deleting = false;
    }
  }
</script>

<Modal
  title="Eliminar movimientos"
  onClose={() => modalService.close()}
  hasUnsavedChanges={dirty}
  isBusy={deleting}
>
  <p class="hint">
    Borra movimientos del historial por cantidad (los más recientes o los más antiguos) o por fecha.
    El stock actual de los productos no cambia.
  </p>
  <div class="grid">
    <Select label="Qué borrar" bind:value={mode} options={modeOptions} />
    {#if needsCount}
      <Input label="Cantidad" type="number" bind:value={countInput} />
    {/if}
  </div>
  {#if needsFrom || needsTo}
    <div class="grid dates">
      {#if needsFrom}
        <Input label="Desde" type="date" bind:value={from} />
      {/if}
      {#if needsTo}
        <Input label="Hasta" type="date" bind:value={to} />
      {/if}
    </div>
  {/if}
  <div class="password">
    <Input label="Contraseña de administrador" type="password" bind:value={password} />
  </div>
  {#if filterError}
    <p class="validation" role="status">{filterError}</p>
  {:else if countError}
    <p class="error" role="alert">{countError}</p>
  {:else}
    <p class="matched-movements" aria-live="polite">{matchedMovementsText}</p>
  {/if}
  {#if deleteError}<p class="error" role="alert">{deleteError}</p>{/if}
  <div class="section-action">
    <Button variant="danger" icon={Trash} disabled={!canDelete} onclick={deleteMovements}>
      {deleting ? "Eliminando…" : "Eliminar movimientos"}
    </Button>
  </div>
</Modal>

<style>
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

  .dates,
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

  .matched-movements {
    margin-top: 0.5rem;
    color: var(--text-muted);
    font-size: 0.8125rem;
  }

  .error {
    margin-top: 0.5rem;
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>
