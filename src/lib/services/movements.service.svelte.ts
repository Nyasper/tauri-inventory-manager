import { invoke } from "$lib/api";
import type { Movement, MovementType } from "$lib/types";

export interface MovementInput {
  product_id: number;
  type: MovementType;
  quantity: number;
  presentation_id: number;
  supplier_id: number | null;
  responsible_person_id: number;
  notes: string | null;
  adjustment?: boolean;
  reason?: string | null;
}

export interface MovementFilters {
  search?: string;
  type?: string;
  product?: string;
  supplier?: string;
  person?: string;
  from?: string;
  to?: string;
  status?: string;
  page?: number;
}

export interface MovementPage {
  items: Movement[];
  total: number;
}

interface MovementQuery {
  search: string | null;
  type: string | null;
  product: number | null;
  supplier: number | null;
  person: number | null;
  from: string | null;
  to: string | null;
  status: string | null;
  page: number;
}

/** Convierte un día local (`YYYY-MM-DD`) al instante UTC equivalente. */
function utcDayBoundary(day: string, nextDay: boolean): string | null {
  const date = new Date(`${day}T00:00:00`);
  if (Number.isNaN(date.getTime())) return null;
  if (nextDay) date.setDate(date.getDate() + 1);
  return date.toISOString().slice(0, 19).replace("T", " ");
}

function toQuery(filters: MovementFilters): MovementQuery {
  return {
    search: filters.search?.trim() || null,
    type: filters.type && filters.type !== "all" ? filters.type : null,
    product: filters.product ? Number(filters.product) : null,
    supplier: filters.supplier ? Number(filters.supplier) : null,
    person: filters.person ? Number(filters.person) : null,
    from: filters.from ? utcDayBoundary(filters.from, false) : null,
    to: filters.to ? utcDayBoundary(filters.to, true) : null,
    status:
      filters.status === "active" || filters.status === "cancelled"
        ? filters.status
        : null,
    page: Math.max(0, filters.page ?? 0),
  };
}

class MovementsService {
  items = $state<Movement[]>([]);
  total = $state(0);
  loading = $state(false);
  error = $state("");
  filters: MovementFilters = {};
  private request = 0;

  page(filters: MovementFilters = {}): Promise<MovementPage> {
    return invoke<MovementPage>("page_movements", { filters: toQuery(filters) });
  }

  async load(filters: MovementFilters = this.filters): Promise<void> {
    this.filters = filters;
    const request = ++this.request;
    this.loading = true;
    this.error = "";
    try {
      const result = await this.page(filters);
      if (request !== this.request) return;
      this.items = result.items;
      this.total = result.total;
    } catch (e) {
      if (request === this.request) this.error = String(e);
    } finally {
      if (request === this.request) this.loading = false;
    }
  }

  async create(input: MovementInput): Promise<void> {
    await invoke("create_movement", { input }, "Movimiento registrado.");
    await this.load();
  }

  async remove(id: number, reason: string): Promise<void> {
    await invoke("cancel_movement", { id, reason }, "Movimiento anulado.");
    await this.load();
  }
}

export const movementsService = new MovementsService();