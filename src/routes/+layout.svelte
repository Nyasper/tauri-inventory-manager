<script lang="ts">
  import "../app.css";
  import { feedback } from "$lib/services/feedback.service.svelte";
  import { errorMessage } from "$lib/utils/errors";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import type { Snippet } from "svelte";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import ModalHost from "$lib/components/ui/ModalHost.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { categoriesService } from "$lib/services/categories.service.svelte";
  import { settingsService } from "$lib/services/settings.service.svelte";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { takeStartupNotice } from "$lib/services/db.service";
  import { keybindingsService } from "$lib/services/keybindings.service";

  const VIEW_PATHS: Record<string, string> = {
    dashboard: "/",
    products: "/products",
    movements: "/movements",
    suppliers: "/suppliers",
  };

  let { children }: { children: Snippet } = $props();

  categoriesService.load();
  suppliersService.load();

  function handleError(event: PromiseRejectionEvent) {
    event.preventDefault();
    feedback.error(errorMessage(event.reason));
  }

  function handleContextMenu(event: MouseEvent) {
    const target = event.target as HTMLElement | null;
    if (target?.closest("input, textarea, [contenteditable='true']")) return;
    event.preventDefault();
  }

  onMount(async () => {
    const notice = await takeStartupNotice().catch(() => null);
    if (notice) feedback.error(notice);
    await settingsService.load();
    const path = VIEW_PATHS[settingsService.settings.default_view] ?? "/";
    if (page.url.pathname === "/" && !page.url.search && path !== "/") goto(path);
  });
</script>

<svelte:window
  onunhandledrejection={handleError}
  onkeydown={keybindingsService.handleKeydown}
  oncontextmenu={handleContextMenu}
/>

<div class="app" inert={modalService.modals.length > 0}>
  <Sidebar />
  <div class="main">
    <Header />
    <main class="content">
      {@render children()}
    </main>
  </div>
</div>

<ModalHost />
{#if feedback.message}
  <div class={["toast", { error: feedback.isError }]} role={feedback.isError ? "alert" : "status"}>
    <span class="toast-message">{feedback.message}</span>
    <button aria-label="Cerrar aviso" onclick={() => (feedback.message = "")}>×</button>
  </div>
{/if}

<style>
  .app {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }
</style>
