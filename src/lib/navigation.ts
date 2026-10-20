import type { Component } from "svelte";
import type { LucideProps } from "@lucide/svelte";
import { ArrowLeftRight, Boxes, LayoutDashboard, Truck } from "@lucide/svelte";
import type { DefaultView } from "$lib/types";

export interface NavigationItem {
  view: DefaultView;
  href: string;
  label: string;
  icon: Component<LucideProps>;
}

export const NAVIGATION: NavigationItem[] = [
  { view: "dashboard", href: "/", label: "Dashboard", icon: LayoutDashboard },
  { view: "products", href: "/products", label: "Productos", icon: Boxes },
  { view: "movements", href: "/movements", label: "Movimientos", icon: ArrowLeftRight },
  { view: "suppliers", href: "/suppliers", label: "Proveedores", icon: Truck },
];

export const PAGE_TITLES: Record<string, string> = Object.fromEntries(
  NAVIGATION.map(({ href, label }) => [href, label]),
);

export const VIEW_ROUTES = Object.fromEntries(
  NAVIGATION.map(({ view, href }) => [view, href]),
) as Record<DefaultView, string>;
