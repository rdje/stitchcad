# Sealed archive — D203 control runtime compatibility

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a01.2` on `2026-10-06` (UTC).

- **Sealed identity:** 15 lines, 1383 bytes, sha256:3be387e2943ce068ac21634d8c986dfa117994ec95614a6d18b74fcdedc5cd08
- **Coverage:** whole authored report absent from aea883c78b36ff3b5828da981c385966b9474618, follow-up reproduction and verified closure.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- **D203** — new copied-book control calls a hard-link API absent from the advertised Python3.9 runtime.
  Found2026-10-06 UTC; owner `G1-SLICE.5b.4c.h2.b.p.o.a01.2`; P1, fix in0200 before signoff.
  Actual system Python3.9.6 runs target/fixture-python39-proof.py; physical_controls exits1 at
  hardlink_to with AttributeError, target/fixture-python39-original.log. Authored report absent HEADaea.
  Impact: the added CI producer fails on a supported interpreter. Use stdlib os.link; rerun actual
  controls under the minimum runtime. OS interpreter is read-only; all stores/logs/fixtures are local.
  Follow-up: current full capture also exits1 on Python3.9 public recursive copytree calls: three
  positional arguments and dirs_exist_ok keyword, same copy function. Actual API probe rc=0 in
  fixture-copytree-api-proof.log; first guessed arity4 excluded. Validate public bound arguments
  and defaults rather than one interpreter call shape; no private library implementation inspected.

  Closed2026-10-06 UTC in0200: os.link and public signature/default binding preserve the file
  and copy interfaces across supported interpreters. Whole actual Python3.9.6 control89347 passes
  all six captures/twelve body reds/fourteen physical refusals/pure unused sidebar/guard-disable
  admissions, fixture-python39-fixed.log, rc=0. OS interpreter/stdlib read-only, no external writes.
