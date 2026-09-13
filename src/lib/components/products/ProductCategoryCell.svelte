<script lang="ts">
  import type { Product } from "$lib/types";
  import ProductIcon from "$lib/components/ui/ProductIcon.svelte";

  let { product }: { product: Product } = $props();
</script>

{#if product.category_name}
  <span class="category">
    <ProductIcon
      icon={product.category_icon}
      color={product.category_color}
      size={22}
    />
    <span class="name" title={product.category_name}>
      {product.category_name}
    </span>
  </span>
{:else}
  <span class="muted">—</span>
{/if}

<style>
  .category {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .muted {
    color: var(--text-muted);
  }
</style>