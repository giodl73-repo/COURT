# Snapshot inspector prototype

Local browser task: paste or edit a full snapshot, then inspect native available
commands, scene-role presence and unsupported-feature hints. A wholly authored
example covers all five action availability states. Original COURT contracts,
root Cargo files and RUNE evidence are unchanged from public main `9af0d6d`.
The standalone crate converts an explicitly labelled JSON DTO into native types;
only unchanged COURT methods decide query results. Adapter observations about
mismatched declarations/negative extents are not native validation errors.

This is a prototype, **not a Pages release**. No actions run. No product rules,
renderer selection, RACKET diagnostics, source fetch, validation-packet checking,
playtest inference, provenance verification or assessment success is provided.
Source IDs and caller text are inert. Available destructive commands are native
contract behavior, not a recommendation to execute them. Reports retain action
warnings and scene provenance under `snapshot`.

## Local qualification

Rust 1.95.0, wasm-bindgen CLI 0.2.127, Python 3.11+, Node 22 and
Playwright 1.61.1. Use a shared Cargo target if desired; never require TRACKER.
From the repo root (set `CARGO_TARGET_DIR` to an absolute build directory):

```text
cargo +1.95.0 test --workspace --locked
cargo +1.95.0 test --manifest-path browser/inspector/Cargo.toml --locked
cargo +1.95.0 clippy --manifest-path browser/inspector/Cargo.toml --locked --all-targets -- -D warnings
cargo +1.95.0 build --manifest-path browser/inspector/Cargo.toml --locked --release --target wasm32-unknown-unknown --lib
cargo +1.95.0 build --manifest-path browser/inspector/Cargo.toml --locked --bin court-inspector-cli
python browser/inspector/build-prototype.py --bindgen /absolute/path/to/wasm-bindgen
```

Then from `browser/inspector`: `npm ci`, `npx playwright install chromium`,
`npm run test:pages`. It serves `/COURT/inspector/` locally on port 4191.
Thirty-eight requests exercise unchanged native queries and strict DTO admission;
the browser requires exact serialized native/WASM response equality.

The native CLI reads at most 512 KiB + 1 byte from stdin; adapter failures are
JSON reports, while invalid UTF-8 is a nonzero I/O error. It is a preview tool,
not the framework's wire-format specification. JSON has strict field/enum
admission, signed 32-bit scene coordinates/extents and labelled item/byte limits.
Unknown extensions reject the entire input instead of disappearing on export.

Remaining gates: file import/original-byte download, stronger resource and stale
completion tests, desktop/mobile visual review, complete independently rebuildable
source/artifact/license downloads, provenance and size budgets, CI validation,
RACKET rehearsal where applicable, deployment of the tested artifact and final
live file/browser verification. Do not publish the prototype staging tree.
