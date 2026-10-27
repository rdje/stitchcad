# Sealed engineering continuity — part579

- **Identity:** 51 lines, 4536 bytes, sha256:6c38230da19a80922a8b483e658dbe669a17403ce4fb613d7f410f5944989e72
- **Coverage:** whole0426 Measurement candidate/D283 task/lesson/ledger from 25f29c8958fbfde0adfca0a528d24f6c85d22c10.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 41451–44454, 3003 bytes, sha256:52a70433fc27f1c000b373a39b555b6dc31da6934ff895283e57b1637f7f4531
- Whole source interval: DEV_NOTES.md bytes 193–1110, 917 bytes, sha256:7294534fd2ed055954c31353c99cef2cb11efe277bfeaa3ddfae5535985a827b
- Whole source interval: CHANGELOG.md bytes 207–823, 616 bytes, sha256:6e98cd4b589bc763fcf672bf258b04c8a61a31f0e37349ed7eabb6025251ae47
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.1.2`
  Status: `done`
  Goal:0426 actual Measurement binding/id guard, validated private canonical candidate and exact
  actual diff. Unknown/Derived remain metadata; external truth/recipe execution remain separately owned.
  - [x] REPRODUCE / ISSUE: git show 411f74d:crates/sc-core/src/command/bus.rs prints preview dispatch
    for only Material/Notch, rc=0; detached canonical replacement alone supplies no actual Measurement binding.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-measure --test design_inputs_contract command_bus:
    18 passed rc=0; actual binding and shared consumers must use the single real canonical owner.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    bind actual metadata, require original declaration id, validate full candidate and normalize exact diff.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:18 passed rc=0;
    inspect-only Unknown/Derived/no-op, wrong id/missing metadata, original shared width/source chain.
  - [x] NO REGRESSION: cargo test -p sc-core --lib command::bus::difference:5 passed rc=0;
    prior bus14 also pass within18; privacy3/selected fmt/strict Clippy pass rc=0. Shared errors moved intact.
  - [x] LOCKSTEP: whole0425 scope/lesson/ledger retained577; README/book/pointer/status/decision aligned.
    Original broad handler-pending wording now names actual read/metadata subset. Native archive/book
    transition_receipts.py records577/578:3+1 exact Git spans rc=0; retention OK rc=0. Named publication
    summary:80 chapters/159 scoped API rows/1462 source links/2439 rendered links rc=0, equal to native
    411f74d original formatter. Exact tuple compatibility retained; full publication probes await .6.9.publication/pre-push.
    Actual expression/recipe/source truth remain .5e/.5f/evidence owners.
  D283 P2, owned/fixed here: manual tuple labels called159 scoped API rows tables and1461 source
  links rows. Native check return/main formatter proves the cause; all1031 retained records scanned,
  exactly575/576/577 affected. Those sealed bytes stay unchanged: read their159 as scoped API rows,
  1461 as source links and2436/2437 as rendered links. No table-count claim was earned by that check.
  Correct live0422 labels; return a named PublicationResult/summary to remove tuple decoding from callers. Native
  wc establishes PLANNING1194 lines and decision184; an ad-hoc print had mislabeled decision184 as
  planning after variable reuse. Stored planning cap assertion used the correct source; native paths
  and named producer fields replace those ambiguous diagnostic labels. No retention bytes are edited.
  Maintained-reference candidate81 files/17501 lines/1221811 bytes rc=0; git diff --check rc=0.
  Cargo commands use scripts/local_environment.py. New metadata reports confer no effect authority;
  model/revision/identity reserve unchanged on success/refusal. Full bus integration remains .6.9.
## _(2026-10-27 UTC)_ — real Measurement metadata candidates

0426 .6.5.3.1.2 resolves actual metadata binding, keeps its declaration id, rebuilds/validates the
whole candidate and derives an exact canonical diff. Reverting only the selected declaration in a
copied inventory proves every other canonical value/order unchanged; other owner fields are checked.
Real shared Material width rejects known zero with original source chain. Unknown/Derived remain
unevaluated; source/evidence truth and recipe execution are separate. Shared errors move intact to
bus/error.rs. Focused bus18/diff5/privacy3/lint/fmt pass. Promotion: command-layer decision.
D283 corrects mislabeled publication populations; named summary/native formatter agree. Sealed erratum in task.

[Whole0425 canonical-replacement task/lesson/ledger](docs/history/stitchcad-devnotes-part577.md) retains exact source proofs and prior lesson pointers.

## STITCHCAD-G1-0426 — preview actual bound Measurement metadata

- Resolve actual Measurement binding and refuse a different canonical declaration identity.
- Rebuild the real candidate; retain exact authored state/source and original shared-consumer errors.
- Derive exact canonical diff; refuse unselected/order/context/Piece/landmark drift, omit no-ops.
- Focused bus18/diff5/privacy3/lint/fmt; original state unchanged, no publication or recipe evaluation.
- D283: correct publication metric labels, preserve sealed historical bytes with an explicit erratum; named summaries match original native formatter.

