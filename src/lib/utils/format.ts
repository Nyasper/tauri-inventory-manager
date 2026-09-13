export function formatNumber(value: number): string {
  return new Intl.NumberFormat("es-ES", {
    maximumFractionDigits: 6,
  }).format(value);
}

/** SQLite almacena los instantes en UTC sin zona (`YYYY-MM-DD HH:MM:SS`). */
const SQLITE_DATETIME = /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$/;

export function formatDate(value: string | null): string {
  if (!value) return "—";
  const normalized = SQLITE_DATETIME.test(value)
    ? `${value.replace(" ", "T")}Z`
    : value;
  const date = new Date(normalized);
  if (Number.isNaN(date.getTime())) return value;
  return new Intl.DateTimeFormat("es-ES", {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(date);
}
