import type { Category } from "$lib/types";
import { getDb } from "$lib/db";

class CategoryService {
  async list(): Promise<Category[]> {
    const db = await getDb();
    return db.select<Category[]>(
      "SELECT id, name, color FROM categories ORDER BY name COLLATE NOCASE",
    );
  }

  async create(name: string, color: string | null): Promise<number> {
    const db = await getDb();
    const result = await db.execute(
      "INSERT INTO categories (name, color) VALUES (?, ?)",
      [name, color],
    );
    return result.lastInsertId ?? -1;
  }

  async update(id: number, name: string, color: string | null): Promise<void> {
    const db = await getDb();
    await db.execute("UPDATE categories SET name = ?, color = ? WHERE id = ?", [
      name,
      color,
      id,
    ]);
  }

  async remove(id: number): Promise<void> {
    const db = await getDb();
    await db.execute(
      "UPDATE products SET category_id = NULL WHERE category_id = ?",
      [id],
    );
    await db.execute("DELETE FROM categories WHERE id = ?", [id]);
  }
}

export const categoryService = new CategoryService();
