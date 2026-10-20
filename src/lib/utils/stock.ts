import type { Product, StockStatus } from "$lib/types";

export function getStockStatus(
  stock: number,
  lowThreshold: number,
  mediumThreshold: number,
): StockStatus {
  if (stock <= 0) return "out";
  if (stock <= lowThreshold) return "low";
  if (stock <= mediumThreshold) return "medium";
  return "high";
}

export function resolveProductStockStatus(
  product: Pick<Product, "stock" | "stock_low_threshold" | "stock_medium_threshold">,
  globalLowThreshold: number,
  globalMediumThreshold: number,
): StockStatus {
  const lowThreshold = product.stock_low_threshold ?? globalLowThreshold;
  const mediumThreshold = product.stock_medium_threshold ?? globalMediumThreshold;
  return getStockStatus(product.stock, lowThreshold, mediumThreshold);
}

export const STOCK_LABELS: Record<StockStatus, string> = {
  out: "Sin stock",
  low: "Bajo",
  medium: "Medio",
  high: "Alto",
};
