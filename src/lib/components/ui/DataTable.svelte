<script lang="ts">
  import { FlexRender } from "@tanstack/svelte-table";
  import type { Snippet } from "svelte";
  import { Search, ChevronUp, ChevronDown } from "@lucide/svelte";

  let {
    table,
    searchPlaceholder = "Buscar...",
    emptyText = "No hay resultados",
    showSearch = true,
    onRowClick = undefined,
    actions,
  }: {
    table: any;
    searchPlaceholder?: string;
    emptyText?: string;
    showSearch?: boolean;
    onRowClick?: (row: any) => void;
    actions?: Snippet<[any]>;
  } = $props();

  function isRightAligned(column: any): boolean {
    return column.columnDef.meta?.align === "right";
  }
</script>

<div class="table-wrap">
  {#if showSearch}
    <div class="search">
      <Search size={16} />
      <input
        type="text"
        class="search-input"
        placeholder={searchPlaceholder}
        aria-label={searchPlaceholder}
        autocomplete="off"
        value={table.atoms.globalFilter.get() ?? ""}
        oninput={(e) =>
          table.setGlobalFilter((e.currentTarget as HTMLInputElement).value)
        }
      />
    </div>
  {/if}

  <table class="table">
    <thead>
      {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
        <tr>
          {#each headerGroup.headers as header (header.id)}
            <th class:num={isRightAligned(header.column)}>
              {#if !header.isPlaceholder && header.column.getCanSort()}
                <button
                  class="th-sort"
                  type="button"
                  aria-label={`Ordenar por ${header.id}`}
                  onclick={header.column.getToggleSortingHandler()}
                >
                  <FlexRender {header} />
                  {#if header.column.getIsSorted() === "asc"}
                    <ChevronUp size={14} />
                  {:else if header.column.getIsSorted() === "desc"}
                    <ChevronDown size={14} />
                  {:else}
                    <span class="sort-idle"><ChevronDown size={14} /></span>
                  {/if}
                </button>
              {:else if !header.isPlaceholder}
                <FlexRender {header} />
              {/if}
            </th>
          {/each}
          {#if actions}
            <th class="actions-col">Acciones</th>
          {/if}
        </tr>
      {/each}
    </thead>
    <tbody>
      {#each table.getRowModel().rows as row (row.id)}
        <tr
          class:clickable={!!onRowClick}
          tabindex={onRowClick ? 0 : undefined}
          onkeydown={(event) => { if (onRowClick && event.target === event.currentTarget && (event.key === "Enter" || event.key === " ")) { event.preventDefault(); onRowClick(row.original); } }}
          onclick={(event) => { if (!(event.target as HTMLElement).closest("button,a,input,select,textarea")) onRowClick?.(row.original); }}
        >
          {#each row.getAllCells() as cell (cell.id)}
            <td class:num={isRightAligned(cell.column)}>
              <FlexRender {cell} />
            </td>
          {/each}
          {#if actions}
            <td class="actions-col">
              {@render actions(row.original)}
            </td>
          {/if}
        </tr>
      {/each}
      {#if table.getRowModel().rows.length === 0}
        <tr>
          <td
            class="empty"
            colspan={table.getVisibleLeafColumns().length + (actions ? 1 : 0)}
          >
            {emptyText}
          </td>
        </tr>
      {/if}
    </tbody>
  </table>
</div>

<style>
  .table-wrap {
    overflow-x: auto;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .search {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    max-width: 320px;
  }

  .search-input {
    flex: 1;
    border: none;
    outline: none;
    background: transparent;
    padding: 0.5rem 0;
    color: var(--text);
  }

  .table {
    width: 100%;
    border-collapse: collapse;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }

  th {
    text-align: left;
    padding: 0.625rem 1rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
    background: var(--surface-2);
  }

  .th-sort {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    border: none;
    background: transparent;
    color: inherit;
    font: inherit;
    text-transform: inherit;
    letter-spacing: inherit;
    padding: 0;
    cursor: pointer;
  }

  .th-sort:hover {
    color: var(--text);
  }

  .sort-idle {
    display: inline-flex;
    opacity: 0.3;
  }

  td {
    padding: 0.625rem 1rem;
    border-bottom: 1px solid var(--border);
    font-size: 0.875rem;
  }

  tbody tr:last-child td {
    border-bottom: none;
  }

  tbody tr.clickable {
    cursor: pointer;
  }

  tbody tr:not(.clickable) {
    cursor: default;
  }

  tbody tr:hover {
    background: var(--surface-2);
  }

  td.empty {
    color: var(--text-muted);
    text-align: center;
    padding: 2rem 1rem;
  }

  .num {
    text-align: right;
  }

  .actions-col {
    min-width: 136px;
  }
</style>
