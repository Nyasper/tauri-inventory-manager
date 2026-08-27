import type { ResponsiblePerson } from "$lib/types";
import { personService } from "$lib/services/person.service";

class PersonsStore {
  items = $state<ResponsiblePerson[]>([]);

  defaultPerson = $derived(this.items.find((p) => p.is_default) ?? null);

  async load(): Promise<void> {
    this.items = await personService.list();
  }

  async create(name: string, isDefault: boolean): Promise<void> {
    await personService.create(name, isDefault);
    await this.load();
  }

  async update(id: number, name: string, isDefault: boolean): Promise<void> {
    await personService.update(id, name, isDefault);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await personService.remove(id);
    await this.load();
  }
}

export const personsStore = new PersonsStore();
