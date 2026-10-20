<script lang="ts">
  import Modal from "#lib/components/ui/Modal.svelte";
  import CategoryEditor from "#lib/components/categories/CategoryEditor.svelte";
  import { modalService } from "#lib/services/modal.service.svelte.js";
  import { newCategoryDraft, type CategoryDraft } from "#lib/category-draft.js";

  let { onCreated }: { onCreated?: (id: number) => void } = $props();

  const initialDraft = newCategoryDraft();
  let draft = $state<CategoryDraft>(newCategoryDraft());

  const dirty = $derived(JSON.stringify(draft) !== JSON.stringify(initialDraft));

  function handleSaved(id: number) {
    onCreated?.(id);
    modalService.close();
  }
</script>

<Modal
  title="Nueva categoría"
  onClose={() => modalService.close()}
  hasUnsavedChanges={dirty}
  width="md"
>
  <CategoryEditor bind:draft onSaved={handleSaved} onCancel={() => modalService.requestClose()} />
</Modal>
