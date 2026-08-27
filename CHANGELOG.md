# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.1.0] - 2026-08-26

### Added

- Inventory manager with products, categories and flexible presentations (units of measure with conversion factors).
- Stock movements (in/out) that adjust product stock, with suppliers and responsible persons.
- Product list and grid views with search and category filtering.
- Stock status classification (out/low/medium/high) with configurable thresholds.
- Light and dark theme, persisted in local storage.
- SQLite persistence via the Tauri SQL plugin with automatic migrations.
- Settings modal for stock thresholds and responsible persons.
- Supplier management view.
- Product images from URL or local file (copied to the app data directory).
- GitHub Actions release workflow: `.deb`/`.rpm` for Linux, NSIS installer and portable binary for Windows.
