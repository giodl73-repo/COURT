# Author flow and release packaging pulse

Local author flow: fourteen browser cases pass with zero whole-test retries;
38 native/browser responses remain byte-identical. Screenshots at 390 px and
1440 px were visually inspected. File/original/current/LF-copy downloads retain
explicit boundaries; errors, cancellation, worker restart, stale completion,
input edits and oversized paste/drop retain input and remove stale exports.
Actual 5/12/10-second deadlines are exercised, not replaced with test-only values.

The first expanded run had eight passes/six failures: five were caused by a
CRLF fixture doubled during test setup or compared to normalized textarea text;
one exposed engine error status hidden by editing. The next run passed thirteen
and reproduced the engine-status bug. Its fix retains failure/restart guidance
through edits. Both failed reports are retained locally. New adapter/wave files
are UTF-8 LF through scoped attributes; native source/manifest/lock remain intact.

Packaging adds a clean-source ZIP, complete file/source hashes, pinned provenance,
original dependency/Rust notices and an explicit 5 MB budget. RUNE's historical
root MIT license is retained despite missing manifest SPDX metadata. No current
RUNE source or license is substituted. Private QA receipts are not in runtime
staging. Source/copy/source replay and hosted/live gates remain distinct.

AI-simulated role follow-up: framework boundary remains unchanged; destructive
availability and source text remain explicit. The workbench is not a renderer,
product host, provenance auditor or assessment validator. Browser/API/contract
readiness claims remain bounded by their own evidence.
