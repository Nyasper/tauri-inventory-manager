import { invoke } from "@tauri-apps/api/core";
import type { Movement, MovementType } from "$lib/types";
import { getDb } from "$lib/db";

export interface MovementInput {
  product_id: number;
  type: MovementType;
  quantity: number;
  presentation_id: number;
  supplier_id: number | null;
  responsible_person_id: number | null;
  notes: string | null;
}

class MovementService {
  async list(): Promise<Movement[]> {
    const db = await getDb();
    return db.select<Movement[]>(
      `SELECT m.*, p.name AS product_name, s.name AS supplier_name, rp.name AS person_name
       FROM movements m
       LEFT JOIN products p ON p.id = m.product_id
       LEFT JOIN suppliers s ON s.id = m.supplier_id
       LEFT JOIN responsible_persons rp ON rp.id = m.responsible_person_id
       ORDER BY m.created_at DESC, m.id DESC`,
    );
  }

  async create(input: MovementInput): Promise<void> {
    await invoke("create_movement", { input });
  }

  async remove(id: number): Promise<void> {
    await invoke("delete_movement", { id });
  }
}

export const movementService = new MovementService();
