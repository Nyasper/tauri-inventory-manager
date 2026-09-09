import type { Supplier } from "$lib/types";
import {
  supplierService,
  type SupplierInput,
} from "$lib/services/supplier.service";

class SuppliersStore {
  items = $state<Supplier[]>([]);

  loading = $state(false);
  error = $state("");
  async load(): Promise<void> {
    this.loading = true; this.error = "";
    try { this.items = await supplierService.list(); } catch(e) { this.error = String(e); } finally { this.loading = false; }
  }

  async create(data: SupplierInput): Promise<void> {
    await supplierService.create(data);
    await this.load();
  }

  async update(data: SupplierInput & { id: number }): Promise<void> {
    await supplierService.update(data);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await supplierService.remove(id);
    await this.load();
  }
}

export const suppliersStore = new SuppliersStore();
