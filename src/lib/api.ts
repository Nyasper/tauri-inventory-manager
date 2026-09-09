import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { feedback } from "$lib/stores/feedback.svelte";
export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const result = await tauriInvoke<T>(command, args);
  feedback.success(command === "cancel_movement" ? "Movimiento anulado." : command === "archive_record" ? (args?.restore ? "Registro restaurado." : "Operación completada.") : "Cambios guardados.");
  return result;
}
