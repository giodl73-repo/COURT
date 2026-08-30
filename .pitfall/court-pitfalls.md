# COURT Pitfalls

## COURT-PF-01: Contract Proof Becomes Product Readiness

**Status:** OPEN

**Pattern:** A green COURT contract, retained RUNE descriptor, or RACKET rehearsal is treated as proof that a product experience is ready for players.

**Domain:** README claims, foundation closeout, product fixture adoption, RACKET smokes, validation packets, and future public demos.

**Detection difficulty:** Portable shape tests and adapter smokes are concrete, but they do not prove player comprehension, enjoyment, learning, or release quality.

**Structural solution:** Require product-owned playtest, critique, assessment, and release evidence before claiming product readiness.

**Evidence:** `specs/playtest-validation-contract.md`, `specs/role-reviews/foundation-closeout-2026-05-18.md`, and `crates/court-core/src/lib.rs`.

## COURT-PF-02: Engine Convenience Leaks Into Portable Contracts

**Status:** MITIGATED

**Pattern:** RACKET or another adapter pushes renderer/backend/input convenience into `court-core` as if it were product-neutral experience shape.

**Domain:** `court-core`, RACKET compatibility, scene nodes, action availability, unsupported-feature hints, and future renderer/backend choices.

**Detection difficulty:** Adapter needs are often real, but they can overfit the first engine.

**Structural solution:** Keep RACKET compatibility as rehearsal evidence, require explicit unsupported-feature diagnostics, and preserve engine-owned rendering/runtime behavior outside COURT.

**Evidence:** `docs/compatibility.md`, `specs/engine-adapter-contract.md`, and `.roles/ROLE.md`.

## COURT-PF-03: Evidence References Become Evidence Ownership

**Status:** MITIGATED

**Pattern:** COURT evidence-reference records are treated as if COURT owns the underlying product playtest scripts, RALLY reports, player details, or interpretation.

**Domain:** validation packets, evidence references, playtest sessions, critique findings, assessment targets, and TRACKER aggregation.

**Detection difficulty:** References can look like bundled evidence unless ownership boundaries are explicit.

**Structural solution:** Store reference shape and summaries only; keep artifact bodies and final interpretation in product repos.

**Evidence:** `specs/playtest-validation-contract.md`, `crates/court-core/src/lib.rs`, and `specs/role-reviews/foundation-closeout-2026-05-18.md`.

## COURT-PF-04: FONTES Research Turns Into Reusable Content

**Status:** MITIGATED

**Pattern:** MIT game-course research, third-party readings, games, student projects, or media are copied into COURT examples/tests as reusable product material.

**Domain:** FONTES research packet, specs, tests, examples, role reviews, and future course-derived design methods.

**Detection difficulty:** Research precedent is useful for design vocabulary, but rights and source-custody limits still apply.

**Structural solution:** Cite the research packet for method grounding while keeping third-party content and media bytes out of COURT fixtures.

**Evidence:** `docs/research/fontes-mit-game-courses.md`, `.roles/ROLE.md`, and `specs/role-reviews/foundation-closeout-2026-05-18.md`.

## COURT-PF-05: Migration Momentum Absorbs MUDDLE Or RALLY

**Status:** OPEN

**Pattern:** COURT foundation success is used to justify moving MUDDLE clients, persistence, runtime behavior, RALLY reports, or product rules into COURT before a real product fixture needs it.

**Domain:** MUDDLE/RALLY migration proposals, product fixture work, dependency adoption, and future authored-scene engine plans.

**Detection difficulty:** A shared contract foundation can make broader consolidation feel inevitable.

**Structural solution:** Require neutral contract expression, adapter consumption, existing behavior preservation, and a concrete product benefit before migration.

**Evidence:** `PRODUCT_PLAN.md`, `README.md`, `specs/experience-framework-foundation.md`, and `context/waves/2026-05-18-court-foundation/WAVE.md`.
