# Sealed engineering continuity — part511

- **Identity:** 86 lines, 6879 bytes, sha256:90f066142b8f0d6d34994b2008ef0308985a0b6a6900a1f0eceb2d96a8157d18
- **Coverage:** complete committed common metadata task/lesson/ledger/D255 and historical operator-proof block from 1cf51944e64c291252fa5bc80cfd49bb35b8af84.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 50235–51636, 1401 bytes, sha256:ded9a78347d6c4632a48a280b4925e54954b08f0db341fbfcc7823b83177c5ba
- Whole source interval: DEV_NOTES.md bytes 193–1993, 1800 bytes, sha256:1d07d658b02160eb586124b75a31bd85fbb6b2a7f1aaf8dd46ab0787c0e9ac24
- Whole source interval: CHANGELOG.md bytes 211–889, 678 bytes, sha256:9c9013fa4174e86ab2c9fb644ce1d243d2b865df99f899a4af593002e976261e
- Whole source interval: docs/tasks/PLANNING.md bytes 87152–87793, 641 bytes, sha256:48b45d0361b658bd4d4f1fc02ac431fecff919dc40d52ca0eda8d0c4fe3970e6
- Whole source interval: docs/book/src/annexes/formula-operator-signatures.md bytes 4269–6548, 2279 bytes, sha256:df700e740532609a9db769f99f49f2759b36b22d119f403b12199ab6b0949fd1
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.2.d.4.3.b`
  Status: `done`
  Goal: immutable common SizeSet metadata/current contexts; D255 concrete break source corrected.
  - [x] REPRODUCE / ISSUE: replacement-context native case reports boxed payload seen=true,
    rc=101 before fix; target/size-metadata-d255-before-0359.log retains the actual failing assertion.
  - [x] ROOT CAUSE (WHY + WHERE): SizeBreaksError::Break source returned Box itself; selected
    cargo test -p sc-measure --test size_set_metadata_contract fails1 rc=101 before issue.as_ref().
  - [x] FIX: private common owner, complete cross-input census, nonescaping temporary contexts;
    native5/0 rc=0 rejects all5 common id-family collisions and finds concrete MissingDelta after D255.
  - [x] ADDRESSED: cargo test -p sc-measure --test size_set_metadata_contract --test
    measurement_inputs_contract gives5+12/0 rc=0, target/size-metadata-native-0359.log.
  - [x] NO REGRESSION: privacy3/strict selected Clippy/fmt/book/publication79/117/1456/2392
    pass0; D250132 exact baseline rows/delta0, glossary rc=1; full chart/Design/integration/CI later.
  - [x] LOCKSTEP: book/API/decision/defect/index/pointer aligned; transition_receipts.py510
    compares5 Git spans rc=0, complete0358/ordered-name/routing proofs; caps unchanged; G1still6/18.
    promotion: promoted to docs/decisions/decision_design-host.md. Commit0359; .4.3.c chart sources next.
## _(2026-10-27 UTC)_ — owned common SizeSet metadata and temporary current contexts

0359 SizeSetMetadata owns exact membership/axes/breaks and a referenced Design table identity.
All canonical/measurement/Ease ids precede set/member/axis/value/break ids in a complete census;
unselected tables cannot alias any common owned family. Existing actual source validators run
before the callback, using temporary contexts over original canonical declarations. A higher-ranked
visitor cannot return local context borrows. Original state and explicit partial break drafts remain;
chart-family composition/grade coverage/numeric execution/readiness are separate obligations.
MeasurementInputs identity iteration is extracted unchanged for reuse; original12 native cases pass.
New5 native cases protect metadata order/actual pointers, all5 common cross-table aliases, partial
coverage, missing exact table and missing current delta/callback suppression. The last exposed D255:
SizeBreaksError::Break source returned Box itself, losing concrete downcasts. Before rc=101 shows
boxed payload seen=true; issue.as_ref() now exposes actual MissingDelta. D255 is logged/owned/fixed.
Native5+12/privacy3/strict selected lint/fmt/book/publication pass0. D250132 rows exactly baseline,
delta0; full glossary rc=1. Continuity510 compares5 exact Git spans for whole0358 and historical
ordered-name/routing receipts; current behavior/commands retained. Caps unchanged; G1still6/18.
Promotion: core-host decision records temporary-context ownership. .6.4.2.d.4.3.c actual chart-family
sources next; full Design/execution/physical/profile/integration/CI remains unverified.


[Complete exact SizeSet-reference lesson](docs/history/stitchcad-devnotes-part510.md) retains
its original scope, checks and promotion decision.
## STITCHCAD-G1-0359 — own common SizeSet metadata over temporary current contexts

- Own exact membership/axes/break metadata, complete cross-input identities and nonescaping temporary contexts.
- Retain actual canonical objects, original current validators and explicit partial drafts; chart-family ownership follows.
- Fix D255: break error source exposes concrete cause rather than Box; failing before/after native case verifies the repair.
- Native5+12/privacy3/strict selected lint/fmt/book/publication pass0; D250132 rows exactly baseline, delta0.
- Whole0358/ordered-name/routing continuity510 matches5 Git spans; caps unchanged; G1still6/18; no full Design/CI claim.
## D255 — boxed break cause hides its concrete source type

P2; owner G1-SLICE.6.4.2.d.4.3.b, fixed in0359. Reproduce: size_set_metadata_contract's replacement
canonical-context case removes delta100; SizeBreaksError::Break exposes Box<SizeBreakError> through
Error::source, so concrete MissingDelta downcast fails (boxed payload seen=true, before rc=101).
Impact: generic error-source consumers lose the original typed cause. Return issue.as_ref(); the
same focused case must find actual SizeBreakError::MissingDelta, with callback unvisited and original
context still valid. Native/privacy/lint/book evidence remains in the owning leaf.
## Verification boundary

Public contracts enumerate all16 unary and640 ordered binary kind cases over the eight-kind
population. There are seven permitted unary cases and71 permitted binary cases, including17 ordered
multiplication pairs and14 directed quotient pairs. Exactly two cases carry the arc_length hint.
The tests compare the complete actual normative product/operator rows in both directions, and all
twelve metadata symbols agree with the existing independent canonical serializer.

```bash
cargo test -p sc-core --test formula_operator_signature_contract
python3 -I -B docs/tasks/artifacts/formula_structure/operator_signature_mutations.py
```

Exact wanted-catalog descriptors, arities and shared type variables are also verified.
Run mutations alone: fourteen compiled faults must fail at current test assertion sites, then restore the
exact source. They change tokens, count negation, square/product/comparison results, Boolean or
mismatched arithmetic, product commutativity, quotient direction/count rules and hint population.
The direct entry resolves implementation, public contract and its compile-time grammar chapter
before reading inputs or activating stores. Ordinary mode plans fourteen fault logs plus the
restored log and one mutable source. Classifier-only mode plans no files or mutable sources,
validates all inputs, anchors, stores and directory, and invokes no native child. Read-only
inputs may have several hard links; unused fault-log aliases do not acquire write authority.

Child calls preserve prepared stores and the original complete five-test target. Replay restores
source bytes before rebuilding that target, including after failures during mutation replay.
Qualifying failures must name a current assertion's exact path, line and UTF-8 byte column in a
failed-body thread panic. Custom macro messages qualify; nonthread notes, compiler errors,
labelled expect panics, changed coordinates and test-name noise refuse. The structural runner
watches actual anchors; independent mode controls check each input omission, complete captures,
physical refusals before reads/writes/dispatch and a real red for the former nonthread predicate.
Dispatch captures remain separate from exclusive compiled fault replay and restored-artifact proof.

End of whole0359 common metadata/D255 and historical operator-proof continuity.
