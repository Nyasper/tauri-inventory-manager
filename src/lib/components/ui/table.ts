import {
  createTableHook,
  tableFeatures,
  rowSortingFeature,
  createSortedRowModel,
  sortFns,
  columnFilteringFeature,
  globalFilteringFeature,
  createFilteredRowModel,
  filterFn_includesString,
} from "@tanstack/svelte-table";
import type { RowData, SvelteTable } from "@tanstack/svelte-table";

export interface AppColumnMeta {
  align?: "left" | "center" | "right";
  width?: string;
}

const features = tableFeatures({
  rowSortingFeature,
  sortedRowModel: createSortedRowModel(),
  sortFns,
  columnFilteringFeature,
  globalFilteringFeature,
  filteredRowModel: createFilteredRowModel(),
  filterFns: { includesString: filterFn_includesString },
  columnMeta: {} as AppColumnMeta,
});

export const { createAppTable, createAppColumnHelper } = createTableHook({
  features,
});

type AppFeatures = typeof features;

/** Tipo de la instancia creada por `createAppTable`, para tipar componentes. */
export type AppTable<TData extends RowData> = SvelteTable<
  AppFeatures,
  TData
>;
