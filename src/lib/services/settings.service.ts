import { invoke } from "$lib/api";
import type { AppSettings, DefaultView } from "$lib/types";
import { getDb } from "$lib/db";

const DEFAULTS: AppSettings = {
  stock_low_threshold: 5,
  stock_medium_threshold: 20,
  default_view: "dashboard",
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
      default_view: this.view(map.get("default_view"), DEFAULTS.default_view),
    };
  }

  async setThresholds(low: number, medium: number): Promise<void> {
    await invoke("save_settings", { low, medium });
  }

  async setDefaultView(view: DefaultView): Promise<void> {
    await invoke("save_settings", { view });
  }

  private num(value: string | undefined, fallback: number): number {
    const n = Number(value);
    return Number.isFinite(n) ? n : fallback;
  }

  private view(value: string | undefined, fallback: DefaultView): DefaultView {
    if (
      value === "dashboard" ||
      value === "products" ||
      value === "movements" ||
      value === "suppliers"
    ) {
      return value;
    }
    return fallback;
  }
}

export const settingsService = new SettingsService();
