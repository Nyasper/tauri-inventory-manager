<script lang="ts">
  import { page } from "$app/state";
  import { afterNavigate } from "$app/navigation";
  import { Sun, Moon, ArrowLeft, ArrowRight } from "@lucide/svelte";
  import { themeService } from "$lib/services/theme.service.svelte";

  const titles: Record<string, string> = {
    "/": "Dashboard",
    "/products": "Productos",
    "/movements": "Movimientos",
    "/suppliers": "Proveedores",
  };

  const title = $derived.by(() => {
    const path = page.url.pathname;
    for (const [key, value] of Object.entries(titles)) {
      const match = key === "/" ? path === "/" : path.startsWith(key);
      if (match) return value;
    }
    return "Gestor de Inventario";
  });

  let canGoBack = $state(false);
  let canGoForward = $state(false);

  function updateNavState() {
    if (typeof window === "undefined") return;
    const nav = (
      window as unknown as {
        navigation?: { canGoBack?: boolean; canGoForward?: boolean };
      }
    ).navigation;
    if (nav && typeof nav.canGoBack === "boolean") {
      canGoBack = nav.canGoBack;
      canGoForward = nav.canGoForward ?? false;
    } else {
      canGoBack = window.history.length > 1;
      canGoForward = true;
    }
  }

  afterNavigate(() => {
    updateNavState();
  });

  function goBack() {
    if (typeof window !== "undefined") {
      window.history.back();
    }
  }

  function goForward() {
    if (typeof window !== "undefined") {
      window.history.forward();
    }
  }
</script>

<header class="header">
  <div class="title-wrap">
    <div class="nav-controls">
      <button
        class="nav-btn"
        type="button"
        title="Atrás"
        aria-label="Ir atrás en el historial"
        disabled={!canGoBack}
        onclick={goBack}
      >
        <ArrowLeft size={16} />
      </button>
      <button
        class="nav-btn"
        type="button"
        title="Adelante"
        aria-label="Ir adelante en el historial"
        disabled={!canGoForward}
        onclick={goForward}
      >
        <ArrowRight size={16} />
      </button>
    </div>
    <h1>{title}</h1>
  </div>

  <div class="actions">
    <button
      class="icon-btn"
      type="button"
      aria-label="Cambiar tema"
      onclick={() => themeService.toggle()}
    >
      {#if themeService.isDark}
        <Sun size={18} />
      {:else}
        <Moon size={18} />
      {/if}
    </button>
  </div>
</header>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
  }

  .title-wrap {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .nav-controls {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .nav-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s,
      border-color 0.15s,
      opacity 0.15s;
  }

  .nav-btn:hover:not(:disabled) {
    background: var(--surface-2);
    color: var(--text);
    border-color: var(--border-strong);
  }

  .nav-btn:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  h1 {
    font-size: 1.25rem;
  }

  .actions {
    display: flex;
    gap: 0.375rem;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 34px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  .icon-btn:hover {
    background: var(--surface-2);
    color: var(--text);
  }
</style>
