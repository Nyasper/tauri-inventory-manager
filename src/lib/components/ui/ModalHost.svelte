<script lang="ts">
  import { modalService } from "#lib/services/modal.service.svelte.js";
</script>

{#each modalService.modals as modal (modal.id)}
  {const Component = modal.component}
  <Component {...modal.props} />
{/each}
