<script lang="ts">
  import type { MovementType } from "$lib/types";
  import Badge from "$lib/components/ui/Badge.svelte";
  import { ArrowDownToLine, ArrowUpFromLine } from "@lucide/svelte";

  let { type }: { type: MovementType } = $props();
</script>

{#if type === "in"}
  <Badge fixedWidth background="var(--success)" textColor="#ffffff">
    <ArrowDownToLine size={13} /> Entrada
  </Badge>
{:else}
  <Badge fixedWidth background="var(--danger)" textColor="#ffffff">
    <ArrowUpFromLine size={13} /> Salida
  </Badge>
{/if}
