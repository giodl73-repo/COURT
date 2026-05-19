---
name: court-pulse
description: Execute one COURT pulse with validation.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# COURT Pulse

Use this skill to execute a single COURT pulse.

## Pulse requirements

- Read `context/waves/PHASES.md`.
- Read the active `WAVE.md`.
- Keep `court-core` product-neutral.
- Run `cargo fmt --check`, `cargo test --quiet`, and `git diff --check`.
- Update pulse status before commit.

