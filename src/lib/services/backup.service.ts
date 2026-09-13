import { invoke } from "$lib/api";
import { open, save } from "@tauri-apps/plugin-dialog";

const FILTERS = [{ name: "Base de datos SQLite", extensions: ["db"] }];

function stamp(): string {
  const now = new Date();
  const pad = (value: number) => String(value).padStart(2, "0");
  return [
    now.getFullYear(),
    pad(now.getMonth() + 1),
    pad(now.getDate()),
    "-",
    pad(now.getHours()),
    pad(now.getMinutes()),
    pad(now.getSeconds()),
  ].join("");
}

class BackupService {
  /** Pide una ruta y crea allí un respaldo consistente de la base de datos. */
  async backup(): Promise<string | null> {
    const path = await save({
      title: "Guardar respaldo",
      defaultPath: `inventario-${stamp()}.db`,
      filters: FILTERS,
    });
    if (!path) return null;
    await invoke("backup_database", { destPath: path });
    return path;
  }

  /** Abre el diálogo para elegir un archivo de respaldo. */
  async pickRestoreFile(): Promise<string | null> {
    const selected = await open({
      title: "Seleccionar respaldo",
      multiple: false,
      directory: false,
      filters: FILTERS,
    });
    return typeof selected === "string" ? selected : null;
  }

  /** Reemplaza la base de datos actual por la del respaldo indicado. */
  async restore(path: string): Promise<void> {
    await invoke("restore_database", { srcPath: path });
  }
}

export const backupService = new BackupService();
