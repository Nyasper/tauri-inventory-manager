import { untrack } from "svelte";
import { SvelteURL } from "svelte/reactivity";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { errorMessage } from "#lib/utils/errors.js";

/**
 * Mantiene la lista de registros archivados. Se activa con `?archived=1` y se
 * recarga al activar la casilla "Archivados".
 */
export class ArchivedList<T> {
  items = $state<T[]>([]);
  loading = $state(false);
  error = $state("");
  #showingArchived = $state(false);
  #request = 0;
  #load: (archivedOnly: boolean) => Promise<T[]>;

  constructor(load: (archivedOnly: boolean) => Promise<T[]>) {
    this.#load = load;
    $effect(() => {
      const active = page.url.searchParams.get("archived") === "1";
      untrack(() => this.#setShowingArchived(active));
    });
  }

  get showingArchived(): boolean {
    return this.#showingArchived;
  }

  set showingArchived(value: boolean) {
    this.#setShowingArchived(value);
    this.#syncUrl(value);
  }

  #setShowingArchived(value: boolean): void {
    if (this.#showingArchived === value) return;
    this.#showingArchived = value;
    if (value) void this.reload();
  }

  #syncUrl(value: boolean): void {
    const url = page.url;
    if ((url.searchParams.get("archived") === "1") === value) return;
    const next = new SvelteURL(url.href);
    if (value) next.searchParams.set("archived", "1");
    else next.searchParams.delete("archived");
    void goto(`${next.pathname}${next.search}`, { replaceState: true, reset: false });
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
