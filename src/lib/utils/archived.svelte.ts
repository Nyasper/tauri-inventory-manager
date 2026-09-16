import { untrack } from "svelte";
import { SvelteURL } from "svelte/reactivity";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { errorMessage } from "$lib/utils/errors";

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
      const active = page.url.searchParams.get("archived") === "1";
      untrack(() => this.#setActive(active));
    });
  }

  get active(): boolean {
    return this.#active;
  }

  set active(value: boolean) {
    this.#setActive(value);
    this.#syncUrl(value);
  }

  #setActive(value: boolean): void {
    if (this.#active === value) return;
    this.#active = value;
    if (value) void this.reload();
  }

  #syncUrl(value: boolean): void {
    const url = page.url;
    if ((url.searchParams.get("archived") === "1") === value) return;
    const next = new SvelteURL(url);
    if (value) next.searchParams.set("archived", "1");
    else next.searchParams.delete("archived");
    void goto(`${next.pathname}${next.search}`, { replaceState: true, noScroll: true });
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
      this.error = errorMessage(cause);
    } finally {
      if (request === this.#request) this.loading = false;
    }
  }
}
