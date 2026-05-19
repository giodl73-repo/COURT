# COURT Waves

Work is organized into small waves and pulses.

## Active wave

- `2026-05-18-court-foundation`

## Tournament roadmap

COURT implementation follows the tennis-tournament phase plan in
`specs\implementation-roadmap.md`.

| Phase | Tournament | Goal |
|---:|---|---|
| 0 | Qualifying Draw | Keep the current scaffold stable while role review finalizes the contract boundaries. |
| 1 | Australian Open | Implement experience identity, intent, provenance, and action availability in `court-core`. |
| 2 | Roland-Garros | Harden snapshot/state and scene/UX intent contracts against MUDDLE lessons. |
| 3 | Wimbledon | Make RACKET consume the reviewed contracts through explicit adapter compatibility checks. |
| 4 | US Open | Add playtest, critique, focus-test, postmortem, and assessment evidence records. |
| 5 | Davis Cup | Prove product-team adoption with one or two product repos without big-bang migration. |
| 6 | Laver Cup | Compare multiple surfaces/adapters and decide what can move from MUDDLE/RALLY into COURT. |

## Protocol

1. Read this file.
2. Read the active wave `WAVE.md`.
3. Read the target pulse under `pulses/`.
4. Implement the smallest complete slice.
5. Keep product-specific behavior out of shared kernels.
6. Update docs and wave/pulse status.
7. Run the repo validation commands.
8. Commit when green.

