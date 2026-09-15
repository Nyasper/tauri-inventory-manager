<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select, { type SelectOption } from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { Trash } from "@lucide/svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { feedback } from "$lib/services/feedback.service.svelte";
  import {
    countMovements,
    type MovementDeleteFilter,
    type MovementDeleteMode,
  } from "$lib/services/db.service";
  import { formatDay } from "$lib/utils/format";

  let mode = $state<MovementDeleteMode>("recent");
  let count = $state("1");
  let from = $state("");
  let to = $state("");
  let password = $state("");
  let deleting = $state(false);
  let deleteError = $state("");
  let preview = $state<number | null>(null);
  let previewLoading = $state(false);

  const modeOptions: SelectOption[] = [
    { value: "recent", label: "Más recientes" },
    { value: "oldest", label: "Más antiguos" },
    { value: "from", label: "Desde una fecha en adelante" },
    { value: "until", label: "Hasta una fecha hacia atrás" },
    { value: "range", label: "Entre dos fechas" },
  ];

  const countValue = $derived(Number(count));
  const needsCount = $derived(mode === "recent" || mode === "oldest");
  const needsFrom = $derived(mode === "from" || mode === "range");
  const needsTo = $derived(mode === "until" || mode === "range");

  const filter = $derived<MovementDeleteFilter>({
    mode,
    count: needsCount ? countValue : undefined,
    from: needsFrom && from ? from : undefined,
    to: needsTo && to ? to : undefined,
  });

  const filterError = $derived.by(() => {
    if (needsCount && !(Number.isInteger(countValue) && countValue >= 1)) {
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
    password !== "" || mode !== "recent" || Number(count) !== 1 || from !== "" || to !== "",
  );

  const canDelete = $derived(
    filterError === "" && preview !== null && preview > 0 && password.trim() !== "" && !deleting,
  );

  // Vista previa con retardo para no consultar en cada tecla del filtro.
  $effect(() => {
    if (filterError) {
      preview = null;
      previewLoading = false;
      return;
    }
    const current = filter;
    let cancelled = false;
    preview = null;
    previewLoading = true;
    const timer = setTimeout(() => {
      countMovements(current)
        .then((total) => {
          if (!cancelled) preview = total;
        })
        .catch(() => {
          if (!cancelled) preview = null;
        })
        .finally(() => {
          if (!cancelled) previewLoading = false;
        });
    }, 250);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  });

  const previewText = $derived.by(() => {
    if (filterError) return "Completa los datos del filtro.";
    if (previewLoading || preview === null) return "Calculando coincidencias…";
    if (preview === 0) return "No hay movimientos que coincidan.";
    return `Se eliminarán ${preview} movimiento${preview === 1 ? "" : "s"}.`;
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

  async function remove() {
    if (!canDelete || preview === null) return;
    const ok = await modalService.confirm(
      `${describe(preview)} El stock actual de los productos no cambia. Esta acción no se puede deshacer.`,
      "Eliminar movimientos",
    );
    if (!ok) return;
    deleting = true;
    deleteError = "";
    try {
      const deleted = await movementsService.deleteBatch(filter, password);
      feedback.success(`Se eliminaron ${deleted} movimiento${deleted === 1 ? "" : "s"}.`);
      modalService.close();
    } catch (e) {
      deleteError = String(e);
    } finally {
      deleting = false;
    }
  }
</script>

<Modal title="Eliminar movimientos" onClose={() => modalService.close()} {dirty} busy={deleting}>
  <p class="hint">
    Borra movimientos del historial por cantidad (los más recientes o los más antiguos) o por fecha.
    El stock actual de los productos no cambia.
  </p>
  <div class="grid">
    <Select label="Qué borrar" bind:value={mode} options={modeOptions} />
    {#if needsCount}
      <Input label="Cantidad" type="number" bind:value={count} />
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
  {:else}
    <p class="preview" aria-live="polite">{previewText}</p>
  {/if}
  {#if deleteError}<p class="error" role="alert">{deleteError}</p>{/if}
  <div class="section-action">
    <Button variant="danger" icon={Trash} disabled={!canDelete} onclick={remove}>
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

  .preview {
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
