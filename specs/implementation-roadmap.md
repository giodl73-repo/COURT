# COURT Implementation Roadmap

## Purpose

This roadmap turns the COURT foundation spec into implementation phases named
after tennis tournaments. The sequence protects the current MUDDLE/RALLY working
systems while COURT proves a cleaner portable experience contract.

## Phase 0: Qualifying Draw

Goal: keep the current scaffold stable and make the spec reviewable before
framework growth.

Deliverables:

- Keep current `court-core` snapshot/action/scene primitives passing tests.
- Keep `docs\research\fontes-mit-game-courses.md` as the evidence baseline.
- Use `.roles\` for Framework Steward, Engine Adapter Architect, Game Design
  Methods, Experience Assessment, and Source Custody review.
- Do not migrate MUDDLE/RALLY behavior yet.

Exit gate:

- Role reviewers agree which fields move into code first.
- RACKET stays a separate consumer, not an embedded engine.

## Phase 1: Australian Open

Goal: implement the portable identity and action layer.

COURT layers:

- experience identity,
- action/rule affordances,
- provenance/custody classes.

Deliverables:

- Add `CourtExperienceIntent`.
- Add `CourtProvenance`.
- Add `CourtActionAvailability`.
- Extend tests so snapshots or experience records expose these fields.
- Keep product rules out of `court-core`; products still decide action legality.

MUDDLE/RALLY relationship:

- MUDDLE command labels and unavailable-command cues inform the shape.
- RALLY action/phase/action-budget primitives inform deterministic wording.
- COURT does not execute policy.

Exit gate:

- `court-core` can describe legal, unavailable, guided-illegal, destructive, and
  diagnostic actions without product-specific rules.

## Phase 2: Roland-Garros

Goal: harden snapshot/state handoff and scene/UX intent.

COURT layers:

- snapshot/state handoff,
- scene and UX intent.

Deliverables:

- Add stronger snapshot metadata for experience id, version, surface class, and
  scene contract version.
- Add scene-node fields for player-read label, product meaning, provenance, and
  unsupported-feature hints.
- Map MUDDLE visual-node and `MuddleClientSnapshot` lessons into COURT terms
  without changing MUDDLE runtime behavior.

MUDDLE/RALLY relationship:

- MUDDLE remains the working room-command client and Macroquad prototype.
- COURT extracts only the portable scene-intent vocabulary.
- RALLY is not required unless deterministic run ids become part of snapshot
  provenance.

Exit gate:

- A MUDDLE-derived fixture can be represented as COURT scene intent while MUDDLE
  still passes its existing tests.

## Phase 3: Wimbledon

Goal: prove the engine-adapter contract through RACKET.

COURT layers:

- engine adapter contract,
- unsupported-feature reporting,
- deterministic adapter smoke checks.

Deliverables:

- Extend RACKET frame plans to consume new COURT identity/action/scene fields.
- Add RACKET diagnostics for unsupported scene roles, action states, provenance
  classes, and assessment markers.
- Add deterministic smoke tests that do not require opening a window.

MUDDLE/RALLY relationship:

- MUDDLE Macroquad remains the reference prototype for native lessons.
- RACKET proves the cleaner engine boundary.
- RALLY validation can be reused later, but RACKET first needs contract checks.

Exit gate:

- RACKET consumes a reviewed COURT snapshot and reports unsupported features
  explicitly without owning product rules.

## Phase 4: US Open

Goal: add playtest, critique, focus-test, postmortem, and assessment evidence.

COURT layers:

- iteration and playtest evidence,
- assessment hooks.

Deliverables:

- Add product-neutral records for prototype revision, playtest session, critique
  finding, focus-test finding, assessment target, and postmortem note.
- Add optional claim types: learning, impact, simulation, comprehension, and
  entertainment-only.
- Add tests proving records can be referenced without carrying private player
  details or third-party content.

MUDDLE/RALLY relationship:

- RALLY validation findings and comparison reports inform evidence shape.
- COURT stores design evidence references, not full simulation metrics.
- Product repos own scripts, player details, and conclusions.

Exit gate:

- COURT can represent why a prototype changed and what evidence is required for
  a learning, impact, simulation, or comprehension claim.

## Phase 5: Davis Cup

Goal: prove product-team adoption without big-bang migration.

COURT layers:

- full experience contract slice,
- product adapter compatibility,
- migration safety.

Deliverables:

- Choose one low-risk product slice, likely AMAZE Prism Vault or TIGRIS
  Parliament, as the first COURT-described experience.
- Keep the existing MUDDLE path working.
- Add a COURT adapter fixture beside the existing product host.
- Compare existing MUDDLE output and COURT/RACKET compatibility diagnostics.

MUDDLE/RALLY relationship:

- MUDDLE remains the shipping/playable path.
- RALLY remains deterministic validation where already adopted.
- COURT proves it can describe the same experience more portably.

Exit gate:

- One product repo benefits from COURT without deleting or weakening existing
  MUDDLE/RALLY gates.

## Phase 6: Laver Cup

Goal: decide what graduates into COURT after multiple surfaces prove value.

COURT layers:

- cross-adapter comparison,
- migration decision records,
- framework maturity gates.

Deliverables:

- Compare browser/MUDDLE, native/Macroquad, and RACKET surfaces over the same
  COURT-described slice.
- Identify MUDDLE concepts that should remain MUDDLE-only versus move under
  COURT contracts.
- Identify RALLY concepts that should remain deterministic simulation substrate
  versus become COURT evidence references.
- Write migration decision records before any product depends directly on COURT.

MUDDLE/RALLY relationship:

- No capability moves under COURT unless at least one product benefits, RACKET or
  another adapter consumes it, and existing validation remains green.

Exit gate:

- TRACKER can state which COURT layer is production-ready, which remains
  experimental, and which stays in MUDDLE/RALLY.

