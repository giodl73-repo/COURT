# Pulse 16: RUNE experience contracts

## Goal

Expose COURT's portable experience and validation records as retained RUNE
descriptor evidence.

## Changes

- Add pinned RUNE descriptor dependencies to `court-core`.
- Derive RUNE contracts for the stable COURT experience, action, scene,
  snapshot, validation packet, and evidence reference records.
- Add `court_core::rune_descriptor_collection()` and retained evidence at
  `docs\rune\experience_contracts.json`.
- Document the RUNE boundary in `docs\rune\README.md`.

## Outcome

AI and portfolio tools can now read COURT's experience contract shape from a
stable descriptor collection without source scraping or product-specific rules.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`

## Status

Complete.

