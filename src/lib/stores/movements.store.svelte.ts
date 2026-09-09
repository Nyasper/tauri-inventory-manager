import type { Movement } from "$lib/types";
import { movementService, type MovementFilters, type MovementInput } from "$lib/services/movement.service";
class MovementsStore {
  items = $state<Movement[]>([]);
  total = $state(0);
  loading = $state(false);
  error = $state("");
  filters: MovementFilters = {};
  private request = 0;
  async load(filters: MovementFilters = this.filters): Promise<void> {
    this.filters = filters;
    const request = ++this.request;
    this.loading = true; this.error = "";
    try {
      const result = await movementService.page(filters);
      if (request !== this.request) return;
      this.items = result.items; this.total = result.total;
    } catch (e) { if (request === this.request) this.error = String(e); }
    finally { if (request === this.request) this.loading = false; }
  }
  async create(input: MovementInput): Promise<void> { await movementService.create(input); await this.load(); }
  async remove(id: number, reason: string): Promise<void> { await movementService.remove(id, reason); await this.load(); }
}
export const movementsStore = new MovementsStore();
