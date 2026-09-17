import type { Component } from "svelte";

type AnyComponent = Component<any, any, any>;

interface ModalEntry {
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
  private pendingConfirms = new Map<number, (result: boolean) => void>();

  open(component: AnyComponent, props: Record<string, unknown> = {}): number {
    const id = this.nextId++;
    this.modals.push({ id, component, props });
    return id;
  }

  close(id?: number): void {
    if (id === undefined) {
      const removed = this.modals[this.modals.length - 1];
      this.modals.pop();
      if (removed) this.resolveConfirm(removed.id);
      return;
    }
    this.modals = this.modals.filter((m) => m.id !== id);
    this.resolveConfirm(id);
  }

  closeAll(): void {
    const ids = this.modals.map((m) => m.id);
    this.modals = [];
    this.guards = [];
    for (const id of ids) this.resolveConfirm(id);
  }

  /**
   * Resuelve con `false` una confirmación cuyo modal se cerró por otra vía
   * (programática), para que ninguna promesa de `confirm()` quede pendiente.
   */
  private resolveConfirm(id: number): void {
    const resolve = this.pendingConfirms.get(id);
    if (!resolve) return;
    this.pendingConfirms.delete(id);
    resolve(false);
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
    const { default: ConfirmDialog } = await import("$lib/components/ui/ConfirmDialog.svelte");
    return new Promise<boolean>((resolve) => {
      const id = this.open(ConfirmDialog, {
        title,
        message,
        onResult: (result: boolean) => {
          this.pendingConfirms.delete(id);
          this.close(id);
          resolve(result);
        },
      });
      this.pendingConfirms.set(id, resolve);
    });
  }
}

export const modalService = new ModalService();
