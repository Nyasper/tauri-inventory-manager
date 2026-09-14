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
  } from "@lucide/svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import HelpModal from "$lib/components/help/HelpModal.svelte";

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

  function openHelp() {
    modalService.open(HelpModal);
  }
</script>

<aside class="sidebar">
  <div class="brand">
    <span class="brand-icon">
      <Boxes size={18} />
    </span>
    <span class="brand-name">Inventory</span>
  </div>

  <nav class="nav">
    {#each nav as item (item.href)}
      {@const Icon = item.icon}
      <a href={item.href} class="nav-item" class:active={isActive(item.href)}>
        <Icon size={18} />
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>

  <div class="spacer"></div>

  <div class="footer">
    <button
      class="help-btn"
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
    border-radius: var(--radius-sm);
    background: linear-gradient(135deg, var(--primary), var(--primary-hover));
    color: #ffffff;
    flex-shrink: 0;
    box-shadow: 0 2px 6px rgba(16, 24, 40, 0.18);
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
    align-items: center;
    padding-top: 0.75rem;
    border-top: 1px solid var(--border);
  }

  .help-btn {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    width: 100%;
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

  .help-btn:hover {
    background: var(--surface-2);
    color: var(--text);
  }
</style>
