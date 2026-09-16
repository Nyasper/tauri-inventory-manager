import type { Supplier } from "$lib/types";
import { errorMessage } from "$lib/utils/errors";
import {
  archiveSupplier,
  deleteSupplier,
  getSuppliers,
  restoreSupplier,
  saveSupplier,
  type SupplierInput,
} from "$lib/services/db.service";

class SuppliersService {
  items = $state<Supplier[]>([]);
  loading = $state(false);
  error = $state("");

  async load(): Promise<void> {
    this.loading = true;
    this.error = "";
    try {
      this.items = await getSuppliers();
    } catch (e) {
      this.error = errorMessage(e);
    } finally {
      this.loading = false;
    }
  }

  async create(data: SupplierInput): Promise<number> {
    const id = await saveSupplier(null, data);
    await this.load();
    return id;
  }

  async update(data: SupplierInput & { id: number }): Promise<void> {
    await saveSupplier(data.id, data);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await archiveSupplier(id);
    await this.load();
  }

  async restore(id: number): Promise<void> {
    await restoreSupplier(id);
    await this.load();
  }

  async removePermanently(id: number, password: string): Promise<void> {
    await deleteSupplier(id, password);
    await this.load();
  }
}

export const suppliersService = new SuppliersService();
