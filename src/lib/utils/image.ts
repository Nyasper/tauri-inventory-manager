import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir } from "@tauri-apps/api/path";

export function isRemoteUrl(value: string): boolean {
  return /^(https?:\/\/|data:)/.test(value);
}

export async function resolveImageUrl(
  value: string | null,
): Promise<string | null> {
  if (!value) return null;
  if (isRemoteUrl(value)) return value;
  const dir = await appDataDir();
  return convertFileSrc(dir + value);
}
