# COURT Role Review Brief

## Review target

Review `specs/experience-framework-foundation.md` before expanding COURT beyond
the current minimal `court-core` contract.

## Evidence packet

- `docs/research/fontes-mit-game-courses.md`
- `specs/experience-framework-foundation.md`
- `crates/court-core/src/lib.rs`
- RACKET consumer: `../racket/crates/racket-core/src/lib.rs`

## Roles

| Role | Review question | Required decision |
|---|---|---|
| Framework Steward | Does COURT define reusable contracts without becoming a product or renderer? | Approve, narrow, or reject each proposed contract area. |
| Engine Adapter Architect | Can RACKET and future engines consume the contract without owning rules? | Identify unsupported adapter requirements and smoke gates. |
| Game Design Methods Reviewer | Do the contracts reflect iteration, critique, playtesting, rules clarity, and documentation from the MIT game-design corpus? | Require missing design-method hooks before implementation. |
| Experience Assessment Reviewer | Do learning, impact, simulation, and comprehension claims have explicit assessment hooks? | Mark assessment hooks required/optional by experience type. |
| Source Custody Reviewer | Are OCW-derived, third-party, media, student work, and product-authored boundaries preserved? | Block any spec that collapses rights/provenance classes. |

## Proposed review outcome

The expected outcome is not "build a full engine." The expected outcome is a
small, reviewable COURT contract increment that RACKET can consume while product
repos keep rules and scene direction.

## Review records

- `role-reviews\qualifying-draw-2026-05-18.md`
- `role-reviews\post-laver-2026-05-18.md`

