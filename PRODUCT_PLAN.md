# COURT Product Plan

## Thesis

Rust has strong pieces for deterministic state and rendering, but the portfolio
needs a scalable experience framework: one portable contract that can scale down
to scripts and tests, then scale up to browser UX, native engines, and future
authored scene engines without forking product rules.

## Product promise

A product can expose state, actions, scene meaning, and UX intent once, then run
through multiple engines or surfaces.

## Dependency placement

| System | Responsibility |
|---|---|
| MUDDLE | Current room-command proof and source of validated client/snapshot lessons. |
| COURT | General scalable experience contracts for state/actions/scenes/surfaces. |
| RACKET | First COURT engine proving native execution over COURT contracts. |
| Product repos | Domain rules, product scene direction, fantasy, and content. |

## First wave

The foundation wave proves the smallest product-neutral core:

1. Define `court-core` workspace and snapshot/action/scene primitives.
2. Prove at least one testable host/snapshot contract.
3. Scaffold repo-local wave and pulse process.
4. Prepare RACKET as the first downstream engine consumer.

## Non-goals

- COURT does not replace MUDDLE until repeated adapters prove the generalized
  contract.
- COURT does not own rendering, audio, physics, product rules, or scene art.
- COURT does not choose a full scene engine before RACKET proves the first
  adapter boundary.

