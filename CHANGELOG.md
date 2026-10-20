# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Opción para ignorar alertas de stock por producto: permite desactivar alertas de reposición para artículos bajo demanda o de stock mínimo permanente (`ignore_stock_alerts`), excluyéndolos del panel de stock bajo del Dashboard y mostrándolos con la insignia distintiva "Sin alertas" en la tabla de productos.
- Límites de stock configurables por producto individual: cada producto puede definir opcionalmente sus propios límites (`stock_low_threshold` y `stock_medium_threshold`), respaldados por una migración SQLite automática no destructiva en el arranque; si no se personalizan, se heredan automáticamente los límites globales del sistema.
- Tooltips informativos en las insignias de estado de stock (`StockBadge`), indicando si la clasificación responde a límites personalizados o a los límites globales por defecto.
- Default supplier per product: products can now define an optional default supplier (`default_supplier_id`) via `SearchableSelect` in `ProductFormModal`, backed by a non-destructive automatic SQLite schema migration on startup; creating an inbound movement for the product automatically pre-selects its default supplier in `MovementFormModal` while preserving full manual editing.
- Navigation history controls: Back and Forward buttons added next to the view title in the top layout header (`Header.svelte`) with history state detection.
- `SearchableSelect` component: dedicated searchable select / combobox primitive in `src/lib/components/ui/SearchableSelect.svelte` with text filtering, accent-tolerant search, keyboard navigation and option highlighting; applied to product and supplier selections in `MovementFormModal`, categories in `ProductFormModal`, and inline supplier editing in `MovementDetailModal`.
- Edit and assign suppliers on registered movements: `update_movement_supplier` command in Rust and dedicated `EditMovementSupplierModal` accessible directly from the movements table actions as well as `MovementDetailModal` allowing users to assign, change, or clear the supplier on active inbound movements.
- English demo dataset (`src-tauri/src/seed_en.sql`) selectable with `bun run seed -- --lang en`: 8 categories, 15 suppliers, 50 products with 122 presentations and 121 movements, including adjustments, cancelled movements and a mix of zero, low and healthy stock levels; covered by a Rust test that validates counts, foreign keys, the single active base presentation per product and stock coherence against the movement history.

### Changed

- Renombrada la configuración general de umbrales a "Límites globales de stock" en `SettingsModal`, el modal de ayuda (`HelpModal`) y el manual de usuario (`MANUAL_DE_USUARIO.md`), clarificando su función como valores por defecto heredados por los productos que no cuenten con límites propios.

### Fixed

- Prevented `SearchableSelect` from closing parent modals on `Escape`: keydown events are now stopped when the dropdown is open so `Escape` exclusively closes the select dropdown and restores focus to the trigger button.

## [1.0.0] - 2026-09-16

### Added

- Movement detail view: clicking a movement in the history or in the dashboard's recent list opens a full card with type and state, product, presentation and conversion factor, quantity and its base equivalent, supplier, date, adjustment reason, complete notes and cancellation details, with actions to view the product history and to cancel the movement while it is active.
- Prettier and ESLint setup with `lint`, `format` and `format:check` scripts.
- `CI` workflow running formatting, lint, type checks, frontend build, Rust formatting, Clippy and tests on pushes to `main` and pull requests.
- Configurable movements per page (25, 50, 100 or 200) from Settings › "Movimientos por página"; the preference is saved immediately in the database and the movements view honors it.

### Changed

- Backend command errors now travel as structured `{ code, message }` values; the frontend reacts to stable codes (for example to detect duplicate names) instead of matching Spanish message text.
- Backups are written to a temporary file next to the destination and renamed only on success, so a failed backup no longer removes a previous backup file.
- Modernized the Svelte components: declaration tags (`{const}`) replace the legacy `{@const}`, clsx-style class attributes replace `class:` directives, the modal focus lifecycle uses an `{@attach}` attachment, and the archived-list URL helper uses the reactive `SvelteURL`.
- Settings thresholds, the default view, the product stock filter and the movement product filter now derive their value from the stored settings or the URL instead of being synchronized with `$effect`; the movement-form quantity is clamped with a function binding and the movements page keeps its page in derived state, so changing a filter no longer triggers an extra fetch.
- Presentation and supplier contact rows use stable keys instead of the array index.
- Right-click is now allowed inside text inputs and textareas (copy/paste), while the global context menu stays disabled everywhere else.
- Select and textarea primitives accept a `disabled` prop, and data tables accept a `rowClickable` predicate to disable row clicks per record.
- Movement pagination now uses icon buttons (first, previous, next and last page) with the page indicator showing the total number of pages, and lists 50 rows per page (up from 25).
- Supplier rows can be clicked to open the edit form, like product rows; archived rows keep their explicit actions.
- The application layout is marked `inert` while a modal is open, so the background cannot be focused or clicked.
- `cargo fmt`, `cargo clippy -- -D warnings` and `cargo test` are now the documented Rust checks.
- Project metadata (description/authors) is no longer placeholder text.
- `Intl` number and date formatters are created once instead of on every cell render, and the dashboard computes each product's stock status a single time.
- Data tables mark column headers with `scope` and `aria-sort`, use the visible column name for sort buttons and announce the row action to screen readers.
- The dashboard's cancelled marker uses the standard badge style, and the movement detail shows "unidades base" when the product no longer exists.
- The movements view counts the total with an indexed query instead of the `count(*) OVER ()` window function, which forced SQLite to materialize and sort the whole history; with 100,000 movements the page went from ~45 ms to ~0.2 ms.
- The database connection uses WAL journaling with `synchronous=NORMAL`, an in-memory temporary store and an 8 MB cache, reducing the latency of every movement write and speeding up sorts.
- The products table paginates on the client (50 rows per page) with a page footer; search, filters and sorting still apply to the whole catalog before paginating.
- The dashboard no longer loads the suppliers list twice (the layout already loads it on startup).
- Release builds enable `lto`, `codegen-units=1` and symbol stripping.

### Fixed

- Clearing the movement quantity field no longer keeps a hidden value: the field resets to `0` and saving is blocked until a valid quantity is entered.
- Movements pagination falls back to the last valid page after movements are deleted or a backup with fewer records is restored.
- Category edits and deletions refresh the products list, and the products filter drops a category that no longer exists.
- Changing the administrator password closes the modal after a successful update.
- Saving the default view reports failures, reverts the select to the stored value and no longer depends on event listener ordering; backup dialogs also report errors instead of failing silently.
- Movement pagination reports the real total when the requested page is out of range instead of zero.
- Saving settings verifies that the stored rows were updated instead of reporting success silently.
- Archiving a product with stock and archiving/restoring a missing record now return distinct, clearer errors.
- The stock preview panel in the movement form uses its shared `stock-preview` style again.
- Modal autofocus moved to the first field in the modal body instead of the header close button.
- Clicking a row in the archived products list no longer restores the product; restoring now requires the explicit "Restaurar" action.
- The batch-deletion modal no longer treats a numerically unchanged count as an unsaved change.
- The user manual and the in-app help now list the real database path per operating system (under the application identifier) instead of outdated locations.
- Backups can no longer overwrite the database in use: the destination is compared against the live file before writing.
- Updating a supplier or category whose id no longer exists now fails with `not_found` instead of silently inserting a record with that id.
- Backup validation now checks required columns, `PRAGMA integrity_check` and `PRAGMA foreign_key_check`; a restore whose data has broken references rolls back to the previous database.
- Legacy databases are migrated with foreign keys disabled, a `pre-migrate` safety copy and a post-migration `foreign_key_check`.
- The movements page index is clamped so an absurd page number cannot overflow the query offset.
- If the database cannot be opened, it is moved aside as `inventory.db.corrupt-<timestamp>`, the app starts with a fresh database and notifies the user.
- Duplicate-name errors are mapped from the expected entity at each call site instead of parsing SQLite message text.
- Saving stock thresholds is blocked by its button state and reports failures through the standard feedback instead of throwing an unhandled error.
- A failed settings load at startup is reported without leaving the default-view redirect pending.
- The user manual and the in-app help again describe the configurable page size and the icon pagination controls (they had regressed to 25 rows and text buttons).
- Programmatic modal closes (`closeAll` or by id) now resolve any pending `confirm()` promise with `false` instead of leaving its `await` hanging forever, and `closeAll` also clears the close-guard stack.
- The window no longer freezes after a few minutes of use when it is covered or backgrounded: Windows WebView2 disables native window occlusion detection, occluded-window backgrounding and renderer backgrounding, which could unload the web view while it was idle.
- First launch on Linux no longer crashes with `unable to open database file` (code 14): the database directory is now created if missing and a missing file is not treated as corruption to be quarantined.

### Removed

- Dead code flagged by the new linting: unused icon and type imports, the unused `Product.presentations` field, an inert CSS class and an unused table prop.
- Unused payload fields (`Product.created_at`, `updated_at`, `base_presentation_id`, `Presentation.product_id`), the `products_category` index, the unused `--shadow` CSS variable and lint-flagged unused exports; `productsService.load` no longer accepts an unused argument.
- Unused `tauri-plugin-opener` dependency, plugin registration and capability.
- Leftover `protocol-asset` feature, asset protocol scope and CSP entries from the removed image feature.
- Unused `core:path:default` capability and unused `--stock-low`/`--stock-medium`/`--stock-high` CSS variables.

## [0.12.0] - 2026-09-15

### Added

- Permanent deletion of archived suppliers from the "Archivados" view; their movements keep the supplier name snapshot with a "Proveedor eliminado" badge.
- Batch movement deletion now supports date criteria: from a date onward, up to a date (backwards) or between two dates, alongside the existing most-recent/most-oldest count modes.
- Live preview in the batch-deletion modal showing how many movements match the selected filter before deleting, with the button disabled when there are none.
- The current application version is shown at the end of the Settings modal, read dynamically from the executable.
- Confirmation dialog when closing a modal with unsaved changes (overlay click, X, Escape, Cancel/Back or `Ctrl + O`).

### Changed

- Permanent deletion of products and suppliers now requires the administration password, the same one that protects batch movement deletion, and shows an inline error when it is wrong.
- Movement type badges (Entrada / Salida) now share a uniform minimum width in tables and the dashboard.
- In the products and suppliers tables, the archive action now uses an archive icon instead of a trash can, and the restore action in the archived views is now an undo icon button instead of a text button.
- The archived views of products and suppliers now show a dedicated empty-state message ("No hay productos/proveedores archivados") instead of the regular empty-list message.
- The "Anular movimiento" action in the movements view now uses an undo icon instead of a trash can.
- The "Eliminar movimientos" batch-deletion button moved from Settings › Advanced to the bottom of the movements view (next to the pagination controls).
- The administration password that protects movement deletion can now only be changed from Settings › Advanced via the new "Cambiar contraseña de administrador" button, and it was removed from the delete-movements modal.
- Closing modals is now centralized through a close guard in `modal.service`, so every close path (buttons, keys and shortcuts) respects in-progress operations and unsaved changes.

### Fixed

- Batch movement deletion now performs a single transactional statement instead of deleting rows one by one.

## [0.11.0] - 2026-09-14

### Added

- Permanent deletion of archived products from the "Archivados" view; their movements are preserved and marked as "Producto eliminado", and existing databases migrate automatically to allow it.
- Batch movement deletion from Settings › Advanced, choosing between the N most recent or the N oldest movements. The operation requires a password stored in the database (initially `admin`), which can be changed from the same modal by entering the current password and the new one twice.
- "Sin categoría" option in the product category filter to list only products without a category.
- Quick "−" action in the product table to register an outbound movement with the product preselected.
- Quick "+" action in the supplier table to register an inbound movement with the supplier preselected.
- Centralized keyboard shortcuts in `keybindings.service` (`F1` help, `Ctrl+O` settings, `Ctrl+T` light/dark toggle, `Ctrl+1-4` view navigation), documented in the help modal and user manual.
- Category icons in each row of the dashboard low-stock panel.

### Changed

- Settings moved from the header to the sidebar footer as an "Ajustes" button next to "Ayuda"; the header now only keeps the theme toggle, and the help modal and user manual were updated accordingly.
- Renamed the "Franjas de stock" / "Umbral" wording to "Límites de stock" across the interface, help modal, error messages and user manual.
- The movement form now asks for product, presentation and quantity before type and inventory adjustment.
- Product row actions: the history button is labeled "Ver movimientos", and the blocked archive tooltip now reads "Eliminar producto (el stock debe ser 0)".
- Missing tooltips added to the remaining icon-only action buttons (supplier, movement, category and product-presentation rows).
- Settings modal sections are separated by dividers instead of uniform spacing.
- Application icon replaced with the new box icon (executable, installers and favicon), and the old clean-icon assets were removed.
- Sidebar brand now shows the box icon in the accent color with a transparent background and reads "Inventario", and the visible product name changed to "Gestor de Inventario" (window title, document title, header, help modal and backup error). The bundle product name is unchanged.
- Dashboard low-stock and recent-movements panels now show up to 10 items instead of 5.
- Movements of permanently deleted products are preserved: the history keeps the product name snapshot and shows a "Producto eliminado" badge, no longer links back to the product, and cancelling them does not affect stock. Existing databases migrate automatically on startup.

### Fixed

- The "Archivados" checkbox in the products and suppliers views now actually filters and syncs with the `?archived=1` URL parameter.
- The global right-click context menu is disabled again, after the handler was lost in an earlier layout refactor.

## [0.10.0] - 2026-09-14

### Removed

- The responsible persons feature: catalog and default responsible, the required "Responsable" field in stock movements, the responsible column and filter in the movements view, the archived-responsibles section in settings, the inline creation from the movement form, and the `list_persons` command. Existing databases are migrated on startup: `movements` is rebuilt without `responsible_person_id`/`person_name` and the `responsible_persons` table is dropped.

### Changed

- User manual, in-app help, README and tests updated to reflect the removal.

## [0.9.0] - 2026-09-13

### Added

- Dashboard as the default landing view, with summary cards (products, low/out-of-stock, movements, suppliers), a low-stock panel sorted from out to low, recent movements including the responsible person, and quick actions to create products, movements and suppliers.
- Reusable sortable data table (TanStack Table) with global search and sortable columns, applied to the product, movement and supplier lists.
- Movement filters by type (in/out), product, supplier, responsible person, date range and status (active/cancelled), with database pagination (25 rows per page).
- Category management with color and Lucide icon selection; products now inherit the icon of their category.
- Auditable movement cancellation: the original record is kept, a mandatory reason is stored and stock is reversed atomically.
- Inventory adjustments (movements marked as "adjustment") that require a reason.
- Archive and restore for products, suppliers and responsible persons without losing history; archived records can be listed and restored, and their names stay reserved.
- Multiple contact lines per supplier.
- Duplicate-name hints in the product and supplier forms.
- Configurable default view (dashboard, products, movements or suppliers).
- Custom number stepper with bounded buttons and quick minimum/maximum values for quantity fields.
- Opt-in demo data through `bun run seed`, which creates the schema when needed, refuses to alter a database that already contains inventory data and accepts a custom `--db` path.
- Toast notifications for successful database operations and unhandled errors.
- Database backup and restore from a new "Advanced" section in the settings modal, with an automatic safety copy before restoring.
- User manual (`MANUAL_DE_USUARIO.md`) covering installation, features and workflows, linked from the README.
- Interactive in-app user guide modal (`HelpModal.svelte`), opened with the F1 key or the help button at the bottom of the sidebar.
- Rust test suite covering normalized duplicates, rollback, invalid references and quantities, simultaneous withdrawals, cancellation, historical snapshots, archiving, default responsible persons, decimal stock and repeated initialization.
- Rust read commands (`list_products`, `list_presentations`, `list_suppliers`, `list_categories`, `list_persons`, `page_movements`, `get_settings`, `find_duplicate`) so every query runs through the same SQLx pool as the writes.
- Tests for the read commands covering joined catalog data, literal search escaping, filters, pagination totals, duplicate lookup and settings fallbacks.

### Changed

- All database access runs through validated Rust commands inside SQLite transactions with foreign keys enabled; the webview no longer opens its own database connection.
- Replaced versioned SQL migrations with idempotent initialization from `schema.sql`, using a fresh `inventory.db` (the old `app.db` is neither migrated nor copied).
- Names are normalized (Unicode NFC, lowercase, collapsed whitespace) and unique for products, suppliers, categories and presentations (within their product).
- Numeric rules: quantities, factors and stock support up to six decimal places with a maximum of 1,000,000,000, and stock can never become negative.
- Historical integrity: base presentations and conversion factors used by movements become immutable, products must have zero stock before archiving, and movements keep snapshots of names and factors.
- Modals use smooth fade transitions and confirmation dialogs are themed Svelte dialogs instead of native ones.
- Impossible actions are disabled with live validation (overselling, archiving products with stock, empty or duplicate fields, etc.).
- Number inputs step by whole units; default spinner styling was replaced by the custom stepper.
- Routes restructured: the dashboard lives at `/`, products moved to `/products`, and navigation was updated accordingly.
- Default preferences are light theme, blue accent and dashboard view when nothing is stored.
- Custom product and supplier images were replaced by Lucide icons and category-based product icons across the interface.
- The product list and dashboard share stock-based sorting, and movement listings show the responsible person.
- The global right-click context menu and browser autocomplete suggestions are disabled.
- Demo data is now opt-in instead of being seeded automatically on startup.
- The sidebar is narrower (196px, 168px on small windows) to save horizontal space.
- Data tables show an "Acciones" header in the actions column and align headers with their column content, with centered columns where appropriate.
- The product list shows the category icon next to the category name (the separate icon column was removed) and truncates long text with ellipsis.
- Database access consolidated on a single SQLx connection pool: the Tauri SQL plugin and its frontend connection are gone.
- Movement and product searches escape `LIKE` wildcards (`%`, `_`) so text matches literally, and pagination resolves the total with `count(*) OVER ()` in one query.
- Duplicate-name lookup now runs in Rust and reuses the same normalization applied on insert.
- The `invoke` helper takes an optional success message instead of hardcoding toast text per command.
- `DataTable` is typed with Svelte generics and TanStack `columnMeta`, and the movements filter options come from the shared stores.
- Archived lists share a new `ArchivedList` helper that protects against stale responses.
- The help modal search box now filters the section list.

### Removed

- Products grid view and all related code (only the list view remains).
- Product "code" field from the interface and the database; the autoincremental id is the only identifier.
- Custom product/supplier images (URL or local file), the image utilities and the generic icon wrapper component.
- Versioned database migrations.
- `@tauri-apps/plugin-sql` dependency, its `sql:default`/`sql:allow-select` capabilities and its preload configuration.
- Unused `serde_json` dependency and the redundant write-lock query previously used to serialize transactions.

### Fixed

- Startup panic `migration 3 was previously applied but has been modified`.
- Error toast close button wrapping onto a new line.
- Category filter alignment in the product toolbar, and layout of supplier and stock-threshold inputs.
- Number inputs incrementing by large decimal values.
- Seed data inconsistencies and minor UI spacing issues.
- `DataTable` called `getVisibleLeafColumns()` without the column visibility feature enabled, which would have thrown when rendering an empty table.
- The help modal search box accepted input but never filtered anything.
- Duplicate hints could drift from the backend normalization because the name key was recomputed in the frontend.
- The demo seed test expectations (26 products, 52 movements) now match the seeded data.

### Security

- Content Security Policy enabled (previously disabled) with a dedicated development variant for Vite HMR.
- Removed SQL plugin capabilities so the webview can no longer run SQL on its own.

## [0.1.0] - 2026-08-26

### Added

- Inventory manager with products, categories and flexible presentations (units of measure with conversion factors).
- Stock movements (in/out) that adjust product stock, with suppliers and responsible persons.
- Product list and grid views with search and category filtering.
- Stock status classification (out/low/medium/high) with configurable thresholds.
- Light and dark theme, persisted in local storage.
- Accent color selection with predefined palettes.
- SQLite persistence via the Tauri SQL plugin with automatic migrations.
- Settings modal for stock thresholds and responsible persons.
- Supplier management view.
- Product images from URL or local file (copied to the app data directory).
- Custom application icon on the executable and in the interface.
- GitHub Actions release workflow: `.deb`/`.rpm` for Linux, NSIS installer and portable binary for Windows.
