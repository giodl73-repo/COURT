# Pulse 05: Roland-Garros

## Goal

Harden snapshot/state handoff and scene/UX intent without changing MUDDLE runtime
behavior.

## Changes

- Add `CourtSnapshotMetadata` with experience id/version, surface, and scene
  contract version.
- Add scene-node player-read label and product meaning.
- Add scene unsupported-feature hints for adapter compatibility planning.
- Add scene roles for media, control, and boundary nodes.
- Update tests so snapshots expose metadata and scene intent.

## Downstream

RACKET consumes the new metadata fields and counts unsupported scene-feature
hints, but full diagnostics remain deferred to Wimbledon.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `git diff --check`

## Status

Complete.

