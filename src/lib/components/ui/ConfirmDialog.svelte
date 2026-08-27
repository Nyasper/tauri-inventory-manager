<script lang="ts">
  import Modal from "./Modal.svelte";
  import Button from "./Button.svelte";

  let {
    title = "Confirmar",
    message,
    onResult,
  }: {
    title?: string;
    message: string;
    onResult: (result: boolean) => void;
  } = $props();
</script>

<Modal {title} onClose={() => onResult(false)}>
  <p class="message">{message}</p>
  {#snippet footer()}
    <Button variant="secondary" onclick={() => onResult(false)}>Cancelar</Button>
    <Button variant="danger" onclick={() => onResult(true)}>Confirmar</Button>
  {/snippet}
</Modal>

<style>
  .message {
    color: var(--text-muted);
  }
</style>
