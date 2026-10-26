# Sealed archive — whole0346 canonical inputs and D251/D252

Immutable historical segment, sealed by leaf `G1-SLICE.6.4.2.b` on `2026-10-26` (UTC).

- **Sealed identity:** 61 lines, 5171 bytes, sha256:56d7ea2bd75f34b69095ff3910511f9d9bb55311a4eaf03c43a91eadc62ac526
- **Coverage:** complete committed canonical input task/reports/lesson/ledger from 21e16beb73899f9f75474f1f202051e7df1483cc.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 46626–48836, 2210 bytes, sha256:8a1a5da6eee92b52ef8d35a7dbf856bb5f6323eebb1d8f6630a7ed92b6bdff54
- Whole source interval: docs/tasks/PLANNING.md bytes 86425–87610, 1185 bytes, sha256:8d79dede438eb8bfe9101777143d6a0eb87e41fd8b1ed666ff8e1a34b88cd77d
- Whole source interval: DEV_NOTES.md bytes 193–1274, 1081 bytes, sha256:386dffb655d32a7fa20c9bfbee53da891d2759bd0a5cfe370505c8b1f22c75f1
- Whole source interval: CHANGELOG.md bytes 207–857, 650 bytes, sha256:2c63cfb250e316ff46a47a8d4ae0f74f01f065f505f887709862a3b36929af18
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.2.a`
  Status: `done`
  Goal: owned CanonicalInputs over exact LengthDeclaration/ScalarInputDeclaration values, with unique
  owned ids across both families and precise first/duplicate family positions. Preserve authored vectors,
  states/source/provenance, typed lookups and empty drafts. No scalar-Length conversion or source/evidence
  inference. No public mutator; sc-measure continues borrowing core records without a dependency cycle.
  Own selected native/privacy/lint, relevant API vocabulary/book/decision/live docs; D251/D252 scope
  corrections below. D250 remains the named preexisting glossary owner after .6.9, no blanket exemptions.
  Own complete prior0345 task/reports/lesson/ledger retirement497 before task-cap growth.
  - [x] REPRODUCE / ISSUE: core declaration families are already canonical/immutable; metadata borrows
    them. Private Design ownership needs one unambiguous owned inventory without a reverse dependency.
  - [x] ROOT CAUSE (WHY + WHERE): existing LengthDeclaration/ScalarInputDeclaration APIs store complete
    state/source but do not jointly own ids. canonical_inputs_contract cross-family/same-family cases
    exercise exact duplicate identity/family/position refusals while preserving references, native7/0, rc=0.
  - [x] FIX: CanonicalInputs owns original vectors, checks ids before publication and exposes shared typed
    views only. General scalar Length stays general; unknown/derived has no new result or fallback.
  - [x] ADDRESSED: python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test
    canonical_inputs_contract gives7/0, rc=0, target/canonical-inputs-native-0346.log; all5 states/6 kinds.
  - [x] NO REGRESSION: length8/scalar5/privacy1/strict selected Clippy/fmt pass0; book/pub77/105/1444/2358
    pass0, target/*0346.log. Glossary132 matches baseline rows exactly, delta0; full census rc=1/D250.
  - [x] LOCKSTEP: vocabulary/API/decision/notes/status/index/pointer/D251-D252 aligned; whole0345 task,
    reports/lesson/ledger497 compares4 exact Git spans0. Caps unchanged; G1still6/18; .6.4.2.b next.
    promotion: declined (existing canonical-source decision). Commit0346; full execution/CI remains later.
## D251 — canonical-input guide still defers implemented structural SizeSet composition

measurement-inputs.md's opening says full composition remains owned work. SizeSetInputs and current
Garment/Mtm input composition are implemented, reviewed under .4c.5/.4d, and sc-measure's module docs
state that scope. Reproduce: rg "full composition" in that guide and SizeSetInputs in sc-measure.
Impact: the canonical-input entry understates available structural inputs. Owner G1-SLICE.6.4.2.a,
P2 fixed0346: entry reports structural composition and keeps numeric execution/persistence pending.

## D252 — recipe-input annex still defers implemented numeric binding conversion

formula-recipe-inputs.md's status header says numerical binding and evaluation remain later work.
FormulaBoundValue and its binding conversion API/native contract are implemented; whole recipe execution
remains pending. Reproduce: rg "numerical binding" in the annex and public constructors in recipe/bound.rs.
Impact: readers cannot distinguish available conversion from unimplemented whole execution. Owner
G1-SLICE.6.4.2.a, P2 fixed0346: header separates available conversion from pending whole execution.

## _(2026-10-26 UTC)_ — owned canonical input inventory

0346 CanonicalInputs owns original dedicated LengthDeclaration and general ScalarInputDeclaration vectors.
One id census spans both families, preserving exact first/repeated roles and positions. Getter borrows
refer to the stored canonical objects; general Length-kind scalars do not become dedicated lengths.
All state/source/provenance remains authored, including unknown observations/derived formulas without
numeric defaults. Referenced ids are not mistaken for owned records. Empty drafts remain explicit.
Native7 covers all5 states/6 kinds, duplicates and canonical borrows; privacy1/length8/scalar5/strict
selected lint/fmt/book/pub pass0. D251/D252 wording separates structural composition/conversion from
pending numeric execution. D250's132 glossary rows remain exactly baseline; no full census/CI pass.
Whole0345 task/reports/lesson/ledger497 compares4 exact Git source spans. Promotion: existing canonical
source decision. G1still6/18; private structural state follows .6.4.2.b, without a reverse dependency.

## STITCHCAD-G1-0346 — own canonical inputs without representation conversion

- Add immutable CanonicalInputs over dedicated-length/general-scalar records, preserving all authored state/provenance.
- Refuse duplicate owned ids with both original family positions; retain typed canonical borrows and explicit empty drafts.
- Native7/privacy1/length8/scalar5/strict selected lint/fmt/book/publication pass0; D251/D252 scope prose corrected.
- D250 glossary132 remains exactly baseline; no new failure. Whole0345 continuity497 matches4 Git source spans0.
- G1still6/18; structural state .6.4.2.b next. No full Design/execution/integration/CI claim.

End of whole0346 canonical input continuity.
