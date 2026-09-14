import { invoke } from "$lib/api";
import type { ResponsiblePerson } from "$lib/types";

class PersonsService {
  items = $state<ResponsiblePerson[]>([]);

  defaultPerson = $derived(this.items.find((p) => p.is_default) ?? null);

  list(archived = false): Promise<ResponsiblePerson[]> {
    return invoke<ResponsiblePerson[]>("list_persons", { archived });
  }

  restore(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "person", id, restore: true },
      "Responsable restaurado.",
    );
  }

  async load(): Promise<void> {
    this.items = await this.list();
  }

  async create(name: string, isDefault = false): Promise<void> {
    await invoke<number>(
      "save_catalog",
      { input: { kind: "person", name, is_default: isDefault } },
      "Responsable guardado.",
    );
    await this.load();
  }

  async update(id: number, name: string, isDefault: boolean): Promise<void> {
    await invoke(
      "save_catalog",
      { input: { kind: "person", id, name, is_default: isDefault } },
      "Responsable guardado.",
    );
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await invoke(
      "archive_record",
      { kind: "person", id, restore: false },
      "Responsable archivado.",
    );
    await this.load();
  }
}

export const personsService = new PersonsService();