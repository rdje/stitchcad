# Sealed engineering continuity — part554

- **Identity:** 69 lines, 5869 bytes, sha256:5be72d70da8cd1c8a0f5f692429d5fc8b1c91ac7fc30da51cd9ccf05f9f9cb93
- **Coverage:** whole0403 grain/allowance task, lesson, ledger and D274/D275/retirement proof from ce44da49a017392326b968632a484755f56cd1cb.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 47846–49919, 2073 bytes, sha256:63422186ef8431edf9bbfd4ede5a14105d1f55ef8e24b6f82bc2c964e69c4638
- Whole source interval: DEV_NOTES.md bytes 193–1928, 1735 bytes, sha256:ddfd3694b63bbf6bc48ad0340fb2433455bf8694084198cee5f9e060eeb86ad9
- Whole source interval: CHANGELOG.md bytes 207–841, 634 bytes, sha256:2d0faac6302599a4a6c8ce895c808ba19f36ee1b402c82ed34ad07576a8ffa5a
- Whole source interval: docs/tasks/PLANNING.md bytes 90009–91436, 1427 bytes, sha256:2c125c7c76923754dd5471049b8de3a2c4a01f57406aed3ecd4917ea87b62db8
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.4.5.4.e.2`
  Status: `done`
  Goal:0403 typed grain/allowance declarations; D274 concurrent size-check scratch repair owned here.
  D274/D275 P2 fixed here: isolated checker scratch and actual-row stale-token probe, no cap changes.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-measure --test design_inputs_contract construction_:
    9 passed rc=0; Angle/Length formulas reject each other's metadata at original later positions.
  - [x] ROOT CAUSE (WHY + WHERE): source review rc=0:grain angles and allowance widths need distinct
    kinds; shared lookup preserves logical policy/original fields. D274 shared scratch rm -rf raced;
    concurrent_contract.py --checker target/live-size-d274-baseline.sh reproduces first rc=2/driver1.
    D275 native probes5pass/2fail rc=1:hardcodedv0.3 failed to mutate actualv0.4 revision field.
  - [x] FIX: same native9 rc=0; whole-root-first grain alignment then allowance width/inclusion borrows,
    exact object/field/position/canonical owner and expected/current kind refusals. Native overlap2/0
    rc=0 after isolated scratch; actual at= mutation retains exact refusal, native probes6/0 rc=0.
  - [x] ADDRESSED: same native9 rc=0; parallel axis without synthesized angle, exact45°/zero width,
    dedicated Derived/general Unknown Length, Unknown Angle, absent metadata, Profile numeric aliases,
    empty host drift/absent current Piece; original intake/closing retained.
  - [x] NO REGRESSION: cargo test -p sc-core --doc design::constructions:2 compile-fail rc=0;
    cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings and affected fmt rc=0;
    size self-test17/overlap2/native probes6 and shell/Python syntax rc=0.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part552.md:4 spans rc=0;
    part553.md:4 exact Git spans rc=0; retention/book/publication/size rc=0; book facts retained.
    Promotion: Design host decision; D274/D275 declined, settled scratch/revision rules unchanged.
    Gather .e.3/component .f/global joins .5/full CI remain owned.
## _(2026-10-27 UTC)_ — typed grain and allowance declarations

0403 borrows grain alignment then each allowance's width/inclusion after whole-current-root
preflight, even empty. Common contextual lookup requires actual Angle versus Length metadata,
without reading Unknown/Derived states or changing representation. Parallel alignment remains an
actual directed axis without synthesized angle; explicit45°/zero width, corner/print direction and
logical Profile inclusion remain original intent. Views keep actual fields/objects/positions and
single canonical owner. Native9 includes previous intake/closing checks, wrong-kind later sources,
bare numeric ids, logical aliases, absent current Piece and empty host drift; lifetime2/lint/fmt0.
Whole0402 scopes552 and completed parents553 retain4+4 exact Git spans; retention/book/publication/size0.
D274 overlapping size checks deleted shared scratch, causing179 false breaches/rc1 or missing file/rc2.
Native-wc barrier replays committed baseline49a6272:driver1/first2; unique local invocation scratch and
owned exit cleanup pass2/0. Watched overlap control/self-test17/syntax pass0. D274 promotion declined:
existing per-run scratch/locality doctrine unchanged; tracked reproduction/correction retained.
D275 real probe hardcodedv0.3 despite actualv0.4, native5pass/2fail rc1. Actual at= token mutation and
exact refusal now pass6/0 rc0 including overlap; promotion declined, existing revision rule unchanged. Original book facts
preserved with repetition tightened. Promotion: Design host decision. Current sources, resolved
width domains, geometry/policies/global ids and full release proof remain separate; Gather .e.3,
component .f/global garment-root .5/full CI remain owned.

## STITCHCAD-G1-0403 — borrow typed grain and allowance declarations

- Preserve actual alignment/width/inclusion field roles after whole-current-root validation, even empty.
- Require Angle versus Length metadata; retain parallel axis, explicit intent and logical Profile policy.
- Native9/lifetime2/strict lint/fmt pass0; original intake/closing behavior retained through shared metadata lookup.
- Fix D274 shared scratch race and D275 version-hardcoded stale probe: native red/green, self-test17, real probes6/0.
- Book/API and whole0402/closed-parent proofs552/553 synchronized; physical/global/release proof remains separate.

## D271/D272/D273 — current causes and derived census
[Whole Dart/census0397](../history/stitchcad-devnotes-part547.md); [whole D273/current0401](../history/stitchcad-devnotes-part551.md).
- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part552.md:4 spans
  rc=0; complete0402 scopes retained before retirement.
- [x] ADDRESSED: part552/553 receipt commands:4+4 exact Git spans rc=0; whole0402 and closed parents retained.
- [x] NO REGRESSION: construction native9/lifetime2/strict affected lint/fmt/book/publication/size/
  retention rc=0; D274 overlap2/self-test17/syntax rc=0; component .4.f, full integration later.

## D274 — concurrent size-check scratch collision
P2, fixed0403 by .6.4.5.4.e.2. Overlap erased shared measurements:179 false breaches/rc1 or missing
file/rc2. Tracked native-wc barrier reproduces exact committed baseline49a6272:driver1/first2.
Unique local mktemp directories/owned EXIT cleanup now pass both2/0; watched probe registered.
Native self-test17/shell/Python syntax rc=0. No caps/coverage/refusal rules changed.

## D275 — hardcoded stale-revision probe
P2, fixed0403 by .6.4.5.4.e.2. Native probes5pass/2fail rc=1:REAL-3 substitutedv0.3 but actualv0.4
left the copy unchanged. Mutating the actual at= field to an absent token now requires exact refusal;
native probes6pass/0fail rc=0, independent of current version. No baseline/coverage rule changed.
