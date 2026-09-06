import { invoke } from "@tauri-apps/api/core";
import type { Presentation, Product } from "$lib/types";
import { getDb } from "$lib/db";

export interface PresentationInput {
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
  async list(): Promise<Product[]> {
    const db = await getDb();
    return db.select<Product[]>(
      `SELECT p.*, c.name AS category_name, c.color AS category_color, c.icon AS category_icon
       FROM products p
       LEFT JOIN categories c ON c.id = p.category_id
       ORDER BY p.name COLLATE NOCASE`,
    );
  }

  async presentations(productId: number): Promise<Presentation[]> {
    const db = await getDb();
    return db.select<Presentation[]>(
      "SELECT id, product_id, name, factor FROM presentations WHERE product_id = ? ORDER BY factor ASC",
      [productId],
    );
  }

  async save(input: ProductInput): Promise<number> {
    const db = await getDb();
    let productId = input.id;

    if (productId) {
      await db.execute(
        "UPDATE products SET name = ?, description = ?, category_id = ?, updated_at = datetime('now') WHERE id = ?",
        [
          input.name,
          input.description,
          input.category_id,
          productId,
        ],
      );
      await db.execute("DELETE FROM presentations WHERE product_id = ?", [
        productId,
      ]);
    } else {
      const result = await db.execute(
        "INSERT INTO products (name, description, category_id) VALUES (?, ?, ?)",
        [
          input.name,
          input.description,
          input.category_id,
        ],
      );
      productId = result.lastInsertId ?? -1;
    }

    const ids: number[] = [];
    for (const p of input.presentations) {
      const result = await db.execute(
        "INSERT INTO presentations (product_id, name, factor) VALUES (?, ?, ?)",
        [productId, p.name, p.factor],
      );
      ids.push(result.lastInsertId ?? -1);
    }

    const baseIndex = input.presentations.findIndex((p) => p.factor === 1);
    const basePresentationId =
      ids[baseIndex >= 0 ? baseIndex : 0] ?? null;

    await db.execute(
      "UPDATE products SET base_presentation_id = ? WHERE id = ?",
      [basePresentationId, productId],
    );

    return productId;
  }

  async remove(id: number): Promise<void> {
    await invoke("delete_product", { id });
  }
}

export const productService = new ProductService();
