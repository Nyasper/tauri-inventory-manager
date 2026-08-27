import type { AccentColor } from "$lib/types";

export interface Accent {
  id: AccentColor;
  label: string;
  light: { primary: string; hover: string; soft: string };
  dark: { primary: string; hover: string; soft: string };
}

export const ACCENTS: Accent[] = [
  {
    id: "blue",
    label: "Azul",
    light: { primary: "#2563eb", hover: "#1d4ed8", soft: "#eff4ff" },
    dark: { primary: "#3b82f6", hover: "#2563eb", soft: "#1e2a44" },
  },
  {
    id: "violet",
    label: "Violeta",
    light: { primary: "#7c3aed", hover: "#6d28d9", soft: "#f3e8ff" },
    dark: { primary: "#a78bfa", hover: "#8b5cf6", soft: "#2e1f4d" },
  },
  {
    id: "emerald",
    label: "Esmeralda",
    light: { primary: "#059669", hover: "#047857", soft: "#ecfdf5" },
    dark: { primary: "#34d399", hover: "#10b981", soft: "#0f2e24" },
  },
  {
    id: "amber",
    label: "Ámbar",
    light: { primary: "#d97706", hover: "#b45309", soft: "#fffbeb" },
    dark: { primary: "#fbbf24", hover: "#f59e0b", soft: "#3b2f14" },
  },
  {
    id: "rose",
    label: "Rosa",
    light: { primary: "#e11d48", hover: "#be123c", soft: "#fff1f2" },
    dark: { primary: "#fb7185", hover: "#f43f5e", soft: "#3b1f2b" },
  },
  {
    id: "teal",
    label: "Teal",
    light: { primary: "#0d9488", hover: "#0f766e", soft: "#f0fdfa" },
    dark: { primary: "#2dd4bf", hover: "#14b8a6", soft: "#0f2f2c" },
  },
  {
    id: "slate",
    label: "Gris",
    light: { primary: "#475569", hover: "#334155", soft: "#e2e8f0" },
    dark: { primary: "#94a3b8", hover: "#64748b", soft: "#26303b" },
  },
];

const KEY = "accent";

class AccentService {
  accent = $state<AccentColor>("blue");

  constructor() {
    this.accent = this.load();
    this.apply();
  }

  setAccent(accent: AccentColor): void {
    this.accent = accent;
    this.apply();
  }

  private load(): AccentColor {
    try {
      const stored = localStorage.getItem(KEY);
      return ACCENTS.some((a) => a.id === stored) ? (stored as AccentColor) : "blue";
    } catch {
      return "blue";
    }
  }

  private apply(): void {
    try {
      localStorage.setItem(KEY, this.accent);
    } catch {
      /* ignore */
    }
    document.documentElement.setAttribute("data-accent", this.accent);
  }
}

export const accentService = new AccentService();