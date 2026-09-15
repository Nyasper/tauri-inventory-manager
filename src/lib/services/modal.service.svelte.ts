import type { Component } from "svelte";

type AnyComponent = Component<any, any, any>;

export interface ModalEntry {
  id: number;
  component: AnyComponent;
  props: Record<string, unknown>;
}

interface CloseGuard {
  id: symbol;
  check: () => Promise<boolean>;
}

class ModalService {
  modals = $state<ModalEntry[]>([]);

  private nextId = 1;
  private guards: CloseGuard[] = [];

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

  /**
   * Registra el guardián de cierre de un modal. Los modales se montan en orden
   * de apilamiento, por lo que el último guardián registrado es el del modal
   * visible superior. Devuelve la función para darlo de baja.
   */
  registerCloseGuard(check: () => Promise<boolean>): () => void {
    const guard: CloseGuard = { id: Symbol("modal-guard"), check };
    this.guards.push(guard);
    return () => {
      this.guards = this.guards.filter((item) => item.id !== guard.id);
    };
  }

  /**
   * Indica si el modal `id` (o el superior si se omite) puede cerrarse ahora.
   * Consulta su guardián: cambios sin guardar u operaciones en curso lo bloquean.
   */
  async canClose(id?: number): Promise<boolean> {
    const topId = this.modals[this.modals.length - 1]?.id;
    const target = id ?? topId;
    if (target === undefined || target !== topId) return false;
    const guard = this.guards[this.guards.length - 1];
    return guard ? await guard.check() : true;
  }

  /** Cierra un modal respetando los cambios sin guardar (solo si es el superior). */
  async requestClose(id?: number): Promise<void> {
    if (!(await this.canClose(id))) return;
    this.close(id);
  }

  /** Confirmación estándar para descartar cambios sin guardar. */
  async confirmDiscard(): Promise<boolean> {
    return this.confirm(
      "Se perderán los cambios que no hayas guardado. ¿Quieres cerrar de todos modos?",
      "Descartar cambios",
    );
  }

  async confirm(message: string, title = "¿Estás seguro?"): Promise<boolean> {
    const { default: ConfirmDialog } = await import(
      "$lib/components/ui/ConfirmDialog.svelte"
    );
    return new Promise<boolean>((resolve) => {
      const id = this.open(ConfirmDialog, {
        title,
        message,
        onResult: (result: boolean) => {
          this.close(id);
          resolve(result);
        },
      });
    });
  }
}

export const modalService = new ModalService();
