import type { AppSettings } from "$lib/types";
import { getDb } from "$lib/db";

const DEFAULTS: AppSettings = {
  stock_low_threshold: 5,
  stock_medium_threshold: 20,
};

class SettingsService {
  async get(): Promise<AppSettings> {
    const db = await getDb();
    const rows = await db.select<{ key: string; value: string }[]>(
      "SELECT key, value FROM settings",
    );
    const map = new Map(rows.map((r) => [r.key, r.value]));

    return {
      stock_low_threshold: this.num(map.get("stock_low_threshold"), DEFAULTS.stock_low_threshold),
      stock_medium_threshold: this.num(map.get("stock_medium_threshold"), DEFAULTS.stock_medium_threshold),
    };
  }

  async setThresholds(low: number, medium: number): Promise<void> {
    await this.set("stock_low_threshold", String(low));
    await this.set("stock_medium_threshold", String(medium));
  }

  private async set(key: string, value: string): Promise<void> {
    const db = await getDb();
    await db.execute(
      "INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
      [key, value],
    );
  }

  private num(value: string | undefined, fallback: number): number {
    const n = Number(value);
    return Number.isFinite(n) ? n : fallback;
  }
}

export const settingsService = new SettingsService();
