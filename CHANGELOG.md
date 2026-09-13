# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

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
- Interactive in-app user guide modal (`HelpModal.svelte`) accessible via the help (?) button at the bottom of the sidebar.
- Rust test suite covering normalized duplicates, rollback, invalid references and quantities, simultaneous withdrawals, cancellation, historical snapshots, archiving, default responsible persons, decimal stock and repeated initialization.

### Changed

- All database writes now run through validated Rust commands inside SQLite transactions with foreign keys enabled; the frontend only performs read queries.
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

### Removed

- Products grid view and all related code (only the list view remains).
- Product "code" field from the interface and the database; the autoincremental id is the only identifier.
- Custom product/supplier images (URL or local file), the image utilities and the generic icon wrapper component.
- Versioned database migrations.

### Fixed

- Startup panic `migration 3 was previously applied but has been modified`.
- Error toast close button wrapping onto a new line.
- Category filter alignment in the product toolbar, and layout of supplier and stock-threshold inputs.
- Number inputs incrementing by large decimal values.
- Seed data inconsistencies and minor UI spacing issues.

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
