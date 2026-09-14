import type { ResponsiblePerson } from "$lib/types";
import {
  archivePerson,
  getPersons,
  restorePerson,
  savePerson,
} from "$lib/services/db.service";

class PersonsService {
  items = $state<ResponsiblePerson[]>([]);

  defaultPerson = $derived(this.items.find((p) => p.is_default) ?? null);

  async load(): Promise<void> {
    this.items = await getPersons();
  }

  async create(name: string, isDefault = false): Promise<number> {
    const id = await savePerson(null, name, isDefault);
    await this.load();
    return id;
  }

  async update(id: number, name: string, isDefault: boolean): Promise<void> {
    await savePerson(id, name, isDefault);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await archivePerson(id);
    await this.load();
  }

  async restore(id: number): Promise<void> {
    await restorePerson(id);
    await this.load();
  }
}

export const personsService = new PersonsService();