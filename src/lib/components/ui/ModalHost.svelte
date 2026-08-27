<script lang="ts">
  import { modalService } from "$lib/stores/modal.service.svelte";
</script>

{#each modalService.modals as modal (modal.id)}
  {const Component = modal.component}
  <Component {...modal.props} />
{/each}
