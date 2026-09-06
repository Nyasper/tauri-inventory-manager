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

const features = tableFeatures({
  rowSortingFeature,
  sortedRowModel: createSortedRowModel(),
  sortFns,
  columnFilteringFeature,
  globalFilteringFeature,
  filteredRowModel: createFilteredRowModel(),
  filterFns: { includesString: filterFn_includesString },
});

export const { createAppTable, createAppColumnHelper } = createTableHook({
  features,
});