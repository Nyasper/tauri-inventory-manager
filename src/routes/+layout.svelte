<script lang="ts">
  import "../app.css";
  import { feedbackService } from "#lib/services/feedback.service.svelte.js";
  import { errorMessage } from "#lib/utils/errors.js";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import type { Snippet } from "svelte";
  import Sidebar from "#lib/components/layout/Sidebar.svelte";
  import Header from "#lib/components/layout/Header.svelte";
  import ModalHost from "#lib/components/ui/ModalHost.svelte";
  import { modalService } from "#lib/services/modal.service.svelte.js";
  import { categoriesService } from "#lib/services/categories.service.svelte.js";
  import { settingsService } from "#lib/services/settings.service.svelte.js";
  import { suppliersService } from "#lib/services/suppliers.service.svelte.js";
  import { takeStartupNotice } from "#lib/services/db.service.js";
  import { keybindingsService } from "#lib/services/keybindings.service.js";
  import { VIEW_ROUTES } from "#lib/navigation.js";

  let { children }: { children: Snippet } = $props();

  categoriesService.load();
  suppliersService.load();

  function handleError(event: PromiseRejectionEvent) {
    event.preventDefault();
    feedbackService.error(errorMessage(event.reason));
  }

  function handleContextMenu(event: MouseEvent) {
    const target = event.target as HTMLElement | null;
    if (target?.closest("input, textarea, [contenteditable='true']")) return;
    event.preventDefault();
  }

  onMount(async () => {
    const notice = await takeStartupNotice().catch(() => null);
    if (notice) feedbackService.error(notice);
    try {
      await settingsService.load();
    } catch (e) {
      feedbackService.error(errorMessage(e));
      return;
    }
    const path = VIEW_ROUTES[settingsService.settings.default_view] ?? "/";
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
{#if feedbackService.message}
  <div
    class={["toast", { error: feedbackService.isError }]}
    role={feedbackService.isError ? "alert" : "status"}
  >
    <span class="toast-message">{feedbackService.message}</span>
    <button aria-label="Cerrar aviso" onclick={() => (feedbackService.message = "")}>×</button>
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
