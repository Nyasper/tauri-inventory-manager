import type { Category } from "$lib/types";
import { categoryService } from "$lib/services/category.service";

class CategoriesStore {
  items = $state<Category[]>([]);

  async load(): Promise<void> {
    this.items = await categoryService.list();
  }

  async create(name: string, color: string | null): Promise<void> {
    await categoryService.create(name, color);
    await this.load();
  }

  async update(id: number, name: string, color: string | null): Promise<void> {
    await categoryService.update(id, name, color);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await categoryService.remove(id);
    await this.load();
  }
}

export const categoriesStore = new CategoriesStore();
