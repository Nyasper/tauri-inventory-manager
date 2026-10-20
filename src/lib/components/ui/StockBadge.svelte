<script lang="ts">
  import type { StockStatus } from "$lib/types";
  import { STOCK_LABELS } from "$lib/utils/stock";
  import Badge from "./Badge.svelte";

  let { status, title }: { status: StockStatus; title?: string } = $props();

  const styles: Record<StockStatus, { bg: string; color: string }> = {
    out: { bg: "var(--stock-out)", color: "#ffffff" },
    low: { bg: "var(--danger-soft)", color: "var(--danger)" },
    medium: { bg: "var(--warning-soft)", color: "var(--warning)" },
    high: { bg: "var(--surface-3)", color: "var(--text-muted)" },
  };
</script>

<Badge bg={styles[status].bg} color={styles[status].color} {title}>
  {STOCK_LABELS[status]}
</Badge>
