import { invoke } from "$lib/api";
import type { Category } from "$lib/types";
import { getDb } from "$lib/db";

class CategoryService {
  async list(): Promise<Category[]> {
    const db = await getDb();
    return db.select<Category[]>(
      "SELECT id, name, color, icon FROM categories ORDER BY name COLLATE NOCASE",
    );
  }

  async create(name: string, color: string | null, icon: string | null): Promise<number> {
    return invoke("save_catalog", { input: { kind: "category", name, color, icon } });
  }
  async update(id: number, name: string, color: string | null, icon: string | null): Promise<void> {
    await invoke("save_catalog", { input: { kind: "category", id, name, color, icon } });
  }
  async remove(id: number): Promise<void> {
    await invoke("archive_record", { kind: "category", id, restore: false });
  }
}

export const categoryService = new CategoryService();
