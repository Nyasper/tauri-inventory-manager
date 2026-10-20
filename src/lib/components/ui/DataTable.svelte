<script lang="ts" generics="TData extends RowData">
  import { FlexRender } from "@tanstack/svelte-table";
  import type { RowData } from "@tanstack/svelte-table";
  import type { Snippet } from "svelte";
  import type { AppColumnMeta, AppTable } from "$lib/components/ui/table";
  import {
    Search,
    ChevronUp,
    ChevronDown,
    ChevronsLeft,
    ChevronLeft,
    ChevronRight,
    ChevronsRight,
  } from "@lucide/svelte";

  let {
    table,
    searchPlaceholder = "Buscar...",
    emptyText = "No hay resultados",
    showSearch = true,
    onRowClick,
    rowClickable,
    rowActionLabel,
    actions,
    fixedLayout = false,
    fillHeight = false,
    showPagination = false,
  }: {
    table: AppTable<TData>;
    searchPlaceholder?: string;
    emptyText?: string;
    showSearch?: boolean;
    onRowClick?: (row: TData) => void;
    rowClickable?: (row: TData) => boolean;
    rowActionLabel?: (row: TData) => string;
    actions?: Snippet<[TData]>;
    fixedLayout?: boolean;
    fillHeight?: boolean;
    showPagination?: boolean;
  } = $props();

  const pageCount = $derived(table.getPageCount());
  const pageIndex = $derived(table.atoms.pagination?.get()?.pageIndex ?? 0);
  const rowCount = $derived(table.getRowCount());

  function isClickable(row: TData): boolean {
    return !!onRowClick && (rowClickable?.(row) ?? true);
  }

  function isCentered(column: { columnDef: { meta?: AppColumnMeta | null } }): boolean {
    return column.columnDef.meta?.align === "center";
  }
</script>

<div class={["table-wrap", { "fill-height": fillHeight }]}>
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
        oninput={(event) => table.setGlobalFilter(event.currentTarget.value)}
      />
    </div>
  {/if}

  <table class={["table", { "fixed-layout": fixedLayout }]}>
    <thead>
      {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
        <tr>
          {#each headerGroup.headers as header (header.id)}
            <th
              scope="col"
              class={{ center: isCentered(header.column) }}
              style:width={header.column.columnDef.meta?.width}
              aria-sort={header.column.getCanSort()
                ? header.column.getIsSorted() === "asc"
                  ? "ascending"
                  : header.column.getIsSorted() === "desc"
                    ? "descending"
                    : "none"
                : undefined}
            >
              {#if !header.isPlaceholder && header.column.getCanSort()}
                <button
                  class="th-sort"
                  type="button"
                  aria-label={`Ordenar por ${header.id.replace(/_/g, " ")}`}
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
            <th class="actions-col center">Acciones</th>
          {/if}
        </tr>
      {/each}
    </thead>
    <tbody>
      {#each table.getRowModel().rows as row (row.id)}
        <tr
          class={{ clickable: isClickable(row.original) }}
          tabindex={isClickable(row.original) ? 0 : undefined}
          aria-label={isClickable(row.original) ? rowActionLabel?.(row.original) : undefined}
          onkeydown={(event) => {
            if (
              isClickable(row.original) &&
              event.target === event.currentTarget &&
              (event.key === "Enter" || event.key === " ")
            ) {
              event.preventDefault();
              onRowClick?.(row.original);
            }
          }}
          onclick={(event) => {
            if (
              isClickable(row.original) &&
              !(event.target as HTMLElement).closest("button,a,input,select,textarea")
            )
              onRowClick?.(row.original);
          }}
        >
          {#each row.getAllCells() as cell (cell.id)}
            <td class={{ center: isCentered(cell.column) }}>
              <FlexRender {cell} />
            </td>
          {/each}
          {#if actions}
            <td class="actions-col">
              <div class="actions-inner">
                {@render actions(row.original)}
              </div>
            </td>
          {/if}
        </tr>
      {/each}
      {#if table.getRowModel().rows.length === 0}
        <tr>
          <td class="empty" colspan={table.getAllLeafColumns().length + (actions ? 1 : 0)}>
            {emptyText}
          </td>
        </tr>
      {/if}
    </tbody>
  </table>

  {#if showPagination && pageCount > 1}
    <nav class="pager" aria-label="Paginación de la tabla">
      <span class="pager-info">Página {pageIndex + 1} de {pageCount} · {rowCount} en total</span>
      <div class="pager-buttons">
        <button
          class="pager-btn"
          type="button"
          title="Primera página"
          aria-label="Primera página"
          disabled={!table.getCanPreviousPage()}
          onclick={() => table.firstPage()}
        >
          <ChevronsLeft size={16} />
        </button>
        <button
          class="pager-btn"
          type="button"
          title="Página anterior"
          aria-label="Página anterior"
          disabled={!table.getCanPreviousPage()}
          onclick={() => table.previousPage()}
        >
          <ChevronLeft size={16} />
        </button>
        <button
          class="pager-btn"
          type="button"
          title="Página siguiente"
          aria-label="Página siguiente"
          disabled={!table.getCanNextPage()}
          onclick={() => table.nextPage()}
        >
          <ChevronRight size={16} />
        </button>
        <button
          class="pager-btn"
          type="button"
          title="Última página"
          aria-label="Última página"
          disabled={!table.getCanNextPage()}
          onclick={() => table.lastPage()}
        >
          <ChevronsRight size={16} />
        </button>
      </div>
    </nav>
  {/if}
</div>

<style>
  .table-wrap {
    overflow-x: auto;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .table-wrap.fill-height {
    flex: 1 1 auto;
    min-height: 0;
    overflow: auto;
  }

  .table-wrap.fill-height table {
    flex: 0 0 auto;
    overflow: visible;
    border-radius: 0;
  }

  .table-wrap.fill-height thead th {
    position: sticky;
    top: 0;
    z-index: 1;
    border-bottom: none;
    box-shadow: inset 0 -1px 0 var(--border);
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

  .table.fixed-layout {
    table-layout: fixed;
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
    white-space: nowrap;
  }

  .center {
    text-align: center;
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
    text-align: left;
    padding: 0.625rem 1rem;
    border-bottom: 1px solid var(--border);
    font-size: 0.875rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
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

  .actions-col {
    min-width: 136px;
  }

  .actions-inner {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .pager {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .pager-info {
    font-size: 0.8125rem;
    color: var(--text-muted);
  }

  .pager-buttons {
    display: flex;
    gap: 0.25rem;
  }

  .pager-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
  }

  .pager-btn:hover:not(:disabled) {
    background: var(--surface-2);
    color: var(--text);
  }

  .pager-btn:disabled {
    opacity: 0.45;
    cursor: default;
  }
</style>
