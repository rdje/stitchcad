# Sealed engineering continuity — part700

- **Identity:** 38 lines, 3940 bytes, sha256:de74b4d9ffa924e915de37aaa356af5bc71bf65d39cf8d1d0f8cd3939d5809b9
- **Coverage:** whole0541 task scope, lesson and changelog from 5334d4e3c3e0e923420a1c1a9ef0dc59228dbe56.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 25518–27805, 2287 bytes, sha256:0469f7d968aa9e98ffc6c39c3951d9798ad60fb1e74cb20107e4adea4ebfcd5a
- Whole source interval: DEV_NOTES.md bytes 193–1296, 1103 bytes, sha256:0a6092c791f846d79c3e0e0887f213be7650aeec2fea6cdd4169fd8e77ec6fe2
- Whole source interval: CHANGELOG.md bytes 207–757, 550 bytes, sha256:5db2d0274d1f23d186567720fada01edd6d698113ac0105477b1cea8b89d9708
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.prepush-400.catalog-mutation`
  Status: `done`
  Goal:0541 D311 P1 missing-authority control follows stable permission key and removes one row.
  - [x] REPRODUCE / ISSUE: full0538 command probes →LEVEL-MISSING mutation absent and unchanged
    book accepted,2 failures,rc1;actual catalog17 commands/5 classes/5 levels still passes.
  - [x] ROOT CAUSE (WHY + WHERE): git log -G 'native preview uses|build a candidate group' --
    docs/book/src/spec/command-layer.md →0476 correctly updated Propose prose;old probe retained
    retired wording from G0,rc=0. Source K3 compares first-cell names against roadmap §7.8;
    prose is not permission identity. Read roadmap4.4/7.8 and current chapter7 before change.
  - [x] FIX: LEVEL-MISSING selects '| `propose` |';droprow requires one unique match before
    writing and deletes only that line. Missing/ambiguous setup refuses3 instead of selecting an
    arbitrary row. Checker/vocabularies/human-only approval remain unchanged. Book documents scope.
  - [x] ADDRESSED (verified): local_environment.py -- bash
    docs/tasks/artifacts/command_layer/run_command_layer_probes.sh →15 pass/0 fail,rc=0;
    actual mutated Propose row removed exactly once and K3 names missing authority,exit1.
    REAL17/5/5/0;unrelated prose control passes,exit0;missing chapter refuses2. Existing eight
    transport refusal/body-red controls,glossary refusal,future/historical positives remain green.
  - [x] NO REGRESSION: bash -n command probes →rc=0;mdbook build/positive publication
    →93/248/1590/2771,rc=0;receipt699 →3 exact Git spans,rc=0;retention →1153 logical/58 working/
    64995 decodedL/5057715 decodedB/2635052 residentB,rc=0;git diff --check →rc=0. Remaining
    D312–D315 fixes precede full gates/push/actual hosted jobs;no broader verdict inferred.
  - [x] LOCKSTEP: whole0540 task/lesson/changelog retained699 before curation. MEM/DEV/CHANGE/
    LIVE/index/ledger/book align;D311 fixed,29 suites/G1 6/18 unchanged,G0 human unapproved.
    README objective/commands unchanged. Commit before D312 matrix-coverage;all remaining owners
    retained.
    promotion: declined (application of indexed semantic identity/independent refusal controls;no new product permission or governance decision).
## _(2026-10-29 UTC)_ — keep authority mutation bound to the permission name

0541 D311:0476 correctly revised Propose semantics;old missing-level probe still matched retired prose. Select the stable first-cell permission key,require one unique match,and delete only that row. K3 and all five permissions/human-only approval remain unchanged.
Command probes15/0,actual missing-Propose K3 refusal1,REAL17/5/5/0,prose control0,missing chapter2;transport8 refusal/body reds and positives pass. bash -n,book93/248/1590/2771,699 receipt3/retention1153 pass0. Full repository retry follows remaining four repairs.
[Whole0540 archive repair](docs/history/stitchcad-devnotes-part699.md) retains exact scope/lesson/changelog. LIVE29 suites/G1 6/18 unchanged,D311 fixed,D312–D315 owned. G0 human unapproved;no push or hosted verdict. Next matrix-coverage D312.
Promotion declined:indexed semantic identity and independent refusal-control contracts. No new authority assumption;book explains name-versus-description contract. Consume each terminal before edits/staging/commit or moving to the next owned leaf.
## STITCHCAD-G1-0541 — bind authority mutation to its stable permission key

- Fix D311 missing-level control by selecting Propose's first-cell key and requiring exactly one matching row. Refuse absent or ambiguous setup;preserve the census,five permissions and human-only approval.
- Command probes15/0,actual K3 refusal,transport refusal/body controls,book93/248/1590/2771 and history checks pass. Whole0540 retained699/three exact Git spans. LIVE29 suites/G1 6/18 unchanged,D311 fixed,D312–D315 owned;full retry/push/hosted verdict pending.


