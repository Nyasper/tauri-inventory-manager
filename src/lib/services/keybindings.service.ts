import { goto } from "$app/navigation";
import HelpModal from "#lib/components/help/HelpModal.svelte";
import SettingsModal from "#lib/components/settings/SettingsModal.svelte";
import { modalService } from "#lib/services/modal.service.svelte.js";
import { themeService } from "#lib/services/theme.service.svelte.js";
import type { AnyComponent } from "#lib/types.js";

type ShortcutId =
  | "help"
  | "settings"
  | "theme"
  | "view-dashboard"
  | "view-products"
  | "view-movements"
  | "view-suppliers";

interface KeyCombo {
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
  requiresNoOpenModals: boolean;
}

const SHORTCUTS: Shortcut[] = [
  {
    id: "help",
    keys: "F1",
    description: "Abrir o cerrar el manual de ayuda",
    combo: { key: "F1" },
    requiresNoOpenModals: false,
  },
  {
    id: "settings",
    keys: "Ctrl + O",
    description: "Abrir o cerrar la configuración",
    combo: { key: "o", ctrl: true },
    requiresNoOpenModals: false,
  },
  {
    id: "theme",
    keys: "Ctrl + T",
    description: "Alternar entre modo claro y oscuro",
    combo: { key: "t", ctrl: true },
    requiresNoOpenModals: false,
  },
  {
    id: "view-dashboard",
    keys: "Ctrl + 1",
    description: "Ir al Dashboard",
    combo: { key: "1", ctrl: true },
    requiresNoOpenModals: true,
  },
  {
    id: "view-products",
    keys: "Ctrl + 2",
    description: "Ir a Productos",
    combo: { key: "2", ctrl: true },
    requiresNoOpenModals: true,
  },
  {
    id: "view-movements",
    keys: "Ctrl + 3",
    description: "Ir a Movimientos",
    combo: { key: "3", ctrl: true },
    requiresNoOpenModals: true,
  },
  {
    id: "view-suppliers",
    keys: "Ctrl + 4",
    description: "Ir a Proveedores",
    combo: { key: "4", ctrl: true },
    requiresNoOpenModals: true,
  },
];

const VIEW_ROUTES: Partial<Record<ShortcutId, string>> = {
  "view-dashboard": "/",
  "view-products": "/products",
  "view-movements": "/movements",
  "view-suppliers": "/suppliers",
};

function findOpenModalId(component: AnyComponent): number | null {
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
  const id = findOpenModalId(HelpModal);
  if (id === null) openHelp();
  else void modalService.requestClose(id);
}

function toggleSettings(): void {
  const id = findOpenModalId(SettingsModal);
  if (id === null) openSettings();
  else void modalService.requestClose(id);
}

function toggleTheme(): void {
  themeService.toggle();
}

function navigateToView(id: ShortcutId): void {
  const path = VIEW_ROUTES[id];
  if (path) void goto(path);
}

const ACTIONS: Record<ShortcutId, () => void> = {
  help: toggleHelp,
  settings: toggleSettings,
  theme: toggleTheme,
  "view-dashboard": () => navigateToView("view-dashboard"),
  "view-products": () => navigateToView("view-products"),
  "view-movements": () => navigateToView("view-movements"),
  "view-suppliers": () => navigateToView("view-suppliers"),
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
        (!item.requiresNoOpenModals || modalService.modals.length === 0),
    );
    if (!shortcut) return;
    event.preventDefault();
    ACTIONS[shortcut.id]();
  };
}

export const keybindingsService = new KeybindingsService();
