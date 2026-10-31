# Sealed engineering continuity — part824

- **Identity:** 39 lines, 3854 bytes, sha256:91abea0666e53c4512e6ab0800f6b3f17c3050ea6c415f371f057aa31dc39ce3
- **Coverage:** whole0664 task scope, lesson and changelog from f009c491caf9c1802d71fd79e808e58fdfd85641.
- Whole source interval: docs/tasks/G1-SLICE-producers.md bytes 62183–64544, 2361 bytes, sha256:ffdf7385aa14d7166ac693e240c5fce962ff8eff30c617767927d0296ea2c389
- Whole source interval: DEV_NOTES.md bytes 193–1235, 1042 bytes, sha256:f6a8044e8b84a05c7775b2d3d09b2491f0b6ecb898852d4b62cdb8a1c96e9b01
- Whole source interval: CHANGELOG.md bytes 207–658, 451 bytes, sha256:c2c48c120130e46039867e088d7babdd33ca1686664897de8e5628eff6031a9f
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.n16.2.effects.platform`
  Status: `done`
  Goal:finish scoped platform full-entry mutable/copy/browser/temp/failure controls.
  Paths: docs/tasks/artifacts/formula_structure/irrational_platform.py;docs/tasks/artifacts/ci_environment/platform_runtime_contract.py;docs/tasks/artifacts/ci_environment/run_ci_environment_probes.sh
  - [x] ISSUE:D363 P1 new helper guard refuses full entry:TMPDIR changes without MAKE_TMPDIR.
  - [x] ROOT CAUSE (WHY + WHERE):platform_runtime_contract.py --diagnose
    254e136a84ea1d7922214ba92211d4dc8cf44393 →actual full main refuses first child with
    effective MAKE_TMPDIR is not local;no compiler/browser dispatched,rc=0. Helper-only model
    missed entry preparation. Existing owned fixture reads/mutations also need explicit admission.
  - [x] FIX:align MAKE_TMPDIR with selected TEMP;declare copied fixture input/mutable/output
    before reads,mutation and restoration. Retain full actual-entry watcher with simulated children.
  - [x] ADDRESSED:python3 -I -B docs/tasks/artifacts/ci_environment/platform_runtime_contract.py
    →full-entry success/six failures/six actual omissions pass,rc=0. Complete source/output/dir
    plans precede activation;caller stores/argv/versions/copied bytes/temp seal/latest are checked.
    Build failure,surviving bad expectation,interruption,version drift,temp replacement and early
    log alias cannot publish acceptance. Receipts explicitly identify simulated compiler/browser.
  - [x] NO REGRESSION:platform_dispatch_contract.py →two successes/eight refusals/four simulated
    cleanups/four omissions;python_numerical_inventory_contract.py →four545/six reds/platform31/
    405/seven reds/three ownership controls pass,rc=0. History823 four Git spans exact;book build
    and git diff --check pass,rc=0. Complete native/browser verdict remains .consumer.
  - [x] LOCKSTEP:book/live/notes/frontier/producer registry sync;D363 fixed. Promotion: declined
    (existing environment consistency/producer admission contracts). effects children complete;
    n16/n16.2 and consumer active/pending. G1 6/18/G0 unapproved unchanged. Complete transitive
    compiler/browser locality and full D156/.h2.b.v/pre-push/app/release retain separate owners.
  Commit:STITCHCAD-G1-0664;Verification:actual full-entry plans/effects with simulated children.
## _(2026-10-31 UTC)_ — complete full platform entry effects

0664 fixes D363:new helper verification exposes stale MAKE_TMPDIR after full entry chooses TEMP. Actual main diagnostic against254e136a refuses before compiler/browser. Align both values;explicitly admit copied fixture reads/mutation/restoration.
Full actual-entry watcher passes modeled success/six failure paths/six actual body omissions;plans/caller stores/argv/versions/source+fixture restoration/temp sealing/latest are checked. Receipts identify simulated compiler/browser;no numerical verdict.
Dispatch two successes/eight refusals/four cleanups/four omissions and original inventory/ownership remain green. History823 four exact Git spans and book build pass;[whole0663](docs/history/stitchcad-devnotes-part823.md) retained.
LIVE states/G1 6/18/G0 unapproved unchanged. Promotion: declined(existing environment/producer contracts). Effects children complete;next fresh native/Chrome. Full D156/.h2.b.v/pre-push/transitive/app/release scopes remain owned;PNT continues.

## STITCHCAD-G1-0664 — complete full platform entry effects

- D363:align browser temporary store aliases and explicitly admit copied fixture mutation. Actual
  full-entry model/six failure paths/six body omissions pass with source/copy/temp/latest controls.
- Dispatch/inventory/ownership/history823/book checks pass. Receipts explicitly simulate children;
  fresh numerical native/Chrome acceptance remains next. G1 6/18/G0 unapproved unchanged.

