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


Source: `045d40c2bc67e9ec02686945cd2245c343271285`, docs/tasks/G1-SLICE-producers.md. Byte intervals are zero-based, end-exclusive.

| Record | Source interval | Bytes / lines | SHA-256 |
| --- | --- | --- | --- |
| [Individual Ease](#individual-ease) | 42363–45254 | 2891 / 9 | `50817bc0333523b33b1b3516c6c0b1632ded7a412e952b5df725327a32759388` |
| [Ease sets](#ease-sets) | 45672–50504 | 4832 / 31 | `b33fe3fe7f63c5b6637d10408564ef357431d1425a58240a173465323c264491` |

### Individual Ease

- [x] REPRODUCE / ISSUE: actual committed Ease predicate accepts passed assertion_name plus expect-only panic; full AST/eval with return101 and no effects, rc=0, target/ease-predicate-diagnostic.log. Entry omits8 filenames/mutable intent/current test source; D156/D177 owned here. Full two entries/wrappers/implementations/public/contracts/tests/book and roadmap reviewed before edits.
- [x] ROOT CAUSE: pure CASES follows activation/reads; directories omit file identity and mutable intent; assertion bytes omit macro provenance. Independent complete eight-child/file capture and four physical first/last output symlink/mutable hard-link/missing-test refusals pass, rc=0, target/ease-producer-controls.log; all alias bytes unchanged/no unsafe native effects.
- [x] FIX: pure6 cases before eight-file/two-input/one-mutable-source guard; five dependent statements/order and final7 cases AST-exact, current failed-body macro byte sites required. Six measurement captures/24 physical refusals/five entry and three classifier reds per entry pass; shared221 body reds/26 classifiers/all prior31/24/26/37/26/two compiler/three static controls0, rc=0, target/ease-producer-controls.log. Simulation proves dispatch only; old filters/stores/full restoration retained.
- [x] ADDRESSED: exclusive caller-store-absent seven real macro reds/full13 restored, rc=0, target/ease-native-final.log. Independent selected-test/sites/eight frozen inputs/two HEAD Rust files/original6 cases/five statements/eight single-link local logs exact, rc=0, ease-native-proof.log. Public Cargo-selected artifact SHAa8cd5d21f5390ba124358f472e5d3501724ab9119fa0c6b6eb38942bd6f02828/all13 tests0, rc=0, ease-native-artifact-proof.log; Rust unchanged. First helper replay1 missed numeric thread IDs; corrected, failed attempt excluded.
- [x] NO REGRESSION: publication45/coverage9/census229paths288checks/effect67paths29groups/file120paths48owners18groups/15refusals12reds/map freshness/ledger9+13 pass, rc=0, target/ease-*-final logs; terminal jobs consumed. Whole ade5165 DEV6673–8312/9L1639B and CH23758–24346/7L588B exact; prior438/all36 archive files/public440/13open163sealed/IDs1–177 except18/no gaps or overlap/seven windows exact, rc=0, ease-retention-final.log.
- [x] LOCKSTEP: five whole ade5165 native bodies copied/verified before retirement; full provenance in owned sibling/canonical headers/IDs/status/Paths exact; map8102B/prior ordered targets exact, rc=0, ease-producer-retention-proof.log/ease-map-preview.log. Pre-copy parse1/title fixture1/checklist608B preflight1 excluded and corrected. Book/live/MEMORY/G1/index route .n03.2; D177 six fixed/9 P1 remain. Fixed caps/grammar/400 cadence retained; promotion: declined (existing source/effect/whole-record rules). Archive23raw/16879L/1224285decodedB/547370residentB fits caps, rc=0.

  Final staged gate13 green, rc=0, target/ease-staged-gate.log.

### Ease sets

0163 owns the complete entry/wrapper/current431-line implementation/599-line fourteen-test
contract/public exports/.4b.2 canonical contract and normative/runtime Ease book reviewed before
changes; reviewed inline macros/current membership and mapping validation spans. Preserve pure8
cases/six dependent statements/ten selected filters/stores/full restoration; derive11 output files
before reads with two inputs/one explicit mutable source/current inline byte sites. Own shared
python_producer_contract.py/Target helpers, full dispatch/physical/omission/noise/inline controls,
exclusive real replay and eight frozen inputs/two HEAD Rust/full current public artifact proof.
Retain the whole oldest chart lesson and two oldest ledgers from1b74946 into fresh seals before
live growth; prove exact Git spans/SHA/prior archive bytes/readers/fixed caps before completion.

- [x] REPRODUCE / ISSUE: actual full committed Ease-set predicate/return101 accepts passed assertion_name plus expect-only failure without writes/native effects, rc=0, target/ease-set-predicate-diagnostic.log. Original entry omits11 filenames/mutable intent/current test source; D156/D177 owned here. Complete entry/wrapper/public/implementation/fourteen-test/canonical/normative/runtime contract reviewed before changes.
- [x] ROOT CAUSE: CASES follows activation/reads; directory identity omits file leaves and mutable intent; broad assertion bytes omit current macro provenance. Independent complete eleven-child/file capture/four physical copied-entry first/last output symlink/mutable hard-link/missing-test refusals pass, rc=0, ease-set-producer-controls.log. Source/alias bytes and prepared stores unchanged; no unsafe native dispatch.
- [x] FIX: pure8 cases before11-file/two-input/one-mutable-source guard; six dependent statements/order and10 faults AST-exact; require current failed-body macro byte sites including inline closures. Shared230 body reds/27 classifiers/seven complete captures/28 physical refusals/all prior31/24/26/37/26/two compiler/three static controls0, rc=0, ease-set-producer-controls.log. ASCII/UTF-8 positives/wrong-byte refusals/actual leading-only helper red pass; simulation proves dispatch only.
- [x] ADDRESSED: exclusive caller-store-absent ten real macro reds/full14 restored, rc=0, ease-set-native-final.log. Independent selected-test/current-byte sites/eight frozen inputs/two HEAD Rust files/pure8 cases/six statements/eleven local single-link logs exact, rc=0, ease-set-native-proof.log. Public Cargo-selected artifact SHAc283cb01b400748d620aac70ec22df4e8ddc95eeb8587db6a826e9123b833adf executes14 tests0, rc=0, ease-set-native-artifact-proof.log; Rust/API/grammar unchanged.
- [x] NO REGRESSION: publication45/coverage9/census229paths288checks/effect67paths29groups/file120paths48owners18groups/15refusals12reds/map freshness/ledger9+13 pass, rc=0, target/ease-set-*-final logs; terminal jobs consumed. Two whole1b74946 spans/DEV6488–8232/8L1744B and CH23232–24329/14L1097B exact; prior440/all38/public442/13open163sealed/no gaps or overlap/seven windows exact, rc=0, ease-set-retention-final.log. Known P3 ls/head stderr again observed; .b.s owns scheduled repair, named DIGEST-RED still passes.
- [x] LOCKSTEP: parent .n03 done; book/spec/live/MEMORY/G1/index route numeric .n04; D177 seven fixed/8 P1 remain. Archive25raw/16923L/1228395decodedB/551480residentB fits fixed caps; full source/provenance/seals/readers exact, rc=0, ease-set-retention-final.log. DEV15444B/CH32255B/map8102B measured within unchanged health; promotion: declined (existing current-source/effect/whole-record rules).400 cadence/local verification retained; runner verdict unobserved.

  Final staged gate13 green, rc=0, target/ease-set-staged-gate.log.

0162 owns the split terminal TSV rows and blocking whole completed producer-record relocation
into docs/tasks/G1-SLICE-producer-proof.md before fresh receipts; retain exact ade5165 body bytes,
canonical IDs/status/Paths/parent membership, direct G1 sibling ownership and all public links.
Full two entries/wrappers/309+431 implementations/470+599 tests/public exports/.4b.1–2 records,
roadmap measurement/Ease and ontology §2.2/runtime book reviewed. Both entries derive whole spans
from guarded implementation text; preserve those statements and complete alias/membership predicates.
First child plans eight logs/two inputs/one mutable source before reads, matches current test macros,
preserves stores/filters/source/artifact restoration, and owns actual old-predicate/full dispatch/
physical aliases/late-test/omission/noise controls. Shared python_producer_contract.py and Target
helpers/logs are owned here. Also retain whole older DEV/CH records from ade5165 before live growth;
verify original Git intervals, prior archive bytes/readers and fixed caps before committing.


Source: `3171eac95adb84cd6f54dd97cf659aa773371938`, docs/tasks/G1-SLICE-producers.md. Byte intervals are zero-based, end-exclusive.

| Record | Source interval | Bytes / lines | SHA-256 |
| --- | --- | --- | --- |
| [Signed rounding](#signed-rounding) | 44763–49287 | 4524 / 27 | `b4d29d3de1508b00fd871c91e89e665b5cb88bd9423da8fe7be849fcf579acbe` |

### Signed rounding

Full4 entries, signed wrapper/round.rs/round_contract.rs/public unit exports/36-row fixture,
.5a.3a canonical contract, units §2/§2.3/formula §4.2 and numeric-rounding annex reviewed.
The total helper catches public unwinds then uses labelled expect; own a guarded exclusive actual
zero-guard fault to determine its panic site before deciding the current-test repair. All Target
helpers/logs/native source restoration are owned here. Extend independent fixture/source/argv
captures to sc-units, keep old measurement captures and prepare stores. Own split TSV rows and
whole older DEV/CH retention from b0bc33c before receipt growth; verify source intervals/prior
archive/readers/fixed caps. Production rounding/grammar stay unchanged; Rust test changes require
full fmt/strict Clippy/tests after exclusive real reds and exact restoration.

Actual guarded exclusive original zero fault reproduces production divide panic at round.rs80:17
and labelled expect at round_contract.rs11:46; original complete predicate accepts rc101. Ten frozen
inputs/two HEAD Rust files exact/full4 restored/public current artifact4 pass, rc=0,
target/round-expect-repro.log. D177 owns explicit macro totality assertion in the current test helper,
preserving four tests/36 rows/production rounding. New classifier must independently verify both
selected-test macros and the guarded total helper macro; no labelled-expect exception. Full Rust
fmt/strict Clippy/tests required for this test-source change. No native job remains from reproduction.

- [x] REPRODUCE / ISSUE: guarded exclusive original zero fault divides at round.rs80:17 then labelled expect at test11:46; original full predicate accepts101. Ten original inputs/two HEAD Rust files/full4 restored/current artifact4 exact, rc=0, target/round-expect-repro.log. D156 missing6 files/mutable intent/imported fixture and D177 current macro evidence owned here; full public/source/tests/fixture/canonical/book reviewed before edits.
- [x] ROOT CAUSE: directories omit leaf identity/mutable intent; broad assertion bytes accept labelled expect without a macro. Independent complete six-child/file/three-source capture and five physical first/last symlink/mutable hard-link/missing-test/missing-fixture refusals pass, rc=0, target/round-producer-controls.log. No unsafe child/write; source/alias bytes unchanged.
- [x] FIX: five original cases precede six-file/three-input/one-mutable-source guard; current failed-body macro byte sites required. Total helper explicitly asserts outer catch_unwind result before extraction, preserving typed inner UnitError/four tests/36 rows. Shared240 body reds/28 classifiers/eight complete captures/33 physical refusals/all prior31/24/26/37/26/two compiler/three static controls0, rc=0, round-producer-controls.log; selective helper/fixture omission and source-panic/expect/wrong-site controls discriminate. Simulation proves dispatch only.
- [x] ADDRESSED: exclusive five real macro reds/full4 restored, rc=0, target/round-native-final.log. Ten frozen inputs/production-module-fixture HEAD exact/helper-only test patch/five cases/six local single-link logs/current sites40:9,23:9,40:9,72:9,12:5 exact, rc=0, round-native-proof.log. Cargo-current artifact SHAe032081de15d5b20713dfa64980a79f8c5b195c4ebeb2974e1546527a5257473 executes4 tests0, rc=0, round-native-artifact-proof.log. Replay review corrected obsolete all-tests-HEAD assumption before execution; production unchanged.
- [x] NO REGRESSION: full Rust fmt/strict Clippy/703 tests across59 groups/0 failed/0 ignored pass, rc=0, target/round-rust-check.log. Publication45/coverage9/census229paths288checks/effect67paths29groups/file120paths51owners21groups/15refusals12reds/map/ledger9+13 pass, rc=0, round-*-final logs; terminal jobs consumed. Whole b0bc33c DEV6939–8254/7L1315B and CH22928–23789/10L861B retained; prior442/all40/public444/13open163sealed/no gaps or overlap/seven windows exact, rc=0, round-retention-final.log.
- [x] LOCKSTEP: signed child done; parent retains four paths with three pending children; book/live/MEMORY/G1/index route unsigned .n04.2. D177 eight fixed/7 P1 remain; production rounding/grammar/400 cadence unchanged. Archive27raw/16962L/1231839decodedB/554924residentB fits fixed caps, rc=0, round-retention-final.log. DEV line324B pre-stage census tightened to318B; promotion: declined (existing current-source/effect/whole-record rules). Local verification only; runner verdict unobserved.

  Final staged gate13 green, rc=0, target/round-staged-gate.log.


Source: `9075e1cc6f6aa773446de374748d7b6ba9f99bbd`, docs/tasks/G1-SLICE-producers.md. Byte intervals are zero-based, end-exclusive.

| Record | Source interval | Bytes / lines | SHA-256 |
| --- | --- | --- | --- |
| [Unsigned rounding](#unsigned-rounding) | 45532–50134 | 4602 / 25 | `0a22c8ca3959616d2e52f5484b2ee38d7eb7252a6c0ba92e45020f135c59f264` |

### Unsigned rounding

Full entry/wrapper/round.rs/public exports/five-test contract/138-row imported fixture/Decimal
verifier, .5a.3c.1 canonical record, roadmap numeric/G1 and units/rounding/build book reviewed.
Own guarded actual old zero fault before any helper repair; require current macro evidence and
full Rust gate if tests change. Preserve CALL/CASES AST and debug/release order, prepare stores,
all12 files/three inputs/one mutable source. Own shared dispatch/physical/classifier/body controls,
exclusive native proof and ten frozen inputs/both current artifacts. Target helpers/logs owned.
Before fresh receipts exceed live health, copy whole committed Ease bodies into the existing
producer-proof sibling and verify before retirement; canonical headers/status/paths stay live.
Own whole older DEV/CH seals from045d40c before growth; prove source intervals/bytes/prior readers
and fixed caps. Production/grammar/400 cadence unchanged; no early push or runner verdict.

Actual guarded original zero fault reaches production80:17 then unsigned test16:10 labelled
expect; original complete predicate accepts101. Ten inputs/two HEAD Rust files/full5 restored/
Cargo-current artifact5 exact, rc=0, target/unsigned-round-expect-repro.log. D177 owns replacing
that helper with an explicit outer-unwind assertion; preserve five tests/138 rows/typed results.

- [x] REPRODUCE / ISSUE: guarded original zero fault panics at production80:17 then labelled expect at unsigned test16:10; original complete predicate accepts101. Ten original inputs/two HEAD Rust files/full5 restored/Cargo-current artifact5 exact, rc=0, target/unsigned-round-expect-repro.log. D156 missing12 files/mutable intent/test-fixture inputs and D177 current macro evidence owned before edits; full source/public/five tests/138 rows/canonical/book reviewed.
- [x] ROOT CAUSE: CALL/CASES follows activation/source read; directory identity omits leaves/mutable intent, and broad assertion bytes omit macro provenance. Independent complete twelve-child/file capture and five physical first/last log symlink/mutable hard-link/missing-test/missing-fixture refusals pass, rc=0, target/unsigned-round-producer-controls-final.log. Alias/source bytes unchanged; no unsafe native effects. Authored case/profile/test/log order checks the release fault before restoration.
- [x] FIX: unchanged CALL/CASES before12-file/three-input/one-mutable-source guard/current failed-body macro byte sites. Total helper asserts outer unwind result then extracts typed inner result; five tests/138 rows intact. Shared256 body reds/29 classifiers/nine complete captures/38 physical refusals/all prior31/24/26/37/26/two compiler/three static controls0, rc=0, unsigned-round-producer-controls-final.log. Actual release-file/profile/premature-restoration/restore-profile/restore-omission/helper/fixture faults discriminate; simulation proves dispatch only.
- [x] ADDRESSED: exclusive nine debug/one release macro reds/full5 restored each, rc=0, unsigned-round-native-final.log. Ten frozen inputs/original CALL/CASES/production-module-fixture HEAD/helper-only patch/twelve local single-link logs/current selected sites exact, rc=0, native-proof.log. Cargo-current debug SHA74e4802f68c9bffe76e390e7be71b51ed9fbc2d5c89eadc13068a49b32fe2af2/releaseSHAa6080e46c5866eeb9f64e87f1d19e8bf95c767945628deb8cebd2cab53751d50 each executes5 tests0, rc=0, native-artifact-proof.log; production unchanged.
- [x] NO REGRESSION: full Rust fmt/strict Clippy/703 tests across59 groups/0 failed/0 ignored pass, rc=0, target/unsigned-round-rust-check.log. Publication45/coverage9/census229paths288checks/effect67paths29groups/file120paths51owners21groups/15refusals12reds/map/ledger9+13 pass, rc=0, unsigned-round-*-final logs; all terminal jobs consumed. Whole045d40c DEV6037–8921/15L2884B and CH23063–23593/6L530B/prior444/all42/public446/13open163sealed/no gaps/seven windows exact, rc=0, unsigned-round-retention-final.log.
- [x] LOCKSTEP: whole045d40c Ease bodies/provenance/prior18917B sibling prefix/canonical headers/status/Paths/map8102B exact, rc=0, unsigned-producer-retention-proof.log. Book/live pointers route .n04.3; length6 faults/two selected/full4 public tests clarified. D177 nine fixed/6 P1; archive29raw/17005L/1236537decodedB/559622residentB fits caps, rc=0. Generic254 preflight1 hit historical8254 before writes; scoped anchors corrected. Pre-stage632B box tightened; failed attempts excluded. promotion: declined (existing source/effect/profile/whole-record rules). Grammar/400/local scope retained.

  Final staged gate13 green, rc=0, target/unsigned-round-staged-gate.log.

