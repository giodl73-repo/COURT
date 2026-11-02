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
