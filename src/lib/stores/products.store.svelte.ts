import type { Product } from "$lib/types";
import { productService } from "$lib/services/product.service";

type ViewMode = "list" | "grid";

const VIEW_KEY = "viewMode";

class ProductsStore {
  items = $state<Product[]>([]);
  loading = $state(false);
  view = $state<ViewMode>("list");
  search = $state("");
  categoryFilter = $state<number | null>(null);

  constructor() {
    this.view = this.loadView();
  }

  filtered = $derived.by(() => {
    const q = this.search.trim().toLowerCase();
    return this.items.filter((p) => {
      if (this.categoryFilter !== null && p.category_id !== this.categoryFilter) {
        return false;
      }
      if (!q) return true;
      return (
        p.name.toLowerCase().includes(q) ||
        (p.description ?? "").toLowerCase().includes(q)
      );
    });
  });

  async load(): Promise<void> {
    this.loading = true;
    try {
      this.items = await productService.list();
    } finally {
      this.loading = false;
    }
  }

  setView(view: ViewMode): void {
    this.view = view;
    try {
      localStorage.setItem(VIEW_KEY, view);
    } catch {
      /* ignore */
    }
  }

  private loadView(): ViewMode {
    try {
      return localStorage.getItem(VIEW_KEY) === "grid" ? "grid" : "list";
    } catch {
      return "list";
    }
  }
}

export const productsStore = new ProductsStore();
