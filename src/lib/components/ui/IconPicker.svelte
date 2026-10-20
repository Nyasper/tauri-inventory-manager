<script lang="ts">
  import { ICONS, ICON_OPTIONS, type IconName } from "#lib/icons.js";
  import type { Component } from "svelte";
  import type { LucideProps } from "@lucide/svelte";
  import { X } from "@lucide/svelte";

  let {
    label = "Icono",
    value = $bindable<IconName | null>(null),
    color = null,
  }: {
    label?: string;
    value?: IconName | null;
    color?: string | null;
  } = $props();

  const groups = $derived(
    Object.entries(
      ICON_OPTIONS.reduce<Record<string, typeof ICON_OPTIONS>>((acc, opt) => {
        (acc[opt.group] ??= []).push(opt);
        return acc;
      }, {}),
    ),
  );

  let Icon: Component<LucideProps> | null = $derived(value ? (ICONS[value] ?? null) : null);

  const tinted = $derived(!!color && !!value);
</script>

{#if label}
  <span class="field-label">{label}</span>
{/if}

<div class="picker">
  <div class="head">
    <div
      class={["preview", { tinted }]}
      style:background={tinted ? color : null}
      style:color={tinted ? "#ffffff" : null}
    >
      {#if Icon}
        <Icon size={28} />
      {:else}
        <span class="preview-none">Sin icono</span>
      {/if}
    </div>

    <button class={["clear", { active: !value }]} type="button" onclick={() => (value = null)}>
      <X size={14} />
      Sin icono
    </button>
  </div>

  <div class="groups">
    {#each groups as [group, options] (group)}
      <div class="group">
        <span class="group-label">{group}</span>
        <div class="options">
          {#each options as option (option.value)}
            {const OptionIcon = ICONS[option.value]}
            <button
              type="button"
              class={["opt", { active: value === option.value }]}
              title={option.label}
              aria-label={option.label}
              aria-pressed={value === option.value}
              onclick={() => (value = option.value)}
            >
              <OptionIcon size={18} />
            </button>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .picker {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
  }

  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .preview {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 56px;
    height: 56px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .preview.tinted {
    box-shadow: 0 1px 3px rgba(16, 24, 40, 0.18);
  }

  .preview-none {
    font-size: 0.7rem;
    color: var(--text-muted);
    text-align: center;
    line-height: 1.2;
  }

  .clear {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-muted);
    border-radius: var(--radius-sm);
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    cursor: pointer;
  }

  .clear:hover {
    background: var(--surface-2);
    color: var(--text);
  }

  .clear.active {
    background: var(--primary-soft);
    color: var(--primary);
    border-color: var(--primary);
  }

  .groups {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-height: 260px;
    overflow-y: auto;
    padding-right: 0.25rem;
  }

  .group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .group-label {
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
  }

  .options {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(40px, 1fr));
    gap: 0.375rem;
  }

  .opt {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    transition:
      background-color 0.12s,
      border-color 0.12s,
      color 0.12s;
  }

  .opt:hover {
    background: var(--surface-2);
    color: var(--text);
    border-color: var(--border-strong);
  }

  .opt.active {
    background: var(--primary-soft);
    color: var(--primary);
    border-color: var(--primary);
  }
</style>
