import type { Supplier } from "$lib/types";
import { getDb } from "$lib/db";

class SupplierService {
  async list(): Promise<Supplier[]> {
    const db = await getDb();
    return db.select<Supplier[]>(
      "SELECT id, name, contact, notes FROM suppliers ORDER BY name COLLATE NOCASE",
    );
  }

  async create(data: Omit<Supplier, "id">): Promise<number> {
    const db = await getDb();
    const result = await db.execute(
      "INSERT INTO suppliers (name, contact, notes) VALUES (?, ?, ?)",
      [data.name, data.contact, data.notes],
    );
    return result.lastInsertId ?? -1;
  }

  async update(data: Supplier): Promise<void> {
    const db = await getDb();
    await db.execute(
      "UPDATE suppliers SET name = ?, contact = ?, notes = ? WHERE id = ?",
      [data.name, data.contact, data.notes, data.id],
    );
  }

  async remove(id: number): Promise<void> {
    const db = await getDb();
    await db.execute(
      "UPDATE movements SET supplier_id = NULL WHERE supplier_id = ?",
      [id],
    );
    await db.execute("DELETE FROM suppliers WHERE id = ?", [id]);
  }
}

export const supplierService = new SupplierService();
