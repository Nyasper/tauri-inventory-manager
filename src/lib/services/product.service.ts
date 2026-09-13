import { invoke } from "$lib/api";
import type { DuplicateRecord, Presentation, Product } from "$lib/types";

export interface PresentationInput {
  id?: number;
  name: string;
  factor: number;
}

export interface ProductInput {
  id: number | null;
  name: string;
  description: string | null;
  category_id: number | null;
  presentations: PresentationInput[];
}

class ProductService {
  list(archived = false): Promise<Product[]> {
    return invoke<Product[]>("list_products", { archived });
  }

  presentations(productId: number): Promise<Presentation[]> {
    return invoke<Presentation[]>("list_presentations", { productId });
  }

  save(input: ProductInput): Promise<number> {
    return invoke<number>("save_product", { input }, "Producto guardado.");
  }

  restore(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "product", id, restore: true },
      "Producto restaurado.",
    );
  }

  remove(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "product", id, restore: false },
      "Producto archivado.",
    );
  }

  findDuplicate(
    name: string,
    currentId?: number,
  ): Promise<DuplicateRecord | null> {
    return invoke<DuplicateRecord | null>("find_duplicate", {
      kind: "product",
      name,
      currentId: currentId ?? null,
    });
  }
}

export const productService = new ProductService();
