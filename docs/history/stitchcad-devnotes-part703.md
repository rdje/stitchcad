# Sealed engineering continuity — part703

- **Identity:** 49 lines, 5482 bytes, sha256:25c2ec97f2b77d8f22bd7434c0d9de02deeb21c17c1a2acdb5adf00fa5fea3c9
- **Coverage:** whole0544 task scope, lesson and changelog from da5ba09ab1f5b1e3502e5ce68f0ae6dbd4c4368b.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 26009–29366, 3357 bytes, sha256:972e09fd08b95db21980b02a34bc8ea6e3f4c0886158c47bd26d0d9e36be687a
- Whole source interval: DEV_NOTES.md bytes 193–1463, 1270 bytes, sha256:87caca1121bf0c4ccc30c5dfa28f8bc6db8d117b0ebc45082ecd757203e41b81
- Whole source interval: CHANGELOG.md bytes 207–1062, 855 bytes, sha256:315ea81dfd978349f29c3bffef3c427b1d50a864e649849a192bb7f30214e9ca
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.prepush-400.cleanup-mutation`
  Status: `done`
  Goal:0544 D313 P1 isolate protected-store mutant;D317 P1 repair stale frozen inspector pin
  exposed by focused companion. No production cleanup code,policy or preserved data changed.
  - [x] REPRODUCE / ISSUE: full0538 cleanup mutation failed when removed protected guard reached
    another Git-boundary refusal in populated Cargo store. GOOD protected refusal already correct.
    Focused companion after D313 repair →new isolation/restoration controls pass,then stale whole
    inspector0ab74212 identity fails,rc1 (D317);target/d313-fixture-contract.log.
  - [x] ROOT CAUSE (WHY + WHERE): actual inspect checks protected prefixes before traversal;
    mutant's real-store input makes an unrelated guard mask the intended counterfactual. Function
    globals are the actual namespace even when runpy returns a mapping copy. Git diff/log0ab74212
    →current cleanup.py show only certificate protection0285/c9e6da4c and idle-CUA0315/35e2650b,
    rc=0;companion whole-file pin predated both reviewed changes. No new production defect inferred.
  - [x] FIX: GOOD still checks genuine store;mutant uses an empty owned fixture temporarily
    appended to actual inspector protected prefixes. Keep every real prefix and restore in finally.
    Companion traps package-store traversal,checks expected/unexpected restoration and actual
    restore-omission red. Original full fixture AST comparison adapts only this operation/helper/
    additional real refusal. Entire inspector remains frozen to explicit reviewed0315 commit
    35e2650bb0bb20f59f2cf3d461416027682fa46a,not moving HEAD;original fixture baseline retained.
  - [x] ADDRESSED (verified): local_environment.py -- bash artifact_cleanup/
    run_artifact_cleanup_probes.sh →15 pass/0 fail,11 compiled guard assertion reds,rc=0.
    local_environment.py -- python3 -I -B ci_environment/python_cleanup_fixture_contract.py
    →capture/body omission/physical17/link/restoration/ownership/AST/root controls pass,rc=0;
    dedicated protected-store traversal trap and restore-body assertions turn red as required.
    Paths above are under docs/tasks/artifacts/. Frozen inspector identity now passes,rc=0.
  - [x] NO REGRESSION: companion --real →original15/11,six source inputs exact,zero fixture
    residue,no project cleanup apply,rc=0. certificate_store_controls.py →protected2/ordinary1/
    actual omission-red1/restored1,data/source exact,rc=0. Book build/positive publication →
    93/248/1590/2771,rc=0;receipt702 →3 exact Git spans,rc=0;retention1156 logical/61 working/
    65151 decodedL/5073013 decodedB/2650350 residentB,rc=0;git diff --check →rc=0.
  - [x] LOCKSTEP: whole0543 task/lesson/changelog retained702. Cleanup annex documents isolated
    fixture and explicit reviewed inspector pin. MEM/DEV/CHANGE/LIVE/index/ledger align;D313/D317
    fixed,29 suites/G1 6/18 unchanged. Cleanup cadence not reset;stores/certificates untouched.
    README objective/commands unchanged. Next D314 static-status,then D315 inventory before full
    gates/push/actual hosted jobs. G0 human closure remains unapproved;commit before moving.
    promotion: declined (application of indexed independent guard isolation,exception restoration and explicit frozen-source review contracts;no new cleanup policy).
## _(2026-10-29 UTC)_ — isolate protected-store faults and retain reviewed source identity

0544 D313:GOOD already protects the real store;removed guard was masked by a real nested Git checkout. Use an empty owned protected fixture in the actual function globals,retain all prefixes,restore on every outcome. Real store only sees intact protections;production cleanup unchanged.
D317:companion whole-file pin0ab74212 predates reviewed certificate0285 and idle-CUA0315 changes. Source diff/log confirms only those changes;pin whole inspector explicitly35e2650b,retain original fixture AST baseline with exact declared adapter. No moving HEAD or removed identity guard.
Cleanup15/11,fixture capture/body/physical17/link/restore/ownership/AST/root,real zero residue/six inputs,certificate omission controls pass0. Book93/248/1590/2771,702 receipt3/retention1156 pass0. Whole0543 retained;LIVE29 suites/G1 6/18 unchanged,D313/D317 fixed,D314–D315 owned.
[Whole0543 transport repair](docs/history/stitchcad-devnotes-part702.md). Promotion declined:indexed guard isolation/exception restoration/frozen-source review. No project cleanup apply,cadence reset or source/store/certificate change. Full gates/push/actual hosted jobs after remaining repairs;G0 human unapproved.
## STITCHCAD-G1-0544 — isolate protected-store faults and retain reviewed source identity

- Fix D313 using an empty owned protected fixture and guaranteed actual-prefix restoration. Preserve the genuine store refusal and all production cleanup protections. Add traversal-trap/restore-omission controls and an exact fixture AST adapter.
- Fix discovered D317 stale whole-inspector baseline by reviewing the two later certificate/idle-CUA changes and explicitly pinning0315. Retain frozen identity and original fixture baseline;no moving HEAD comparison or policy weakening.
- Cleanup15/11,fixture/physical17/real preservation,certificate,book/history checks pass. Whole0543 retained702/three exact Git spans. LIVE29 suites/G1 6/18 unchanged,D313/D317 fixed,D314–D315 owned;full gates/push/actual hosted jobs pending. No cleanup apply/cadence reset.


