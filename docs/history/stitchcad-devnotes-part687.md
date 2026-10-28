# Sealed engineering continuity — part687

- **Identity:** 51 lines, 5697 bytes, sha256:4a8375a7db3396572e0dade4a52e0496f8f7f61fc0e71699dc058ac8c621de87
- **Coverage:** whole0528 task scope, lesson and changelog from 85c72b5898525231a8018f10af5832700f909d50.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 21150–24562, 3412 bytes, sha256:fcbbf64ab55a6cee7c0b7ae3fdd14eaa638c14fc4c092bb6369d414bfabec519
- Whole source interval: DEV_NOTES.md bytes 193–1513, 1320 bytes, sha256:97cf30f716d6eef81469e8a7c639259ad32f0580aac3485499e9d1df94c80af8
- Whole source interval: CHANGELOG.md bytes 207–1172, 965 bytes, sha256:178999ed9139dd435a511332917c001a6861f8a978bb1b19eac3a2e1d53f26b2
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.oracle-watch`
  Status: `done`
  Goal:0528 P1 D306 wire all nine maintained seam oracle producers into discovered make probes
  and actual CI before new stop oracle/next push. Native fixtures and mathematical sources unchanged.
  - [x] REPRODUCE / ISSUE: 0528 git ls-tree -r --name-only 6759525d census →9 tracked seam
    *_oracle.py producers/28 run_*probe*.sh runners;git grep scoped .github/crates/scripts/Makefile/
    .githooks/artifact shell entrypoints for seam_partition/nonuniform_ease_oracle/notch_quantity_oracle
    →0 invocation matches,expected grep rc=1;combined Python assertion diagnostic rc=0. Existing
    native tests consume frozen TSV rows,while independent producers were only invoked manually.
  - [x] ROOT CAUSE: 0528 git ls-tree -r --name-only 6759525d plus scoped git grep census
    →9 producers/28 discovered runners/0 invocations at6759525dbb0d8ab9a8eb59670f9ffab66ae3d066,rc=0. Make only discovers run_*probe*.sh;
    standalone *_oracle.py files were never registered. CI's native/doctrine/handoff steps supplied
    no independent producer invocation. Missing automation did not invalidate observed oracle runs.
  - [x] FIX: 0528 explicit closed driver runs seven --check producers and streams older progress/
    Uniform stdout through exact cmp with pipefail. Discoverable runner composes actual producer
    gate plus finite controls;real doctrines CI step invokes that same runner. Inventory refuses
    missing/unknown producers before admitting success;unknown scripts never execute. Native
    local_environment.py -- bash run_seam_partition_probes.sh →9 exact producers and9 controls
    passed,rc=0. No data/kernel/fixture retuning or fallback success output.
  - [x] ADDRESSED: 0528 same focused runner →9 producers/11 frozen fixtures unchanged,9 finite
    controls pass,29 discovered probe suites and actual CI/Make wiring checked,rc=0. Baseline,
    stdout mismatch,failed --check child,correct stdout+failed child,missing/unknown producer,
    unsupported argument and removed CI/Make wiring all retain the intended verdict. Unknown
    producer sentinel remains absent;original statuses cannot be hidden by plausible stdout.
  - [x] NO REGRESSION: 0528 bash -n check_seam_oracles.sh run_seam_partition_probes.sh →rc=0.
    git diff --quiet HEAD -- crates docs/tasks/artifacts/seam_partition/*_oracle.py →rc=0;
    all actual mathematical producers/native fixtures unchanged. Fixture controls only copy known
    files to target/seam_oracle_watch,verify same-volume real directories and source hashes,then
    remove owned scratch. Book/publication92/223/1587/2745,686 receipt3 exact Git spans,retention1140
    logical/45 working/64024 decodedL/4958409 decodedB/2535746 residentB →pass,rc=0. Full CI remains
    required before push400;actual hosted CI verdict/monitoring follows that authorized push.
  - [x] LOCKSTEP: 0528 D306 tracked/fixed in PLANNING (four surplus blank lines tightened,not caps),
    LIVE inventory28→29/D306 fix,build book/task index/MEMORY/DEV/CHANGE align. README reviewed
    unchanged,G1 6/18. Whole0527 retained686/3 exact Git spans. Existing closed stop membership
    recovery recorded under pending .3.3;resume its owned native contract/bounded partition next.
    promotion: declined (repair enforces existing indexed claim-verification/CI doctrine;no new policy or physical assumption).
## _(2026-10-28 UTC)_ — D306 exact seam oracle gate repaired

0528 native Git census at6759525d proves9 seam producers/28 discovered probe runners/0 scoped invocation matches. Standalone oracle scripts were never registered with Make/CI;observed manual results remain valid. Own P1 D306 before next stop oracle/push;closed driver/discovered runner and actual CI step repair it.
Focused runner9 exact producers/11 fixtures unchanged and9 finite controls pass0. Preserve stdout cmp/pipefail and child --check status;missing/unknown producers,args and removed wiring refuse. Unknown sentinel never executes. Scratch copies stay target/seam_oracle_watch on this volume;source hashes unchanged.
Bash syntax/native source+oracle unchanged/book92/223/1587/2745/686 receipt3/retention1140 pass0. LIVE inventory29/D306 fixed;README/G1 6/18 unchanged. Actual hosted verdict follows push400/full gates. Mathematical producers/fixtures untouched. Closed stop membership recovery durably recorded;semantic partition next.
[Whole0527 nonuniform checkpoint](docs/history/stitchcad-devnotes-part686.md) retains exact commands/proofs. Promotion declined:existing indexed claim-verification/CI rule enforcement. No new physical assumption;stop cut references preserve one semantic object across adjacent closed children before effects.
## STITCHCAD-G1-0528 — wire exact seam oracle producers into required gates

- Fix P1 D306: nine standalone seam oracle producers had no CI/discovered-probe invocation at6759525d. Add closed exact driver,discovered runner and actual doctrines CI step. Seven --check producers and two streaming stdout/fixture comparisons preserve original statuses and
  complete inventory.
- Focused9 exact producers/11 fixtures unchanged and9 finite gate controls pass. Mismatch,failed --check or stdout child,missing/unknown producer,args and removed CI/Make wiring refuse;unknown sentinel never executes. Same-volume scratch/source hashes and Bash syntax pass;no
  mathematical/native fixture changes.
- Whole0527 retained686/three exact Git spans;book92/223/1587/2745,retention1140 pass. LIVE inventory28→29/D306 fixed;README/G1 6/18 unchanged. Actual hosted verdict waits for push400/full gates. Resume semantic stops;closed cut-boundary recovery is durable under .3.3.


