import { confirm } from "@tauri-apps/plugin-dialog";
import type { Component } from "svelte";

type AnyComponent = Component<any, any, any>;

export interface ModalEntry {
  id: number;
  component: AnyComponent;
  props: Record<string, unknown>;
}

class ModalService {
  modals = $state<ModalEntry[]>([]);

  private nextId = 1;

  open(component: AnyComponent, props: Record<string, unknown> = {}): number {
    const id = this.nextId++;
    this.modals.push({ id, component, props });
    return id;
  }

  close(id?: number): void {
    if (id === undefined) {
      this.modals.pop();
      return;
    }
    this.modals = this.modals.filter((m) => m.id !== id);
  }

  closeAll(): void {
    this.modals = [];
  }

  async confirm(message: string, title = "¿Estás seguro?"): Promise<boolean> {
    return confirm(message, { title, kind: "warning" });
  }
}

export const modalService = new ModalService();
