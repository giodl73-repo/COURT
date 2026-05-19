# Pulse 04: Australian Open

## Goal

Implement the approved Australian Open slice: portable experience identity,
provenance/custody classes, and action availability.

## Changes

- Add `CourtExperienceIntent`.
- Add `CourtProvenance` and `CourtProvenanceClass`.
- Add `CourtActionAvailability`.
- Extend scene nodes with optional provenance.
- Update snapshot tests so only player-available commands are exposed.
- Keep product rule predicates, renderer behavior, playtest records, and
  migration out of this slice.

## Downstream

RACKET must update fixtures for the new COURT contract and count player-available
commands separately from all action records.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`

## Status

Complete.

