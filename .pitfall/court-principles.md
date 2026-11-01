# COURT Principles

## COURT-P-01: Product-Neutral Experience Contracts

**Status:** ACTIVE

**Statement:** COURT owns portable state, action, scene, UX intent, provenance, validation, and descriptor contracts, not product rules or fantasy.

**Decision rule:** Any new `court-core` field must describe experience shape or evidence shape without deciding a product-specific rule.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `specs/experience-framework-foundation.md`, and `.roles/ROLE.md`.

## COURT-P-02: Engines Consume, They Do Not Infer Rules

**Status:** ACTIVE

**Statement:** RACKET and future adapters consume COURT snapshots and metadata, report unsupported features, and preserve boundaries without deriving legality or game policy from visuals.

**Decision rule:** Adapter-facing changes need compatibility diagnostics and must not move product-rule ownership into engine code.

**Evidence:** `docs/compatibility.md`, `specs/engine-adapter-contract.md`, and `specs/role-reviews/foundation-closeout-2026-05-18.md`.

## COURT-P-03: Product Repos Own Player Evidence

**Status:** ACTIVE

**Statement:** COURT may store product-neutral evidence references and record shapes, but product repos own playtest scripts, player details, RALLY reports, and final interpretation.

**Decision rule:** COURT records should point to product-owned artifacts rather than copying player-study bodies or product-specific conclusions.

**Evidence:** `specs/playtest-validation-contract.md`, `crates/court-core/src/lib.rs`, and `specs/role-reviews/foundation-closeout-2026-05-18.md`.

## COURT-P-04: RUNE Descriptors Are Retained Contract Evidence

**Status:** ACTIVE

**Statement:** COURT retains RUNE descriptors for the stable experience slice so agents and consumers can inspect contract shape without source scraping.

**Decision rule:** Descriptor changes must be deliberate contract changes and keep the retained fixture in sync.

**Evidence:** `docs/rune/README.md`, `docs/rune/experience_contracts.json`, and `crates/court-core/src/lib.rs`.

## COURT-P-05: Migration Requires Product Need And Rehearsal

**Status:** ACTIVE

**Statement:** MUDDLE, RALLY, and product capabilities move under COURT only after a neutral contract, adapter rehearsal, existing behavior preservation, and a real product benefit exist.

**Decision rule:** Do not run a big-bang migration or absorb MUDDLE clients/RALLY reports because COURT's foundation is green.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/compatibility.md`, and `context/waves/2026-05-18-court-foundation/WAVE.md`.
