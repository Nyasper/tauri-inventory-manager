import { invoke } from "$lib/api";
import type { ResponsiblePerson } from "$lib/types";
import { getDb } from "$lib/db";

class PersonService {
  async list(archived = false): Promise<ResponsiblePerson[]> {
    const db = await getDb();
    const rows = await db.select<
      { id: number; name: string; is_default: number }[]
    >("SELECT id, name, is_default FROM responsible_persons WHERE archived=? ORDER BY name COLLATE NOCASE", [archived ? 1 : 0]);
    return rows.map((r) => ({
      id: r.id,
      name: r.name,
      is_default: Boolean(r.is_default),
    }));
  }

  async create(name: string, isDefault = false): Promise<number> {
    return invoke("save_catalog", { input: { kind: "person", name, is_default: isDefault } });
  }
  async update(id: number, name: string, isDefault: boolean): Promise<void> {
    await invoke("save_catalog", { input: { kind: "person", id, name, is_default: isDefault } });
  }
  async restore(id:number):Promise<void> { await invoke("archive_record",{kind:"person",id,restore:true}); }
  async remove(id: number): Promise<void> {
    await invoke("archive_record", { kind: "person", id, restore: false });
  }
}

export const personService = new PersonService();
