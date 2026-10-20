<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import type { SelectOption } from "#lib/components/ui/select-option.js";

  let {
    label,
    value = $bindable(""),
    options = [],
    placeholder,
    onchange,
    ariaLabel,
    disabled = false,
  }: {
    label?: string;
    value?: string;
    options?: SelectOption[];
    placeholder?: string;
    onchange?: (event: Event) => void;
    ariaLabel?: string;
    disabled?: boolean;
  } = $props();

  const id = $props.id();
</script>

{#if label}
  <label class="field-label" for={id}>{label}</label>
{/if}
<div class="wrap">
  <select
    {id}
    bind:value
    {onchange}
    class="select"
    autocomplete="off"
    aria-label={ariaLabel}
    {disabled}
  >
    {#if placeholder}
      <option value="" disabled>{placeholder}</option>
    {/if}
    {#each options as option (option.value)}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
  <ChevronDown class="chevron" size={16} />
</div>

<style>
  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .wrap {
    position: relative;
    display: flex;
    align-items: center;
  }

  .select {
    width: 100%;
    padding: 0.5rem 2rem 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    outline: none;
    cursor: pointer;
    appearance: none;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  .select:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
  }

  :global(.chevron) {
    position: absolute;
    right: 0.625rem;
    pointer-events: none;
    color: var(--text-muted);
  }
</style>
