# Pulse 07: US Open

## Goal

Add product-neutral playtest, critique, focus-test, postmortem, and assessment
evidence records.

## Changes

- Add `CourtValidationPacket`.
- Add prototype revision, playtest session, critique finding, focus-test finding,
  assessment target, and postmortem note records.
- Add assessment claim types for learning, impact, simulation, comprehension, and
  entertainment-only experiences.
- Add tests proving COURT stores evidence shape and references without private
  player details, product-owned scripts, or third-party content bodies.

## Boundary

COURT stores evidence references and product-neutral record shape. Product repos
own player-study details, playtest scripts, interpretation, and any product
content.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`

## Status

Complete.

