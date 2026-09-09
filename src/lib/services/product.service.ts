import { invoke } from "$lib/api";
import type { Presentation, Product } from "$lib/types";
import { getDb } from "$lib/db";

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
  async list(archived = false): Promise<Product[]> {
    const db = await getDb();
    return db.select<Product[]>(
      `SELECT p.*, b.name AS base_unit, c.name AS category_name, c.color AS category_color, c.icon AS category_icon
       FROM products p
       LEFT JOIN presentations b ON b.id = p.base_presentation_id
       LEFT JOIN categories c ON c.id = p.category_id
       WHERE p.archived = ? ORDER BY p.name COLLATE NOCASE`, [archived ? 1 : 0],
    );
  }

  async presentations(productId: number): Promise<Presentation[]> {
    const db = await getDb();
    return db.select<Presentation[]>(
      "SELECT id, product_id, name, factor FROM presentations WHERE product_id = ? AND archived=0 ORDER BY factor ASC",
      [productId],
    );
  }

  async save(input: ProductInput): Promise<number> {
    return invoke<number>("save_product", { input });
  }

  async restore(id: number): Promise<void> {
    await invoke("archive_record", { kind: "product", id, restore: true });
  }

  async remove(id: number): Promise<void> {
    await invoke("archive_record", { kind: "product", id, restore: false });
  }
}

export const productService = new ProductService();
