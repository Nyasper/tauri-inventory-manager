import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { feedback } from "$lib/services/feedback.service.svelte";

/**
 * Envoltorio de `invoke` que muestra un aviso de éxito cuando el llamador lo
 * indica. Los comandos sin mensaje (por ejemplo, respaldos) no generan toast.
 */
export async function invoke<T>(
  command: string,
  args?: Record<string, unknown>,
  successMessage?: string,
): Promise<T> {
  const result = await tauriInvoke<T>(command, args);
  if (successMessage) feedback.success(successMessage);
  return result;
}
