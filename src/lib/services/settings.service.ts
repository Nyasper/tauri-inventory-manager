import { invoke } from "$lib/api";
import type { AppSettings, DefaultView } from "$lib/types";

class SettingsService {
  get(): Promise<AppSettings> {
    return invoke<AppSettings>("get_settings");
  }

  setThresholds(low: number, medium: number): Promise<void> {
    return invoke("save_settings", { low, medium }, "Umbrales guardados.");
  }

  setDefaultView(view: DefaultView): Promise<void> {
    return invoke(
      "save_settings",
      { view },
      "Vista por defecto actualizada.",
    );
  }
}

export const settingsService = new SettingsService();
