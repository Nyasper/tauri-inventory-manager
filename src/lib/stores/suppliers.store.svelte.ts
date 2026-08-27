import type { Supplier } from "$lib/types";
import { supplierService } from "$lib/services/supplier.service";

class SuppliersStore {
  items = $state<Supplier[]>([]);

  async load(): Promise<void> {
    this.items = await supplierService.list();
  }

  async create(data: Omit<Supplier, "id">): Promise<void> {
    await supplierService.create(data);
    await this.load();
  }

  async update(data: Supplier): Promise<void> {
    await supplierService.update(data);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await supplierService.remove(id);
    await this.load();
  }
}

export const suppliersStore = new SuppliersStore();
