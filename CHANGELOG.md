# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

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
