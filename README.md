# Inventory Manager

Desktop stock/inventory manager built with [Tauri 2](https://tauri.app) and [Svelte 5](https://svelte.dev) (SvelteKit + runes).

Para consultar la documentación completa de uso y todas las características en español, consulta el [Manual de Usuario](MANUAL_DE_USUARIO.md).

## Features

- Products with categories and flexible presentations (units of measure with conversion factors).
- Stock movements (in/out) that adjust product stock, with suppliers.
- List and grid views, search and category filters.
- Stock status (out/low/medium/high) with configurable thresholds.
- Light/dark theme persisted in local storage.
- SQLite persistence with validated Rust commands and read-only frontend queries.
- Unique normalized product, supplier, category and presentation names.
- Archive/restore products and suppliers without losing history.
- Multiple contact lines per supplier.
- Auditable movement cancellation and inventory adjustments with mandatory reasons.
- Movement filters and database pagination (25 rows per page).

## Development

```bash
bun install
bun run tauri dev
```

## Checks

```bash
bun run check      # svelte-check (types + runes)
cargo test --manifest-path src-tauri/Cargo.toml
bun run build
```

## Release

Tag a version (`v*`) to trigger the GitHub Actions workflow, which builds and
publishes `.deb`/`.rpm` for Linux and an NSIS installer + portable binary for
Windows, with release notes taken from `CHANGELOG.md`.

## Database and inventory rules

The application uses **inventory.db** in Tauri's application configuration directory. It does not use the old test **app.db**, copy its data, create backups or run migrations. A new installation starts without products or movements.

Demo data is opt-in. Close the application and run `bun run seed` to populate the normal local database. The command creates the schema when needed, refuses to alter a database that already contains inventory data, and prints the exact database path. For isolated development or testing, use `bun run seed -- --db ./path/to/inventory.db`.

All writes go through Rust commands and SQLite transactions with foreign keys enabled. Names are normalized with Unicode NFC, lowercase and collapsed whitespace. Accents and ñ remain significant. Names reserved by archived records cannot be reused; restore the existing record instead. Categories and presentation names are also unique (presentations within their product).

Each product has exactly one base presentation with factor 1. Editing preserves presentation IDs. Used conversion factors and the base presentation cannot change after movements exist. Retired presentations remain available to the historical records. Movement snapshots preserve names and factors at the time of registration.

Stock cannot become negative. Quantities, factors and resulting base quantities support up to six decimal places, with a maximum of 1,000,000,000. Stock updates round to six decimal places to avoid floating-point residue. A product must have zero stock before archiving. Inventory adjustments require a reason. Cancellation retains the original movement, records a reason and reverses stock atomically; it is rejected if the reversal would make stock negative or the product is archived.

The Rust tests cover normalized duplicates, rollback, invalid references and quantities, simultaneous withdrawals, cancellation, historical snapshots, archiving, legacy database migrations, decimal stock and repeated initialization. UI validation is checked with Svelte Check and the production build.
