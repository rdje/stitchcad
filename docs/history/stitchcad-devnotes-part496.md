# Sealed archive — whole retention continuation and0344 closure

Immutable historical segment, sealed by leaf `G1-SLICE.6.4.1` on `2026-10-26` (UTC).

- **Sealed identity:** 106 lines, 8392 bytes, sha256:dbd95028fcb646e44d722b6ce2c67bdbbf566c50d92f97a52315c4034763b35b
- **Coverage:** complete committed retention subtree, defect reports, lesson and ledger from c588e5e86e5320a11939b88c7944f8f089fc8166.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 27688–32608, 4920 bytes, sha256:5964d844cb149af461c105a07616bb7e75055d1d3fbc22dc8645b3650650aa9d
- Whole source interval: docs/tasks/PLANNING.md bytes 84810–86600, 1790 bytes, sha256:e7005994ef5926320d372eb4398d65ae83cbc20af012734bd469e7c671797598
- Whole source interval: DEV_NOTES.md bytes 193–1177, 984 bytes, sha256:888db1b032cae5e7196bf04f56ac2b633860fc67aad3931dab8f053f81d416dd
- Whole source interval: CHANGELOG.md bytes 207–856, 649 bytes, sha256:1405a2d4b6985f33055f0a4b714cc918b25c04641f9d0da5af92f52731f25b53
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.RETENTION-CAPACITY`
  Status: `done`
  Goal: P1 design/verify bounded retention continuation before the next packing after16; current schema refuses17.
  Preserve immutable records/manifests, logical addresses and all independent caps; do not silently raise bounds.
  Trigger: before future required seal exceeds64 with16 windows registered; product slices proceed meanwhile.
  Origin: renamed pending maintenance followup from .4c.5.h16.capacity, whose whole declaration remains
  [retained471](../history/window20.md#stitchcad-devnotes-part471md); independent of completed product-review acceptance.
  Candidate for falsification: flat re-packing of logical records plus byte-exact legacy control/payload
  evidence into fresh bounded packets; prove every existing cap before adoption. No protocol change is adopted by this structural review.
- ID: `G1-SLICE.RETENTION-CAPACITY.1`
  Status: `done`
  Goal:0339 capacity feasibility checked; [whole transitional receipt](../history/stitchcad-devnotes-part494.md).
- ID: `G1-SLICE.RETENTION-CAPACITY.2`
  Status: `done`
  Goal: versioned immutable-byte transition under the [recorded draft](../decisions/decision_history-windows-retain-self-contained-bytes.md).
  Children .a bounded reader; .b committed-byte/source/backing controls; .c guarded migration and consumers.
  Keep all caps/v1 behavior/frozen producers. Pack committed64-file baseline before creating494.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.a`
  Status: `done`
  Goal:0340 bounded v2 reader verified; [whole transitional receipt](../history/stitchcad-devnotes-part494.md).
- ID: `G1-SLICE.RETENTION-CAPACITY.2.b`
  Status: `done`
  Goal:0341 exact relocation/source/future guards; [whole receipt](../history/stitchcad-devnotes-part494.md).
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c`
  Status: `done`
  Goal: guarded atomic copy/verify/use/delete migration with rollback; offline/materialization/current consumers
  and all archive refusal controls, then494 whole transition receipts and product .6.4 resumes.
  Children .c.1 committed installer/recovery and consumer support; .c.2 current-HEAD installation/proofs/494.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.1`
  Status: `done`
  Goal:0342 recoverable installer/consumer controls; [whole receipt](../history/stitchcad-devnotes-part494.md).
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.2`
  Status: `done`
  Goal: children .a installs/retains494; .b adapts legacy verifiers before closure/product .6.4.
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.2.a`
  Status: `done`
  Goal:0343 installed-byte/source/public proof; [whole receipt](../history/stitchcad-devnotes-part495.md).
- ID: `G1-SLICE.RETENTION-CAPACITY.2.c.2.b`
  Status: `done`
  Goal: adapt current archive-client/reader/fixture predecessor controls to retained v1 evidence and
  complete v2 inventories, preserving original predicates/authority; focused native checks before .6.4.
  Own whole prior closure495 before doc growth; D246 owns packed-record omission-control classification.
  - [x] REPRODUCE / ISSUE: three default verifier entries fail1 after v2; D246 raw-read fault raises
    FileNotFoundError before its dispatch assertion, also absent at source55ed18e. D247 book entry fails1.
  - [x] ROOT CAUSE (WHY + WHERE): at dd20dc5, python3 -I -B
    docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py fails in old-body/v2 trap;
    fixture bootstrap reads retired window1.json and predecessors read physical controls. Before logs
    target/*before-0344.log give rc=1; original failure sites match the source adapters.
  - [x] FIX: old reader/producer use exact retained-v1 fixtures; current reader tests v2. Original Git
    capture remains independent; all97 snapshot paths/bytes match. D246 raw fault reaches dispatch oracle;
    D247 book invokes legacy_refusals.py. Whole prior closures/0343 continuity retained495 before growth.
  - [x] ADDRESSED: python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py
    --real gives949 exact files/14 physical/8 body reds;
    python_archive_client_contract.py --real preserves8 real children/7 physical/7 body/3 content refusals;
    fixture default capture411 plans/203 effects/9 children passes, rc=0, target/v2-*-0344.log.
  - [x] NO REGRESSION: python3 -I -B docs/tasks/artifacts/history_archive/legacy_refusals.py gives
    original28/0, original source/provenance/CLI exact; no residue,
    rc=0, target/v2-legacy-native-0344.log. transition_receipts.py records494/495 proves3/4 whole Git spans
    plus9 receipt hashes/8 nested Git intervals; changed-byte comparison refuses, rc=0.
  - [x] LOCKSTEP: book/decision/status/notes/index/pointer/D246-D247 aligned; all caps unchanged;
    G1still6/18. Commit0344 closes retention parent; next .6.4. promotion: declined (existing decision).
    Focused native3.14 proof only; optional deep fixture/platform/full integration/CI remain deferred.
## D246 — packed-record omission control escapes its intended assertion

Default python_archive_client_contract.py fails1 in omission_controls: its deliberate replacement of
public logical reads with (ROOT / record).read_bytes() raises FileNotFoundError for packed part303,
instead of reaching the required "D222 logical read omitted" assertion. Actual retained client capture
and seven physical refusals pass first. Source55ed18e Git cat-file -e part303 also fails128: the raw
path was absent before the v2 transition. Impact: this verifier cannot complete its named logical-read
fault calibration after ordinary packing; no retained-byte loss or product-client failure is claimed.
Owner G1-SLICE.RETENTION-CAPACITY.2.c.2.b, P2 immediate: supply exact logical bytes only to that
controlled raw fallback so the independent dispatched-child population assertion remains the oracle.
Fixed0344: default/native client reaches all7 intended body reds and3 content refusals with8 real
children; source/backing/provenance exact, rc=0, target/v2-archive-native-0344.log.

## D247 — archive guide invokes the v1-only refusal producer directly

The archive-reader annex still publishes history_archive_probes.py as the native command after v2.
Actual invocation returns1 at window1.json absence before fixture effects, target/archive-book-before-0344.log.
The original producer is intentionally v1-only; the maintained legacy_refusals.py entry reconstructs its
exact v1 inputs. Owner G1-SLICE.RETENTION-CAPACITY.2.c.2.b, P2 immediate: route the published command
through that adapter and execute it; preserve original producer bytes and all28 native refusal cases.
Fixed0344: published legacy_refusals.py returns original28/0 with source bytes unchanged, rc=0,
target/v2-legacy-native-0344.log.

## _(2026-10-26 UTC)_ — v2-compatible legacy verification

0344 keeps original old-reader traps and refusal producer on exact v1 fixtures. Current reader materializes
v2; original predecessor controls/payloads resolve through backing rows. Git at the original v1 capture
remains the fixture oracle, with all97 data paths/bytes exact. Original CLI/provenance assertions remain.
D246's deliberate raw-read fault gets exact logical bytes so the original child-dispatch assertion fires;
D247 routes the book through the maintained legacy entry. Native reader949/14 physical/8 body and
client8 children/7 physical/7 body/3 content, fixture capture and original28 pass0, target/*0344.log.
Whole prior closure/0343 continuity is retained495; records494/495 prove3/4 exact Git spans and a byte-fault
control. Promotion: existing history decision. Retention parent closes; next product .6.4, G1still6/18.
Focused3.14 only; optional deep fixture/platform/full integration/CI stays deferred.

## STITCHCAD-G1-0344 — preserve verification authority across flat retention

- Adapt old-reader/producer fixtures and predecessor comparisons to exact retained v1 bytes; current native reads use v2.
- D246 restores the intended raw-read fault assertion; D247 publishes the valid legacy native command.
- Native reader949/14 physical/8 body, client8 children/7 physical/7 body/3 content, fixture capture and original28 pass0.
- Whole prior closures/0343 retained495;494/495 exact Git spans3/4 verified. Retention closes; product .6.4 next.
- G1 remains6/18. Focused3.14 proof; optional deep fixture/platform/full integration/CI remains deferred.

End of whole retained-verifier closure receipts.
