<script lang="ts">
  import type { MovementType } from "$lib/types";
  import { productService } from "$lib/services/product.service";
  import { movementService } from "$lib/services/movement.service";
  import { productsStore } from "$lib/stores/products.store.svelte";
  import { suppliersStore } from "$lib/stores/suppliers.store.svelte";
  import { personsStore } from "$lib/stores/persons.store.svelte";
  import { movementsStore } from "$lib/stores/movements.store.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Textarea from "$lib/components/ui/Textarea.svelte";
  import Select, { type SelectOption } from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";

  let productId = $state("");
  let presentationId = $state("");
  let type = $state<MovementType>("in");
  let quantity = $state("");
  let supplierId = $state("");
  let personId = $state("");
  let notes = $state("");
  let saving = $state(false);
  let error = $state("");

  const qtyId = $props.id();

  let presentationOptions = $state<SelectOption[]>([]);

  const productOptions = $derived(
    productsStore.items.map((p) => ({
      value: String(p.id),
      label: p.name,
    })),
  );
  const supplierOptions = $derived(
    suppliersStore.items.map((s) => ({
      value: String(s.id),
      label: s.name,
    })),
  );
  const personOptions = $derived(
    personsStore.items.map((p) => ({
      value: String(p.id),
      label: p.name,
    })),
  );
  const typeOptions: SelectOption[] = [
    { value: "in", label: "Entrada" },
    { value: "out", label: "Salida" },
  ];

  $effect(() => {
    const pid = productId ? Number(productId) : null;
    if (!pid) {
      presentationOptions = [];
      presentationId = "";
      return;
    }

    let cancelled = false;
    productService.presentations(pid).then((rows) => {
      if (cancelled) return;
      presentationOptions = rows.map((r) => ({
        value: String(r.id),
        label: r.name,
      }));
      const base = rows.find((r) => r.factor === 1) ?? rows[0];
      presentationId = base ? String(base.id) : "";
    });

    return () => {
      cancelled = true;
    };
  });

  $effect(() => {
    const def = personsStore.defaultPerson;
    if (def && !personId) personId = String(def.id);
  });

  async function save() {
    const pid = productId ? Number(productId) : null;
    const presId = presentationId ? Number(presentationId) : null;
    const qty = Number(quantity);

    if (!pid || presId === null) {
      error = "Selecciona un producto y una presentación.";
      return;
    }
    if (!Number.isFinite(qty) || qty <= 0) {
      error = "La cantidad debe ser mayor que cero.";
      return;
    }

    saving = true;
    error = "";
    try {
      await movementService.create({
        product_id: pid,
        type,
        quantity: qty,
        presentation_id: presId,
        supplier_id: type === "in" && supplierId ? Number(supplierId) : null,
        responsible_person_id: personId ? Number(personId) : null,
        notes: notes.trim() || null,
      });
      await movementsStore.load();
      await productsStore.load();
      modalService.close();
    } catch {
      error = "No se pudo registrar el movimiento.";
    } finally {
      saving = false;
    }
  }
</script>

<Modal
  title="Nuevo movimiento"
  onClose={() => modalService.close()}
  width="md"
>
  <div class="form">
    <Select label="Tipo" bind:value={type} options={typeOptions} />

    <Select
      label="Producto"
      bind:value={productId}
      options={productOptions}
      placeholder="Selecciona un producto"
    />

    <Select
      label="Presentación"
      bind:value={presentationId}
      options={presentationOptions}
      placeholder="Selecciona una presentación"
    />

    <div>
      <label class="field-label" for={qtyId}>Cantidad</label>
      <input id={qtyId} type="number" min="0" step="any" class="input" bind:value={quantity} />
    </div>

    {#if type === "in"}
      <Select
        label="Proveedor"
        bind:value={supplierId}
        options={supplierOptions}
        placeholder="Sin proveedor"
      />
    {/if}

    <Select
      label="Responsable"
      bind:value={personId}
      options={personOptions}
      placeholder="Sin responsable"
    />

    <Textarea
      label="Notas"
      bind:value={notes}
      placeholder="Observaciones del movimiento"
    />

    {#if error}
      <p class="error">{error}</p>
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="secondary" onclick={() => modalService.close()}>
      Cancelar
    </Button>
    <Button disabled={saving} onclick={save}>Registrar</Button>
  {/snippet}
</Modal>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
  }

  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    outline: none;
  }

  .input:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
  }

  .error {
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>
