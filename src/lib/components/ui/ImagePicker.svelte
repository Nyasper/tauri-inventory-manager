<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import Input from "./Input.svelte";
  import Button from "./Button.svelte";
  import ResolvedImage from "./ResolvedImage.svelte";

  let {
    label = "Imagen",
    value = $bindable<string | null>(null),
  }: {
    label?: string;
    value?: string | null;
  } = $props();

  let url = $state("");

  async function pickFile() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Imágenes",
          extensions: ["png", "jpg", "jpeg", "webp", "gif", "svg", "bmp"],
        },
      ],
    });
    if (typeof selected === "string") {
      try {
        value = await invoke<string>("save_product_image", {
          sourcePath: selected,
        });
      } catch (error) {
        console.error("Failed to save image", error);
      }
    }
  }

  function applyUrl() {
    const trimmed = url.trim();
    if (trimmed) value = trimmed;
  }

  function clear() {
    value = null;
    url = "";
  }
</script>

{#if label}
  <span class="field-label">{label}</span>
{/if}

<div class="picker">
  <ResolvedImage src={value} size={64} />

  <div class="controls">
    <div class="url-row">
      <Input placeholder="https://..." bind:value={url} />
      <Button variant="secondary" size="sm" onclick={applyUrl}>URL</Button>
    </div>
    <div class="btn-row">
      <Button variant="secondary" size="sm" icon="image" onclick={pickFile}>
        Elegir archivo
      </Button>
      {#if value}
        <Button variant="ghost" size="sm" onclick={clear}>Quitar</Button>
      {/if}
    </div>
  </div>
</div>

<style>
  .field-label {
    display: block;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text-muted);
  }

  .picker {
    display: flex;
    gap: 0.75rem;
    align-items: flex-start;
  }

  .controls {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex: 1;
  }

  .url-row {
    display: flex;
    gap: 0.5rem;
  }

  .btn-row {
    display: flex;
    gap: 0.5rem;
  }
</style>
