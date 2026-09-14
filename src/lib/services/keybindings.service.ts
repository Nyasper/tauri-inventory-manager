import { goto } from "$app/navigation";
import type { Component } from "svelte";
import HelpModal from "$lib/components/help/HelpModal.svelte";
import SettingsModal from "$lib/components/settings/SettingsModal.svelte";
import { modalService } from "$lib/services/modal.service.svelte";
import { themeService } from "$lib/services/theme.service.svelte";

type AnyComponent = Component<any, any, any>;

export type ShortcutId =
  | "help"
  | "settings"
  | "theme"
  | "view-dashboard"
  | "view-products"
  | "view-movements"
  | "view-suppliers";

export interface KeyCombo {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
}

export interface Shortcut {
  id: ShortcutId;
  keys: string;
  description: string;
  combo: KeyCombo;
  whenNoModal: boolean;
}

export const SHORTCUTS: Shortcut[] = [
  {
    id: "help",
    keys: "F1",
    description: "Abrir o cerrar el manual de ayuda",
    combo: { key: "F1" },
    whenNoModal: false,
  },
  {
    id: "settings",
    keys: "Ctrl + O",
    description: "Abrir o cerrar la configuración",
    combo: { key: "o", ctrl: true },
    whenNoModal: false,
  },
  {
    id: "theme",
    keys: "Ctrl + T",
    description: "Alternar entre modo claro y oscuro",
    combo: { key: "t", ctrl: true },
    whenNoModal: false,
  },
  {
    id: "view-dashboard",
    keys: "Ctrl + 1",
    description: "Ir al Dashboard",
    combo: { key: "1", ctrl: true },
    whenNoModal: true,
  },
  {
    id: "view-products",
    keys: "Ctrl + 2",
    description: "Ir a Productos",
    combo: { key: "2", ctrl: true },
    whenNoModal: true,
  },
  {
    id: "view-movements",
    keys: "Ctrl + 3",
    description: "Ir a Movimientos",
    combo: { key: "3", ctrl: true },
    whenNoModal: true,
  },
  {
    id: "view-suppliers",
    keys: "Ctrl + 4",
    description: "Ir a Proveedores",
    combo: { key: "4", ctrl: true },
    whenNoModal: true,
  },
];

const VIEW_ROUTES: Partial<Record<ShortcutId, string>> = {
  "view-dashboard": "/",
  "view-products": "/products",
  "view-movements": "/movements",
  "view-suppliers": "/suppliers",
};

function openEntryId(component: AnyComponent): number | null {
  const entry = modalService.modals.find((modal) => modal.component === component);
  return entry ? entry.id : null;
}

export function openHelp(): void {
  modalService.open(HelpModal, { shortcuts: SHORTCUTS });
}

export function openSettings(): void {
  modalService.open(SettingsModal);
}

function toggleHelp(): void {
  const id = openEntryId(HelpModal);
  if (id === null) openHelp();
  else modalService.close(id);
}

function toggleSettings(): void {
  const id = openEntryId(SettingsModal);
  if (id === null) openSettings();
  else modalService.close(id);
}

function toggleTheme(): void {
  themeService.toggle();
}

function navigate(id: ShortcutId): void {
  const path = VIEW_ROUTES[id];
  if (path) void goto(path);
}

const ACTIONS: Record<ShortcutId, () => void> = {
  help: toggleHelp,
  settings: toggleSettings,
  theme: toggleTheme,
  "view-dashboard": () => navigate("view-dashboard"),
  "view-products": () => navigate("view-products"),
  "view-movements": () => navigate("view-movements"),
  "view-suppliers": () => navigate("view-suppliers"),
};

function matches(event: KeyboardEvent, combo: KeyCombo): boolean {
  return (
    event.key.toLowerCase() === combo.key.toLowerCase() &&
    event.ctrlKey === !!combo.ctrl &&
    event.shiftKey === !!combo.shift &&
    event.altKey === !!combo.alt &&
    !event.metaKey
  );
}

class KeybindingsService {
  readonly shortcuts = SHORTCUTS;

  handleKeydown = (event: KeyboardEvent): void => {
    if (event.repeat || event.defaultPrevented) return;
    const shortcut = this.shortcuts.find(
      (item) =>
        matches(event, item.combo) &&
        (!item.whenNoModal || modalService.modals.length === 0),
    );
    if (!shortcut) return;
    event.preventDefault();
    ACTIONS[shortcut.id]();
  };
}

export const keybindingsService = new KeybindingsService();
