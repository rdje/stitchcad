# Sealed engineering continuity — part594

- **Identity:** 48 lines, 4296 bytes, sha256:d564a629a1199eed34b6a57c4653a9a1bea3a7f22174f7be80857ce5dfeff2d0
- **Coverage:** whole0442 shared reserve/D290/D291 scope/lesson/ledger from 9d499ae143288c6ad8ae155befe78d26cf365aaa.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 44268–47304, 3036 bytes, sha256:132f653ad3c4b899f8f3f19803e3829953f71bb06634ba4886bbe060a0559e92
- Whole source interval: DEV_NOTES.md bytes 193–977, 784 bytes, sha256:5efee152ca0c81c3ebc189d2010bdb7cad69ddf760eea9cd6279180709cd3efe
- Whole source interval: CHANGELOG.md bytes 207–683, 476 bytes, sha256:d2a46c91dd850af4169212275806f09734f2ec104ae16bcede68b5417c1aa63d
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.2.1`
  Status: `done`
  Goal:0442 D290 remove complete unused-reserve copies on publication; D291 standalone Notch setup fix.
  One immutable Arc-backed reserved identity slice remains for this bus lifetime. A checked forward
  consumed prefix and independent borrowed stage cursors preserve exact generator order/exhaustion.
  Publication prepares only shared-handle/prefix metadata, even for zero creation; no public reserve
  getter, refill, request override, undo rewind or identity reuse. Initial conversion/storage stays explicit.
  - [x] REPRODUCE / ISSUE: python3 -I -B native HEAD reserve/caller census prints remaining.to_vec()
    for every new accepted group, including count0 rc=0; this is source proof of a backing copy, not a
    measured latency/allocator benchmark. Actual semantic cases already existed and remain the oracle.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part593.md:
    4 exact Git spans rc=0; prior slice removed repeated H copies, while identity.after_consuming still
    independently copied every unused reserved ID before effects, regardless of actual creation count.
  - [x] FIX: cargo clippy -p sc-core --lib -- -D warnings:finished rc=0; shared immutable backing,
    checked prefix addition/range and concrete Skip<slice::Iter> staging remove publication backing copies.
  - [x] ADDRESSED: cargo test -p sc-core --lib command::bus::identity:1 passed rc=0, expanded for zero/
    partial/full consumption, independent prefix-local counts, unchanged original and overflow/range refusal.
    cargo test -p sc-measure --test design_inputs_contract command_bus:46 passed rc=0; actual source-copy,
    original ID order, exhaustion, grouped alias, cancellation/replay, undo/redo and branch nonreuse retained.
  - [x] NO REGRESSION: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished
    rc=0; selected cargo fmt check rc=0. Source census prints one shared handle/checked prefix, no backing
    Vec copy rc=0; no total allocation-free publication, timing or initial-conversion bound is claimed.
  - [x] LOCKSTEP: python3 -I -B native prior/current Notch example census:policy-install0/1 rc=0;
    D291 standalone preview now installs explicit policy before use. Book/pointer/frontier/lesson/indexed
    decision aligned; whole0441/D289 proof593, retention1047 logical/77 aggregate working Markdown rc=0.
  Cargo invocations use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Native book/publication81 chapters/159 scoped API rows/1474 source links/2470 rendered links rc=0;
  maintained-reference82 files/18090 lines/1262878 bytes owner .6.8.2.2.2.1 rc=0.
  Backing includes consumed IDs and stays allocated for bus lifetime; this deliberate bounded-initial-
  backing tradeoff removes reserve-size-dependent per-commit copies. Remaining model/initial/retained
  resource dimensions .6.8.2.2.2.2, deeper source/allocation .6.5.5 and full .6.9 stay owned/deferred.
## _(2026-10-27 UTC)_ — shared reserve backing and forward-only publication

0442 .6.8.2.2.2.1 fixes D290: native source/caller census shows remaining.to_vec() even for count0,
not a latency benchmark. Share immutable reserved-ID backing and check forward prefix addition/range;
independent borrowed cursors keep local counts/exhaustion. Expanded cursor1 and actual bus46 pass;
strict selected core/native lint/fmt pass. D291 standalone Notch example now installs explicit policy,
native prior/current setup census0/1. Promotion: command-layer record. Initial backing, including
consumed IDs, remains for bus lifetime; no total resource/initial allocation claim. Remaining model/
retained work .6.8.2.2.2.2, deeper .6.5.5/full .6.9; whole0441/D289 scope/lesson/ledger retained593.

## STITCHCAD-G1-0442 — remove per-commit identity-reserve backing copies

- D290: share immutable initial reserve backing; publish a checked forward prefix, including zero creation.
- Expanded cursor1/native bus46 and strict selected core/native lint/fmt pass; IDs never rewind on undo.
- Document backing retention and remaining resource limits; D291 standalone Notch preview installs policy.
- Whole0441/D289 scope/lesson/ledger retained593 with4 exact native Git spans.

