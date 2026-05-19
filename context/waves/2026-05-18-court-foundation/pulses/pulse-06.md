# Pulse 06: Wimbledon

## Goal

Prove the engine-adapter contract through RACKET compatibility diagnostics.

## Changes

- RACKET reports diagnostics for action availability states that are not directly
  player-executable.
- RACKET reports unsupported scene roles and unsupported scene-feature hints.
- RACKET reports provenance boundaries that must be preserved rather than
  rendered as unrestricted product-authored assets.

## Boundary

RACKET still does not own product rules, rendering backend selection, or product
scene authorship. MUDDLE remains the working room-command and Macroquad prototype
surface.

## Validation

- COURT: `cargo fmt --check`, `cargo test --quiet`, `git diff --check`
- RACKET: `cargo fmt --check`, `cargo test --quiet`, `git diff --check`

## Status

Complete.

