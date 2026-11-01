# COURT inspector build and delivery

The portable preview uses the unchanged `court-core` at the exact source commit
in provenance.json. Full first-party Git source, including both Cargo locks,
native tests, the authored fixture, compile-time RUNE descriptor fixture,
browser tests and build scripts, is in source.zip. Source-file hashes are listed
in source-files.json. Root software/content and original dependency grants remain
separate. The Rust library notices are copied verbatim from Rust 1.95.0.

Tool versions: Rust 1.95.0 (wasm32-unknown-unknown), wasm-bindgen CLI 0.2.127,
Python 3.11+, Node 22, Playwright 1.61.1. The README in browser/inspector documents
native tests, strict lint, release WASM, native CLI and local browser qualification.
No TRACKER paths or secrets are required. Set CARGO_TARGET_DIR to any absolute
build directory. Use a clean Git checkout for package-site.py; the portable source
ZIP is for independent build/test replay and does not pretend to have Git metadata.

Compiled WASM: pkg/court_inspector_bg.wasm. JavaScript adapter:
pkg/court_inspector.js. These are distinct from Rust source in source.zip.
The source archive is a complete first-party checkout; Cargo obtains versioned
crates and exact RUNE Git revision from the committed lockfile. It is not an
offline vendored dependency archive. There is no cross-directory binary identity
claim; independently rebuilt native/WASM responses must match the original cases.

The release candidate stages only explicit runtime files, source, provenance and
notices under target/court-release/COURT/inspector. Native QA input/output cases,
Cargo metadata (which includes local paths), scratch requests, test reports and
screenshots stay outside that tree. The 5 MB inspector budget is enforced.
Publication also requires hosted checks, complete artifact verification before
and after artifact transfer, deployment from main, and fresh live verification.

Independent delivered-source replay (after clean packaging):

```text
python browser/inspector/verify-source.py --bindgen /absolute/path/to/wasm-bindgen
```

It extracts the delivered ZIP outside Git, repeats seven foundation and five
adapter native tests, builds native CLI/release WASM, requires identical inputs
and serialized native responses, and runs fourteen rebuilt browser behavior
checks including native/WASM equality. The two exact original-artifact delivery
cases are deliberately separate; the full packaged site requires all sixteen.
Original/rebuilt WASM hashes are recorded without claiming binary identity.
The owned temporary extraction is removed after QA receipts are retained.

The feature branch merges the two pre-existing PITFALL commits from TRACKER's
canonical COURT snapshot `8c75bfb`. These add two readiness-boundary tests and
compile-time policy fixtures; all seven foundation tests pass. Production code
before `#[cfg(test)]` is byte-identical to public entry `9af0d6d`; full foundation
source, manifests, lock and RUNE fixtures are now protected against `8c75bfb`.
This retains existing work rather than dropping it during pointer advancement.
