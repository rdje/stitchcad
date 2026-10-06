# Sealed archive — D210/D211 cleanup safety repairs

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a22.1` on `2026-10-06` (UTC).

- **Sealed identity:** 26 lines, 2298 bytes, sha256:2b0a0428398153c0e619549ecf80bcdad37e57258ee785958bf77e41eaefec6c
- **Coverage:** whole authored D210/D211 reports and verified closures; absent at 80e9d330bcd1be431203881e380a4452c2b019b0.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- **D210** — cleanup apply uses the legacy permissive background-job checker rather than the
  canonical strict checker. With an actual lsof child returning1 and no evidence, the legacy
  shell reports OK0 while check_handoff.sh refuses2. An actual apply body with a controlled
  frozen census reaches trapped deletion; the victim and producer bytes remain unchanged.
  Reproduce: python_cleanup_entry_contract.py, target/cleanup-entry-before.log, rc=0.
  Impact: generated project data can be deleted without proof that other project jobs are idle.
  Root: stale dispatch to check_no_background_jobs.sh after SPINE.23 introduced check_handoff.sh.
  Own G1-SLICE.5b.4c.h2.b.p.o.a22.1, P1 immediately before any real cleanup deletion;
  replace dispatch, retain strict refused/blocked statuses and calibrate actual omission controls.

- **D211** — cleanup inspect admits a selected regular artifact with two hard links despite the
  documented no-links deletion policy. A controlled ignored alias.log linked to shared.dat
  yields one candidate file with link count2; both names and bytes remain unchanged, no deletion.
  Impact: ownership of every alias is unproved, so selection cannot certify project-only deletion.
  Root: inspect checks symlinks/type/device/tracked paths but omits the regular-file link count.
  Own G1-SLICE.5b.4c.h2.b.p.o.a22.1, P1 now before real deletion; refuse nonunique files,
  preserve directory handling and read-only tracked-input aliases; add an actual guard omission.

- Verified closure: Python3.9/3.14 python_cleanup_entry_contract.py passes actual legacy gap,
  canonical statuses0/1/2, trapped stale-dispatch red, fourteen physical refusals/five omissions,
  complete original policy projection and actual hard-link guard red/read-only alias positive;
  target/cleanup-entry-controls{,39}.log, rc=0. No selected real project output was removed.
- Original cleanup_probes.py retains15 cases/11 actual reds on both versions, rc=0,
  target/cleanup-entry-original-probes{,39}.log. Only explicit Git-env anchor and stat nlink adapter changed.
- Real plan both versions:636 tracked input bytes exact, no apply; target/cleanup-entry-real{,39}.log, rc=0.
- Status: D210/D211 fixed by .a22.1; standing controls watch strict process dispatch/unique-link admission.
