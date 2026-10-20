<script lang="ts">
  import type { StockStatus } from "#lib/types.js";
  import { STOCK_LABELS } from "#lib/utils/stock.js";
  import Badge from "./Badge.svelte";
  import { BellOff } from "@lucide/svelte";

  let {
    status,
    title,
    ignored = false,
  }: {
    status: StockStatus;
    title?: string;
    ignored?: boolean;
  } = $props();

  const styles: Record<StockStatus, { bg: string; color: string }> = {
    out: { bg: "var(--stock-out)", color: "#ffffff" },
    low: { bg: "var(--danger-soft)", color: "var(--danger)" },
    medium: { bg: "var(--warning-soft)", color: "var(--warning)" },
    high: { bg: "var(--surface-3)", color: "var(--text-muted)" },
  };
</script>

{#if ignored}
  <Badge background="var(--surface-3)" textColor="var(--text-muted)" {title}>
    <BellOff size={12} />
    <span>Sin alertas</span>
  </Badge>
{:else}
  <Badge background={styles[status].bg} textColor={styles[status].color} {title}>
    {STOCK_LABELS[status]}
  </Badge>
{/if}
