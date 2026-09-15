<script lang="ts">
  import { backupService } from "$lib/services/backup.service";
  import Modal from "$lib/components/ui/Modal.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select, { type SelectOption } from "$lib/components/ui/Select.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import CategoryEditor from "$lib/components/categories/CategoryEditor.svelte";
  import AdminPasswordModal from "$lib/components/settings/AdminPasswordModal.svelte";
  import { ICONS, type IconName } from "$lib/icons";
  import { Plus, Pencil, Trash, DatabaseBackup, Upload, KeyRound } from "@lucide/svelte";
  import { settingsService } from "$lib/services/settings.service.svelte";
  import { getAppVersion } from "$lib/services/app.service";
  import { categoriesService } from "$lib/services/categories.service.svelte";
  import { productsService } from "$lib/services/products.service.svelte";
  import { movementsService } from "$lib/services/movements.service.svelte";
  import { suppliersService } from "$lib/services/suppliers.service.svelte";
  import { feedback } from "$lib/services/feedback.service.svelte";
  import { modalService } from "$lib/services/modal.service.svelte";
  import type { DefaultView } from "$lib/types";
  import {
    accentService,
    ACCENTS,
  } from "$lib/services/accent.service.svelte";
  import {
    CATEGORY_COLORS,
    newCategoryDraft,
    type CategoryDraft,
  } from "$lib/categories";

  let low = $state("");
  let medium = $state("");
  let defaultView = $state("dashboard");
  let editingCat = $state<CategoryDraft | null>(null);
  let busy = $state(false);

  const appVersion = getAppVersion();

  const dirty = $derived(
    Number(low) !== settingsService.settings.stock_low_threshold ||
      Number(medium) !== settingsService.settings.stock_medium_threshold ||
      editingCat !== null,
  );

  const thresholdsValid = $derived(
    Number.isFinite(Number(low)) &&
      Number.isFinite(Number(medium)) &&
      Number(low) >= 0 &&
      Number(low) < Number(medium),
  );

  const viewOptions: SelectOption[] = [
    { value: "dashboard", label: "Dashboard" },
    { value: "products", label: "Productos" },
    { value: "movements", label: "Movimientos" },
    { value: "suppliers", label: "Proveedores" },
  ];

  const selectedAccentLabel = $derived(
    ACCENTS.find((a) => a.id === accentService.accent)?.label ?? "",
  );

  $effect(() => {
    low = String(settingsService.settings.stock_low_threshold);
    medium = String(settingsService.settings.stock_medium_threshold);
  });

  $effect(() => {
    defaultView = settingsService.settings.default_view;
  });

  async function saveDefaultView() {
    await settingsService.saveDefaultView(defaultView as DefaultView);
  }

  async function saveThresholds() {
    const lowValue = Number(low);
    const mediumValue = Number(medium);
    if (!Number.isFinite(lowValue) || !Number.isFinite(mediumValue) || lowValue < 0 || mediumValue <= lowValue) {
      throw new Error("Los límites deben cumplir: 0 ≤ bajo < medio.");
    }
    await settingsService.saveThresholds(lowValue, mediumValue);
  }

  function startNewCategory() {
    editingCat = newCategoryDraft();
  }

  function startEditCategory(id: number, name: string, color: string | null, icon: string | null) {
    editingCat = {
      id,
      name,
      color: color ?? CATEGORY_COLORS[0],
      icon: icon && icon in ICONS ? (icon as IconName) : null,
    };
  }

  async function removeCategory(id: number, name: string) {
    const ok = await modalService.confirm(
      `¿Eliminar la categoría "${name}"? Los productos asociados quedarán sin categoría.`,
      "Eliminar categoría",
    );
    if (ok) await categoriesService.remove(id);
  }

  async function createBackup() {
    if (busy) return;
    busy = true;
    try {
      const path = await backupService.backup();
      if (path) feedback.success("Respaldo creado correctamente.");
    } catch (e) {
      feedback.error(e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }

  async function restoreBackup() {
    if (busy) return;
    const path = await backupService.pickRestoreFile();
    if (!path) return;
    const ok = await modalService.confirm(
      "Se reemplazarán todos los datos actuales por los del respaldo seleccionado. Esta acción no se puede deshacer.",
      "Restaurar respaldo",
    );
    if (!ok) return;
    busy = true;
    try {
      await backupService.restore(path);
      await Promise.all([
        settingsService.load(),
        categoriesService.load(),
        suppliersService.load(),
        productsService.load(),
        movementsService.load(),
      ]);
      feedback.success("Base de datos restaurada correctamente.");
    } catch (e) {
      feedback.error(e instanceof Error ? e.message : String(e));
    } finally {
      busy = false;
    }
  }
</script>

<Modal
  title="Configuración"
  onClose={() => modalService.close()}
  dirty={dirty}
  busy={busy}
  width="md"
>
  <div class="settings">
    <section>
      <h3>Límites de stock</h3>
      <p class="hint">
        Define los límites para clasificar el estado de stock de cada producto.
      </p>
      <div class="thresholds">
        <div>
          <Input label="Límite bajo" type="number" bind:value={low} />
        </div>
        <div>
          <Input label="Límite medio" type="number" bind:value={medium} />
        </div>
      </div>
      <div class="section-action">
        <Button size="sm" disabled={!thresholdsValid} onclick={saveThresholds}>Guardar límites</Button>
      </div>
      {#if !thresholdsValid}
        <p class="validation" role="status">Los límites deben cumplir: 0 ≤ bajo &lt; medio.</p>
      {/if}
    </section>

    <section>
      <h3>Color de acento</h3>
      <p class="hint">
        Personaliza el color de énfasis de toda la aplicación.
      </p>
      <div class="accents">
        {#each ACCENTS as accent (accent.id)}
          <button
            type="button"
            class="swatch"
            class:active={accentService.accent === accent.id}
            style="--swatch: {accent.light.primary}"
            aria-label={accent.label}
            title={accent.label}
            onclick={() => accentService.setAccent(accent.id)}
          ></button>
        {/each}
      </div>
      <p class="selected-accent">Seleccionado: {selectedAccentLabel}</p>
    </section>

    <section>
      <h3>Vista por defecto</h3>
      <p class="hint">
        Vista que se muestra al abrir la aplicación.
      </p>
      <Select
        bind:value={defaultView}
        options={viewOptions}
        onchange={saveDefaultView}
      />
    </section>

    <section>
      <h3>Categorías</h3>
      <p class="hint">
        Los productos muestran el icono y el color de su categoría.
      </p>

      {#if editingCat}
        <div class="cat-editor">
          <CategoryEditor
            draft={editingCat}
            onSaved={() => (editingCat = null)}
            onCancel={() => (editingCat = null)}
          />
        </div>
      {:else}
        <div class="section-action">
          <Button size="sm" variant="secondary" icon={Plus} onclick={startNewCategory}>
            Nueva categoría
          </Button>
        </div>
      {/if}

      {#if categoriesService.items.length === 0}
        <p class="empty-note">Sin categorías configuradas.</p>
      {:else}
        <ul class="cats">
          {#each categoriesService.items as category (category.id)}
            {@const Icon = category.icon && category.icon in ICONS ? ICONS[category.icon as IconName] : ICONS.box}
            <li class="cat">
              <span
                class="cat-tile"
                style:background={category.color ?? "var(--surface-2)"}
                style:color={category.color ? "#ffffff" : "var(--text-muted)"}
              >
                <Icon size={16} />
              </span>
              <span class="cat-name" title={category.name}>{category.name}</span>
              <div class="cat-actions">
                <button
                  class="icon-btn"
                  type="button"
                  title="Editar categoría"
                  aria-label="Editar categoría"
                  onclick={() =>
                    startEditCategory(category.id, category.name, category.color, category.icon)
                  }
                >
                  <Pencil size={16} />
                </button>
                <button
                  class="icon-btn"
                  type="button"
                  title="Eliminar categoría"
                  aria-label="Eliminar categoría"
                  onclick={() => removeCategory(category.id, category.name)}
                >
                  <Trash size={16} />
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </section>

    <section>
      <h3>Avanzado</h3>
      <p class="hint">
        Crea una copia de seguridad de la base de datos o restáurala desde un
        archivo. También puedes cambiar la contraseña de administrador, que
        protege la eliminación de movimientos, productos y proveedores.
      </p>
      <div class="backup-actions">
        <Button
          size="sm"
          variant="secondary"
          icon={DatabaseBackup}
          disabled={busy}
          onclick={createBackup}
        >
          Crear respaldo
        </Button>
        <Button
          size="sm"
          variant="secondary"
          icon={Upload}
          disabled={busy}
          onclick={restoreBackup}
        >
          Restaurar respaldo
        </Button>
        <Button
          size="sm"
          variant="secondary"
          icon={KeyRound}
          disabled={busy}
          onclick={() => modalService.open(AdminPasswordModal)}
        >
          Cambiar contraseña de administrador
        </Button>
      </div>
      {#if busy}
        <p class="hint backup-busy">Procesando…</p>
      {/if}
    </section>

    {#await appVersion then version}
      {#if version}
        <p class="app-version">Gestor de Inventario v{version}</p>
      {/if}
    {/await}
  </div>
</Modal>

<style>
  .settings {
    display: flex;
    flex-direction: column;
  }

  .settings section {
    padding: 0.875rem 0;
  }

  .settings section:first-child {
    padding-top: 0;
  }

  .settings section:last-child {
    padding-bottom: 0;
  }

  .settings section + section {
    border-top: 1px solid var(--border);
  }

  h3 {
    font-size: 0.9375rem;
    margin-bottom: 0.25rem;
  }

  .hint {
    color: var(--text-muted);
    font-size: 0.8125rem;
    margin-bottom: 0.75rem;
  }

  .thresholds {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .section-action {
    margin-top: 0.75rem;
    margin-bottom: 0.40rem;
    display: flex;
    justify-content: flex-end;
  }

  .backup-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .backup-busy {
    margin-top: 0.5rem;
    margin-bottom: 0;
  }

  .accents {
    display: flex;
    flex-wrap: wrap;
    gap: 0.625rem;
  }

  .swatch {
    width: 32px;
    height: 32px;
    padding: 0;
    border-radius: 50%;
    border: 2px solid var(--surface);
    background: var(--swatch);
    box-shadow: 0 0 0 1px var(--border);
    cursor: pointer;
    transition:
      transform 0.12s,
      box-shadow 0.12s;
  }

  .swatch:hover {
    transform: scale(1.1);
  }

  .swatch.active {
    box-shadow: 0 0 0 2px var(--primary);
  }

  .selected-accent {
    margin-top: 0.5rem;
    color: var(--text-muted);
    font-size: 0.8125rem;
  }

  .cat-editor {
    padding: 0.875rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    margin-bottom: 0.75rem;
  }

  .cats {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .cat {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.375rem 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
  }

  .cat-tile {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border-radius: var(--radius-sm);
    flex-shrink: 0;
  }

  .cat-name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: 500;
  }

  .cat-actions {
    display: flex;
    gap: 0.125rem;
  }

  .empty-note {
    color: var(--text-muted);
    font-size: 0.8125rem;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .icon-btn:hover {
    background: var(--surface-2);
    color: var(--text);
  }

  .validation {
    margin-top: 0.5rem;
    color: var(--warning);
    font-size: 0.8125rem;
  }

  .app-version {
    padding-top: 0.875rem;
    border-top: 1px solid var(--border);
    text-align: center;
    color: var(--text-muted);
    font-size: 0.75rem;
  }
</style>