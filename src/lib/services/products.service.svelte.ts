import type { Product } from "$lib/types";
import {
  archiveProduct,
  deleteProduct,
  getProducts,
  restoreProduct,
  saveProduct,
  type ProductInput,
} from "$lib/services/db.service";

class ProductsService {
  items = $state<Product[]>([]);
  loading = $state(false);
  error = $state("");

  async load(archived = false): Promise<void> {
    this.loading = true;
    this.error = "";
    try {
      this.items = await getProducts(archived);
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async save(input: ProductInput): Promise<void> {
    await saveProduct(input);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await archiveProduct(id);
    await this.load();
  }

  async restore(id: number): Promise<void> {
    await restoreProduct(id);
    await this.load();
  }

  async removePermanently(id: number): Promise<void> {
    await deleteProduct(id);
    await this.load();
  }
}

export const productsService = new ProductsService();
