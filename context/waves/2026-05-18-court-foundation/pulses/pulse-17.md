# Pulse 17: PITFALL doctrine integration

## Goal

Make COURT's reusable failure memory visible in the repo's normal foundation
workflow before additional product fixtures, RACKET rehearsals, or migration
proposals expand the portable experience contract.

## PITFALL findings

- `COURT-PF-01` remains open: a green COURT contract, retained RUNE descriptor,
  or RACKET rehearsal is not product readiness. Product repos still owe their
  own playtest, critique, assessment, and release evidence.
- `COURT-PF-02` is mitigated by keeping renderer/backend/input convenience out
  of `court-core` and requiring adapter diagnostics for unsupported features.
- `COURT-PF-03` is mitigated by storing evidence references and summaries
  without moving product-owned scripts, player details, or interpretation into
  COURT.
- `COURT-PF-04` is mitigated by treating FONTES as method grounding, not as a
  reusable content source.
- `COURT-PF-05` remains open: MUDDLE, RALLY, runtime behavior, reports, and
  product rules do not move into COURT unless a neutral contract, adapter
  rehearsal, behavior preservation, and concrete product benefit exist.

## Role coverage

- Framework Steward owns product-neutral scope and blocks product-rule or
  migration drift.
- Engine Adapter Architect owns RACKET compatibility without engine leakage.
- Game Design Methods Reviewer and Experience Assessment Reviewer keep product
  evidence separate from contract shape and bounded measurement hooks.
- Source Custody Reviewer keeps FONTES research, third-party examples, student
  work, and media out of reusable COURT fixtures unless rights are explicit.

## Integration

- README now points maintainers to `.pitfall/PITFALL.md` before framework
  expansion.
- The foundation wave table records PITFALL as a completed doctrine pass.
- Future COURT adoption packets should cite the relevant PITFALL IDs when they
  claim product readiness, adapter portability, evidence custody, research
  reuse, or migration readiness.

## Validation

- `cargo fmt --all -- --check`
- `cargo test --quiet`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `git diff --check`

## Status

Complete.
