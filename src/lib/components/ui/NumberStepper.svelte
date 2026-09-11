<script lang="ts">
  import { Minus, Plus } from "@lucide/svelte";

  let {
    value = $bindable(1),
    min = 1,
    max = undefined,
    label,
    disabled = false,
  }: {
    value?: number;
    min?: number;
    max?: number;
    label?: string;
    disabled?: boolean;
  } = $props();

  const id = $props.id();

  const atMin = $derived(
    disabled || !Number.isFinite(value) || value <= min,
  );
  const atMax = $derived(
    disabled || (max !== undefined && Number.isFinite(value) && value >= max),
  );

  function bump(delta: number) {
    if (disabled) return;
    const current = Number.isFinite(value) ? Math.round(value) : min;
    let next = current + delta;
    if (next < min) next = min;
    if (max !== undefined && next > max) next = max;
    value = next;
  }
</script>

{#if label}
  <label class="field-label" for={id}>{label}</label>
{/if}
<div class="stepper" class:disabled>
  <button
    class="step-btn"
    type="button"
    aria-label="Disminuir"
    disabled={atMin}
    onclick={() => bump(-1)}
  >
    <Minus size={14} />
  </button>
  <input
    {id}
    type="number"
    class="step-input"
    bind:value
    {min}
    max={max}
    {disabled}
    step="1"
    autocomplete="off"
  />
  <button
    class="step-btn"
    type="button"
    aria-label="Aumentar"
    disabled={atMax}
    onclick={() => bump(1)}
  >
    <Plus size={14} />
  </button>
</div>

<style>
  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .stepper {
    display: flex;
    align-items: stretch;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    overflow: hidden;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  .stepper:focus-within {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
  }

  .step-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    flex-shrink: 0;
    border: none;
    background: var(--surface-2);
    color: var(--text-muted);
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  .step-btn:hover:not(:disabled) {
    background: var(--surface-3);
    color: var(--text);
  }

  .step-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .step-input {
    flex: 1;
    min-width: 0;
    width: 100%;
    border: none;
    outline: none;
    background: transparent;
    color: var(--text);
    text-align: center;
    padding: 0.5rem 0.25rem;
  }

  .stepper .step-input::-webkit-outer-spin-button,
  .stepper .step-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
</style>