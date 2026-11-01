# COURT snapshot workbench qualification and delivery

Candidate `0bfae0e8b89bd463d3803f4854139ca925b0989e` passes hosted run
[41361446381](https://github.com/giodl73-repo/COURT/actions/runs/41361446381).
All seven integrated foundation and five adapter tests pass, with strict
foundation/adapter formatting and native/WASM Clippy. Six RACKET downstream
checks also pass through a metadata-confirmed local COURT override; the separate
[rehearsal record](downstream-rehearsal.json) preserves its exact scope and the
initial incorrectly configured attempt. No downstream checkout or pin changed.

Sixteen hosted browser cases pass with zero skips/flaky outcomes or automatic
whole-test retries. They cover own input, authored example, empty results,
raw BOM/CRLF and explicit LF-copy downloads, invalid UTF-8, oversized file/paste/
drop, keyboard/mobile, deadlines/restart/crashes, stale file/job/worker messages,
inert text, original/current/report separation, exact native/WASM equality and
independent source/build/legal access under WASM failure.

The delivered source ZIP independently rebuilds outside Git in a path containing
spaces: seven foundation tests, five adapter tests, fourteen browser behavior
checks and all 38 native requests/responses match. Original/rebuilt WASM hashes
are retained, with no binary-identity claim. Descriptive 2,447-byte example
measurements include browser/worker/message overhead and record user agent and
hardware concurrency; they are not comparative performance evidence.

All 73 candidate inspector files and the pinned root landing page are verified
again after artifact download. The source ZIP has 79 exact Git source files;
24 Cargo package notice groups and twelve Rust license files plus the original
Rust library copyright remain separate from software/content grants. No current
RUNE license replaces its pinned historical MIT grant. Private QA cases, local
Cargo metadata, scratch requests and captures are excluded from the public tree.

## Promotion and current source

The workflow validates PRs with read-only repository access. Main must repeat
qualification and deploy only the tested public tree. Deployment rechecks exact
source commit and all downloaded artifact hashes before the scoped Pages job.
The qualified helper is at `/COURT/inspector/`; the root landing page links it.
This dated candidate record does not claim every subsequent main revision is
live-verified. Check deployed `inspector/provenance.json`, the successful default-
branch build/deploy and exact live bytes/browser evidence for that revision.

## Boundaries retained

COURT production Rust is byte-identical to public entry `9af0d6d`; full native
source/tests, root manifests/lock and RUNE fixtures equal the integrated existing
PITFALL snapshot `8c75bfb`. Input limits and declaration observations belong to
this adapter. It does not execute actions, infer product legality, certify an
engine/renderer, resolve source references, validate rights/workspace custody,
claim player/learning success or authorize MUDDLE/RALLY migration. Original
readiness boundaries and product/adapter authority stay intact.

AI-simulated final role review: custody accepts authored data/original notices;
framework accepts the isolated adapter and exact production boundary; engine
review accepts the scoped rehearsal without renderer claims; methods and
assessment review retain product evidence ownership. This is not human approval.
