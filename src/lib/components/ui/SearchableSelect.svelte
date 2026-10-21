<script lang="ts">
  import { ChevronDown, Search, Check, X } from "@lucide/svelte";
  import type { SelectOption } from "#lib/components/ui/select-option.js";

  let {
    label,
    value = $bindable(""),
    options = [],
    placeholder = "Seleccionar...",
    searchPlaceholder = "Buscar...",
    disabled = false,
    clearable = false,
    onchange,
    ariaLabel,
  }: {
    label?: string;
    value?: string;
    options?: SelectOption[];
    placeholder?: string;
    searchPlaceholder?: string;
    disabled?: boolean;
    clearable?: boolean;
    onchange?: (value: string) => void;
    ariaLabel?: string;
  } = $props();

  const id = $props.id();
  const listId = `${id}-list`;

  let isOpen = $state(false);
  let searchQuery = $state("");
  let highlightedIndex = $state(0);
  let container: HTMLDivElement | null = $state(null);
  let triggerElement: HTMLButtonElement | null = $state(null);
  let searchInput: HTMLInputElement | null = $state(null);
  let listElement: HTMLUListElement | null = $state(null);

  function normalize(str: string): string {
    return str
      .normalize("NFD")
      .replace(/[\u0300-\u036f]/g, "")
      .toLowerCase();
  }

  const selectedOption = $derived(options.find((opt) => opt.value === value));

  const filteredOptions = $derived(
    searchQuery.trim() === ""
      ? options
      : options.filter((opt) => {
          const normQuery = normalize(searchQuery);
          const normLabel = normalize(opt.label);
          const normSub = opt.sublabel ? normalize(opt.sublabel) : "";
          return normLabel.includes(normQuery) || normSub.includes(normQuery);
        }),
  );

  $effect(() => {
    // Escuchar cambios en la búsqueda para resetear el índice resaltado
    void searchQuery;
    highlightedIndex = 0;
  });

  $effect(() => {
    if (isOpen) {
      setTimeout(() => {
        searchInput?.focus();
      }, 20);

      const idx = filteredOptions.findIndex((opt) => opt.value === value);
      if (idx >= 0) {
        highlightedIndex = idx;
      }
    } else {
      searchQuery = "";
    }
  });

  $effect(() => {
    if (!isOpen) return;

    function handleMouseDown(e: MouseEvent) {
      if (container && !container.contains(e.target as Node)) {
        isOpen = false;
      }
    }

    window.addEventListener("mousedown", handleMouseDown);
    return () => {
      window.removeEventListener("mousedown", handleMouseDown);
    };
  });

  function selectOption(opt: SelectOption) {
    value = opt.value;
    isOpen = false;
    onchange?.(opt.value);
  }

  function handleClear(e: MouseEvent) {
    e.stopPropagation();
    value = "";
    onchange?.("");
  }

  function toggleOpen() {
    if (disabled) return;
    isOpen = !isOpen;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (disabled) return;

    if (!isOpen) {
      if (e.key === "Enter" || e.key === " " || e.key === "ArrowDown") {
        e.preventDefault();
        isOpen = true;
      }
      return;
    }

    if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      e.stopImmediatePropagation();
      isOpen = false;
      triggerElement?.focus();
      return;
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      if (filteredOptions.length > 0) {
        highlightedIndex = (highlightedIndex + 1) % filteredOptions.length;
        scrollHighlighted();
      }
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      if (filteredOptions.length > 0) {
        highlightedIndex = (highlightedIndex - 1 + filteredOptions.length) % filteredOptions.length;
        scrollHighlighted();
      }
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (
        filteredOptions.length > 0 &&
        highlightedIndex >= 0 &&
        highlightedIndex < filteredOptions.length
      ) {
        selectOption(filteredOptions[highlightedIndex]);
      }
    }
  }

  function scrollHighlighted() {
    if (!listElement) return;
    const item = listElement.children[highlightedIndex] as HTMLElement | undefined;
    item?.scrollIntoView({ block: "nearest" });
  }
</script>

{#if label}
  <label class="field-label" for={id}>{label}</label>
{/if}

<div
  class={["searchable-select", { disabled, open: isOpen }]}
  bind:this={container}
  onkeydown={handleKeydown}
  role="presentation"
>
  <div class="trigger-wrap">
    <button
      {id}
      bind:this={triggerElement}
      type="button"
      class="trigger"
      onclick={toggleOpen}
      {disabled}
      aria-haspopup="listbox"
      aria-expanded={isOpen}
      aria-controls={listId}
      aria-label={ariaLabel ?? label}
    >
      <span class={["value-text", { placeholder: !selectedOption || !selectedOption.label }]}>
        {selectedOption ? selectedOption.label : placeholder}
      </span>
      <ChevronDown class="chevron" size={16} />
    </button>
    {#if clearable && value && !disabled}
      <button
        type="button"
        class="clear-btn"
        title="Limpiar selección"
        aria-label="Limpiar selección"
        onclick={handleClear}
      >
        <X size={14} />
      </button>
    {/if}
  </div>

  {#if isOpen}
    <div class="dropdown">
      <div class="search-wrap">
        <Search size={14} class="search-icon" />
        <input
          bind:this={searchInput}
          type="text"
          class="search-input"
          placeholder={searchPlaceholder}
          bind:value={searchQuery}
          autocomplete="off"
        />
        {#if searchQuery}
          <button
            type="button"
            class="search-clear"
            title="Borrar búsqueda"
            aria-label="Borrar búsqueda"
            onclick={() => (searchQuery = "")}
          >
            <X size={12} />
          </button>
        {/if}
      </div>

      <ul id={listId} class="options-list" role="listbox" bind:this={listElement}>
        {#if filteredOptions.length === 0}
          <li class="no-options">No se encontraron resultados</li>
        {:else}
          {#each filteredOptions as option, index (option.value)}
            {@const isSelected = option.value === value}
            {@const isHighlighted = index === highlightedIndex}
            <li role="none">
              <button
                type="button"
                role="option"
                aria-selected={isSelected}
                class={["option-item", { selected: isSelected, highlighted: isHighlighted }]}
                onclick={() => selectOption(option)}
                onmouseenter={() => (highlightedIndex = index)}
              >
                <div class="option-content">
                  <span class="option-label">{option.label}</span>
                  {#if option.sublabel}
                    <span class="option-sublabel">{option.sublabel}</span>
                  {/if}
                </div>
                {#if isSelected}
                  <Check size={14} class="check-icon" />
                {/if}
              </button>
            </li>
          {/each}
        {/if}
      </ul>
    </div>
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

  .searchable-select {
    position: relative;
    width: 100%;
  }

  .trigger-wrap {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    font-size: 0.875rem;
    text-align: left;
    outline: none;
    cursor: pointer;
    user-select: none;
    transition:
      border-color 0.15s,
      box-shadow 0.15s;
  }

  .searchable-select.open .trigger,
  .trigger:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-soft);
  }

  .searchable-select.disabled .trigger {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .value-text {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .value-text.placeholder {
    color: var(--text-muted);
  }

  .clear-btn {
    position: absolute;
    right: 2.25rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    border: none;
    border-radius: 999px;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .clear-btn:hover {
    color: var(--text);
    background: var(--surface-3);
  }

  :global(.chevron) {
    color: var(--text-muted);
    transition: transform 0.15s ease;
  }

  .searchable-select.open :global(.chevron) {
    transform: rotate(180deg);
  }

  .dropdown {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    z-index: 60;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    overflow: hidden;
    animation: fadeIn 0.1s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .search-wrap {
    position: relative;
    display: flex;
    align-items: center;
    padding: 0.375rem 0.5rem;
    border-bottom: 1px solid var(--border);
    background: var(--surface-2);
  }

  :global(.search-icon) {
    position: absolute;
    left: 0.875rem;
    color: var(--text-muted);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: 0.375rem 1.75rem 0.375rem 1.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    font-size: 0.8125rem;
    outline: none;
  }

  .search-input:focus {
    border-color: var(--primary);
  }

  .search-clear {
    position: absolute;
    right: 0.75rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    border: none;
    border-radius: 999px;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .search-clear:hover {
    color: var(--text);
  }

  .options-list {
    list-style: none;
    margin: 0;
    padding: 0.25rem 0;
    max-height: 200px;
    overflow-y: auto;
  }

  .option-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.45rem 0.75rem;
    border: none;
    background: transparent;
    font-size: 0.8125rem;
    font-family: inherit;
    text-align: left;
    color: var(--text);
    cursor: pointer;
    user-select: none;
    transition: background-color 0.1s;
  }

  .option-item.highlighted {
    background: var(--surface-2);
  }

  .option-item.selected {
    color: var(--primary);
    font-weight: 500;
  }

  .option-content {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .option-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .option-sublabel {
    font-size: 0.6875rem;
    color: var(--text-muted);
  }

  :global(.check-icon) {
    color: var(--primary);
    flex-shrink: 0;
    margin-left: 0.5rem;
  }

  .no-options {
    padding: 0.875rem 0.75rem;
    font-size: 0.8125rem;
    color: var(--text-muted);
    text-align: center;
  }
</style>
