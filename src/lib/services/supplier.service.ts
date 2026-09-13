import { invoke } from "$lib/api";
import type { DuplicateRecord, Supplier } from "$lib/types";

export interface SupplierInput {
  name: string;
  contacts: string[];
  notes: string | null;
}

interface SupplierRow {
  id: number;
  name: string;
  contact: string | null;
  notes: string | null;
  archived: number;
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
          (candidate) => candidate.toLowerCase() === contact.toLowerCase(),
        ) === index,
    );
  return unique.length ? JSON.stringify(unique) : null;
}

class SupplierService {
  async list(archived = false): Promise<Supplier[]> {
    const rows = await invoke<SupplierRow[]>("list_suppliers", { archived });
    return rows.map(({ contact, ...supplier }) => ({
      ...supplier,
      contacts: parseContacts(contact),
    }));
  }

  create(data: SupplierInput): Promise<number> {
    return invoke<number>(
      "save_catalog",
      {
        input: {
          kind: "supplier",
          name: data.name,
          contact: contactPayload(data.contacts),
          notes: data.notes,
        },
      },
      "Proveedor guardado.",
    );
  }

  update(data: SupplierInput & { id: number }): Promise<void> {
    return invoke(
      "save_catalog",
      {
        input: {
          kind: "supplier",
          id: data.id,
          name: data.name,
          contact: contactPayload(data.contacts),
          notes: data.notes,
        },
      },
      "Proveedor guardado.",
    );
  }

  restore(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "supplier", id, restore: true },
      "Proveedor restaurado.",
    );
  }

  remove(id: number): Promise<void> {
    return invoke(
      "archive_record",
      { kind: "supplier", id, restore: false },
      "Proveedor archivado.",
    );
  }

  findDuplicate(
    name: string,
    currentId?: number,
  ): Promise<DuplicateRecord | null> {
    return invoke<DuplicateRecord | null>("find_duplicate", {
      kind: "supplier",
      name,
      currentId: currentId ?? null,
    });
  }
}

export const supplierService = new SupplierService();
