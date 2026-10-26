# Sealed archive — completed actual membership size-read scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1.6` on `2026-10-26` (UTC).

- **Sealed identity:** 15 lines, 1362 bytes, sha256:a7aed52aacaff1dcfe9d5c8216e230e88097f7ec74f129266ccb1edb4e4e7711
- **Coverage:** whole committed source interval from eed45a4823916d610d387932f7812929f4333ccf.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 33185–34547, 1362 bytes, sha256:a7aed52aacaff1dcfe9d5c8216e230e88097f7ec74f129266ccb1edb4e4e7711
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.5.2`
  Status: `done`
  Goal:0266 actual pinned membership/selected-id bridge to exact core size reads.
  Paths: crates/sc-measure/src/size_membership.rs;crates/sc-measure/src/size_membership/formula.rs;crates/sc-measure/src/lib.rs;crates/sc-measure/tests/formula_size_membership_contract.rs
- [x] REPRODUCE:0266 bf8f4c2 has core supplied frames but no actual membership bridge; source read rc=0.
- [x] ROOT CAUSE:0266 canonical vector/base/pin differ from label/ordinal inference; actual borrowed
  membership and explicit expected reference supply the values. Focused native5/0, rc=0.
- [x] FIX: expected pin before selected-id lookup, exact ordinal/count/base, original borrows and causes.
- [x] ADDRESSED:0266 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-measure
  --test formula_size_membership_contract:5 pass/0 fail, rc=0.
- [x] NO REGRESSION:0266 all intents/pins/reorder/base/singleton/cross-set identities pass0; cargo
  clippy --offline -p sc-measure --test formula_size_membership_contract -- -D warnings, rc=0.
  D70/current-registry/full SizeSet/path and broad .5g/pre-push remain owned, not inferred here.
- [x] LOCKSTEP:347/238 whole Git/public records and book render rc=0; live/book/frontier align.
  Promotion declined: existing authored-order/pin/base/formula3.1 contracts govern this bridge.
