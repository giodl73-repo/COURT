# COURT compatibility policy

COURT is a pre-1.0 shared experience foundation. Compatibility is deliberate
because engines and products compile against its snapshots, actions, scene
nodes, intent, provenance, validation records, and RUNE descriptors.

## Protected contract

The protected surface includes:

- public `court-core` types, traits, helpers, and error meanings;
- `CourtExperience`, intent, surface, provenance, action, scene, snapshot,
  host, validation, evidence, playtest, critique, assessment, and postmortem
  records;
- `CourtHost::snapshot`, `CourtHost::apply_action`, and `CourtHostError`
  behavior;
- action-availability semantics, including which actions are player-available;
- scene-role, unsupported-feature, provenance-boundary, and player-read
  meanings;
- snapshot metadata, experience identity, scene-contract version, action
  ordering, scene ordering, and deterministic query behavior;
- `court.experience_contracts` version `v0`, every retained `court.*` RUNE
  descriptor, field requirement, alias, stability marker, unit, and fixture;
- validation-packet evidence-reference and finding-count semantics; and
- the boundary that engines own rendering and runtime adaptation while product
  repositories own rules, content, fantasy, scenes, and win conditions.

Internal refactoring is compatible only when these observable contracts remain
stable.

## Versioning and migration

- Additive APIs, enum variants, optional metadata, or descriptors may remain in
  the current `0.y` line only when existing consumers retain their behavior.
- Breaking APIs, required fields, trait methods, enum handling, defaults,
  ordering, availability meanings, provenance meanings, or descriptor metadata
  require a minor-version bump while `court-core` is below `1.0`.
- An incompatible scene or RUNE contract requires a new contract version rather
  than silently changing `court.scene.v1` or a retained `v0` descriptor.
- Prefer deprecation plus migration notes before removing a public item.
- Migration notes must name affected engines and products, replacement APIs,
  descriptor changes, and any changed fallback or diagnostic expectations.
- Downstream repositories should pin commits for reproducible builds. A branch
  consumer must run the downstream rehearsal before accepting an update.

If a COURT update breaks RACKET, either migrate RACKET in the same admission or
restore its last passing COURT revision. Do not hide unsupported states or
weaken diagnostics to make the rehearsal pass.

## Foundation gate

From COURT:

```powershell
cargo test --workspace
```

This protects snapshot queries, action availability, scene roles, validation
records, RUNE metadata, and the retained descriptor fixture.

## Downstream breakage rehearsal

RACKET is the required first rehearsal because it is the native engine adapter
for COURT. Its tests construct real COURT snapshots and consume experience
metadata, available actions, scene roles, provenance, unsupported features, and
scene-contract versions.

Use a local Cargo patch for the COURT and RUNE sibling checkouts, then run from
RACKET:

```powershell
cargo test -p racket-core
```

A compile failure exposes public shape drift. Frame-plan, diagnostic, readiness,
or runtime-loop failures expose behavioral drift. RUNE fixture failures expose
descriptor drift.

COURT foundation changes are not ready until both the COURT gate and RACKET
rehearsal pass.
