# COURT Engine Adapter Contract

## Purpose

RACKET is the first real engine adapter for COURT. This contract defines what an
adapter must prove while keeping product rules in product repos and reusable
experience intent in COURT.

## Adapter inputs

An adapter consumes a COURT snapshot plus any declared experience metadata. It
must treat these as authoritative:

- experience id and version,
- available/unavailable actions,
- scene nodes and roles,
- UX intent and player-read labels,
- provenance and boundary classes,
- optional assessment/playtest markers.

## Adapter responsibilities

An adapter must:

1. Translate snapshots into frame plans, rendered frames, input affordances, or
   runtime events.
2. Preserve action availability and illegal-action guidance.
3. Preserve scene role and provenance metadata in diagnostics or debug surfaces.
4. Report unsupported node roles, action states, provenance classes, or
   assessment markers explicitly.
5. Provide deterministic smoke checks for frame-plan readiness.
6. Avoid inferring product rules from visuals, labels, or engine state.

## RACKET first proof

RACKET should continue as a small Rust-native consumer before selecting a full
rendering backend. The next reviewed proof should add compatibility checks for:

- required scene roles,
- unsupported action availability states,
- missing player-read labels,
- missing provenance on external or media-like nodes,
- frame-plan diagnostics.

## Non-goals

- RACKET does not author product scenes.
- RACKET does not decide legal actions.
- RACKET does not claim engine maturity from drawing alone.
- COURT does not hard-code a RACKET-only rendering model.

