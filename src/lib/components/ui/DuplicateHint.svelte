<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    findProductDuplicate,
    findSupplierDuplicate,
  } from "$lib/services/db.service";
  import { modalService } from "$lib/services/modal.service.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import type { DuplicateRecord } from "$lib/types";

  let {
    name,
    kind,
    error,
    currentId,
  }: {
    name: string;
    kind: "product" | "supplier";
    error: string;
    currentId?: number;
  } = $props();

  function lookup(): Promise<DuplicateRecord | null> {
    if (!error.includes("Ya existe")) return Promise.resolve(null);
    const request =
      kind === "product"
        ? findProductDuplicate(name, currentId)
        : findSupplierDuplicate(name, currentId);
    return request.catch(() => null);
  }

  const existing = $derived(lookup());

  function open(record: DuplicateRecord) {
    modalService.closeAll();
    goto(
      `/${kind === "product" ? "products" : "suppliers"}?id=${record.id}&archived=${record.archived}`,
    );
  }
</script>

{#await existing then record}
  {#if record}
    <Button variant="secondary" onclick={() => open(record)}>
      Ver registro existente{record.archived ? " (archivado)" : ""}
    </Button>
  {/if}
{/await}
