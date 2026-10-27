# Sealed engineering continuity — part593

- **Identity:** 61 lines, 5382 bytes, sha256:a55aadcd2aac9a76b681555bcc3feca72a875bab49b4672deb506b4b934465a2
- **Coverage:** whole0441 source-copy scope/lesson/ledger and D289 closure from 6fa816ace87a01652bd7dd36c8fcd14add8231bc.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 43851–46880, 3029 bytes, sha256:f9a225855a38ecd903b6835950c4c2cf45117ec5fb3ce1e7c81e386cb91c802e
- Whole source interval: DEV_NOTES.md bytes 193–977, 784 bytes, sha256:d22676f624fc28f60a7495c66b66c1a9ba7f97e21f882a7f7ddd249b8469b1c4
- Whole source interval: CHANGELOG.md bytes 207–703, 496 bytes, sha256:707610e77c65ea1fee3eae3f73920503160217bdbe31532fb2f8500d7ab23d79
- Whole source interval: docs/tasks/PLANNING.md bytes 246–1319, 1073 bytes, sha256:745aeddccdb1ecf97743f27a6c83e58ffeca83c939008f2273808d74887bfec5
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.1`
  Status: `done`
  Goal:0441 D289 bound source-host copy amplification while preserving actual validating reconstruction.
  Consume copied Structure/Inputs/Sewing definitions through crate-private methods instead of copying
  nested ownership again. No public extraction or mutable CommandBus model access; all constructors,
  original typed source failures and fresh final effect authority remain on the actual paths.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-measure --test design_inputs_contract
    command_bus::resource_copies:0 passed/1 failed rc=101; actual immutable source-host copies measured
    [3,3,2] against required [1,1,1] for individual Measurement/Material/Notch previews.
  - [x] ROOT CAUSE (WHY + WHERE): python3 -I -B native HEAD reconstruction census prints redundant
    nested copies2/2/1/2 in measurement.rs/material.rs/notch.rs/execution.rs rc=0. Outer definition
    already owns its copied H; cloning nested wrappers recopies the same opaque actual source host.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    three consuming crate-private helpers move copied fields through unchanged validating constructors;
    all four candidate/revision rebuild paths retain actual validators/difference guards and effect ordering.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:46 passed rc=0;
    real DesignSources/public readonly trait observer verifies individual1/1/1, mixed group preview3,
    commit4 (fresh revision included), exact alias no copy, replay/Undo/Redo0 and original Piece address.
    Instrumentation delegates actual source checks/bindings and never supplies a fabricated model/authority.
  - [x] NO REGRESSION: cargo test -p sc-core --doc design::DesignStructure:3 passed rc=0;
    design::inputs::DesignInputs:4 and design::sewing_state::DesignSewing:4 passed rc=0; external extraction
    remains refused. Selected cargo fmt check rc=0; prior45 actual bus cases retain semantic/failure coverage.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part592.md:5 exact
    Git spans rc=0; whole0440/D287/D288 and0432–0436 decision continuity retained without editing bytes.
    Retention1046 logical/76 aggregate working Markdown rc=0. Book/pointer/frontier/lesson/indexed
    decision aligned; full/deeper .6.9/.6.5.5 acceptance deferred.
  Cargo invocations use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Native book/publication81 chapters/159 scoped API rows/1473 source links/2468 rendered links rc=0;
  maintained-reference82 files/18063 lines/1260706 bytes owner .6.8.2.2.1 rc=0.
  For n non-alias members the bus requests n host copies for preview and n+1 for a new commit; explicit
  member policy bounds n. Host-internal copy/validation time/bytes, remaining collection copies, raw
  ingress and retained-state work remain .6.8.2.2.2. This is no total CPU/memory or integration certificate.
## _(2026-10-27 UTC)_ — bound candidate source-host copy amplification

0441 .6.8.2.2.1 fixes D289: native real-source observer measured individual copies3/3/2, regression
failed101; outer copied owners were cloned again at inner layers. Consume copied Structure/Inputs/
Sewing definitions privately, rebuilding through all original validators. Native bus46 verifies1/1/1,
group preview3/commit4, aliases/replay/Undo/Redo0 extra copies and original Piece address after Undo.
Strict selected lint/fmt and affected privacy3/4/4 pass; no public extraction/mutable model escape.
Promotion: command-layer record. Copy-call count is bounded, not opaque per-copy bytes/time, total
model work or retained memory. Remaining .6.8.2.2.2/deeper .6.5.5/full .6.9; whole0440/D287/D288 proof592.

## STITCHCAD-G1-0441 — bound source-host copies during candidate reconstruction

- D289: consume copied inner definitions instead of repeating opaque source-host clones; validators retained.
- Native real-source copies1/1/1, group preview3/commit4 and replay/Undo/Redo0; bus46 passes.
- Strict selected lint/fmt and affected compiler privacy3/4/4 pass; extraction remains crate-private.
- Book scopes invocation-count proof; whole0440/D287/D288 and0432–0436 decision continuity retained592.

## D289 — repeated opaque source-host copies during candidate preparation
P2 fixed0441, owner G1-SLICE.6.8.2.2.1: copied inner owners are consumed through private helpers.
- [x] ROOT CAUSE: native cargo test -p sc-measure --test design_inputs_contract command_bus::resource_copies:
  0 passed/1 failed rc=101; actual source-host counts3/3/2 exceed required1/1/1.
- [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:46 passed rc=0;
  actual individual1/1/1, group preview3/commit4, replay/Undo/Redo0 and original Piece address verified.
- [x] NO REGRESSION: selected strict Clippy/fmt rc=0; affected compiler privacy3/4/4 passed rc=0.
D287/D288 fixed0440; [whole native bypass/fix and package metadata proof](../history/stitchcad-devnotes-part592.md).
D285 runtime fixed0437; persistence/reopen issuance/order remains owned/pending in G1-SLICE.7.
[Whole D285/D286 diagnosis and verified runtime/claim closure](../history/stitchcad-devnotes-part591.md).
D283/D284 fixed; [whole correction/proofs](../history/stitchcad-devnotes-part581.md).

