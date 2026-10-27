# Sealed engineering continuity — part575

- **Identity:** 44 lines, 3757 bytes, sha256:37ec11fce29dc1b542aa1aca554f9a720b73b1d69654f818deaf3754aece73b4
- **Coverage:** whole0423 private identity task/lesson/ledger and predecessor lesson pointers from c665276275d90bd0b70f3db5ef1536e39fbd101e.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 40275–42367, 2092 bytes, sha256:a964cd6f04ef77a144e35f0caad4c6610bf517d4a4fb19dff96645d5ae73c57a
- Whole source interval: DEV_NOTES.md bytes 193–1301, 1108 bytes, sha256:e5109720b69faad8465f1e9b65e3232a8433b5f113fef2307354121ec13e8a0e
- Whole source interval: CHANGELOG.md bytes 207–764, 557 bytes, sha256:85579a6b5d8a982ef3c6fa9153582897435f934c9fd2ff1388705ba1b256c1b2
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.2.b.3`
  Status: `done`
  Goal:0423 explicit private trusted identity reserve; strict generator order/actual whole ownership
  disjointness, independent staged cursor, real AddNotch candidate/report. Publication/refill remain owned.
  - [x] REPRODUCE / ISSUE: git show 93776a2:crates/sc-core/src/command/bus.rs shows no private ids
    or creation preparation; rc=0. Native AddNotch payload carries no client-supplied result identity.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --lib command::bus::identity:1 passed rc=0;
    direct generator use/aliased clone would consume original state; immutable value reserve avoids both.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    trusted constructor validates ids, stage owns iterator position, actual Notch/sewing/garment checks run.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:14 passed rc=0;
    repeated previews/bad anchor retain next id; exact original exhaustion/collision/anchor causes survive.
  - [x] NO REGRESSION: command_bus14 rc=0 includes prior actual reads5/Material5; identity staging1
    passes rc=0. No publication/history/refill/global persistence claim. Full integration remains .6.9.
  - [x] LOCKSTEP: DEV_NOTES promotion in identity decision; README/book/live pointer/task/ledger aligned.
    transition_receipts.py --record docs/history/stitchcad-devnotes-part574.md retains whole0422 lesson;
    1 exact Git span rc=0; verify-retention OK rc=0. Book build/publication80 chapters/159 tables/
    1461 rows/2436 local links rc=0; maintained-reference81 files/17409 lines/1215580 bytes rc=0.
    cargo test -p sc-core --doc command::bus:3 passed rc=0; selected fmt --check rc=0.
  Cargo invocations use scripts/local_environment.py. Production root supplies reserved output from
  its injected clock/entropy generator; test/replay uses deterministic generator. Requests cannot mint ids.
  Empty reserves refuse explicitly. Persistent global nonreuse/refill/publication remain .6.5.4/.6.8/.7.
## _(2026-10-27 UTC)_ — private reserved identities and Notch preview

0423 .6.5.2.b.3 owns explicitly reserved generator ids; checks strict order/complete current ownership.
Creation stages an independent immutable iterator, with no external provider call or Clone assumption.
Real AddNotch preview resolves actual Piece/anchor, builds complete sewing/garment candidate and
returns only exact proposed Notch/base revision. Repetition/refusal leaves reserved ids unchanged.
Empty reserve returns original exhaustion; profile bindings remain DeferredToG4. Publication/runtime
refill/persistent global issuance stay owned, with no default counter/entropy source in domain code.
Focused bus14/staging1/privacy3/strict selected lint/fmt check this scope. Promotion: identity decision.

[Whole0422 private Material-preview lesson](docs/history/stitchcad-devnotes-part574.md) retains its original registry/source/diff and focused proof boundaries.

[Whole0421 canonical task-source/byte-storage lesson](docs/history/stitchcad-devnotes-part573.md) retains original migration controls and exact proof boundaries.

## STITCHCAD-G1-0423 — stage private identities for actual Notch preview

- Explicit trusted reserve rejects repeated/reversed ids and collisions anywhere in the actual owner.
- Private staged iterators preserve the reserve on preview/refusal/exhaustion without provider calls.
- AddNotch inspect-preview builds/validates a real candidate and reports its exact tentative Notch.
- Focused bus14/staging1/privacy3/lint/fmt; existing Material/query behavior preserved.
- No publication/refill/global persistence claim; profile bindings remain DeferredToG4.

