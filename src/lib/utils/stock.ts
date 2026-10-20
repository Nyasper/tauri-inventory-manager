import type { Product, StockStatus } from "$lib/types";

export function getStockStatus(stock: number, low: number, medium: number): StockStatus {
  if (stock <= 0) return "out";
  if (stock <= low) return "low";
  if (stock <= medium) return "medium";
  return "high";
}

export function resolveProductStockStatus(
  product: Pick<Product, "stock" | "stock_low_threshold" | "stock_medium_threshold">,
  globalLow: number,
  globalMedium: number,
): StockStatus {
  const low = product.stock_low_threshold ?? globalLow;
  const medium = product.stock_medium_threshold ?? globalMedium;
  return getStockStatus(product.stock, low, medium);
}

export const STOCK_LABELS: Record<StockStatus, string> = {
  out: "Sin stock",
  low: "Bajo",
  medium: "Medio",
  high: "Alto",
};
