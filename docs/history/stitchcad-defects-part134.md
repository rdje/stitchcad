# Sealed archive — whole authored ledger setup defect report and verified repair

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a23.1` on `2026-10-06` (UTC).

- **Sealed identity:** 16 lines, 1630 bytes, sha256:890e114145c49c8db424505004ceb32d8e80c4c46966992c9d8d5411e735db4f
- **Coverage:** whole authored D215 report and verified closure; not present at 5bde28b.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- **D215** — ledger pointer archive-child failure strands its workspace.
  Owner: G1-SLICE.5b.4c.h2.b.p.o.a23.1; priority P1, fix in current0225.
  Reproduce: python_ledger_fixture_contract.py --before executes actual5bde28b body in a guarded mirror.
  Exact archive-read argv fails with controlled CalledProcessError1 after mkdir; the new empty WORK remains,
  rc=0, target/ledger-entry-before.log. Only that proven empty owned mirror directory is removed afterward.
  Root cause: reader setup occurs before the fixture loop's try/finally. Guard complete inputs/files and
  bounded tree ownership first, then protect workspace setup/reader/fixtures together; verify a scope omission red.
  Impact: a failed archive read contaminates later fixtures with stranded scratch.
  Verified repair0225: complete source/backing archive and finite destination plans precede activation;
  guarded children preserve prepared stores, and workspace setup/read/cases share bounded protected teardown.
  Actual reader failure leaves zero residue; its original-scope omission reproduces D215. Both versions pass
  default82 plans/121 effect calls/14 guarded children and failure4/4/1, seven omission reds and twelve CLI
  physical refusals with exact mirrors/sources, rc=0, target/ledger-entry-controls{,39}.log.
  Native default/masked13 verdicts each pass on3.9/3.14 with all six sources/archive bytes exact and zero residue,
  rc=0, target/ledger-entry-real{,39}.log. Original complete bodies/cases/arguments/assertions remain exact
  through declared guards/environment/protected-reader adapters. Closed; mutation producer remains separate.
