<script lang="ts">
  export interface SelectOption {
    value: string;
    label: string;
  }

  let {
    label,
    value = $bindable(""),
    options = [],
    placeholder,
    onchange,
  }: {
    label?: string;
    value?: string;
    options?: SelectOption[];
    placeholder?: string;
    onchange?: (event: Event) => void;
  } = $props();

  const id = $props.id();
</script>

{#if label}
  <label class="field-label" for={id}>{label}</label>
{/if}
<select {id} bind:value {onchange} class="select" autocomplete="off">
  {#if placeholder}
    <option value="" disabled>{placeholder}</option>
  {/if}
  {#each options as option (option.value)}
    <option value={option.value}>{option.label}</option>
  {/each}
</select>

<style>
  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .select {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    outline: none;
    cursor: pointer;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  .select:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
  }
</style>
