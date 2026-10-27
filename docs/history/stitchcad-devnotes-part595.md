# Sealed engineering continuity — part595

- **Identity:** 46 lines, 4068 bytes, sha256:8c5e05c357a1f558356c114b22f921b2d1a49f483d9537906ead15c3085e55b5
- **Coverage:** whole0443 logical resource-contract scope/lesson/ledger from b308372228f08c2c7209b9555ce6334c6c1782e2.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 44785–47588, 2803 bytes, sha256:69ae855e19c79160d7ebde09f743cdc79c8971df6451386284f1904961f80e9b
- Whole source interval: DEV_NOTES.md bytes 193–965, 772 bytes, sha256:7f8265f540ca43deb308b6e922d6f095e36e3b012663da533c87e72b2bb76721
- Whole source interval: CHANGELOG.md bytes 207–700, 493 bytes, sha256:25b74a174cfe31f196f983b7d1dd13b66204a9505d6282e190a0e07a11a893a9
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.2.2.1`
  Status: `done`
  Goal:0443 explicit logical collection-entry/UTF-8-text/unbounded-counter-bit limits, demand and meter.
  A sealed data-only charging sink exposes no policy setter/reset/finish/model/publication capability.
  Preserve exact first refusal across ignored/later/zero charges and refuse finish after any failure.
  - [x] REPRODUCE / ISSUE: python3 -I -B native HEAD Design export assertions print no resource
    meter/sink rc=0; prior work bounds submitted metadata inputs/copy requests, not logical model content.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part594.md:
    3 exact Git spans rc=0; whole0442 scope/lesson/ledger retains its explicit remaining model boundary.
    An empty data meter/declared demand alone cannot prove exhaustive model/source traversal or runtime admission.
  - [x] FIX: cargo clippy -p sc-core --test design_resource_contract -- -D warnings:finished rc=0;
    positive explicit maxima, constant-state accounting, remaining-room guards, static field/index locations
    and first-refusal latch; producers receive only the sealed sink, not concrete replaceable policy state.
  - [x] ADDRESSED: cargo test -p sc-core --test design_resource_contract:6 passed rc=0; exact3 entries/
    5 UTF-8 bytes/129 bits, independent maxima, original counts/locations, cross-unit/zero latch and
    platform/u64 exact maximum/successor refusal verified without clipping or overflowing arithmetic.
  - [x] NO REGRESSION: cargo test -p sc-core --doc design::resource:4 passed rc=0; external policy/state
    writes, foreign sink implementation and producer finish fail compilation. Selected cargo fmt check
    rc=0; git diff --quiet HEAD -- crates/sc-core/src/command.rs crates/sc-core/src/command rc=0.
  - [x] LOCKSTEP: whole0442 proof594 retained unchanged with3 exact Git spans rc=0; book/pointer/frontier/
    lesson/indexed decision aligned. Actual core/source visitors .2/.3 and runtime prospective admission .4
    remain owned/pending, along with deeper .6.5.5 and full .6.9. No actual bus enforcement claim.
  Cargo invocations use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Native book/publication81 chapters/159 scoped API rows/1475 source links/2472 rendered links rc=0;
  maintained-reference82 files/18162 lines/1267199 bytes owner .6.8.2.2.2.2.1 rc=0.
  Retention1048 logical/78 aggregate working Markdown rc=0; active generation fits64/group.
  Logical fields count without backing deduplication; no total allocator RSS/capacity, host-internal
  algorithm-time, raw-ingress or model-validity certificate. An honest exhaustive producer is a trusted
  integration obligation; the charging sink supplies no arbitrary-host security boundary.
## _(2026-10-27 UTC)_ — first-refusal logical-content accounting contract

0443 .6.8.2.2.2.2.1 adds explicit positive logical collection-entry/UTF-8-text/unbounded-counter-bit
limits and a constant-state meter. Remaining-room checks preserve exact attempted components; the
first refusal persists across ignored/later/zero charges and finish. A sealed charging trait object
exposes no policy reset, concrete-state replacement or model/publication handle. Contract6/privacy4/
strict selected lint/fmt pass; actual bus source unchanged. Promotion: command-layer record. Actual
core/source traversal .2/.3 and runtime admission .4 remain; no RSS/time/actual-model-walk certificate.
Whole0442 reserve/D290/D291 scope/lesson/ledger retained594 with3 exact native Git spans.

## STITCHCAD-G1-0443 — define fail-closed logical model-resource accounting

- Explicit collection-entry/UTF-8-text/unbounded-counter-bit limits and first-failure-latching meter.
- Sealed charging capability carries no policy reset, model access or publication authority.
- Native contract6/privacy4/strict selected lint/fmt pass; runtime bus unchanged, actual visitors/admission owned next.
- Book scopes logical units and pending producers; whole0442 reserve/D290/D291 proof retained594.

