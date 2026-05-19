# COURT

COURT is a scalable experience framework: product-neutral contracts for turning
one product state into many UX surfaces.

COURT is not a game product and not a full engine. It defines the portable
experience boundary: state snapshots, actions, scene nodes, UX intent, replay,
and validation surfaces that can be consumed by terminal, browser, native, or
future authored-scene engines.

The foundation design is grounded in the local FONTES MIT game-course corpus.
See `docs\research\fontes-mit-game-courses.md` and the review specs in
`specs\` before expanding `court-core`.

## First command

```powershell
cargo test --quiet
```

## Relationship to RACKET and MUDDLE

- COURT owns scalable experience contracts.
- RACKET is the first real engine that runs COURT contracts.
- MUDDLE remains the current room-command proof and learning source.
- Product repos own product rules, fantasy, and scene direction.

## Foundation review packet

- `docs\research\fontes-mit-game-courses.md` records the local MIT game-course
  evidence and rights boundaries.
- `specs\experience-framework-foundation.md` defines the first contract areas.
- `specs\engine-adapter-contract.md` defines what RACKET must prove without
  owning product rules.
- `specs\playtest-validation-contract.md` defines playtest, critique, and
  assessment evidence.
- `.roles\` defines the COURT review roles.

## Non-goals

- No product-specific game rules in `court-core`.
- No full editor or asset pipeline in the foundation wave.
- No replacement of MUDDLE before COURT proves a cleaner generalized contract.
- No engine-specific rendering behavior in the core crate.

