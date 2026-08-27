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
    const { default: ConfirmDialog } = await import(
      "$lib/components/ui/ConfirmDialog.svelte"
    );
    return new Promise<boolean>((resolve) => {
      this.open(ConfirmDialog, {
        title,
        message,
        onResult: (result: boolean) => {
          this.closeAll();
          resolve(result);
        },
      });
    });
  }
}

export const modalService = new ModalService();
