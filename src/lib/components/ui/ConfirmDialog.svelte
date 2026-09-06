<script lang="ts">
  import Modal from "./Modal.svelte";
  import Button from "./Button.svelte";
  import Icon from "./Icon.svelte";

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

<Modal {title} onClose={() => onResult(false)} width="sm">
  <div class="confirm">
    <span class="confirm-icon">
      <Icon name="alertTriangle" size={22} />
    </span>
    <p class="message">{message}</p>
  </div>
  {#snippet footer()}
    <Button variant="secondary" onclick={() => onResult(false)}>Cancelar</Button>
    <Button variant="danger" onclick={() => onResult(true)}>Confirmar</Button>
  {/snippet}
</Modal>

<style>
  .confirm {
    display: flex;
    gap: 0.75rem;
    align-items: flex-start;
  }

  .confirm-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--danger-soft);
    color: var(--danger);
    flex-shrink: 0;
  }

  .message {
    color: var(--text);
    font-size: 0.875rem;
    line-height: 1.5;
    padding-top: 0.25rem;
  }
</style>