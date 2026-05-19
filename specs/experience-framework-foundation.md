# COURT Experience Framework Foundation Spec

## Purpose

COURT is a scalable experience framework. It should let product repos describe
state, actions, scene meaning, UX intent, provenance, and validation once, then
run those experiences through terminal, browser, native, and future authored
scene engines.

This spec is grounded in the local FONTES MIT game-course research pass in
`docs/research/fontes-mit-game-courses.md`.

## Design thesis

COURT should prove that experience design scales across surfaces without moving
product rules into engines. RACKET is the first real engine consumer, but COURT
must stay framework-first:

1. Product repos own rules, fantasy, scene direction, and assessment claims.
2. COURT owns portable contracts and reviewable experience intent.
3. RACKET and later engines own rendering/input/runtime implementation.

## Required contract areas

### 1. Experience identity

Every experience needs:

- stable id,
- title,
- product owner,
- intended surface class,
- player/audience description,
- design thesis,
- non-goals,
- provenance/custody summary.

### 2. Action and rule affordances

Every action needs:

- stable id,
- player-facing label,
- engine-facing command/action payload,
- legal-state predicate or explanation,
- recoverability policy,
- feedback expectation,
- optional rule/reference surface.

COURT should distinguish:

- legal action,
- unavailable action,
- illegal but guided action,
- destructive/irreversible action,
- diagnostic/test-only action.

### 3. Scene and UX intent

Every scene node needs:

- stable id,
- role: surface, zone, actor, prop, HUD, text, media, control, boundary,
- placement/transform intent,
- state frame,
- player-read label,
- product meaning,
- provenance/rights classification.

The scene graph is not just drawable geometry. It is also the handoff between
product scene direction and engine rendering.

### 4. Iteration and critique

COURT should support:

- prototype version labels,
- playtest session ids,
- observation records,
- critique findings,
- change rationale,
- postmortem hooks.

This keeps the framework aligned with iterative design instead of treating
snapshots as static UI data.

### 5. Assessment hooks

Experiences may need to declare:

- learning goal,
- impact goal,
- simulation claim,
- player comprehension target,
- field-test target,
- success/failure evidence.

These are optional for entertainment slices but mandatory for educational,
simulation, or social-impact experiences.

### 6. Engine adapter contract

An engine adapter such as RACKET must:

- consume COURT snapshots,
- render or plan the scene without owning rules,
- report unsupported node/action features explicitly,
- expose deterministic smoke checks,
- preserve provenance and boundary labels,
- surface recoverable errors to the player/test harness.

## First implementation deltas

Current `court-core` has minimal experience/action/scene/snapshot primitives.
Next deltas should add:

1. `CourtProvenance` for OCW-derived, metadata-only, local, product-authored, and
   external-boundary source classes.
2. `CourtActionAvailability` for legal/guided/unavailable/destructive/test-only
   states.
3. `CourtExperienceIntent` for design thesis, audience, goal, and non-goals.
4. `CourtAssessment` for playtest, learning, impact, simulation, and
   comprehension targets.
5. RACKET compatibility checks that fail when required scene/action roles are
   unsupported.

## Acceptance criteria

- COURT tests prove the new contract objects serialize through snapshots or are
  reachable from snapshots.
- RACKET tests consume the new snapshot fields without rule ownership.
- Specs cite FONTES evidence and role review records decisions before product
  migration.
- No product-specific game rules enter `court-core`.

