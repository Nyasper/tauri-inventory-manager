import type { AppSettings, DefaultView } from "$lib/types";
import {
  getSettings,
  saveSettingsDefaultView,
  saveSettingsThresholds,
} from "$lib/services/db.service";

class SettingsService {
  settings = $state<AppSettings>({
    stock_low_threshold: 5,
    stock_medium_threshold: 20,
    default_view: "dashboard",
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
}

export const settingsService = new SettingsService();
