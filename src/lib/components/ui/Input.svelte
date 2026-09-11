<script lang="ts">
  let {
    label,
    value = $bindable(""),
    placeholder = "",
    type = "text",
    disabled = false,
  }: {
    label?: string;
    value?: string;
    placeholder?: string;
    type?: string;
    disabled?: boolean;
  } = $props();

  const id = $props.id();
</script>

<div class="field">
  {#if label}
    <label class="field-label" for={id}>{label}</label>
  {/if}
  <input {id} {type} bind:value {placeholder} {disabled} class="input" autocomplete="off" />
</div>

<style>
  .field {
    width: 100%;
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
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  .input:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
  }

  .input::placeholder {
    color: var(--text-muted);
  }
</style>
