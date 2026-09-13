import { invoke } from "$lib/api";
import type { ResponsiblePerson } from "$lib/types";

class PersonService {
  list(archived = false): Promise<ResponsiblePerson[]> {
    return invoke<ResponsiblePerson[]>("list_persons", { archived });
  }

  create(name: string, isDefault = false): Promise<number> {
    return invoke<number>(
      "save_catalog",
      { input: { kind: "person", name, is_default: isDefault } },
      "Responsable guardado.",
    );
  }

  update(id: number, name: string, isDefault: boolean): Promise<void> {
    return invoke(
      "save_catalog",
      { input: { kind: "person", id, name, is_default: isDefault } },
      "Responsable guardado.",
    );
  }

  restore(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "person", id, restore: true },
      "Responsable restaurado.",
    );
  }

  remove(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "person", id, restore: false },
      "Responsable archivado.",
    );
  }
}

export const personService = new PersonService();
