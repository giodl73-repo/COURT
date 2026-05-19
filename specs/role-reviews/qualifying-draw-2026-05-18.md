# Qualifying Draw Role Review

## Review target

Review the COURT foundation spec before implementing the Australian Open phase:
portable experience identity, provenance/custody classes, and action
availability.

Evidence packet:

- `docs\research\fontes-mit-game-courses.md`
- `specs\experience-framework-foundation.md`
- `specs\engine-adapter-contract.md`
- `specs\playtest-validation-contract.md`
- `specs\implementation-roadmap.md`
- `crates\court-core\src\lib.rs`
- RACKET consumer: `..\racket\crates\racket-core\src\lib.rs`

## Decision summary

Approved for Australian Open:

- `CourtExperienceIntent`
- `CourtProvenance`
- `CourtActionAvailability`
- tests proving the new fields are reachable from snapshots or experience
  records

Deferred:

- playtest, critique, focus-test, postmortem, and assessment records stay in spec
  until US Open.
- full RACKET unsupported-feature diagnostics wait until Wimbledon after COURT
  fields exist.
- product migration waits until Davis Cup.

Blocked:

- copying MIT third-party readings, commercial games, student projects, or media
  bytes into examples/tests.
- moving MUDDLE/RALLY runtime behavior under COURT before an adapter and product
  slice prove the value.

## Role decisions

| Role | Decision | Rationale | Required follow-up |
|---|---|---|---|
| Framework Steward | Approve for `court-core`. | Identity, provenance, and action availability are reusable contract shape, not product rules or engine implementation. | Keep game-specific rule predicates out of `court-core`; use labels/reasons, not executable policy. |
| Engine Adapter Architect | Needs compatibility shim. | RACKET can consume identity/action metadata, but unsupported-feature diagnostics should be explicit once fields exist. | During Wimbledon, RACKET must report unsupported action states/provenance classes instead of silently dropping them. |
| Game Design Methods Reviewer | Method-aligned for first slice. | Action availability and feedback expectations support rules clarity before richer playtest records exist. | US Open must add playtest/critique records before COURT claims method completeness. |
| Experience Assessment Reviewer | Assessment optional for Australian Open. | Identity/action/provenance can land before learning, impact, simulation, or comprehension claims. | Assessment hooks become required when a product declares educational, simulation, or social-impact intent. |
| Source Custody Reviewer | Custody-safe with provenance field required. | COURT can model source classes and references without copying restricted bodies or media. | Tests/examples must use product-authored placeholder text or source ids, not third-party course bodies. |

## Approved Australian Open scope

The next code slice may add:

1. `CourtExperienceIntent` with product owner, audience, design thesis, and
   non-goals.
2. `CourtProvenance` with product-authored, OCW-derived, metadata-only,
   local-cache, and external-boundary classes.
3. `CourtActionAvailability` with legal, unavailable, guided-illegal,
   destructive, and diagnostic/test-only states.
4. Optional provenance on scene nodes and the experience.
5. Tests proving snapshots expose only the contract shape.

The next code slice must not add:

1. executable product rule predicates,
2. renderer/backend behavior,
3. playtest participant details,
4. third-party content bodies,
5. a MUDDLE or RALLY migration.

## Exit decision

Qualifying Draw is approved. Proceed to Australian Open implementation with the
scope above.

