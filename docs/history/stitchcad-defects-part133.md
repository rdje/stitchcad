# Sealed archive — whole authored cleanup fixture defect reports and verified repairs

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a22.2` on `2026-10-06` (UTC).

- **Sealed identity:** 41 lines, 3806 bytes, sha256:f37cae7ca2d10ef705671f8ddb7d28ca5e2b44ae5a6e40644607125bf529467d
- **Coverage:** whole authored D212/D213/D214 reports and verified closures; not present at 0ab7421.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- **D212** — cleanup fixture setup failure strands an owned directory.
  Owner: G1-SLICE.5b.4c.h2.b.p.o.a22.2; priority P1, fix in current0224.
  Reproduce: python_cleanup_fixture_contract.py --before compiles actual0ab7421 fixture body in a guarded mirror.
  Its15 cases/11 reds pass; a controlled setup RuntimeError leaves callback-failure present, rc=0,
  target/cleanup-fixture-before.log. The control removes only its proven empty owned directory afterward.
  Root cause: setup occurs before try/finally; the valid-case first write also precedes its teardown block.
  Impact: failed setup can strand scratch and contaminate a later run. Move owned setup into protected scope;
  verify callback failures after marker/file creation, independent omission reds, sources exact and actual15/11.

- **D213** — cleanup fixture source alias can rebase writes before authority.
  Owner: G1-SLICE.5b.4c.h2.b.p.o.a22.2; priority P1, fix in current0224.
  Reproduce: python_cleanup_fixture_contract.py --before executes actual0ab7421 fixture with its mirrored cleanup.py
  linked to the read-only real entry. SOURCE.resolve rebases ROOT; actual first mkdir reaches the real project base,
  trapped before writing, rc=0, target/cleanup-fixture-before.log. All project sources remain exact.
  Root cause: ROOT derives from the unchecked dependency instead of the fixture entry, with mkdir before input checks.
  Fix: derive ROOT from this entry and validate complete sources before activation; validate the first profile loader
  using bounded local regular-file/parent checks before delegation. Verify actual source/helper alias refusals before effects.

- **D214** — newer cleanup hard-link guard masks original fixture fault assertions.
  Owner: G1-SLICE.5b.4c.h2.b.p.o.a22.2; priority P1, fix in current0224.
  Reproduce: python_cleanup_fixture_contract.py --before runs actual0ab7421 fixtures with linked read-only README.
  Removing the symlink guard follows that input; the newer deletion-link guard refuses first and the original fault
  assertion fails for the wrong reason, rc=0, target/cleanup-fixture-before.log; aliases/source bytes exact.
  Root cause: old symlink/nonignored mutants can expose a linked read-only source to the new deletion guard.
  Isolate that later guard only in these two in-memory original mutants; retain the real inspector and its independent
  D211 guard/refusal/red controls unchanged. Verify full15/11 with linked README/profile and an isolation omission red.

Verified closure D212: protected callback/first-valid-write scopes remove owned fixtures on actual injected failures.
Both actual scope omissions leave residue and fail independent assertions. Default control on3.14/3.9 rc=0,
target/cleanup-fixture-controls{,39}.log; real15 cases/11 reds/source exact/zero residue rc=0,
target/cleanup-fixture-real{,39}.log. No selected project cleanup output was removed.

Verified closure D213: entry-derived root and bounded first-profile metadata/source/parent validation precede
helper execution; complete six sources and all finite fixture destinations precede activation. Actual source/helper
alias refusals are among17 physical CLI negatives with unchanged mirror snapshots; helper guard omission is caught
before delegate execution. Actual source-root legacy mkdir remains trapped. Both control versions rc=0, controls logs.

Verified closure D214: later deletion-link guard isolated only in symlink/nonignored in-memory original mutants.
Complete15/11 on linked read-only README/profile passes; independent hashes/aliases/link counts remain exact.
Isolation omission recreates the original masked fault, both versions rc=0, controls logs. Real inspector bytes match
0ab7421; D211 unique-link policy and its separate actual refusal/omission proof remain unchanged.
