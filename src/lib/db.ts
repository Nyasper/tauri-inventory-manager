import Database from "@tauri-apps/plugin-sql";

let dbPromise: Promise<Database> | null = null;

export function getDb(): Promise<Database> {
  if (!dbPromise) {
    dbPromise = Database.load("sqlite:inventory.db").catch((error) => { dbPromise = null; throw error; });
  }
  return dbPromise;
}

/** Cierra la conexión actual para poder reemplazar el archivo de la base de datos. */
export async function closeDb(): Promise<void> {
  const pending = dbPromise;
  dbPromise = null;
  if (!pending) return;
  try {
    const db = await pending;
    await db.close();
  } catch {
    /* la conexión ya no está disponible */
  }
}
