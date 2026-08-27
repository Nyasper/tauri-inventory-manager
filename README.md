# Inventory Manager

Desktop stock/inventory manager built with [Tauri 2](https://tauri.app) and [Svelte 5](https://svelte.dev) (SvelteKit + runes).

## Features

- Products with categories and flexible presentations (units of measure with conversion factors).
- Stock movements (in/out) that adjust product stock, with suppliers and responsible persons.
- List and grid views, search and category filters.
- Stock status (out/low/medium/high) with configurable thresholds.
- Light/dark theme persisted in local storage.
- SQLite persistence via the Tauri SQL plugin.

## Development

```bash
bun install
bun run tauri dev
```

## Checks

```bash
bun run check      # svelte-check (types + runes)
cargo check        # from src-tauri
```

## Release

Tag a version (`v*`) to trigger the GitHub Actions workflow, which builds and
publishes `.deb`/`.rpm` for Linux and an NSIS installer + portable binary for
Windows, with release notes taken from `CHANGELOG.md`.
