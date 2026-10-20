<script lang="ts">
  import { untrack } from "svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { errorMessage } from "$lib/utils/errors";
  import { formatDate, formatNumber } from "$lib/utils/format";
  import type { Movement } from "$lib/types";

  let { movement }: { movement: Movement } = $props();

  suppliersService.load();

  let supplierId = $state<string>(
    untrack(() => (movement.supplier_id ? String(movement.supplier_id) : "")),
  );
  let saving = $state(false);
  let error = $state("");

  const supplierOptions = $derived([
    { value: "", label: "(Sin proveedor)" },
    ...suppliersService.items.map((s) => ({
      value: String(s.id),
      label: s.name,
    })),
  ]);

  const dirty = $derived(
    supplierId !== (movement.supplier_id ? String(movement.supplier_id) : ""),
  );

  async function save() {
    saving = true;
    error = "";
    try {
      const nextId = supplierId ? Number(supplierId) : null;
      await movementsService.updateSupplier(movement.id, nextId);
      movement.supplier_id = nextId;
      if (nextId === null) {
        movement.supplier_name = null;
      } else {
        const found = suppliersService.items.find((s) => s.id === nextId);
        movement.supplier_name = found ? found.name : movement.supplier_name;
      }
      modalService.close();
    } catch (e) {
      error = errorMessage(e);
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title={`Editar proveedor — Movimiento #${movement.id}`}
  onClose={() => modalService.close()}
  {dirty}
  busy={saving}
  width="sm"
>
  <div class="edit-supplier-form">
    <div class="summary">
      <div class="summary-row">
        <span class="label">Producto:</span>
        <span class="val">{movement.product_name}</span>
      </div>
      <div class="summary-row">
        <span class="label">Cantidad:</span>
        <span class="val">{formatNumber(movement.quantity)} {movement.presentation_name}</span>
      </div>
      <div class="summary-row">
        <span class="label">Fecha:</span>
        <span class="val">{formatDate(movement.created_at)}</span>
      </div>
    </div>

    <div class="field">
      <SearchableSelect
        label="Proveedor asociado"
        bind:value={supplierId}
        options={supplierOptions}
        placeholder="Selecciona un proveedor"
        searchPlaceholder="Buscar proveedor..."
      />
      <p class="hint">Selecciona un proveedor existente o "(Sin proveedor)" para desvincularlo.</p>
    </div>

    {#if error}
      <p role="alert" class="error">{error}</p>
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" disabled={saving} onclick={() => modalService.requestClose()}>
      Cancelar
    </Button>
    <Button disabled={saving} onclick={save}>
      {saving ? "Guardando…" : "Guardar cambios"}
    </Button>
  {/snippet}
</Modal>

<style>
  .edit-supplier-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .summary {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    padding: 0.75rem;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    font-size: 0.8125rem;
  }

  .summary-row {
    display: flex;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .label {
    color: var(--text-muted);
  }

  .val {
    font-weight: 500;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .hint {
    margin: 0;
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .error {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--danger);
  }
</style>
