import type { AppSettings, DefaultView } from "$lib/types";
import { settingsService } from "$lib/services/settings.service";

class SettingsStore {
  settings = $state<AppSettings>({
    stock_low_threshold: 5,
    stock_medium_threshold: 20,
    default_view: "dashboard",
  });

  async load(): Promise<void> {
    this.settings = await settingsService.get();
  }

  async saveThresholds(low: number, medium: number): Promise<void> {
    await settingsService.setThresholds(low, medium);
    this.settings.stock_low_threshold = low;
    this.settings.stock_medium_threshold = medium;
  }

  async saveDefaultView(view: DefaultView): Promise<void> {
    await settingsService.setDefaultView(view);
    this.settings.default_view = view;
  }
}

export const settingsStore = new SettingsStore();
