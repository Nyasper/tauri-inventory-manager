<script lang="ts">
  import type { Product } from "$lib/types";

  let { product }: { product: Product } = $props();
</script>

<span class="name" title={product.name}>{product.name}</span>
{#if product.description}
  <span class="desc" title={product.description}>{product.description}</span>
{/if}

<style>
  .name,
  .desc {
    display: block;
  }

  .name {
    font-weight: 600;
  }

  .desc {
    font-size: 0.75rem;
    color: var(--text-muted);
    max-width: 320px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>