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

- `docs\compatibility.md` defines the protected pre-1.0 experience contract,
  versioning rules, and mandatory RACKET downstream rehearsal.
- `docs\research\fontes-mit-game-courses.md` records the local MIT game-course
  evidence and rights boundaries.
- `docs\rune\README.md` describes the retained RUNE experience contract evidence.
- `specs\experience-framework-foundation.md` defines the first contract areas.
- `specs\engine-adapter-contract.md` defines what RACKET must prove without
  owning product rules.
- `specs\playtest-validation-contract.md` defines playtest, critique, and
  assessment evidence.
- `specs\role-reviews\foundation-closeout-2026-05-18.md` records the current
  pause point: COURT contracts, RACKET diagnostics/runtime smokes, AMAZE/TIGRIS
  fixtures, evidence references, and deferred migration work.
- `.roles\` defines the COURT review roles.

## Current stop point

The foundation wave is wrapped. COURT now retains RUNE descriptor evidence for
its portable experience contracts and is ready for additional product fixtures or
evidence references, but not for absorbing MUDDLE clients, RALLY reports, or
product rules.

## Non-goals

- No product-specific game rules in `court-core`.
- No full editor or asset pipeline in the foundation wave.
- No replacement of MUDDLE before COURT proves a cleaner generalized contract.
- No engine-specific rendering behavior in the core crate.

## License

COURT uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright Gio Della-Libera). Original
non-software content is licensed CC BY-NC 4.0 (copyright Gio Della-Libera);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.
