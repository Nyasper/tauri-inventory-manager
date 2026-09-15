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
  notes: string | null;
  adjustment?: boolean;
  reason?: string | null;
}

export interface MovementFilters {
  search?: string;
  type?: string;
  product?: string;
  supplier?: string;
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
async function invoke<T>(
  command: string,
  args?: Record<string, unknown>,
  successMessage?: string,
): Promise<T> {
  const result = await tauriInvoke<T>(command, args);
  if (successMessage) feedback.success(successMessage);
  return result;
}

/** Indica si el error devuelto por el backend corresponde a un nombre duplicado. */
export function isDuplicateError(error: string): boolean {
  return error.includes("Ya existe");
}

/* ------------------------------- Productos ------------------------------- */

export function getProducts(archived = false): Promise<Product[]> {
  return invoke<Product[]>("list_products", { archived });
}

export function getPresentations(productId: number): Promise<Presentation[]> {
  return invoke<Presentation[]>("list_presentations", { productId });
}

export function saveProduct(input: ProductInput): Promise<number> {
  return invoke<number>("save_product", { input }, "Producto guardado.");
}

export function archiveProduct(id: number): Promise<void> {
  return invoke("archive_record", { kind: "product", id, restore: false }, "Producto archivado.");
}

export function restoreProduct(id: number): Promise<void> {
  return invoke("archive_record", { kind: "product", id, restore: true }, "Producto restaurado.");
}

export function deleteProduct(id: number, password: string): Promise<void> {
  return invoke("delete_product", { id, password }, "Producto eliminado definitivamente.");
}

export function findProductDuplicate(
  name: string,
  currentId?: number,
): Promise<DuplicateRecord | null> {
  return invoke<DuplicateRecord | null>("find_duplicate", {
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
    from: filters.from ? utcDayBoundary(filters.from, false) : null,
    to: filters.to ? utcDayBoundary(filters.to, true) : null,
    status: filters.status === "active" || filters.status === "cancelled" ? filters.status : null,
    page: Math.max(0, filters.page ?? 0),
  };
}

export function getMovements(filters: MovementFilters = {}): Promise<MovementPage> {
  return invoke<MovementPage>("page_movements", { filters: toQuery(filters) });
}

export function createMovement(input: MovementInput): Promise<void> {
  return invoke("create_movement", { input }, "Movimiento registrado.");
}

export function cancelMovement(id: number, reason: string): Promise<void> {
  return invoke("cancel_movement", { id, reason }, "Movimiento anulado.");
}

export type MovementDeleteMode = "recent" | "oldest" | "from" | "until" | "range";

/** Filtro de borrado en lote. `from`/`to` son días locales (`YYYY-MM-DD`). */
export interface MovementDeleteFilter {
  mode: MovementDeleteMode;
  count?: number;
  from?: string;
  to?: string;
}

interface MovementDeleteQuery {
  mode: MovementDeleteMode;
  count: number | null;
  from: string | null;
  to: string | null;
}

function toDeleteQuery(filter: MovementDeleteFilter): MovementDeleteQuery {
  const amount = Number(filter.count);
  return {
    mode: filter.mode,
    count:
      filter.mode === "recent" || filter.mode === "oldest"
        ? Number.isFinite(amount)
          ? amount
          : null
        : null,
    from: filter.from ? utcDayBoundary(filter.from, false) : null,
    to: filter.to ? utcDayBoundary(filter.to, true) : null,
  };
}

/** Cuenta los movimientos que borraría el filtro, sin eliminar nada. */
export function countMovements(filter: MovementDeleteFilter): Promise<number> {
  return invoke<number>("count_movements", { filter: toDeleteQuery(filter) });
}

/** Borra movimientos en lote; el stock de los productos no se modifica. */
export function deleteMovements(filter: MovementDeleteFilter, password: string): Promise<number> {
  return invoke<number>("delete_movements", {
    filter: toDeleteQuery(filter),
    password,
  });
}

export function changeDeletePassword(oldPassword: string, newPassword: string): Promise<void> {
  return invoke("change_delete_password", { oldPassword, newPassword }, "Contraseña actualizada.");
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
        all.findIndex((candidate) => candidate.toLowerCase() === contact.toLowerCase()) === index,
    );
  return unique.length ? JSON.stringify(unique) : null;
}

export async function getSuppliers(archived = false): Promise<Supplier[]> {
  const rows = await invoke<SupplierRow[]>("list_suppliers", { archived });
  return rows.map(({ contact, ...supplier }) => ({
    ...supplier,
    contacts: parseContacts(contact),
  }));
}

export function saveSupplier(id: number | null, data: SupplierInput): Promise<number> {
  return invoke<number>(
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
  return invoke("archive_record", { kind: "supplier", id, restore: false }, "Proveedor archivado.");
}

export function restoreSupplier(id: number): Promise<void> {
  return invoke("archive_record", { kind: "supplier", id, restore: true }, "Proveedor restaurado.");
}

export function deleteSupplier(id: number, password: string): Promise<void> {
  return invoke("delete_supplier", { id, password }, "Proveedor eliminado definitivamente.");
}

export function findSupplierDuplicate(
  name: string,
  currentId?: number,
): Promise<DuplicateRecord | null> {
  return invoke<DuplicateRecord | null>("find_duplicate", {
    kind: "supplier",
    name,
    currentId: currentId ?? null,
  });
}

/* ------------------------------- Categorías ------------------------------ */

export function getCategories(): Promise<Category[]> {
  return invoke<Category[]>("list_categories");
}

export function saveCategory(
  id: number | null,
  name: string,
  color: string | null,
  icon: string | null,
): Promise<number> {
  return invoke<number>(
    "save_catalog",
    { input: { kind: "category", id, name, color, icon } },
    "Categoría guardada.",
  );
}

export function archiveCategory(id: number): Promise<void> {
  return invoke("archive_record", { kind: "category", id, restore: false }, "Categoría eliminada.");
}

/* -------------------------------- Ajustes -------------------------------- */

export function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export function saveSettingsThresholds(low: number, medium: number): Promise<void> {
  return invoke("save_settings", { low, medium }, "Límites guardados.");
}

export function saveSettingsDefaultView(view: DefaultView): Promise<void> {
  return invoke("save_settings", { view }, "Vista por defecto actualizada.");
}

/* --------------------------------- Backup -------------------------------- */

export function backupDatabase(destPath: string): Promise<void> {
  return invoke("backup_database", { destPath });
}

export function restoreDatabase(srcPath: string): Promise<void> {
  return invoke("restore_database", { srcPath });
}
