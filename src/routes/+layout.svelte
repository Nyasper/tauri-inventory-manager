<script lang="ts">
  import "../app.css";
  import { feedback } from "$lib/stores/feedback.svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import type { Snippet } from "svelte";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import ModalHost from "$lib/components/ui/ModalHost.svelte";
  import { categoriesStore } from "$lib/stores/categories.store.svelte";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import { personsStore } from "$lib/stores/persons.store.svelte";
  import { suppliersStore } from "$lib/stores/suppliers.store.svelte";

  import { modalService } from "$lib/stores/modal.service.svelte";
  import HelpModal from "$lib/components/help/HelpModal.svelte";

  const VIEW_PATHS: Record<string, string> = {
    dashboard: "/",
    products: "/products",
    movements: "/movements",
    suppliers: "/suppliers",
  };

  let { children }: { children: Snippet } = $props();

  categoriesStore.load();
  personsStore.load();
  suppliersStore.load();

  function handleError(event: PromiseRejectionEvent) {
    event.preventDefault(); feedback.error(String(event.reason));
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "F1") {
      event.preventDefault();
      const isOpen = modalService.modals.some((m) => m.component === HelpModal);
      if (isOpen) {
        modalService.close();
      } else {
        modalService.open(HelpModal);
      }
    }
  }

  onMount(async () => {
    await settingsStore.load();
    const path = VIEW_PATHS[settingsStore.settings.default_view] ?? "/";
    if (page.url.pathname === "/" && !page.url.search && path !== "/") goto(path);
  });
</script>

<svelte:window onunhandledrejection={handleError} onkeydown={handleKeydown} />

<div class="app">
  <Sidebar />
  <div class="main">
    <Header />
    <main class="content">
      {@render children()}
    </main>
  </div>
</div>

<ModalHost />
{#if feedback.message}<div class="toast" class:error={feedback.isError} role={feedback.isError ? "alert" : "status"}><span class="toast-message">{feedback.message}</span><button aria-label="Cerrar aviso" onclick={()=>feedback.message=""}>×</button></div>{/if}

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
