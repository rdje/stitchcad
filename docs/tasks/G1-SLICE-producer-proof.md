# G1-SLICE — completed producer verification

Bounded evidence sibling of [G1-SLICE](G1-SLICE.md). Whole completed records are copied
byte-exact from the committed producer tree; canonical IDs/status/Paths and fresh current
checklists remain in [the producer tree](G1-SLICE-producers.md). Historical counters and next
actions below describe their original commits. Current execution follows MEMORY.md.

Source: `ade516526f7a18a1b722eeb152737cb3c7f85a96`, docs/tasks/G1-SLICE-producers.md. Byte intervals are zero-based, end-exclusive.

| Record | Source interval | Bytes / lines | SHA-256 |
| --- | --- | --- | --- |
| [native membership](#native-membership) | 37808–41028 | 3220 / 10 | `f6e866e54310d89582ff6b44c98ebf4274c16581083a7ea14c19c87b8c81fe9c` |
| [size chart observations](#size-chart-observations) | 42681–46212 | 3531 / 15 | `5f0f25ae551e49641ee1160da3bd1d0fcbc2781bde70d584997ef606f0da0b9c` |
| [measurement tables](#measurement-tables) | 46664–50056 | 3392 / 14 | `df74cae2805fb3e637e314994721301748eefcf960d923beb7ef931d8d7872aa` |
| [made to measure inputs](#made-to-measure-inputs) | 50443–53827 | 3384 / 15 | `1f18af6e18198ef4d17cc6eac55fdec1d13f51e7f3234f5735c5d016b07acc12` |
| [size chart collections](#size-chart-collections) | 54227–58121 | 3894 / 19 | `941419651d8256943f7b94194a446732affc9f3c233ed30b8d330db027c778ed` |

### Native membership

Whole preimplementation record retained in [part193](../history/stitchcad-devnotes-part193.md); completed acceptance boxes and canonical ownership remain below.

- [x] REPRODUCE / ISSUE: directory-only membership entry omits eight file destinations/mutable writing intent; original actual predicate accepts passed assertion_name with nonmacro failure, rc=0, membership-predicate-diagnostic.log. Refined12 byte entries/13 conditions and three actual prefix helpers reproduce D17715-entry scope, rc=0, predicate-census/prefix-diagnostic logs; original membership introduced0b77235. Remaining14 owned P1 .n02–06.
- [x] ROOT CAUSE: CASES follows activation/source reads, preventing derived file planning; output directories omit leaf identity, source guard omits writing intent, and broad panic messages do not prove macro sites. Actual complete eight-child/eight-file capture/four physical copied-entry first/last output alias/mutable source hard link/late test refusal pass, rc=0, membership-producer-controls.log. No alias/source bytes changed by those controls.
- [x] FIX: move unchanged pure seven-case population before the guard; plan all numbered/restored logs/two sources/one mutable source, and guard current test macro locations. Source/mutable keyword calibration uses actual AST identity. Shared31/24file/26mutable/37prefix+late/26child/two compiler/three static/21 classifiers/179 body reds pass, rc=0, membership-producer-controls.log. Five actual entry/three classifier omissions fail independently; simulation proves dispatch only.
- [x] ADDRESSED: exclusive standalone caller stores absent passes seven real current macro reds, rc=0, membership-native-final.log. Independent byte-position replay verifies seven actual logs/sites; eight frozen inputs/original cases/implementation+test HEAD bytes exact/all eight outputs regular-local-singlelink/full12 restored, rc=0, native-restoration.log. Public Cargo artifact SHAe61d4f6a…/current twelve-test execution0, rc=0, native-artifact.log; no tracked Rust/API/grammar change.
- [x] NO REGRESSION: publication45/coverage9/census229paths288checks/effect67paths29owners8refusals7reds/file120paths44owners15refusals12reds/map freshness pass, rc=0, membership-*-final logs. Three whole fc46ad8 source spans/five closures+six lessons+three ledgers exact; prior427/all25 controls/public430 exact/13open163sealed/no gaps or overlap, rc=0, membership-retention-proof.log. First retention syntax1 before writes and census1 omitting alternate D131/D176 headings excluded; corrected canonical union0.
- [x] LOCKSTEP: book/spec/live/MEMORY/G1/index route .f.n02 and D177 remaining14; new13working/16654L/1205039logicalB/528124residentB fit unchanged fixed caps, rc=0, membership-retention-proof.log. Two mistargeted no-op book patches failed before any change; reread actual section before applying correct patch. promotion: declined (existing current-source proof, explicit effect ownership and whole-record retention).400 cadence/local verification retained; no runner verdict claimed.
  Ledger9 arms/pointer13 pass0, terminal consumed, target/membership-ledger-final.log; no pending job remains.
  Final staged gate13 green, rc=0, target/membership-staged-gate.log.

### Size-chart observations

Full chart entry/wrapper/implementation/current sixteen-test contract, public exports, original cases,
roadmap §3.1/G1 and current/normative chart book contracts read before edits. Source-dependent table
and multi-anchor collection populations justify four children; watched terminal ownership replaces
four ledger rows before implementation. All native activity was exclusive and terminal results
consumed before any other mutation/probe; public current artifact follows exact source restoration.

- [x] REPRODUCE / ISSUE: actual committed chart fault predicate accepts passed assertion_name with expect-only panic, AST/eval without source/native effects, rc=0, target/chart-predicate-diagnostic.log. Directory-only entry omits nine leaf destinations/mutable intent/current test source; D156/D177 owned here. Split four families before implementation; full original source/public/test/book inspection above.
- [x] ROOT CAUSE: original docs/tasks/artifacts/size_chart/size_chart_mutations.py places CASES after activation; directory/source guards omit filenames/writing intent, and substring messages do not prove macro provenance. Independent full captures/four physical copied-entry first/last log symlink/mutable hard-link/missing-test refusals pass, rc=0, target/chart-producer-controls-final.log; no alias bytes or unsafe native effects.
- [x] FIX: guard nine derived logs/two inputs/one mutable implementation/current failed-body macro byte sites before reads/effects; preserve eight original cases/argv/stores. Generic measurement controls use independent authored dispatch fixtures for membership/chart. Shared187 body reds/22 classifiers/31 runtime/24 files/26 mutable/37 prefixes+late/26 child/two compiler/three static captures0, chart-producer-controls.log; five entry/three classifier faults per entry.
- [x] ADDRESSED: exclusive standalone caller stores absent observes eight real current macro reds, rc=0, chart-native-final.log. Independent byte-position replay verifies all eight selected logs/sites/eight frozen inputs/original cases/two HEAD Rust inputs/nine regular-local-singlelink files/full16 restored, rc=0, native-restoration.log. Public Cargo artifact6235f6e8… executes16 tests0, native-artifact.log; Rust/API/grammar unchanged.
- [x] NO REGRESSION: publication45/coverage9/census229paths288checks/effect67paths29owners8refusals7reds/file120paths47owners17groups15refusals12reds/split positive/map freshness pass0, chart-*-final logs. Whole467191f ledger interval22557–23940/19L1383B/SHA6e08f574 exact; prior430/all28 files/public431 exact/13open163sealed/no gaps or overlap, rc=0, chart-retention-proof.log. Initial Target-only archive preflight1 refused before writes; corrected tracked-directory/fresh-file validation0.
- [x] LOCKSTEP: book/spec/live/MEMORY/G1/index route .f.n02.2; D177 membership/chart fixed/13 entries P1 remain. Pressure14working/16684L/1207010decodedB/530095residentB fits unchanged caps, rc=0, chart-retention-proof.log. promotion: declined (existing current-source proof, finite ownership splits, mutable/file declarations and whole-record retention).400 cadence/local verification preserved; runner verdict unobserved. Ledger9/pointer13 terminal0 consumed, chart-ledger-final.log. Final187 body reds/22 classifiers terminal0 consumed, chart-producer-controls-final.log.
  First staged gate1 refused ROOT CAUSE pass0 without an rc signature; corrected the located cause and actual rc=0 receipt, then rerun.

  Final staged gate13 green, rc=0, target/chart-staged-gate.log.

### Measurement tables

Full entry/wrapper/table implementation/current sixteen-test source, public exports, original
.4a.3 contract and normative/runtime book reviewed. Move pure eight-case data before the profile;
derive the existing whole current-check span only after guarded input reads. Preserve its removal,
every case/filter and full restoration; extend independent measurement fixtures to this entry's
nine files/two inputs/mutable source/current test macros. This child also owns bounded whole-record
retention before growing live lessons, preserving exact committed bytes and fixed ceilings.

- [x] REPRODUCE / ISSUE: actual committed table predicate accepts passed assertion_name plus expect-only panic, AST/eval without source/native effects, rc=0, target/table-predicate-diagnostic.log. Original entry omits nine destinations, mutable intent and current test source; D156/D177 owned here. Full implementation/test/public/normative/runtime contract review above; no Rust change.
- [x] ROOT CAUSE: original docs/tasks/artifacts/measurement_table/table_mutations.py puts CASES after activation/source reads and uses whole-log assertion bytes; directories do not identify log files or mutable sources. Independent authored complete nine-child/file capture/four physical first/last log symlink/mutable hard-link/missing-test refusals pass, rc=0, target/table-producer-controls.log; all alias bytes unchanged/no unsafe native dispatch.
- [x] FIX: move pure eight-case data before nine-file/two-input/one-mutable-source profile; derive unchanged whole current-check span after guarded reads and require current failed-body macro byte sites. All eight cases/four dependent statements AST-exact; independent three-entry captures/five entry and three classifier faults each pass. Shared195 body reds/23 classifiers/31 runtime/24 file/26 mutable/37 entry+late/26 child/two compiler/three static captures0, rc=0, table-producer-controls.log; simulation proves dispatch only.
- [x] ADDRESSED: exclusive caller-store-absent run passes eight real current macro reds, rc=0, table-native-final.log. Independent selected-log/byte-site replay verifies eight frozen inputs/original cases/four span statements/two HEAD Rust inputs/nine regular-local-singlelink logs/full16 restored, rc=0, native-restoration.log. Public Cargo artifactaccf600a… executes16 tests0, native-artifact.log; Rust/API/grammar unchanged.
- [x] NO REGRESSION: publication45/coverage9/census229paths288checks/effect67paths29owners8refusals7reds/file120paths47owners17groups15refusals12reds/map freshness pass, rc=0, table-*-final logs. Whole8e2b486 DEV span13854–14355/4L501B/SHAdbb2c10d exact; prior431/all29 files/public432 exact/13open163sealed/no gaps or overlap, rc=0, table-retention-final.log. Seven committed windows and all prior sealed records remain exact.
- [x] LOCKSTEP: book/spec/live/MEMORY/G1/index route .f.n02.3; D177 membership/chart/table fixed/12 entries P1 remain. Pressure15working/16699L/1208098decodedB/531183residentB fits fixed caps; DEV16154B/CH31832B within unchanged health, rc=0, table-retention-final.log. promotion: declined (existing current-source evidence, explicit mutable/file ownership and whole-record retention rules).400 cadence/local verification preserved; runner unobserved. Ledger9/pointer13 terminal0 consumed, table-ledger-final.log.
  Final staged gate13 green, rc=0, target/table-staged-gate.log.

### Made-to-measure inputs

Full entry/wrapper/implementation/fifteen-test source/public exports, .4c.3c canonical record and
normative/main/annex books reviewed. Resolve pure twelve-case population before profile effects;
guard thirteen logs/two inputs/one mutable implementation/current test macros. Preserve all cases,
filters/stores/full restoration, especially body/Ease zero refusals and grade-rule refusal. Own
actual predicate/capture/physical/omission/noise controls before exclusive native replay and exact
source/current artifact proof. Own whole committed older lesson retention before live growth,
with immutable prior records/public readers and fixed caps verified before commit.

- [x] REPRODUCE / ISSUE: actual committed MTM predicate accepts passed assertion_name plus expect-only panic, AST/eval without source/native effects, rc=0, target/mtm-predicate-diagnostic.log. Entry omits thirteen filenames, mutable intent and current test source; existing D156/D177 own these defects. Full implementation/fifteen-test/public/normative/runtime contract reviewed before changes.
- [x] ROOT CAUSE: CASES follows activation/source reads; directory identity omits leaves/writing intent, and whole-log assertion bytes do not prove macro sites. Independent authored complete thirteen-child/file capture/four physical first/last log symlink/mutable hard-link/missing-test refusals pass, rc=0, target/mtm-producer-controls.log. Alias bytes/environment/stores remain unchanged; no unsafe native dispatch.
- [x] FIX: move unchanged pure twelve-case population before thirteen-file/two-input/one-mutable-source guard; require current failed-body macro byte locations. Four independent measurement captures/sixteen physical refusals/five entry and three classifier faults per entry pass. Shared203 body reds/24 classifiers/31 runtime/24 file/26 mutable/37 entry+late/26 child/two compiler/three static captures0, rc=0, mtm-producer-controls.log; simulation proves dispatch only.
- [x] ADDRESSED: exclusive caller-store-absent run passes twelve real current macro reds, rc=0, mtm-native-final.log. Independent selected-log/byte-site replay verifies eight frozen inputs/original12 cases/two HEAD Rust inputs/thirteen regular-local-singlelink logs/full15 restored, rc=0, native-restoration.log. Public Cargo artifact5614214a… executes all15 tests0, rc=0, native-artifact.log; Rust/API/grammar unchanged.
- [x] NO REGRESSION: publication45/coverage9/census229paths288checks/effect67paths29owners8refusals7reds/file120paths47owners17groups15refusals12reds/map freshness pass, rc=0, mtm-*-final logs. Whole5b37891 DEV span7594–9941/16L2347B/SHA9f77b65f exact; prior432/all30 archive files/public433 exact/13open163sealed/no gaps or overlap, rc=0, mtm-retention-final.log. Seven windows/all prior sealed records remain exact.
- [x] LOCKSTEP: book/annex/live/MEMORY/G1/index route .f.n02.4; D177 membership/chart/table/MTM fixed/11 entries P1 remain. Pressure16working/16726L/1211112decodedB/534197residentB fits fixed caps; DEV15442B/CH32410B within health, rc=0, retention-final.log and measured byte census. promotion: declined (existing current-source evidence, explicit mutable/file ownership and whole-record retention rules).400 cadence/local verification preserved; runner unobserved. Ledger9/pointer13 terminal0 consumed, mtm-ledger-final.log.
  Final staged gate13 green, rc=0, target/mtm-staged-gate.log.

### Size-chart collections

Full entry/wrapper/587-line implementation/752-line eighteen-test source, public exports,
.4c.3b canonical record, roadmap measurement objects and normative/runtime books reviewed.
Preserve pure fourteen-case data and each ordered replacement, including the two-anchor uniqueness
case; guard fifteen logs/current test/implementation with explicit mutable ownership before reads.
Own actual old-predicate reproduction, independent complete selected argv/multi-anchor writes,
physical aliases/late-input refusals and classifier/body controls before exclusive native proof.
Current tests have inline closure macros: recognize their exact UTF-8 byte columns and calibrate
actual leading-only site omission plus second-anchor omission before any native source mutation.
Before receipt growth, retain whole .n01/.f.l/.f.h.v preimplementation records, older mutable/window
lessons and oldest ledger from current HEAD; keep completed boxes/canonical ID/status/Paths live
and preserve all prior seals/readers; prove exact source spans and unchanged fixed caps. Own Target helpers/logs for these proofs.

- [x] REPRODUCE / ISSUE: actual committed collection predicate accepts passed assertion_name plus expect-only panic, AST/eval without source/native effects, rc=0, collection-predicate-diagnostic.log. Entry omits15 files/mutable intent/current test source; existing D156/D177 own the repair. Full implementation/eighteen-test/public/normative/runtime contracts read before changes.
- [x] ROOT CAUSE: CASES follows activation/source reads; directories omit leaves/writing intent, and substring evidence omits current macro sites. Independent authored complete15-child/file capture/four physical first/last symlink/mutable hard-link/missing-test refusals pass, rc=0, collection-producer-controls.log. All alias bytes/environment/stores unchanged; no unsafe native dispatch.
- [x] FIX: preserve14 cases/15 ordered anchors; plan15 logs/two inputs/one mutable source before reads and match current failed-body macro byte sites, including inline closures. Shared213 body reds/25 classifiers/31 runtime/24 file/26 mutable/37 entry+late/26 child/two compiler/three static captures0, rc=0, producer-controls.log. Five measurement captures/twenty physical refusals/eight faults per entry plus second-anchor/leading-only omissions pass; UTF-8 columns calibrated.
- [x] ADDRESSED: exclusive caller-store-absent run passes14 real current macro reds, rc=0, collection-native-final.log. Independent selected-test/byte-site replay verifies eight frozen inputs/original14 cases/15 ordered anchors/two HEAD Rust inputs/15 regular-local-singlelink logs/full18 restored, rc=0, native-restoration.log. Public Cargo artifactb356e6f1… executes18 tests0, rc=0, native-artifact.log; Rust/API/grammar unchanged.
- [x] NO REGRESSION: publication45/coverage9/census229paths288checks/effect67paths29owners8refusals7reds/file120paths47owners17groups15refusals12reds/map freshness pass, rc=0, collection-*-final logs. Five whole9d840bc source intervals/three preimplementation records/two lessons/one ledger exact; prior433/all31 archive files/public438 exact/13open163sealed/no gaps or overlap, rc=0, collection-retention-final.log. Completed boxes/canonical ID/status/Paths stay live; all prior seals/seven windows exact.
- [x] LOCKSTEP: parent .f.n02 done; book/spec/live/MEMORY/G1/index route Ease .f.n03. D177 five measurement families fixed/10 entries P1 remain;21working/16841L/1220791decodedB/543876residentB fits caps, rc=0, retention-final.log. Live DEV/CH/task health targets unchanged and measured. promotion: declined (existing current-source evidence, explicit mutable/file ownership and whole-record retention rules).400 cadence/local verification retained; runner unobserved. Ledger9/pointer13 terminal0 consumed, collection-ledger-final.log.
  Final staged gate13 green, rc=0, target/collection-staged-gate.log.

