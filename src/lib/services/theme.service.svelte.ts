type Theme = "light" | "dark";

const STORAGE_KEY = "theme";

class ThemeService {
  theme = $state<Theme>("light");

  constructor() {
    this.theme = this.load();
    this.persistAndApply();
  }

  get isDark(): boolean {
    return this.theme === "dark";
  }

  toggle(): void {
    this.theme = this.isDark ? "light" : "dark";
    this.persistAndApply();
  }

  private load(): Theme {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      return stored === "dark" ? "dark" : "light";
    } catch {
      return "light";
    }
  }

  private persistAndApply(): void {
    try {
      localStorage.setItem(STORAGE_KEY, this.theme);
    } catch {
      /* ignore */
    }
    document.documentElement.setAttribute("data-theme", this.theme);
  }
}

export const themeService = new ThemeService();
