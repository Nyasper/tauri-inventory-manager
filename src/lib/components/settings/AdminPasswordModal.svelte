<script lang="ts">
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { KeyRound } from "@lucide/svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import { changeDeletePassword } from "$lib/services/db.service";

  let oldPassword = $state("");
  let newPassword = $state("");
  let repeatPassword = $state("");
  let changing = $state(false);
  let error = $state("");

  const canChange = $derived(
    oldPassword.trim() !== "" &&
      newPassword.trim() !== "" &&
      newPassword === repeatPassword &&
      !changing,
  );

  async function updatePassword() {
    if (!canChange) return;
    changing = true;
    error = "";
    try {
      await changeDeletePassword(oldPassword, newPassword);
      oldPassword = "";
      newPassword = "";
      repeatPassword = "";
    } catch (e) {
      error = String(e);
    } finally {
      changing = false;
    }
  }
</script>

<Modal
  title="Cambiar contraseña de administrador"
  onClose={() => {
    if (!changing) modalService.close();
  }}
>
  <p class="hint">
    La contraseña de administrador protege la eliminación masiva de movimientos
    y la eliminación definitiva de productos y proveedores archivados. La
    inicial es «admin».
  </p>
  <div>
    <Input label="Contraseña actual" type="password" bind:value={oldPassword} />
  </div>
  <div class="grid">
    <Input label="Nueva contraseña" type="password" bind:value={newPassword} />
    <Input
      label="Repetir nueva contraseña"
      type="password"
      bind:value={repeatPassword}
    />
  </div>
  {#if newPassword !== "" && repeatPassword !== "" && newPassword !== repeatPassword}
    <p class="validation" role="status">Las contraseñas nuevas no coinciden.</p>
  {/if}
  {#if error}<p class="error" role="alert">{error}</p>{/if}
  <div class="section-action">
    <Button
      variant="secondary"
      icon={KeyRound}
      disabled={!canChange}
      onclick={updatePassword}
    >
      {changing ? "Cambiando…" : "Cambiar contraseña"}
    </Button>
  </div>
</Modal>

<style>
  .hint {
    color: var(--text-muted);
    font-size: 0.8125rem;
    margin-bottom: 0.75rem;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
    margin-top: 0.75rem;
  }

  .section-action {
    margin-top: 0.75rem;
    display: flex;
    justify-content: flex-end;
  }

  .validation {
    margin-top: 0.5rem;
    color: var(--warning);
    font-size: 0.8125rem;
  }

  .error {
    margin-top: 0.5rem;
    color: var(--danger);
    font-size: 0.8125rem;
  }
</style>
