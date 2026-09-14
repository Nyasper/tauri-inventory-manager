import { invoke } from "$lib/api";
import type { AppSettings, DefaultView } from "$lib/types";

class SettingsService {
  settings = $state<AppSettings>({
    stock_low_threshold: 5,
    stock_medium_threshold: 20,
    default_view: "dashboard",
  });

  async load(): Promise<void> {
    this.settings = await invoke<AppSettings>("get_settings");
  }

  async saveThresholds(low: number, medium: number): Promise<void> {
    await invoke("save_settings", { low, medium }, "Umbrales guardados.");
    this.settings.stock_low_threshold = low;
    this.settings.stock_medium_threshold = medium;
  }

  async saveDefaultView(view: DefaultView): Promise<void> {
    await invoke("save_settings", { view }, "Vista por defecto actualizada.");
    this.settings.default_view = view;
  }
}

export const settingsService = new SettingsService();