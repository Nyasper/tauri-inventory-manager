import type { AppSettings, DefaultView } from "$lib/types";
import {
  getSettings,
  saveSettingsDefaultView,
  saveSettingsPageSize,
  saveSettingsThresholds,
} from "$lib/services/db.service";

class SettingsService {
  settings = $state<AppSettings>({
    stock_low_threshold: 5,
    stock_medium_threshold: 20,
    default_view: "dashboard",
    movements_page_size: 50,
  });

  async load(): Promise<void> {
    this.settings = await getSettings();
  }

  async saveThresholds(low: number, medium: number): Promise<void> {
    await saveSettingsThresholds(low, medium);
    this.settings.stock_low_threshold = low;
    this.settings.stock_medium_threshold = medium;
  }

  async saveDefaultView(view: DefaultView): Promise<void> {
    await saveSettingsDefaultView(view);
    this.settings.default_view = view;
  }

  async savePageSize(pageSize: number): Promise<void> {
    await saveSettingsPageSize(pageSize);
    this.settings.movements_page_size = pageSize;
  }
}

export const settingsService = new SettingsService();
