import { getVersion } from "@tauri-apps/api/app";

/** Versión de la aplicación definida en `tauri.conf.json`. */
export async function getAppVersion(): Promise<string> {
  try {
    return await getVersion();
  } catch {
    return "";
  }
}
