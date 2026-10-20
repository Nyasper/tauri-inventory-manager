<script lang="ts">
  import { Minus, Plus } from "@lucide/svelte";

  let {
    value = $bindable(1),
    min = 1,
    max = undefined,
    label,
    disabled = false,
    showBoundaryButtons = false,
  }: {
    value?: number;
    min?: number;
    max?: number;
    label?: string;
    disabled?: boolean;
    showBoundaryButtons?: boolean;
  } = $props();

  const id = $props.id();

  const atMin = $derived(disabled || !Number.isFinite(value) || value <= min);
  const atMax = $derived(disabled || (max !== undefined && Number.isFinite(value) && value >= max));

  function clamp(next: number): number {
    if (next < min) next = min;
    if (max !== undefined && next > max) next = max;
    return next;
  }

  function adjustBy(delta: number) {
    if (disabled) return;
    const current = Number.isFinite(value) ? Math.round(value) : min;
    value = clamp(current + delta);
  }

  function jumpToBoundary(bound: number) {
    if (disabled) return;
    value = clamp(bound);
  }
</script>

{#if label}
  <label class="field-label" for={id}>{label}</label>
{/if}
<div class={["stepper", { disabled }]}>
  {#if showBoundaryButtons}
    <button
      class="bound-btn"
      type="button"
      title="Ir al mínimo"
      aria-label="Ir al mínimo"
      disabled={atMin}
      onclick={() => jumpToBoundary(min)}
    >
      Mín
    </button>
  {/if}
  <button
    class="step-btn"
    type="button"
    aria-label="Disminuir"
    disabled={atMin}
    onclick={() => adjustBy(-1)}
  >
    <Minus size={14} />
  </button>
  <input
    {id}
    type="number"
    class="step-input"
    bind:value
    {min}
    {max}
    {disabled}
    step="1"
    autocomplete="off"
  />
  <button
    class="step-btn"
    type="button"
    aria-label="Aumentar"
    disabled={atMax}
    onclick={() => adjustBy(1)}
  >
    <Plus size={14} />
  </button>
  {#if showBoundaryButtons && max !== undefined}
    <button
      class="bound-btn"
      type="button"
      title="Ir al máximo"
      aria-label="Ir al máximo"
      disabled={atMax}
      onclick={() => jumpToBoundary(max ?? min)}
    >
      Máx
    </button>
  {/if}
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
    width: max-content;
    max-width: 100%;
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

  .bound-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0 0.5rem;
    flex-shrink: 0;
    border: none;
    background: var(--surface-2);
    color: var(--text-muted);
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.02em;
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  .step-btn:hover:not(:disabled),
  .bound-btn:hover:not(:disabled) {
    background: var(--surface-3);
    color: var(--text);
  }

  .step-btn:disabled,
  .bound-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .step-input {
    flex: none;
    width: 3rem;
    min-width: 0;
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
