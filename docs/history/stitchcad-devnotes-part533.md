# Sealed engineering continuity — part533

- **Identity:** 129 lines, 9600 bytes, sha256:4250bc7e38f9ae36e9f3763f6b061566f27c68233d7e8dfe29ab9a570905ea88
- **Coverage:** complete Hem cause task/lesson/ledger, D261 report and historical read-only reference evidence from 2e595e7bce26dccb89623824eaa0fc845379a2cd.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 50844–52130, 1286 bytes, sha256:85b4d3d1d763fe58d580759baa2a29c44a2ac0e31fbfe9cc5b970d4e189e11fe
- Whole source interval: DEV_NOTES.md bytes 193–842, 649 bytes, sha256:e6bb83c65c2cd34dd25cd2d2846a6e23c7caf2de93892e219f2f51e892b77422
- Whole source interval: CHANGELOG.md bytes 210–570, 360 bytes, sha256:6fc54fef2de78e27f8d94565ee10e89b79b1a413c40675cc6911c60c7727214c
- Whole source interval: docs/tasks/PLANNING.md bytes 88163–89061, 898 bytes, sha256:5f8cdc241c9e923e186c3bedef3475364933b88f6846552b8b577505a606698c
- Whole source interval: docs/book/src/annexes/formula-static-signatures.md bytes 1413–7753, 6340 bytes, sha256:f0eeff204144194935f18970a847923476f859d278426b887e7d779157052769
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.5.3.a`
  Status: `done`
  Goal: D261 preserve original InvalidFacing LayerError cause through HemError; exact red/green,
  unchanged validation/schema, current book/defect records and whole532 continuity.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test hem_contract
    hem_invalid_facing_error_keeps_original_layer_cause -- --exact:0/1 rc=101 at source() None.
  - [x] ROOT CAUSE (WHY + WHERE): same exact control rc=101; empty HemError Error impl hid
    its stored InvalidFacing LayerError from standard chained diagnostic tools.
  - [x] FIX: cargo clippy -p sc-core --test hem_contract -- -D warnings:rc=0;
    borrow original concrete LayerError; no wrapper erases its type.
  - [x] ADDRESSED: same exact cargo test control:1/0 rc=0; original concrete cause/pointer retained.
  - [x] NO REGRESSION: python3 Git-source comparison: every non-Error-impl production byte
    identical rc=0; fmt0, glossary132 failures rc=1, exact D250 baseline rows compare rc=0.
  - [x] LOCKSTEP: bash scripts/run_make.sh book:rc=0; scoped publication79/148/1454/2405 rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part532.md:4 exact spans rc=0.
    promotion: declined (local standard Error repair, canonical D261). Completed STITCHCAD-G1-0381.
## _(2026-10-27 UTC)_ — original Hem Facing error cause

0381 fixes D261: empty HemError Error impl hid the stored InvalidFacing LayerError.
Actual faced Hem/deleted Facing source exact control red0/1 rc101, green1/0 rc0 after direct borrowing.
Concrete type/same pointer retained; all production bytes outside Error impl identicaldd4fae3.
Strict affected lint/fmt/book/publication79/148 pass0; D250132 exact prior rows unchanged.532 retains
closed notion/Closure subtree/0380/retirement in4 exact Git spans0. Promotion declined: local standard
Error repair with canonical D261 report. Shared current layer/Hem validators next; full CI deferred.

## STITCHCAD-G1-0381 — preserve original Hem Facing error cause

- Fix D261 standard source chain; exact original concrete LayerError/pointer retained.
- Exact control red101/green0; all other production bytes identical; lint/fmt/book/publication pass0.
- D250132 unchanged; whole notion/Closure subtree/0380/retirement532. Shared layer/Hem validators next.
## D261 — Hem hid original InvalidFacing layer cause (fixed0381)

HemError's empty Error impl hid its stored InvalidFacing LayerError. Actual born faced Hem with
subsequently deleted Facing source: hem_invalid_facing_error_keeps_original_layer_cause in
hem_contract red0/1 rc=101, green1/0 rc=0 through local_environment.py; target/hem-cause-*-0381.log.
Concrete type/same original pointer retained; every production byte outside Error impl identical.
P2; owner G1-SLICE.6.4.5.3.a. Closed focused; full CI integration/pre-push.
- [x] ROOT CAUSE: same exact cargo test control red0/1 rc=101 at source() None; empty Error impl.
- [x] ADDRESSED: same control green1/0 rc=0 after original concrete cause borrowing.
- [x] NO REGRESSION: cargo clippy -p sc-core --test hem_contract -- -D warnings rc=0;
  non-Error-impl source comparison/fmt/book/publication pass0; whole532 receipt4 exact spans rc=0.
## Read-only reference plans

| Tooling name | Role |
| --- | --- |
| `Refusal` | Shared local-profile validation exception, distinct from formula diagnostics |

The signature, namespace and static-review helpers inspect their own bootstrap inputs before
loading another helper. Each borrowed reference call freshly validates the complete source list:
the shell reference, public helper chain, local profile and five normative book files (formula
contract, grammar, examples, reference skirt and units/tolerances). These calls create no stores,
files or child processes, and preserve prepared store values, unrelated environment values and
caller argv. The existing permissive `--mutations` recognition is unchanged.

Standalone invalid inputs print `local-environment: REFUSED — …` and exit2. A borrowed call that
reaches its reference plan raises the shared provider's `Refusal`; unrelated parsing/assertion
failures retain their own type. Missing files, symlinks, directory inputs and another Git boundary
refuse before reference reads. Invalid selected stores or toolchain names also refuse before effects.
Pure calls use only their declared inputs: they do not validate or copy the whole book tree.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_namespace_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/static_review_contract.py --mutations
python3 -I -B docs/tasks/artifacts/ci_environment/python_static_loader_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_static_loader_contract.py --native
```

The watched control executes all four original bodies: imports, ordinary and mutation CLI modes,
borrowed contracts, both reference loaders and exception-path argv restoration. Per-call source
plans are checked independently before reads and delegation; omission of an actual guard must fail.
Real isolated filesystem faults and disabled actual planner bodies distinguish refusal from admission.
The control's `--native` mode activates guarded local stores, runs the published first-phase and
coupled consumers, and checks every frozen input after each child. Logs, source identities and the
Cargo-current library identity are saved under `target/loader-*`; the probe runner watches this mode.
Every fixture and capture store stays under repository-derived `target/` on the repository volume.
Python3.9+ and its standard library are read-only runtime dependencies.

The static metadata contracts retain their execution traps. The review helper's D126 parameter
quantum control deliberately evaluates the reference arc-length counterexample and independent
rational bounds; its numerical proof remains separate from product geometry signoff. The native
bridge and copied-book instruments check nested read-only plans as well as their producing plans.

The dimension- and ambiguity-payload libraries use the same pure boundary: bootstrap plans
cover four and five sources, and fresh borrowed plans cover nine and ten. They reuse the
provider's Refusal class. Their original diagnostic metadata, operand/source order, kind/value
traps, case populations and CLI argument recognition remain unchanged. For example:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/dimension_payload_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/ambiguity_payload_contract.py --mutations
python3 -I -B docs/tasks/artifacts/ci_environment/python_payload_library_contract.py
```

The watched payload control checks repeated borrowed calls, exception-path argv restoration,
actual guard omissions, helper/book filesystem faults and unchanged prepared environments.
The native control above also runs both real payload mutation CLIs before the native consumers
and verifies frozen inputs after each. Diagnostic inference remains metadata-only.

The geometry-argument and call-lookup libraries also validate four bootstrap and nine reference
inputs. Geometry's original static refusals and successful numerical cache/provenance replay both
remain active; these in-memory checks create no files or stores. Call lookup retains callee-before-
argument refusal and its unchanged catalog. The standalone mutation flags retain their original rules.

The direct curve_contract() reader validates only its own script, the profile and the units/curve
chapter. It can inspect that table while an unrelated reference chapter is missing or an unused
log alias exists. A complete reference contract still refuses its missing required chapter.
The shared payload control checks these boundaries, repeated calls, all actual guard omissions and
physical helper/book refusals; the native control runs both original mutation CLIs before consumers.

`reserved_diagnostic_review.py` applies the same four-input bootstrap and fresh nine-input borrowed
plan before checking its canonical diagnostic row or compiling the reference. Its original 3,624
D131 source-bearing cases and nineteen faults retain formula_rebinding, actual available locations
and metadata-only inputs. It preserves permissive mutation argv and shares the provider's Refusal.
The payload control covers this fifth library; native mode runs its real mutation CLI too.
The [D131 decision](formula-static-validation.md#reserved-name-diagnostic-sources), grammar and
independent-approval boundary are unchanged by these file/source plans.

The location library validates six bootstrap inputs before either helper is loaded, then eleven
reference inputs before its original AST, traceback or delegated reference checks. Standalone
mutations also validate the selected helper source before reading or compiling it in memory.
Both loader parameters remain available; calls preserve caller argv and prepared stores. The
existing real filename/line/text checks and explicitly virtual fault filenames remain active.
For example, contracts() checks the signature prefix, review_contracts() checks the whole-review
prefix, and mutations(REVIEW_SOURCE, review=True) exercises the second actual loader. These calls
create no files or child processes. The shared static-loader control covers both borrowed paths,
repeated calls, selected-source guard omissions and physical helper/book refusals; native mode
runs the real CLI and both prefixes' original offset/filename faults before the engine consumers.

End of whole0381/D261 and historical read-only reference evidence.
