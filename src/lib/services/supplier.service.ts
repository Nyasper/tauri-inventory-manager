import { invoke } from "$lib/api";
import type { Supplier } from "$lib/types";
import { getDb } from "$lib/db";

class SupplierService {
  async list(archived = false): Promise<Supplier[]> {
    const db = await getDb();
    return db.select<Supplier[]>(
      "SELECT id, name, contact, notes, archived FROM suppliers WHERE archived = ? ORDER BY name COLLATE NOCASE", [archived ? 1 : 0],
    );
  }

  async create(data: Omit<Supplier, "id">): Promise<number> {
    return invoke("save_catalog", { input: { kind: "supplier", ...data } });
  }

  async update(data: Supplier): Promise<void> {
    await invoke("save_catalog", { input: { kind: "supplier", ...data } });
  }

  async restore(id: number): Promise<void> {
    await invoke("archive_record", { kind: "supplier", id, restore: true });
  }

  async remove(id: number): Promise<void> {
    await invoke("archive_record", { kind: "supplier", id, restore: false });
  }
}

export const supplierService = new SupplierService();
