# COURT Playtest and Validation Contract

## Purpose

COURT must support iterative game-design practice, not only runtime snapshots.
This contract names the evidence product repos should be able to attach to an
experience as it moves through prototype, critique, playtest, and release gates.

## Evidence records

COURT should eventually represent these records as product-neutral data:

| Record | Required fields |
|---|---|
| Prototype revision | Experience id, revision id, design thesis, changed areas, non-goals. |
| Playtest session | Session id, audience, build/revision, script, observed blockers, completion outcome. |
| Critique finding | Reviewer role, finding id, source scene/action, severity, recommendation. |
| Focus-test finding | Test prompt, player action, observed comprehension, follow-up change. |
| Assessment target | Learning/impact/simulation/comprehension claim, evidence needed, pass/fail rule. |
| Postmortem note | Release/build id, what worked, what failed, next design constraint. |

## Validation surfaces

The first validation path should stay lightweight:

1. Unit tests prove records can be created without product-specific types.
2. Engine adapters report whether they preserve or ignore validation markers.
3. Product repos keep playtest scripts and player findings outside `court-core`.
4. TRACKER can later aggregate validation status across product repos.

## Required boundaries

- COURT may store evidence shape and references.
- Product repos own player-study details, game-specific scripts, and final
  interpretation.
- FONTES-owned source records can inform design methods, but COURT must not copy
  third-party readings, games, student projects, or media bytes.

