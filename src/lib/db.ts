import Database from "@tauri-apps/plugin-sql";

let dbPromise: Promise<Database> | null = null;

export function getDb(): Promise<Database> {
  if (!dbPromise) {
    dbPromise = Database.load("sqlite:inventory.db").catch((error) => { dbPromise = null; throw error; });
  }
  return dbPromise;
}
