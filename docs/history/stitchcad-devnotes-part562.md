# Sealed engineering continuity — part562

- **Identity:** 75 lines, 5445 bytes, sha256:43ddf7ad0d7c345087c982657ddea575251d4c7a3b0a6b5cfb6f13733a804371
- **Coverage:** whole completed garment-owner subtree,0412 lesson, ledger and retirement from 1f868a97bdb5f0cfde89a9e0fc7d835dc2928bb9.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 48155–51438, 3283 bytes, sha256:9c030382ad76cb1c220edb7239c03b7f5adc9d40833cdf5d85dbf67fd6df5861
- Whole source interval: DEV_NOTES.md bytes 193–1152, 959 bytes, sha256:aedd2c4bf13c6f78f7c238726e3481351bfe43f48d01e61609b87f8f9ec3e2d8
- Whole source interval: CHANGELOG.md bytes 207–690, 483 bytes, sha256:92cb96fb12a5f6e105a48f7d3ddd21a6aa5ec437160565062a831b8b2029a6ed
- Whole source interval: docs/tasks/PLANNING.md bytes 90009–90729, 720 bytes, sha256:3f297d3684fddf70e45f6bf93421951800e96843ab47a35e59a5ddf6729438f9
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.4.5.5`
  Status: `done`
  Goal: coupled garment ownership/current declarations over one canonical root.
  Children .a immutable admission/global owned ids; .b current sources; .c complete declarations;
  .d requested execution envelope; .cleanup1 daily cleanup. Final scoped acceptance .6/aggregate .6.4.6.
- ID: `G1-SLICE.6.4.5.5.a`
  Status: `done`
  Goal:0406 exact garment/root ownership; [whole proof](../history/stitchcad-devnotes-part556.md).
- ID: `G1-SLICE.6.4.5.5.b`
  Status: `done`
  Goal:0407 complete current garment sources; [whole proof](../history/stitchcad-devnotes-part557.md).
- ID: `G1-SLICE.6.4.5.5.c`
  Status: `done`
  Goal: complete typed actual garment declarations over one current root, no partial views.
  Children .1 actual logical Pocket opening adapter; .2 unified original adapters after full preflight.
- ID: `G1-SLICE.6.4.5.5.c.1`
  Status: `done`
  Goal:0410 original logical Pocket opening views; [whole proof](../history/stitchcad-devnotes-part560.md).
- ID: `G1-SLICE.6.4.5.5.c.2`
  Status: `done`
  Goal:0411 complete borrowed garment declarations; [whole proof](../history/stitchcad-devnotes-part561.md).
- ID: `G1-SLICE.6.4.5.5.d`
  Status: `done`
  Goal: original Lining execution envelope and pre-construction Fly refusal, independent of geometry.
  Children .1 D276 scope correction before .c; .2 requested envelope.
- ID: `G1-SLICE.6.4.5.5.d.1`
  Status: `done`
  Goal:0409 D276 Fly birth/documentation boundary; [whole proof](../history/stitchcad-devnotes-part559.md).
- ID: `G1-SLICE.6.4.5.5.d.2`
  Status: `done`
  Goal:0412 original explicit requested garment envelope, independently of current sources/geometry.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-measure --test design_inputs_contract garment_scope:
    3 passed rc=0; real current garment supports inspection while modelled Lining refuses execution.
  - [x] ROOT CAUSE (WHY + WHERE): source review rc=0:original ClosureDefinition guard is pre-birth;
    LayerInventory scope is separate from structural/source validation. Delegate both without inference.
  - [x] FIX: same native3 plus construction_and_garment_ownership_rechecks_whole_root_before_empty_scope:
    1 passed rc=0; root require_in_scope checks authored Closure definitions then complete layer scope.
  - [x] ADDRESSED: native4 rc=0:original Lining id/family/index/served Piece/env_lining/G7/concrete
    chains, damaged sources cannot mask scope, supported/empty scope proves no currentness; host drift
    stays separate. Actual Fly request refuses env_fly/G7/Closure/gap before birth, even invalid context.
  - [x] NO REGRESSION: core --lib / measure --test design_inputs_contract strict Clippy and affected
    cargo fmt rc=0; git diff --exit-code -- crates/sc-core/src/ontology/closure.rs rc=0; no constructor change.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part561.md:4 spans rc=0;
    retention/book/publication/size rc=0. Original book facts/examples retained; Design host decision
    promotion. Scoped component .6/aggregate .6.4.6/full runtime/physical/release proof remain owned.
- ID: `G1-SLICE.6.4.5.5.cleanup1`
  Status: `done`
  Goal:0408 guarded daily cleanup/D276 capture; [whole proof](../history/stitchcad-devnotes-part558.md).
## _(2026-10-27 UTC)_ — original requested garment envelope

0412 require_in_scope delegates authored actual Closure definitions then complete original layer
scope, independently of host/current sources/metadata/geometry. Native3 plus shared host-drift1 pass0:
modelled Lining remains inspectable but refuses original env_lining/G7 with layer/family/index/served
Piece and concrete chain, even damaged geometry. Supported/empty scope proves no currentness. Actual
Fly request retains pre-birth env_fly/G7/Closure/gap refusal even empty instances/invalid geometry;
no immutable Fly is modelled, constructor unchanged. Strict core+measure lint/fmt0. No physical/
execution completeness or release proof follows. Whole0411 scopes561 preserve4 exact Git spans;
retention/book/publication/size0. Original book facts/examples retained through tighter prose.
Promotion: Design host decision. Scoped .6/aggregate .6.4.6/full runtime/physical/release remain owned.

## STITCHCAD-G1-0412 — enforce original explicit garment envelope

- Delegate original Closure definitions then layer scope before requested execution, independent of geometry.
- Preserve named Lining/Fly causes and pre-birth Fly refusal; inspection stays available without substitution.
- Native4/core+measure strict lint/fmt/book/publication/size pass0; whole0411 proof561 retained.
- Scoped/coupled acceptance and full runtime/physical/release proof remain owned; G1still6/18.

## D271–D276 — original causes, native controls and Fly correction
[D271/D272](../history/stitchcad-devnotes-part547.md), [D273](../history/stitchcad-devnotes-part551.md),
[D274/D275](../history/stitchcad-devnotes-part554.md), [D276 corrected0409](../history/stitchcad-devnotes-part559.md).
- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part561.md:4 spans
  rc=0; complete0411 scopes retained before retirement.
- [x] ADDRESSED: same rc=0; whole declaration task/lesson/ledger/retirement preserved.
- [x] NO REGRESSION: requested garment envelope native4/core+measure strict lint/fmt/book/publication/
  size/retention rc=0; scoped/coupled acceptance and full integration remain.
