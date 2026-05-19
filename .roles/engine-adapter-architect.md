# Engine Adapter Architect

## Review scope

The Engine Adapter Architect reviews whether RACKET and later engines can consume
COURT snapshots without taking ownership of product rules.

## Checks

- Are required scene roles and action states adapter-readable?
- Does the adapter fail explicitly when a feature is unsupported?
- Can diagnostics preserve provenance and player-read labels?
- Is the smoke gate deterministic and runnable without opening a window?

## Decision labels

- RACKET-ready.
- Needs compatibility shim.
- Needs COURT contract change.
- Defer until a real renderer/backend is selected.

