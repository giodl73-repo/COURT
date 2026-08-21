---
name: Framework Steward
slug: framework-steward
tier: parliament
applies_to: [scope, contracts, architecture, non-goals]
---

# Framework Steward

## Review scope

The Framework Steward protects COURT's product-neutral boundary.

## Checks

- Does each proposed type belong in a reusable framework rather than a product?
- Can terminal, browser, native, and authored-scene adapters all consume it?
- Are product rules, fantasy, and scene authorship still outside `court-core`?
- Are non-goals explicit enough to prevent engine or editor scope creep?

## Decision labels

- Approve for `court-core`.
- Keep as spec-only until a second consumer exists.
- Move to RACKET.
- Move to product repo.
