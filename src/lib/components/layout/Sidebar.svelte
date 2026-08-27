<script lang="ts">
  import { page } from "$app/state";
  import Icon from "$lib/components/ui/Icon.svelte";
  import type { IconName } from "$lib/components/ui/Icon.svelte";

  const nav: { href: string; label: string; icon: IconName }[] = [
    { href: "/", label: "Productos", icon: "box" },
    { href: "/movements", label: "Movimientos", icon: "swap" },
    { href: "/suppliers", label: "Proveedores", icon: "truck" },
  ];

  function isActive(href: string): boolean {
    const path = page.url.pathname;
    return href === "/" ? path === "/" : path.startsWith(href);
  }
</script>

<aside class="sidebar">
  <div class="brand">
    <span class="brand-icon">
      <img src="/clean-icon.svg" alt="Inventory Manager" />
    </span>
    <span class="brand-name">Inventory</span>
  </div>

  <nav class="nav">
    {#each nav as item (item.href)}
      <a href={item.href} class="nav-item" class:active={isActive(item.href)}>
        <Icon name={item.icon} size={18} />
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    flex-shrink: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border-right: 1px solid var(--border);
    padding: 1rem 0.75rem;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.25rem 0.625rem 1rem;
  }

  .brand-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 34px;
    border-radius: var(--radius-sm);
    background: #ffffff;
    border: 1px solid var(--border);
    overflow: hidden;
    flex-shrink: 0;
  }

  .brand-icon img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: contain;
    padding: 4px;
  }

  .brand-name {
    font-weight: 600;
    font-size: 1rem;
  }

  .nav {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0.625rem;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    font-weight: 500;
    transition:
      background-color 0.15s,
      color 0.15s;
  }

  .nav-item:hover {
    background: var(--surface-2);
    color: var(--text);
  }

  .nav-item.active {
    background: var(--primary-soft);
    color: var(--primary);
  }
</style>
