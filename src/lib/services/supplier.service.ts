import { invoke } from "$lib/api";
import type { Supplier } from "$lib/types";
import { getDb } from "$lib/db";

export interface SupplierInput {
  name: string;
  contacts: string[];
  notes: string | null;
}

interface SupplierRow extends Omit<Supplier, "contacts"> {
  contact: string | null;
}

function parseContacts(value: string | null): string[] {
  if (!value) return [];
  try {
    const parsed: unknown = JSON.parse(value);
    if (Array.isArray(parsed)) {
      return parsed.filter((item): item is string => typeof item === "string");
    }
  } catch {
    // Older databases may contain a single plain-text contact.
  }
  return [value];
}

function contactPayload(contacts: string[]): string | null {
  const unique = contacts
    .map((contact) => contact.trim())
    .filter(
      (contact, index, all) =>
        contact &&
        all.findIndex(
          (candidate) => candidate.toLocaleLowerCase() === contact.toLocaleLowerCase(),
        ) === index,
    );
  return unique.length ? JSON.stringify(unique) : null;
}

class SupplierService {
  async list(archived = false): Promise<Supplier[]> {
    const db = await getDb();
    const rows = await db.select<SupplierRow[]>(
      "SELECT id, name, contact, notes, archived FROM suppliers WHERE archived = ? ORDER BY name COLLATE NOCASE", [archived ? 1 : 0],
    );
    return rows.map(({ contact, ...supplier }) => ({
      ...supplier,
      contacts: parseContacts(contact),
    }));
  }

  async create(data: SupplierInput): Promise<number> {
    return invoke("save_catalog", {
      input: {
        kind: "supplier",
        name: data.name,
        contact: contactPayload(data.contacts),
        notes: data.notes,
      },
    });
  }

  async update(data: SupplierInput & { id: number }): Promise<void> {
    await invoke("save_catalog", {
      input: {
        kind: "supplier",
        id: data.id,
        name: data.name,
        contact: contactPayload(data.contacts),
        notes: data.notes,
      },
    });
  }

  async restore(id: number): Promise<void> {
    await invoke("archive_record", { kind: "supplier", id, restore: true });
  }

  async remove(id: number): Promise<void> {
    await invoke("archive_record", { kind: "supplier", id, restore: false });
  }
}

export const supplierService = new SupplierService();
