import type { Movement } from "#lib/types.js";
import { errorMessage } from "#lib/utils/errors.js";
import {
  cancelMovement,
  createMovement,
  deleteMovements,
  getMovements,
  updateMovementSupplier,
  type MovementDeleteFilter,
  type MovementFilters,
  type MovementInput,
} from "#lib/services/db.service.js";

class MovementsService {
  items = $state<Movement[]>([]);
  total = $state(0);
  loading = $state(false);
  error = $state("");
  filters: MovementFilters = {};
  private request = 0;

  async load(filters: MovementFilters = this.filters): Promise<void> {
    this.filters = filters;
    const request = ++this.request;
    this.loading = true;
    this.error = "";
    try {
      const result = await getMovements(filters);
      if (request !== this.request) return;
      this.items = result.items;
      this.total = result.total;
    } catch (e) {
      if (request === this.request) this.error = errorMessage(e);
    } finally {
      if (request === this.request) this.loading = false;
    }
  }

  async create(input: MovementInput): Promise<void> {
    await createMovement(input);
    await this.load();
  }

  async cancel(id: number, reason: string): Promise<void> {
    await cancelMovement(id, reason);
    await this.load();
  }

  async updateSupplier(id: number, supplierId: number | null): Promise<void> {
    await updateMovementSupplier(id, supplierId);
    await this.load();
  }

  async deleteBatch(filter: MovementDeleteFilter, password: string): Promise<number> {
    const deleted = await deleteMovements(filter, password);
    await this.load();
    return deleted;
  }
}

export const movementsService = new MovementsService();
