type Theme = "light" | "dark";

const KEY = "theme";

class ThemeService {
  theme = $state<Theme>("light");

  constructor() {
    this.theme = this.load();
    this.apply();
  }

  get isDark(): boolean {
    return this.theme === "dark";
  }

  toggle(): void {
    this.theme = this.isDark ? "light" : "dark";
    this.apply();
  }

  private load(): Theme {
    try {
      const stored = localStorage.getItem(KEY);
      return stored === "dark" ? "dark" : "light";
    } catch {
      return "light";
    }
  }

  private apply(): void {
    try {
      localStorage.setItem(KEY, this.theme);
    } catch {
      /* ignore */
    }
    document.documentElement.setAttribute("data-theme", this.theme);
  }
}

export const themeService = new ThemeService();
