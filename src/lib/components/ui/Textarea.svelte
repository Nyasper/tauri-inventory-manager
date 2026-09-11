<script lang="ts">
  let {
    label,
    value = $bindable(""),
    placeholder = "",
    rows = 3,
  }: {
    label?: string;
    value?: string;
    placeholder?: string;
    rows?: number;
  } = $props();

  const id = $props.id();
</script>

{#if label}
  <label class="field-label" for={id}>{label}</label>
{/if}
<textarea {id} bind:value {placeholder} {rows} class="textarea" autocomplete="off"></textarea>

<style>
  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .textarea {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    outline: none;
    resize: vertical;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  .textarea:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
  }

  .textarea::placeholder {
    color: var(--text-muted);
  }
</style>
