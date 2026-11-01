# COURT snapshot inspector wave — November 1, 2026

Goal: let a reader inspect their own portable experience snapshot using existing
COURT Rust queries in a browser. Affected crate: standalone unpublished
`browser/inspector`; downstream RACKET remains unchanged. Native core/public API,
RUNE descriptors, root manifest/lock and dependency pins stay fixed at `9af0d6d`.
No new COURT framework fields, renderer, game rules or migration are proposed.

1. Qualify the narrow portable boundary with an authored fixture, native tests,
   release WASM and native/browser equality. Completed locally; see qualification.
2. Finish resilient author flow and independent source/build/legal delivery.
   Gate: own local file bytes, stale work, deadlines/failures/retry, mobile/keyboard,
   limits, source replay and appropriate downstream compatibility rehearsal.
3. Qualify hosted CI/Pages packaging and deploy the tested default-branch artifact.
   Gate: exact source/artifact pins, private QA exclusion, legal closure, size
   budgets, project prefix, least privileges and fresh live hashes/browser suite.
4. Record the released child first, then advance only COURT's TRACKER gitlink
   through a portfolio snapshot. No pointer advancement for the current prototype.

Non-goals: rendering/engine compatibility certification, action execution,
product state policy, validation-packet/assessment success, private studies,
third-party course/media copies, MUDDLE/RALLY migration, RACKET implementation.
A snapshot query is not a structural or semantic validity certificate.
