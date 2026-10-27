# Sealed engineering continuity — part541

- **Identity:** 71 lines, 5223 bytes, sha256:15e05162918f03d3b3798c425c51c745781d7c6c3c76fd65448026afc9867ce5
- **Coverage:** whole0391/0390 proposal/0389 acceptance, lessons, ledger, D263 report and retirement proof from 7ad38536c37af1a481d9f8361c61def9518d8849.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 50415–51212, 797 bytes, sha256:4ce68366d6b1137f8e37724ae00a27206451a2f1612b09cfa5b027a86b7b4eb1
- Whole source interval: docs/tasks/G1-SLICE.md bytes 49541–50144, 603 bytes, sha256:1018a82542d2fd312f1214b59b84ec99e81f5052869f99a7083a539900e1df46
- Whole source interval: docs/tasks/G1-SLICE.md bytes 48677–49381, 704 bytes, sha256:84941cecc1f52c9fc9d7a67ce5acb13b7ebc1a88f8cc86a13af5cc09432424ec
- Whole source interval: DEV_NOTES.md bytes 193–873, 680 bytes, sha256:3350645599a80effbb41173319867b3234d3b3a9d97231444aa0e94f4116e3b8
- Whole source interval: CHANGELOG.md bytes 207–615, 408 bytes, sha256:e678ae173c26c5523b19f2ed6c9f00c068f55b4c259ab2ec4f03bad3fe87690c
- Whole source interval: docs/tasks/PLANNING.md bytes 89128–89766, 638 bytes, sha256:0cbd95f95e4dc791d2605aadd429141b52046b0b6d412c4dd0d1f349a9bbfc9c
- Whole source interval: docs/tasks/PLANNING.md bytes 89766–90128, 362 bytes, sha256:8e44ef75e9ed0035dc066c175d51a3afa74db61e07b20d7b9e873933ea3446fc
- Whole source interval: docs/tasks/G1-SLICE.md bytes 6174–7205, 1031 bytes, sha256:311f24d3702dd7fb7b3d27016e35a9ae3316104d42bf03fc6f9caa3d306eb270
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.4.5.capacity.apply.design`
  Status: `done`
  Goal:0391 finite topology authority; whole0388 task/parent540. No live cutover.
  - [x] REPRODUCE / ISSUE: generation_budget.py:3999898 decodedB rc=0; v2 headroom102B.
  - [x] ROOT CAUSE (WHY + WHERE): same rc=0; aggregate and local reader budgets were coupled.
  - [x] FIX: generation_budget.py --self-test:25 oracle/boundary/refusal controls rc=0.
  - [x] ADDRESSED: generation_budget.py:20071898B workload needs8 slots; OS wc exact rc=0.
  - [x] NO REGRESSION: history_archive.sh verify-retention rc=0; original reader/caps unchanged.
  - [x] LOCKSTEP: book/publication/size rc=0; transition_receipts.py --record540:2 exact spans0.
  Promotion: decision_history-generations.md; runtime/transaction proof remains .cutover/.accept.
- ID: `G1-SLICE.6.4.5.capacity.proposal`
  Status: `done`
  Goal: measure next complete retirement, present bounded retention alternatives; no cap/record changes.
  Own correction of the obsolete no-archive rationale in the containment adoption note.
  - [x] ROOT CAUSE: Git/wc diagnostic:3706>2641B rc=0; exact spans in history decision.
  - [x] ADDRESSED: same diagnostic rc=0; A/B policy proposal committed without changing caps/records.
  - [x] NO REGRESSION: history_archive.sh verify-retention/book/publication/size rc=0.
  Promotion: history decision. Completed0390;0391 delegates and selects A.
- ID: `G1-SLICE.6.4.5.3.e`
  Status: `done`
  Goal:0389 focused layer/Hem component acceptance; broader integration after .6.9.
  - [x] ROOT CAUSE: Python Git/source review rc=0: all .3 children implemented; acceptance remained open.
  - [x] ADDRESSED: cargo test -p sc-core --test layer_contract inventory_:4/0 and hem_contract
    inventory_:5/0; -p sc-measure --test design_inputs_contract hem_declarations_:4/0 and
    layer_offset_declarations_:4/0; each rc=0,17 total in target/layer-hem-acceptance-0389.log.
  - [x] NO REGRESSION: book/publication79/159/1454/2410 rc=0; no Rust/manifest changes.
  Promotion declined: existing component/current-source/scope rules. Full Design proof remains open.
## _(2026-10-27 UTC)_ — finite self-contained generation authority

0391 chooses A under director delegation: version3 will seal old bytes and bound each generation.
The tracked budget producer measures3999898B and derives20071898B for100 explicitly bounded slices;
minimum6 slots round to8, with unchanged4MB/60000-line generation limits.25 oracle/refusal controls
and independent physical OS wc pass0. This is a planning envelope, not parser-memory or cutover proof.
Two exact Git spans retain whole0388 task/parent540. Live v2 remains unchanged with102B headroom.
Promotion: decision_history-generations.md. Reader/transaction/current-client proof remains .cutover/.accept.

## STITCHCAD-G1-0391 — derive bounded self-contained generation authority

- Director delegates retention choice; adopt version3 design with at most8 independently bounded generations.
- Tracked source/OS wc and25 boundary/oracle/refusal controls pass0; explicit100-slice planning assumption.
- Original v2 reader/caps/bytes unchanged; no live cutover or resource-runtime claim. Whole0388 task/parent540.

## D263 — installer fixture omits helper source authority

Code inspection: v2_install_contract.py calls changelog_pointers.py after constructing sources/frozen
without that helper or its current CHANGELOG input. Reproduce by inspecting lines31–33 and108–110.
Effect:17 transaction controls prove bytes/effects, not complete producer authority. P3; owner
G1-SLICE.5b.4c.h2.b.p.o.f.fixture-verification, after .6.9 before integration/pre-push. Repair complete
helper/input/child plans and calibrate omissions while preserving every original transaction predicate.

[0387 retirement proof](../history/stitchcad-devnotes-part539.md).

- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part539.md:
  5 exact Git spans rc=0; completed D262/0387 proof occupied live capacity.
- [x] ADDRESSED: same command rc=0; whole task/lesson/ledger/report/retirement retained539.
- [x] NO REGRESSION: book and scoped publication79/159/1454/2410 rc=0; exact D250132 rows compare0.
- ID: `G1-SLICE.3c.4`
  Status: `done`
  Goal: semantic `Dart`/`Tuck`/`Pleat`/`Gather`, `Hem`, `Facing`/`Lining`/`Interfacing`, `Closure` and
  `Pocket` with required content and validated structural references; close the object-type parent.
  Acceptance: required anchors, operation identities, composition and parameter references are carried;
  buttonhole size derives from its button rather than a second input; unsupported constructions are
  refused explicitly; intake conservation stays visibly deferred to G2. Every ontology §4 object has
  implementation evidence, synchronized book content and wasm-safe tests.
  Verification: object/support suites 149 contracts; sc-core 260 tests incl. 18 docs; full strict
  Rust/WASM/book, 22 probe suites, focused censuses/ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0023`

  Children: `.3c.4a` (intake constructions), `.3c.4b` (hem/layers), `.3c.4c` (closures),
  `.3c.4d` (pockets and family signoff). Safe child slices preserve the entire §4.3/§4.7 scope.

