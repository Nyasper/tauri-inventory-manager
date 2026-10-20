import type { IconName } from "#lib/icons.js";

export const CATEGORY_COLORS = [
  "#3b82f6",
  "#22c55e",
  "#8b5cf6",
  "#14b8a6",
  "#64748b",
  "#020202",
  "#f59e0b",
  "#efec44",
  "#e11d48",
];

export interface CategoryDraft {
  id: number | null;
  name: string;
  color: string;
  icon: IconName | null;
}

export function newCategoryDraft(): CategoryDraft {
  return { id: null, name: "", color: CATEGORY_COLORS[0], icon: "box" };
}
