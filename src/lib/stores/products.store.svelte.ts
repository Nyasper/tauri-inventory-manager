import type { Product } from "$lib/types";
import { productService } from "$lib/services/product.service";

class ProductsStore {
  items = $state<Product[]>([]);
  loading = $state(false);
  error = $state("");

  async load(): Promise<void> {
    this.loading = true;
    this.error = "";
    try {
      this.items = await productService.list();
    } catch(e) { this.error = String(e); } finally {
      this.loading = false;
    }
  }
}

export const productsStore = new ProductsStore();
