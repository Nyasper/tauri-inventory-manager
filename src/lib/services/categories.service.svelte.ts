import { invoke } from "$lib/api";
import type { Category } from "$lib/types";

class CategoriesService {
  items = $state<Category[]>([]);

  list(): Promise<Category[]> {
    return invoke<Category[]>("list_categories");
  }

  async load(): Promise<void> {
    this.items = await this.list();
  }

  async create(
    name: string,
    color: string | null,
    icon: string | null,
  ): Promise<void> {
    await invoke<number>(
      "save_catalog",
      { input: { kind: "category", name, color, icon } },
      "Categoría guardada.",
    );
    await this.load();
  }

  async update(
    id: number,
    name: string,
    color: string | null,
    icon: string | null,
  ): Promise<void> {
    await invoke(
      "save_catalog",
      { input: { kind: "category", id, name, color, icon } },
      "Categoría guardada.",
    );
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await invoke(
      "archive_record",
      { kind: "category", id, restore: false },
      "Categoría eliminada.",
    );
    await this.load();
  }
}

export const categoriesService = new CategoriesService();