# Shared snapshot decoder for RACKET

Status: implementation; release qualification pending.

RACKET's diagnostic workbench needs existing JSON admission without a second
schema. Expose `court_inspector::decode_snapshot` by factoring the parser;
inspector responses and errors must remain exact. No core, root Cargo, RUNE,
product rules or renderer contracts change.

Agent role review: Framework Steward accepts adapter reuse without core growth;
Engine Adapter Architect requires existing errors/native fields and downstream
RACKET rehearsal; Source Custody Reviewer finds no new content or rights inputs.
The synthetic fixture remains authored software test content. No product,
assessment or renderer-readiness approval is claimed.

Correct extraction custody: the old source replay temporary directory was inside
the checkout. Require a real system temporary path with spaces, failed Git
discovery and contained first-party manifests. Retain native outputs and rerun
full release/browser/source checks before recording release qualification.

## Local qualification of implementation 4c12e665

Seven foundation and six adapter tests pass, with strict native/host/WASM Clippy.
The packaged candidate verifies 73 inventoried files, 82 exact Git source files,
24 Cargo notice groups and 12 Rust notice files. All 16 packaged browser checks
pass, including 38 byte-exact native/WASM responses.

Independent ZIP replay passes all native tests, 14 rebuilt browser behaviors and
all 38 native responses. Extraction is in a system temporary directory with
spaces; Git discovery exits 128; both first-party manifests are contained in the
extraction. Cross-directory binary identity is not claimed.

Game Design Methods lens: defer player-study/rules changes to product owners;
this decoder adds none. Experience Assessment lens: no success claim is supported
by decoder/adapter tests. Hosted qualification, downstream RACKET rehearsal and
default-branch/live delivery remain distinct gates; PR 2 tracks this follow-up.
