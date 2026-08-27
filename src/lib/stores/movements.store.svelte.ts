import type { Movement } from "$lib/types";
import { movementService } from "$lib/services/movement.service";
import type { MovementInput } from "$lib/services/movement.service";

class MovementsStore {
  items = $state<Movement[]>([]);

  async load(): Promise<void> {
    this.items = await movementService.list();
  }

  async create(input: MovementInput): Promise<void> {
    await movementService.create(input);
    await this.load();
  }

  async remove(id: number): Promise<void> {
    await movementService.remove(id);
    await this.load();
  }
}

export const movementsStore = new MovementsStore();
