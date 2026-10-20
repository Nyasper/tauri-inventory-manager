import type { Product } from "$lib/types";
import { errorMessage } from "$lib/utils/errors";
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

  async load(): Promise<void> {
    this.loading = true;
    this.error = "";
    try {
      this.items = await getProducts();
    } catch (e) {
      this.error = errorMessage(e);
    } finally {
      this.loading = false;
    }
  }

  async save(input: ProductInput): Promise<void> {
    await saveProduct(input);
    await this.load();
  }

  async archive(id: number): Promise<void> {
    await archiveProduct(id);
    await this.load();
  }

  async restore(id: number): Promise<void> {
    await restoreProduct(id);
    await this.load();
  }

  async deletePermanently(id: number, password: string): Promise<void> {
    await deleteProduct(id, password);
    await this.load();
  }
}

export const productsService = new ProductsService();
