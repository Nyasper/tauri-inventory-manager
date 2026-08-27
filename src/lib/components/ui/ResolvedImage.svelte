<script lang="ts">
  import { resolveImageUrl } from "$lib/utils/image";
  import Icon from "./Icon.svelte";

  let {
    src,
    alt = "",
    size = 48,
  }: {
    src: string | null;
    alt?: string;
    size?: number;
  } = $props();

  let resolved = $state<string | null>(null);

  $effect(() => {
    let cancelled = false;
    resolved = null;
    resolveImageUrl(src).then((url) => {
      if (!cancelled) resolved = url;
    });
    return () => {
      cancelled = true;
    };
  });
</script>

{#if resolved}
  <img
    class="thumb"
    src={resolved}
    {alt}
    style:width="{size}px"
    style:height="{size}px"
  />
{:else}
  <div
    class="thumb placeholder"
    style:width="{size}px"
    style:height="{size}px"
  >
    <Icon name="box" size={Math.round(size * 0.5)} />
  </div>
{/if}

<style>
  .thumb {
    border-radius: var(--radius-sm);
    object-fit: cover;
    background: var(--surface-2);
    display: block;
    flex-shrink: 0;
  }

  .placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
  }
</style>
