import type { ResponsiblePerson } from "$lib/types";
import { getDb } from "$lib/db";

class PersonService {
  async list(): Promise<ResponsiblePerson[]> {
    const db = await getDb();
    const rows = await db.select<
      { id: number; name: string; is_default: number }[]
    >("SELECT id, name, is_default FROM responsible_persons ORDER BY name COLLATE NOCASE");
    return rows.map((r) => ({
      id: r.id,
      name: r.name,
      is_default: Boolean(r.is_default),
    }));
  }

  async create(name: string, isDefault = false): Promise<number> {
    const db = await getDb();
    if (isDefault) {
      await db.execute(
        "UPDATE responsible_persons SET is_default = 0 WHERE is_default = 1",
      );
    }
    const result = await db.execute(
      "INSERT INTO responsible_persons (name, is_default) VALUES (?, ?)",
      [name, isDefault ? 1 : 0],
    );
    return result.lastInsertId ?? -1;
  }

  async update(id: number, name: string, isDefault: boolean): Promise<void> {
    const db = await getDb();
    if (isDefault) {
      await db.execute(
        "UPDATE responsible_persons SET is_default = 0 WHERE is_default = 1",
      );
    }
    await db.execute(
      "UPDATE responsible_persons SET name = ?, is_default = ? WHERE id = ?",
      [name, isDefault ? 1 : 0, id],
    );
  }

  async remove(id: number): Promise<void> {
    const db = await getDb();
    await db.execute(
      "UPDATE movements SET responsible_person_id = NULL WHERE responsible_person_id = ?",
      [id],
    );
    await db.execute("DELETE FROM responsible_persons WHERE id = ?", [id]);
  }
}

export const personService = new PersonService();
