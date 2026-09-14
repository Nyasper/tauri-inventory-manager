import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { feedback } from "$lib/services/feedback.service.svelte";
import type {
  AppSettings,
  Category,
  DefaultView,
  DuplicateRecord,
  Movement,
  MovementType,
  Presentation,
  Product,
  ResponsiblePerson,
  Supplier,
} from "$lib/types";

export interface PresentationInput {
  id?: number;
  name: string;
  factor: number;
}

export interface ProductInput {
  id: number | null;
  name: string;
  description: string | null;
  category_id: number | null;
  presentations: PresentationInput[];
}

export interface SupplierInput {
  name: string;
  contacts: string[];
  notes: string | null;
}

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

interface SupplierRow {
  id: number;
  name: string;
  contact: string | null;
  notes: string | null;
  archived: number;
}

/** Ejecuta un comando de Tauri y, si se indica, muestra un aviso de éxito. */
async function call<T>(
  command: string,
  args?: Record<string, unknown>,
  successMessage?: string,
): Promise<T> {
  const result = await tauriInvoke<T>(command, args);
  if (successMessage) feedback.success(successMessage);
  return result;
}

/* ------------------------------- Productos ------------------------------- */

export function getProducts(archived = false): Promise<Product[]> {
  return call<Product[]>("list_products", { archived });
}

export function getPresentations(productId: number): Promise<Presentation[]> {
  return call<Presentation[]>("list_presentations", { productId });
}

export function saveProduct(input: ProductInput): Promise<number> {
  return call<number>("save_product", { input }, "Producto guardado.");
}

export function archiveProduct(id: number): Promise<void> {
  return call(
    "archive_record",
    { kind: "product", id, restore: false },
    "Producto archivado.",
  );
}

export function restoreProduct(id: number): Promise<void> {
  return call(
    "archive_record",
    { kind: "product", id, restore: true },
    "Producto restaurado.",
  );
}

export function findProductDuplicate(
  name: string,
  currentId?: number,
): Promise<DuplicateRecord | null> {
  return call<DuplicateRecord | null>("find_duplicate", {
    kind: "product",
    name,
    currentId: currentId ?? null,
  });
}

/* ------------------------------ Movimientos ------------------------------ */

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

export function getMovements(
  filters: MovementFilters = {},
): Promise<MovementPage> {
  return call<MovementPage>("page_movements", { filters: toQuery(filters) });
}

export function createMovement(input: MovementInput): Promise<void> {
  return call("create_movement", { input }, "Movimiento registrado.");
}

export function cancelMovement(id: number, reason: string): Promise<void> {
  return call("cancel_movement", { id, reason }, "Movimiento anulado.");
}

/* ------------------------------ Proveedores ------------------------------ */

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

export async function getSuppliers(archived = false): Promise<Supplier[]> {
  const rows = await call<SupplierRow[]>("list_suppliers", { archived });
  return rows.map(({ contact, ...supplier }) => ({
    ...supplier,
    contacts: parseContacts(contact),
  }));
}

export function saveSupplier(
  id: number | null,
  data: SupplierInput,
): Promise<number> {
  return call<number>(
    "save_catalog",
    {
      input: {
        kind: "supplier",
        id,
        name: data.name,
        contact: contactPayload(data.contacts),
        notes: data.notes,
      },
    },
    "Proveedor guardado.",
  );
}

export function archiveSupplier(id: number): Promise<void> {
  return call(
    "archive_record",
    { kind: "supplier", id, restore: false },
    "Proveedor archivado.",
  );
}

export function restoreSupplier(id: number): Promise<void> {
  return call(
    "archive_record",
    { kind: "supplier", id, restore: true },
    "Proveedor restaurado.",
  );
}

export function findSupplierDuplicate(
  name: string,
  currentId?: number,
): Promise<DuplicateRecord | null> {
  return call<DuplicateRecord | null>("find_duplicate", {
    kind: "supplier",
    name,
    currentId: currentId ?? null,
  });
}

/* ------------------------------ Responsables ----------------------------- */

export function getPersons(archived = false): Promise<ResponsiblePerson[]> {
  return call<ResponsiblePerson[]>("list_persons", { archived });
}

export function savePerson(
  id: number | null,
  name: string,
  isDefault: boolean,
): Promise<number> {
  return call<number>(
    "save_catalog",
    { input: { kind: "person", id, name, is_default: isDefault } },
    "Responsable guardado.",
  );
}

export function archivePerson(id: number): Promise<void> {
  return call(
    "archive_record",
    { kind: "person", id, restore: false },
    "Responsable archivado.",
  );
}

export function restorePerson(id: number): Promise<void> {
  return call(
    "archive_record",
    { kind: "person", id, restore: true },
    "Responsable restaurado.",
  );
}

/* ------------------------------- Categorías ------------------------------ */

export function getCategories(): Promise<Category[]> {
  return call<Category[]>("list_categories");
}

export function saveCategory(
  id: number | null,
  name: string,
  color: string | null,
  icon: string | null,
): Promise<number> {
  return call<number>(
    "save_catalog",
    { input: { kind: "category", id, name, color, icon } },
    "Categoría guardada.",
  );
}

export function archiveCategory(id: number): Promise<void> {
  return call(
    "archive_record",
    { kind: "category", id, restore: false },
    "Categoría eliminada.",
  );
}

/* -------------------------------- Ajustes -------------------------------- */

export function getSettings(): Promise<AppSettings> {
  return call<AppSettings>("get_settings");
}

export function saveSettingsThresholds(
  low: number,
  medium: number,
): Promise<void> {
  return call("save_settings", { low, medium }, "Umbrales guardados.");
}

export function saveSettingsDefaultView(view: DefaultView): Promise<void> {
  return call(
    "save_settings",
    { view },
    "Vista por defecto actualizada.",
  );
}

/* --------------------------------- Backup -------------------------------- */

export function backupDatabase(destPath: string): Promise<void> {
  return call("backup_database", { destPath });
}

export function restoreDatabase(srcPath: string): Promise<void> {
  return call("restore_database", { srcPath });
}