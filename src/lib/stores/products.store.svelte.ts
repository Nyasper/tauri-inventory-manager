import type { Product } from "$lib/types";
import { productService } from "$lib/services/product.service";

type ViewMode = "list" | "grid";

const VIEW_KEY = "viewMode";

class ProductsStore {
  items = $state<Product[]>([]);
  loading = $state(false);
  error = $state("");
  view = $state<ViewMode>("list");

  constructor() {
    this.view = this.loadView();
  }

  async load(): Promise<void> {
    this.loading = true;
    this.error = "";
    try {
      this.items = await productService.list();
    } catch(e) { this.error = String(e); } finally {
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
