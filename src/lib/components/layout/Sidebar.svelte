<script lang="ts">
  import { page } from "$app/state";
  import type { Component } from "svelte";
  import type { LucideProps } from "@lucide/svelte";
  import {
    Boxes,
    LayoutDashboard,
    ArrowLeftRight,
    Truck,
    CircleHelp,
    Settings,
  } from "@lucide/svelte";
  import { openHelp, openSettings } from "$lib/services/keybindings.service";

  const nav: { href: string; label: string; icon: Component<LucideProps> }[] = [
    { href: "/", label: "Dashboard", icon: LayoutDashboard },
    { href: "/products", label: "Productos", icon: Boxes },
    { href: "/movements", label: "Movimientos", icon: ArrowLeftRight },
    { href: "/suppliers", label: "Proveedores", icon: Truck },
  ];

  function isActive(href: string): boolean {
    const path = page.url.pathname;
    return href === "/" ? path === "/" : path.startsWith(href);
  }
</script>

<aside class="sidebar">
  <div class="brand">
    <span class="brand-icon" aria-hidden="true">
      <svg viewBox="0 0 24 24" fill="none">
        <path
          d="M15.5777 3.38197L17.5777 4.43152C19.7294 5.56066 20.8052 6.12523 21.4026 7.13974C22 8.15425 22 9.41667 22 11.9415V12.0585C22 14.5833 22 15.8458 21.4026 16.8603C20.8052 17.8748 19.7294 18.4393 17.5777 19.5685L15.5777 20.618C13.8221 21.5393 12.9443 22 12 22C11.0557 22 10.1779 21.5393 8.42229 20.618L6.42229 19.5685C4.27063 18.4393 3.19479 17.8748 2.5974 16.8603C2 15.8458 2 14.5833 2 12.0585V11.9415C2 9.41667 2 8.15425 2.5974 7.13974C3.19479 6.12523 4.27063 5.56066 6.42229 4.43152L8.42229 3.38197C10.1779 2.46066 11.0557 2 12 2C12.9443 2 13.8221 2.46066 15.5777 3.38197Z"
          stroke="currentColor"
          stroke-width="1.44"
          stroke-linecap="round"
        />
        <path
          d="M21 7.5L17 9.5M12 12L3 7.5M12 12V21.5M12 12C12 12 14.7426 10.6287 16.5 9.75C16.6953 9.65237 17 9.5 17 9.5M17 9.5V13M17 9.5L7.5 4.5"
          stroke="currentColor"
          stroke-width="1.44"
          stroke-linecap="round"
        />
      </svg>
    </span>
    <span class="brand-name">Inventario</span>
  </div>

  <nav class="nav">
    {#each nav as item (item.href)}
      {const Icon = item.icon}
      <a href={item.href} class={["nav-item", { active: isActive(item.href) }]}>
        <Icon size={18} />
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>

  <div class="spacer"></div>

  <div class="footer">
    <button
      class="footer-btn"
      type="button"
      aria-label="Configuración"
      title="Configuración"
      onclick={openSettings}
    >
      <Settings size={18} />
      <span>Ajustes</span>
    </button>
    <button
      class="footer-btn"
      type="button"
      aria-label="Manual de usuario y ayuda"
      title="Manual de usuario y ayuda"
      onclick={openHelp}
    >
      <CircleHelp size={18} />
      <span>Ayuda</span>
    </button>
  </div>
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
    color: var(--primary);
    flex-shrink: 0;
  }

  .brand-icon svg {
    display: block;
    width: 24px;
    height: 24px;
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

  .spacer {
    flex: 1;
  }

  .footer {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--border);
  }

  .footer-btn {
    display: flex;
    width: 100%;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0.625rem;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    font-weight: 500;
    font-size: 0.875rem;
    cursor: pointer;
    transition:
      background-color 0.15s,
      color 0.15s;
    text-align: left;
  }

  .footer-btn:hover {
    background: var(--surface-2);
    color: var(--text);
  }
</style>
