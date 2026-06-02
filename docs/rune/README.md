# COURT RUNE contracts

COURT exposes its portable experience records as RUNE descriptor evidence so AI
agents and downstream tooling can inspect the experience boundary without
scraping Rust source.

## Retained evidence

- `docs\rune\experience_contracts.json` is generated from
  `court_core::rune_descriptor_collection()`.
- The collection id is `court.experience_contracts`.
- The first slice covers experience identity, actions, scene nodes, snapshots,
  snapshot metadata, validation packets, and evidence references.

## Boundary

RUNE metadata describes stable COURT data contracts only. Product rules, scripts,
private playtest details, renderer behavior, and product-owned evidence bodies
remain outside `court-core`.

