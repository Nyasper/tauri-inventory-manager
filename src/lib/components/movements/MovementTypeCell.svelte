<script lang="ts">
  import type { MovementType } from "$lib/types";
  import Badge from "$lib/components/ui/Badge.svelte";
  import Icon from "$lib/components/ui/Icon.svelte";

  let { type }: { type: MovementType } = $props();
</script>

{#if type === "in"}
  <Badge bg="var(--success)" color="#ffffff">
    <Icon name="arrowDown" size={13} /> Entrada
  </Badge>
{:else}
  <Badge bg="var(--danger)" color="#ffffff">
    <Icon name="arrowUp" size={13} /> Salida
  </Badge>
{/if}