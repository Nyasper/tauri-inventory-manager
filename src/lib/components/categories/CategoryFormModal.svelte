<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import CategoryEditor from "$lib/components/categories/CategoryEditor.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { newCategoryDraft, type CategoryDraft } from "$lib/categories";

  let { onCreated }: { onCreated?: (id: number) => void } = $props();

  let draft = $state<CategoryDraft>(newCategoryDraft());

  function handleSaved(id: number) {
    onCreated?.(id);
    modalService.close();
  }
</script>

<Modal title="Nueva categoría" onClose={() => modalService.close()} width="md">
  <CategoryEditor
    {draft}
    onSaved={handleSaved}
    onCancel={() => modalService.close()}
  />
</Modal>