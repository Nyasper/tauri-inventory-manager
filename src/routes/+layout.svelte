<script lang="ts">
  import "../app.css";
  import type { Snippet } from "svelte";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import ModalHost from "$lib/components/ui/ModalHost.svelte";
  import { categoriesStore } from "$lib/stores/categories.store.svelte";
  import { settingsStore } from "$lib/stores/settings.store.svelte";
  import { personsStore } from "$lib/stores/persons.store.svelte";
  import { suppliersStore } from "$lib/stores/suppliers.store.svelte";
  import { accentService } from "$lib/stores/accent.service.svelte";

  let { children }: { children: Snippet } = $props();

  accentService;
  categoriesStore.load();
  settingsStore.load();
  personsStore.load();
  suppliersStore.load();

  function preventContextMenu(event: MouseEvent) {
    event.preventDefault();
  }
</script>

<svelte:window oncontextmenu={preventContextMenu} />

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
