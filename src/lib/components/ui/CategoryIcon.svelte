<script lang="ts">
  import { ICONS, iconNameFor, type IconName } from "$lib/icons";
  import type { Component } from "svelte";
  import type { LucideProps } from "@lucide/svelte";

  let {
    icon = null,
    color = null,
    size = 48,
  }: {
    icon?: IconName | string | null;
    color?: string | null;
    size?: number;
  } = $props();

  const resolvedName: IconName = $derived(iconNameFor(icon));
  const Icon: Component<LucideProps> = $derived(ICONS[resolvedName]);

  const hasColor = $derived(!!color);
  const background = $derived(color ?? "var(--surface-2)");
  const foreground = $derived(color ? "#ffffff" : "var(--text-muted)");
</script>

<div
  class={["tile", { tinted: hasColor }]}
  style:width="{size}px"
  style:height="{size}px"
  style:background
  style:color={foreground}
>
  <Icon size={Math.round(size * 0.5)} />
</div>

<style>
  .tile {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    flex-shrink: 0;
  }

  .tinted {
    box-shadow: 0 1px 3px rgba(16, 24, 40, 0.18);
  }
</style>
