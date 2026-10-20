import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { feedbackService } from "#lib/services/feedback.service.svelte.js";
import { toAppError } from "#lib/utils/errors.js";
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
} from "#lib/types.js";

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
  default_supplier_id?: number | null;
  stock_low_threshold?: number | null;
  stock_medium_threshold?: number | null;
  ignore_stock_alerts?: boolean;
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
  pageSize?: number;
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
  page_size: number | null;
}

interface ProductRow extends Omit<Product, "isArchived"> {
  archived: number;
}

interface SupplierRow {
  id: number;
  name: string;
  contact: string | null;
  notes: string | null;
  archived: number;
}

interface DuplicateRow {
  id: number;
  archived: number;
}

function toDuplicateRecord(row: DuplicateRow | null): DuplicateRecord | null {
  return row ? { id: row.id, isArchived: row.archived === 1 } : null;
}

/** Ejecuta un comando de Tauri y, si se indica, muestra un aviso de éxito. */
async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
  successMessage?: string,
): Promise<T> {
  let result: T;
  try {
    result = await tauriInvoke<T>(command, args);
  } catch (cause) {
    throw toAppError(cause);
  }
  if (successMessage) feedbackService.success(successMessage);
  return result;
}

/* ------------------------------- Productos ------------------------------- */

export async function getProducts(archivedOnly = false): Promise<Product[]> {
  const rows = await invokeCommand<ProductRow[]>("list_products", { archivedOnly });
  return rows.map(({ archived, ...product }) => ({ ...product, isArchived: archived === 1 }));
}

export function getPresentations(productId: number): Promise<Presentation[]> {
  return invokeCommand<Presentation[]>("list_presentations", { productId });
}

export function saveProduct(input: ProductInput): Promise<number> {
  return invokeCommand<number>("save_product", { input }, "Producto guardado.");
}

export function archiveProduct(id: number): Promise<void> {
  return invokeCommand(
    "archive_record",
    { recordType: "product", id, restore: false },
    "Producto archivado.",
  );
}

export function restoreProduct(id: number): Promise<void> {
  return invokeCommand(
    "archive_record",
    { recordType: "product", id, restore: true },
    "Producto restaurado.",
  );
}

export function deleteProduct(id: number, password: string): Promise<void> {
  return invokeCommand("delete_product", { id, password }, "Producto eliminado definitivamente.");
}

export async function findProductDuplicate(
  name: string,
  currentId?: number,
): Promise<DuplicateRecord | null> {
  const row = await invokeCommand<DuplicateRow | null>("find_duplicate", {
    entityType: "product",
    name,
    currentId: currentId ?? null,
  });
  return toDuplicateRecord(row);
}

/* ------------------------------ Movimientos ------------------------------ */

/** Convierte un día local (`YYYY-MM-DD`) al instante UTC equivalente. */
function utcDayBoundary(day: string, nextDay: boolean): string | null {
  const date = new Date(`${day}T00:00:00`);
  if (Number.isNaN(date.getTime())) return null;
  if (nextDay) date.setDate(date.getDate() + 1);
  return date.toISOString().slice(0, 19).replace("T", " ");
}

function toMovementQuery(filters: MovementFilters): MovementQuery {
  return {
    search: filters.search?.trim() || null,
    type: filters.type && filters.type !== "all" ? filters.type : null,
    product: filters.product ? Number(filters.product) : null,
    supplier: filters.supplier ? Number(filters.supplier) : null,
    from: filters.from ? utcDayBoundary(filters.from, false) : null,
    to: filters.to ? utcDayBoundary(filters.to, true) : null,
    status: filters.status === "active" || filters.status === "cancelled" ? filters.status : null,
    page: Math.max(0, filters.page ?? 0),
    page_size: filters.pageSize && filters.pageSize > 0 ? Math.floor(filters.pageSize) : null,
  };
}

export function getMovements(filters: MovementFilters = {}): Promise<MovementPage> {
  return invokeCommand<MovementPage>("list_movements_page", {
    filters: toMovementQuery(filters),
  });
}

export function createMovement(input: MovementInput): Promise<void> {
  return invokeCommand("create_movement", { input }, "Movimiento registrado.");
}

export function updateMovementSupplier(id: number, supplierId: number | null): Promise<void> {
  return invokeCommand("update_movement_supplier", { id, supplierId }, "Proveedor actualizado.");
}

export function cancelMovement(id: number, reason: string): Promise<void> {
  return invokeCommand("cancel_movement", { id, reason }, "Movimiento anulado.");
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

function toMovementDeleteQuery(filter: MovementDeleteFilter): MovementDeleteQuery {
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
  return invokeCommand<number>("count_movements", { filter: toMovementDeleteQuery(filter) });
}

/** Borra movimientos en lote; el stock de los productos no se modifica. */
export function deleteMovements(filter: MovementDeleteFilter, password: string): Promise<number> {
  return invokeCommand<number>("delete_movements", {
    filter: toMovementDeleteQuery(filter),
    password,
  });
}

export function changeDeletePassword(oldPassword: string, newPassword: string): Promise<void> {
  return invokeCommand(
    "change_delete_password",
    { oldPassword, newPassword },
    "Contraseña actualizada.",
  );
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

function serializeContacts(contacts: string[]): string | null {
  const unique = contacts
    .map((contact) => contact.trim())
    .filter(
      (contact, index, all) =>
        contact &&
        all.findIndex((candidate) => candidate.toLowerCase() === contact.toLowerCase()) === index,
    );
  return unique.length ? JSON.stringify(unique) : null;
}

export async function getSuppliers(archivedOnly = false): Promise<Supplier[]> {
  const rows = await invokeCommand<SupplierRow[]>("list_suppliers", { archivedOnly });
  return rows.map(({ contact, archived, ...supplier }) => ({
    ...supplier,
    contacts: parseContacts(contact),
    isArchived: archived === 1,
  }));
}

export function saveSupplier(id: number | null, data: SupplierInput): Promise<number> {
  return invokeCommand<number>(
    "save_catalog",
    {
      input: {
        recordType: "supplier",
        id,
        name: data.name,
        contact: serializeContacts(data.contacts),
        notes: data.notes,
      },
    },
    "Proveedor guardado.",
  );
}

export function archiveSupplier(id: number): Promise<void> {
  return invokeCommand(
    "archive_record",
    { recordType: "supplier", id, restore: false },
    "Proveedor archivado.",
  );
}

export function restoreSupplier(id: number): Promise<void> {
  return invokeCommand(
    "archive_record",
    { recordType: "supplier", id, restore: true },
    "Proveedor restaurado.",
  );
}

export function deleteSupplier(id: number, password: string): Promise<void> {
  return invokeCommand("delete_supplier", { id, password }, "Proveedor eliminado definitivamente.");
}

export async function findSupplierDuplicate(
  name: string,
  currentId?: number,
): Promise<DuplicateRecord | null> {
  const row = await invokeCommand<DuplicateRow | null>("find_duplicate", {
    entityType: "supplier",
    name,
    currentId: currentId ?? null,
  });
  return toDuplicateRecord(row);
}

/* ------------------------------- Categorías ------------------------------ */

export function getCategories(): Promise<Category[]> {
  return invokeCommand<Category[]>("list_categories");
}

export function saveCategory(
  id: number | null,
  name: string,
  color: string | null,
  icon: string | null,
): Promise<number> {
  return invokeCommand<number>(
    "save_catalog",
    { input: { recordType: "category", id, name, color, icon } },
    "Categoría guardada.",
  );
}

export function archiveCategory(id: number): Promise<void> {
  return invokeCommand(
    "archive_record",
    { recordType: "category", id, restore: false },
    "Categoría eliminada.",
  );
}

/* -------------------------------- Ajustes -------------------------------- */

/** Aviso generado durante el arranque (p. ej. base de datos apartada). */
export function takeStartupNotice(): Promise<string | null> {
  return invokeCommand<string | null>("take_startup_notice");
}

export function getSettings(): Promise<AppSettings> {
  return invokeCommand<AppSettings>("get_settings");
}

export function saveSettingsThresholds(
  lowThreshold: number,
  mediumThreshold: number,
): Promise<void> {
  return invokeCommand(
    "save_settings",
    { low: lowThreshold, medium: mediumThreshold },
    "Límites guardados.",
  );
}

export function saveSettingsDefaultView(view: DefaultView): Promise<void> {
  return invokeCommand("save_settings", { view }, "Vista por defecto actualizada.");
}

export function saveSettingsPageSize(pageSize: number): Promise<void> {
  return invokeCommand("save_settings", { pageSize }, "Elementos por página actualizados.");
}

/* --------------------------------- Backup -------------------------------- */

export function backupDatabase(destPath: string): Promise<void> {
  return invokeCommand("backup_database", { destPath });
}

export function restoreDatabase(srcPath: string): Promise<void> {
  return invokeCommand("restore_database", { srcPath });
}
