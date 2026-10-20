const NUMBER_FORMAT = new Intl.NumberFormat("es-ES", {
  maximumFractionDigits: 6,
});

/** SQLite almacena los instantes en UTC sin zona (`YYYY-MM-DD HH:MM:SS`). */
const SQLITE_DATETIME = /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$/;

const DATETIME_FORMAT = new Intl.DateTimeFormat("es-ES", {
  dateStyle: "medium",
  timeStyle: "short",
});

const DAY_FORMAT = new Intl.DateTimeFormat("es-ES", { dateStyle: "medium" });

export function formatNumber(value: number): string {
  return NUMBER_FORMAT.format(value);
}

export function formatDateTime(value: string | null): string {
  if (!value) return "—";
  const normalized = SQLITE_DATETIME.test(value) ? `${value.replace(" ", "T")}Z` : value;
  const date = new Date(normalized);
  if (Number.isNaN(date.getTime())) return value;
  return DATETIME_FORMAT.format(date);
}

/** Formatea un día local (`YYYY-MM-DD`) sin hora. */
export function formatDay(value: string): string {
  if (!value) return "—";
  const date = new Date(`${value}T00:00:00`);
  if (Number.isNaN(date.getTime())) return value;
  return DAY_FORMAT.format(date);
}
