<script lang="ts">
  import { page } from "$app/state";
  import Icon from "$lib/components/ui/Icon.svelte";
  import { themeService } from "$lib/stores/theme.service.svelte";
  import { modalService } from "$lib/stores/modal.service.svelte";
  import SettingsModal from "$lib/components/settings/SettingsModal.svelte";

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
    return "Inventory Manager";
  });

  function openSettings() {
    modalService.open(SettingsModal);
  }
</script>

<header class="header">
  <h1>{title}</h1>
  <div class="actions">
    <button
      class="icon-btn"
      type="button"
      aria-label="Cambiar tema"
      onclick={() => themeService.toggle()}
    >
      <Icon name={themeService.isDark ? "sun" : "moon"} size={18} />
    </button>
    <button
      class="icon-btn"
      type="button"
      aria-label="Configuración"
      onclick={openSettings}
    >
      <Icon name="settings" size={18} />
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
