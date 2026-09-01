# COURT Invariants

## COURT-I-01: Player-Available Actions Are Deliberate

**Status:** VERIFIED

**Invariant:** Legal and destructive actions are player-available; unavailable, guided-illegal, and diagnostic actions are not player-available by default.

**Why it matters:** Engines can expose actions without deciding product legality.

**Evidence:** `crates/court-core/src/lib.rs`.

**Test:** `cargo test --quiet`.

## COURT-I-02: Snapshot Queries Are Deterministic

**Status:** VERIFIED

**Invariant:** COURT snapshots expose available commands, scene-role checks, and unsupported feature hints with deterministic query behavior.

**Why it matters:** Adapters and tests need stable snapshot semantics before rendering or runtime choices.

**Evidence:** `crates/court-core/src/lib.rs` and `docs/compatibility.md`.

**Test:** `cargo test --quiet`.

## COURT-I-03: Validation Packets Reference Evidence Without Owning It

**Status:** VERIFIED

**Invariant:** COURT validation packets can count findings and reference product-owned evidence while leaving scripts, player details, and report bodies outside `court-core`.

**Why it matters:** COURT should preserve evidence shape without becoming a product research archive.

**Evidence:** `crates/court-core/src/lib.rs` and `specs/playtest-validation-contract.md`.

**Test:** `cargo test --quiet`.

## COURT-I-04: RUNE Fixture Matches Generated Descriptors

**Status:** VERIFIED

**Invariant:** The generated RUNE descriptor collection matches `docs/rune/experience_contracts.json`.

**Why it matters:** Retained descriptor evidence must reflect the actual Rust contract registry.

**Evidence:** `docs/rune/experience_contracts.json`, `docs/rune/README.md`, and `crates/court-core/src/lib.rs`.

**Test:** `cargo test --quiet`.

## COURT-I-05: Foundation Gate Includes Static And Runtime-Free Checks

**Status:** VERIFIED

**Invariant:** COURT foundation changes are checked by formatter, Rust tests, strict clippy, and whitespace validation before portfolio adoption.

**Why it matters:** Contract foundations should stay green before downstream rehearsal or product fixture work.

**Evidence:** `README.md`, `context/waves/2026-05-18-court-foundation/WAVE.md`, and `docs/compatibility.md`.

**Test:** `cargo fmt --all -- --check`, `cargo test --quiet`, `cargo clippy --workspace --all-targets -- -D warnings`, and `git diff --check`.

## COURT-I-06: Readiness And Migration Boundaries Are Machine-Readable

**Status:** VERIFIED

**Invariant:** COURT contract proof, adapter rehearsal, product readiness, and
MUDDLE/RALLY migration claims expose what they can support and which product,
release, or downstream-adoption claims they cannot support.

**Why it matters:** A green shared contract is valuable evidence, but it does
not prove players understand or enjoy a product and does not authorize a broad
MUDDLE/RALLY migration.

**Evidence:** `docs/court-readiness-boundaries.v1.json` and
`crates/court-core/src/lib.rs`.

**Test:** `cargo test --quiet`.
