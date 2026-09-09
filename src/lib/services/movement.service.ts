import { invoke } from "$lib/api";
import type { Movement, MovementType } from "$lib/types";
import { getDb } from "$lib/db";
export interface MovementInput {
  product_id: number; type: MovementType; quantity: number; presentation_id: number;
  supplier_id: number | null; responsible_person_id: number | null; notes: string | null;
  adjustment?: boolean; reason?: string | null;
}
export interface MovementFilters {
  search?: string; type?: string; product?: string; supplier?: string; person?: string;
  from?: string; to?: string; status?: string; page?: number;
}
class MovementService {
  async page(filters: MovementFilters = {}): Promise<{ items: Movement[]; total: number }> {
    const db = await getDb();
    const clauses: string[] = []; const values: (string | number)[] = [];
    const add = (clause: string, value: string | number) => { clauses.push(clause); values.push(value); };
    if (filters.search?.trim()) { add("(product_name LIKE ? OR supplier_name LIKE ? OR person_name LIKE ? OR notes LIKE ?)", `%${filters.search.trim()}%`); values.push(values[0],values[0],values[0]); }
    if (filters.type && filters.type !== "all") add("type=?", filters.type);
    if (filters.product) add("product_id=?", Number(filters.product));
    if (filters.supplier) add("supplier_id=?", Number(filters.supplier));
    if (filters.person) add("responsible_person_id=?", Number(filters.person));
    if (filters.from) add("created_at>=?", new Date(`${filters.from}T00:00:00`).toISOString().slice(0,19).replace("T"," "));
    if (filters.to) { const end = new Date(`${filters.to}T00:00:00`); end.setDate(end.getDate()+1); add("created_at<?", end.toISOString().slice(0,19).replace("T"," ")); }
    if (filters.status === "active") clauses.push("cancelled_at IS NULL");
    if (filters.status === "cancelled") clauses.push("cancelled_at IS NOT NULL");
    const where = clauses.length ? `WHERE ${clauses.join(" AND ")}` : "";
    const count = await db.select<{ total: number }[]>(`SELECT count(*) AS total FROM movements ${where}`, values);
    const items = await db.select<Movement[]>(`SELECT * FROM movements ${where} ORDER BY created_at DESC,id DESC LIMIT 25 OFFSET ?`, [...values, (filters.page ?? 0)*25]);
    return { items, total: count[0].total };
  }
  async list(): Promise<Movement[]> { return (await this.page()).items; }
  async create(input: MovementInput): Promise<void> { await invoke("create_movement", { input }); }
  async remove(id: number, reason: string): Promise<void> { await invoke("cancel_movement", { id, reason }); }
}
export const movementService = new MovementService();
