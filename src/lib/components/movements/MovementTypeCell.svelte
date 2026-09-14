<script lang="ts">
  import type { MovementType } from "$lib/types";
  import Badge from "$lib/components/ui/Badge.svelte";
  import { ArrowDownToLine, ArrowUpFromLine } from "@lucide/svelte";

  let { type }: { type: MovementType } = $props();
</script>

{#if type === "in"}
  <Badge uniform bg="var(--success)" color="#ffffff">
    <ArrowDownToLine size={13} /> Entrada
  </Badge>
{:else}
  <Badge uniform bg="var(--danger)" color="#ffffff">
    <ArrowUpFromLine size={13} /> Salida
  </Badge>
{/if}