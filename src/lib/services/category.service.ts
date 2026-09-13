import { invoke } from "$lib/api";
import type { Category } from "$lib/types";

class CategoryService {
  list(): Promise<Category[]> {
    return invoke<Category[]>("list_categories");
  }

  create(
    name: string,
    color: string | null,
    icon: string | null,
  ): Promise<number> {
    return invoke<number>(
      "save_catalog",
      { input: { kind: "category", name, color, icon } },
      "Categoría guardada.",
    );
  }

  update(
    id: number,
    name: string,
    color: string | null,
    icon: string | null,
  ): Promise<void> {
    return invoke(
      "save_catalog",
      { input: { kind: "category", id, name, color, icon } },
      "Categoría guardada.",
    );
  }

  remove(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "category", id, restore: false },
      "Categoría eliminada.",
    );
  }
}

export const categoryService = new CategoryService();
