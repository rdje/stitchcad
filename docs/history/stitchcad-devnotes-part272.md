# Sealed archive — whole original Ease main-task acceptance

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a08` on `2026-10-06` (UTC).

- **Sealed identity:** 10 lines, 710 bytes, sha256:c9a97e5bcdcfd158e33ec6f6116d8215cc107fa56d4e4155b4e29013200b23b0
- **Coverage:** complete old main-task acceptance from 8d1721d5699642600991eacfdd312caf3e7c67f4.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 59771–60481, 710 bytes, sha256:c9a97e5bcdcfd158e33ec6f6116d8215cc107fa56d4e4155b4e29013200b23b0
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

### G1-SLICE.5b.4c.h2.b.p.n.m1 — current producer acceptance

- [x] **ROOT CAUSE** — real entry activation omissions fail pre-write assertions;
  guarded runtime controls, rc=0, target/d156-ease-ci.log.
- [x] **ADDRESSED** — both Ease entries adopt shared source/output/profile guards;
 17 runtime/9 actual body reds/3 captures/3 late-source refusals, rc=0, target/d156-ease-ci.log.
- [x] **NO REGRESSION** — actual absent-export sequential Ease7/set10 assertion reds,
  source/current artifacts restored13/14pass, rc=0, target/d156-ease{-set,}-native.log;
  existing CI/local/census and publication10 pass, rc=0; Rust diff empty. D156 open.
  [Full receipts](G1-SLICE-recipes.md#g1-slice5b4ch2bpnm1).
