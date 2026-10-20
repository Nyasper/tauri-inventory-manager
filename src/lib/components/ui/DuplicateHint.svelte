<script lang="ts">
  import { goto } from "$app/navigation";
  import { findProductDuplicate, findSupplierDuplicate } from "$lib/services/db.service";
  import { isDuplicateError, type AppError } from "$lib/utils/errors";
  import { modalService } from "$lib/services/modal.service.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import type { DuplicateRecord } from "$lib/types";

  let {
    name,
    recordType,
    error,
    currentId,
  }: {
    name: string;
    recordType: "product" | "supplier";
    error: AppError;
    currentId?: number;
  } = $props();

  function findExistingRecord(): Promise<DuplicateRecord | null> {
    if (!isDuplicateError(error)) return Promise.resolve(null);
    const request =
      recordType === "product"
        ? findProductDuplicate(name, currentId)
        : findSupplierDuplicate(name, currentId);
    return request.catch(() => null);
  }

  const existingRecord = $derived(findExistingRecord());

  function open(record: DuplicateRecord) {
    modalService.closeAll();
    goto(
      `/${recordType === "product" ? "products" : "suppliers"}?id=${record.id}&archived=${record.isArchived ? 1 : 0}`,
    );
  }
</script>

{#await existingRecord then record}
  {#if record}
    <Button variant="secondary" onclick={() => open(record)}>
      Ver registro existente{record.isArchived ? " (archivado)" : ""}
    </Button>
  {/if}
{/await}
