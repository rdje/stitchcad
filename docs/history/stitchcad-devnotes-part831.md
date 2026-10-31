# Sealed engineering continuity — part831

- **Identity:** 39 lines, 4097 bytes, sha256:aa142d86a70b6666dc29fd32636ed7a8b3779f8c336d0dd69cfce63779ecf8a4
- **Coverage:** whole0668 task scope, lesson, changelog and defect ledger from afdee54c8e9298a013efefede04d37ba3667492e.
- Whole source interval: docs/tasks/G1-SLICE-producers.md bytes 32478–34537, 2059 bytes, sha256:3930744df56a116553f4d15a20f90f298988c4e7b1e6f2d7d6b6b49d7b1eb80a
- Whole source interval: DEV_NOTES.md bytes 193–1326, 1133 bytes, sha256:8b38e39c1657c13508afb9b7199b57d0a14258dfc91331501a23a0744ae814b2
- Whole source interval: CHANGELOG.md bytes 207–666, 459 bytes, sha256:4796297909e110e38f097fbd6f83d78451b43ff59503d4627812906b0b6ba3f8
- Whole source interval: docs/tasks/PLANNING.md bytes 94497–94943, 446 bytes, sha256:63f2e8e544c6d21f03502d18af98dd5c2b55e96130990ae081f3ef1474a030d6
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.a41.effects.ownership`
  Status: `done`
  Goal:refuse reused fixture roots and preserve prior artifacts before any setup/capture.
  Paths: docs/tasks/artifacts/ci_environment/python_numerical_runtime_contract.py;docs/tasks/artifacts/ci_environment/numerical_fixture_startup_contract.py
  - [x] ISSUE:D365 P1 actual bootstrap adopts an existing per-PID directory;actual modeled
    capture overwrites a prior controlled artifact. Original sources unchanged;no compiler/browser.
  - [x] ROOT CAUSE (WHY + WHERE):numerical_fixture_startup_contract.py --diagnose-reuse
    d440f708cce6746b784102248c47efec9889f80a →old actual modeled capture changes existing
    controlled artifact bytes,rc=0. Nonexclusive work prepare plus reused artifact names cause it.
  - [x] FIX:use UTC/PID work names;admit work/common-parent paths,freeze inputs,create only
    the validated common parent,reserve work exclusively,then activate stores. Keep existing inputs.
  - [x] ADDRESSED:numerical_fixture_startup_contract.py --macos-readonly-git →existing-root
    refusal/two fresh default roots/three startup+three ownership omissions/source/caller values
    pass,rc=0. Sentinel exact,no refused-run stores/env effects;metadata write-denial control passes.
  - [x] NO REGRESSION:python_numerical_runtime_contract.py →original4/30 simulated variants/
    seven failure paths/12 preflight/8 body faults;five tangent configurations/27 simulated variants/
    two failures/three wrapper faults;actual timeout/group/full source+copy restoration pass,rc=0.
    History828–830 six Git spans/book/diff pass,rc=0. No native or broader CI rerun implied.
  - [x] LOCKSTEP:book/live/notes/frontier/registry sync;D365 fixed. Promotion: declined(existing
    exclusive ownership/input-freeze contracts). a41/effects active;D366 emitter identity is owned
    next before signoff. Native/Chrome proof inputs unchanged;G1 6/18/G0 unapproved unchanged.
  Commit:STITCHCAD-G1-0668;Verification:actual root-reuse/refusal plus complete modeled fixture replay.
## _(2026-10-31 UTC)_ — reserve fresh fixture work roots exclusively

0668 fixes D365:actual prior bootstrap adopts an existing per-PID root and modeled capture overwrites a controlled artifact. Use UTC/PID names;admit parent/work paths,freeze inputs,create validated parent,reserve work exclusively,then activate stores.
Actual refusal preserves artifact/caller state;two default bootstraps get fresh roots. Three startup/three ownership omissions and optional metadata write-denial control pass. Complete original fixture4/30 plus tangent5/27 simulations/failures/refusals/actual timeout controls restore all bytes.
D366 sample modeled report547 records original inputs but omits fixture-emitter identity;owned a41.effects.provenance,next before signoff. Actual native/Chrome proof basis unchanged. [Whole0667](docs/history/stitchcad-devnotes-part828.md) and ledger829 retain prior scope.
History828–830 six Git spans/book/diff pass. LIVE states/G1 6/18/G0 unapproved unchanged. Promotion: declined(existing exclusive-root/freeze contracts). a41/effects stay active;full D156/integration/pre-push remain owned. PNT continues.

## STITCHCAD-G1-0668 — reserve fresh fixture work roots exclusively

- D365:existing roots refuse before stores/env changes;two default roots are fresh. Three startup/
  three ownership omissions pass. Original4/30+tangent5/27 modeled replay restores sources/copies.
- History828/829/book/diff pass. D366 modeled emitter identity owned next;actual native/Chrome
  proof basis unchanged. No new numerical/full-suite verdict;G1 6/18/G0 unapproved unchanged.

## D308–D366 — fixture work-root ownership and emitter identity

D308–D364:[whole prior ledger](../history/stitchcad-devnotes-part829.md). D365 P1 fixed0668;owner a41.effects.ownership:existing work root adopted/prior controlled artifact overwritten;fresh exclusive roots and six omissions now pass. D366 P1 modeled emitter identity missing;owner a41.effects.provenance,next before signoff. D334 P2 after G1/before G0;D223/PLANNING.5 open.
