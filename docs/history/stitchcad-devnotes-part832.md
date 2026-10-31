# Sealed engineering continuity — part832

- **Identity:** 35 lines, 3563 bytes, sha256:32dde8d91ee47dcd2ca4f85100b5007e17dd6443a02630c8443adddea64fed0e
- **Coverage:** whole0669 task scope, lesson and changelog from cb843c964625bc186980ce01f59eee147933a5bd.
- Whole source interval: docs/tasks/G1-SLICE-producers.md bytes 32641–34691, 2050 bytes, sha256:3f4464127e93a88a1e57198c028231d3a4ceef29a3239a073f60d5c28bfd70ec
- Whole source interval: DEV_NOTES.md bytes 193–1243, 1050 bytes, sha256:45955433be08b600f459999bb160e17f48d3fdec76e9260ed477caeb3acc9914
- Whole source interval: CHANGELOG.md bytes 207–670, 463 bytes, sha256:6bfc131b3fa150aff6acb71c0cd2bc0d2e54caf6898978d2512ec1c2193a0996
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.a41.effects.provenance`
  Status: `done`
  Goal:D366 bind modeled receipts to their fixture emitter;keep historical captures immutable.
  Paths: docs/tasks/artifacts/ci_environment/python_numerical_runtime_contract.py;docs/tasks/artifacts/ci_environment/numerical_fixture_provenance_contract.py;docs/tasks/artifacts/ci_environment/run_ci_environment_probes.sh
  - [x] ISSUE:D366 P1 modeled receipt lists547 controller inputs but omits fixture emitter basis.
  - [x] ROOT CAUSE (WHY + WHERE):numerical_fixture_provenance_contract.py --diagnose on the
    before-reuse controlled report →emitter identity absent,rc=0. publish() adds scope/results
    after controller assembly without recording its own source/runtime inputs. Actual proofs separate.
  - [x] FIX:bind modeled report to repository-relative fixture path,frozen source SHA256 and
    Python version/implementation;refuse source-basis drift before exclusive receipt publication.
  - [x] ADDRESSED:python3 -I -B docs/tasks/artifacts/ci_environment/numerical_fixture_provenance_contract.py
    →actual identity/four field refusals/changed-basis no publication/two actual body omissions
    pass,rc=0. In-memory frozen-basis changes only;no source file is amended. Receipts stay SIMULATED.
  - [x] NO REGRESSION:numerical_fixture_startup_contract.py →valid/late refusal/existing-root
    refusal/two fresh roots/three startup+three ownership omissions pass,rc=0. Source/caller state
    exact;history831 four whole Git intervals/book build/git diff --check pass,rc=0. Full current
    fixture/own alias acceptance remains signoff;no numerical/full-suite verdict implied.
  - [x] LOCKSTEP:book/live/notes/frontier/registry sync;D366 fixed. Promotion: declined(existing
    artifact-emitter identity/immutable evidence contracts). a41/effects active;signoff next.
    Historical modeled/native/Chrome captures stay untouched;G1 6/18/G0 unapproved unchanged.
  Commit:STITCHCAD-G1-0669;Verification:actual modeled publication and source-basis guard controls.
## _(2026-10-31 UTC)_ — bind modeled receipts to fixture emitter identity

0669 fixes D366:modeled receipt records controller inputs but omits fixture emitter identity. publish() now records repository-relative path,frozen source SHA256 and Python version/implementation;changed source basis refuses before exclusive publication.
Actual modeled publication/four field refusals/changed-basis no publication/two actual body omissions pass. Only in-memory basis changes;source files remain exact. Startup/root-refusal/freshness/six omission controls remain green. Simulations provide no native numerical verdict.
[Whole0668](docs/history/stitchcad-devnotes-part831.md) preserves prior scope/ledger;history831 four Git spans/book/diff pass. Historical captures untouched;actual native/Chrome source basis unchanged.
LIVE states/G1 6/18/G0 unapproved unchanged. Promotion: declined(existing emitter identity/evidence contracts). a41/effects active;focused own alias/full modeled signoff next. Full D156/integration/pre-push remain owned;PNT continues.

## STITCHCAD-G1-0669 — bind modeled receipts to fixture emitter identity

- D366:record fixture source/runtime identity and refuse changed basis before publication. Actual
  publication/four field refusals/two body omissions and unchanged-source controls pass.
- Startup/ownership/history831/book/diff pass. Historical captures stay intact;simulated compiler
  scope remains explicit. a41 signoff next;no new numerical/full-suite verdict;G1 6/18/G0 unchanged.

