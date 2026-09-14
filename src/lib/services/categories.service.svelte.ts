import type { Category } from "$lib/types";
import {
  archiveCategory,
  getCategories,
  saveCategory,
} from "$lib/services/db.service";

class CategoriesService {
  items = $state<Category[]>([]);

  async load(): Promise<void> {
    this.items = await getCategories();
  }

  async create(
    name: string,
    color: string | null,
    icon: string | null,
  ): Promise<number> {
    const id = await saveCategory(null, name, color, icon);
    await this.load();
    return id;
  }

  async update(
    id: number,
    name: string,
    color: string | null,
    icon: string | null,
  ): Promise<void> {
    await saveCategory(id, name, color, icon);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await archiveCategory(id);
    await this.load();
  }
}

export const categoriesService = new CategoriesService();