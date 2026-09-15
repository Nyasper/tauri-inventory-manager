import type { StockStatus } from "$lib/types";

export function getStockStatus(stock: number, low: number, medium: number): StockStatus {
  if (stock <= 0) return "out";
  if (stock <= low) return "low";
  if (stock <= medium) return "medium";
  return "high";
}

export const STOCK_LABELS: Record<StockStatus, string> = {
  out: "Sin stock",
  low: "Bajo",
  medium: "Medio",
  high: "Alto",
};
