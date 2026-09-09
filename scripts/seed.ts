import { Database } from "bun:sqlite";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, resolve } from "node:path";

const APP_IDENTIFIER = "com.herre.tauri-inventory-manager";

function defaultDatabasePath(): string {
  if (process.platform === "win32") {
    const appData = process.env.APPDATA;
    if (!appData) throw new Error("APPDATA is not defined.");
    return join(appData, APP_IDENTIFIER, "inventory.db");
  }
  if (process.platform === "darwin") {
    return join(
      homedir(),
      "Library",
      "Application Support",
      APP_IDENTIFIER,
      "inventory.db",
    );
  }
  return join(
    process.env.XDG_CONFIG_HOME ?? join(homedir(), ".config"),
    APP_IDENTIFIER,
    "inventory.db",
  );
}

function argumentValue(name: string): string | undefined {
  const index = process.argv.indexOf(name);
  return index >= 0 ? process.argv[index + 1] : undefined;
}

const customPath = argumentValue("--db");
if (process.argv.includes("--db") && !customPath) {
  throw new Error("Provide a path after --db.");
}

const databasePath = resolve(customPath ?? defaultDatabasePath());
mkdirSync(dirname(databasePath), { recursive: true });

const schemaPath = resolve(import.meta.dir, "../src-tauri/src/schema.sql");
const seedPath = resolve(import.meta.dir, "../src-tauri/src/seed.sql");
if (!existsSync(schemaPath) || !existsSync(seedPath)) {
  throw new Error("Schema or seed SQL file not found.");
}

const db = new Database(databasePath, { create: true });
try {
  db.exec("PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 10000;");
  db.exec(readFileSync(schemaPath, "utf8"));

  const counts = db
    .query<{
      products: number;
      movements: number;
      categories: number;
      suppliers: number;
      persons: number;
    }, []>(`
      SELECT
        (SELECT count(*) FROM products) AS products,
        (SELECT count(*) FROM movements) AS movements,
        (SELECT count(*) FROM categories) AS categories,
        (SELECT count(*) FROM suppliers) AS suppliers,
        (SELECT count(*) FROM responsible_persons) AS persons
    `)
    .get();

  if (!counts) throw new Error("Could not inspect the database.");
  if (Object.values(counts).some((count) => count > 0)) {
    console.log(`Seed skipped: database already contains inventory data.\n${databasePath}`);
  } else {
    const insertSeed = db.transaction(() => {
      db.exec(readFileSync(seedPath, "utf8"));
    });
    insertSeed.immediate();

    const foreignKeyErrors = db.query("PRAGMA foreign_key_check").all();
    if (foreignKeyErrors.length) {
      throw new Error("Seed failed foreign-key validation.");
    }

    console.log(`Demo data inserted successfully.\n${databasePath}`);
  }
} finally {
  db.close();
}
