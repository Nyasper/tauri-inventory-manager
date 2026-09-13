import { page } from "$app/state";

/**
 * Mantiene la lista de registros archivados. Se activa con `?archived=1` y se
 * recarga al activar la casilla "Archivados".
 */
export class ArchivedList<T> {
  items = $state<T[]>([]);
  loading = $state(false);
  error = $state("");
  #active = $state(false);
  #request = 0;
  #load: (archived: boolean) => Promise<T[]>;

  constructor(load: (archived: boolean) => Promise<T[]>) {
    this.#load = load;
    $effect(() => {
      this.active = page.url.searchParams.get("archived") === "1";
    });
  }

  get active(): boolean {
    return this.#active;
  }

  set active(value: boolean) {
    if (this.#active === value) return;
    this.#active = value;
    if (value) void this.reload();
  }

  async reload(): Promise<void> {
    const request = ++this.#request;
    this.loading = true;
    this.error = "";
    try {
      const items = await this.#load(true);
      if (request !== this.#request) return;
      this.items = items;
    } catch (cause) {
      if (request !== this.#request) return;
      this.error = String(cause);
    } finally {
      if (request === this.#request) this.loading = false;
    }
  }
}
