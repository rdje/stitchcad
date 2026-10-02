# CHANGELOG.md

Newest first: one section per completed slice, in commit order. Older slices live in sealed, immutable
segments under `docs/history/`, each named below with its identity and retrieval path.

# Sealed archive — earlier slices

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`stitchcad-changelog-part1.md`](docs/history/window1.md#stitchcad-changelog-part1md) | slices 1–15, `STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004b` | 365 lines, 30452 bytes, `sha256:f4aec75a…` |
| [`stitchcad-changelog-part2.md`](docs/history/window1.md#stitchcad-changelog-part2md) | slices 16–20, `STITCHCAD-SPINE-0014` … `STITCHCAD-G0-0002` | 152 lines, 12811 bytes, `sha256:5783ac36…` |
| [`stitchcad-changelog-part3.md`](docs/history/window1.md#stitchcad-changelog-part3md) | slices 21–24, `STITCHCAD-G0-0013` … `STITCHCAD-G0-0018` | 147 lines, 12289 bytes, `sha256:14ad5278…` |
| [`stitchcad-changelog-part4.md`](docs/history/window1.md#stitchcad-changelog-part4md) | slices 25–29, `STITCHCAD-G0-0004` … `STITCHCAD-SPINE-0017` | 177 lines, 15440 bytes, `sha256:a8cc1de6…` |
| [`stitchcad-changelog-part5.md`](docs/history/window1.md#stitchcad-changelog-part5md) | slices 30–31, `STITCHCAD-G0-0005` … `STITCHCAD-G0-0013c` | 82 lines, 7505 bytes, `sha256:18548ff7…` |
| [`stitchcad-changelog-part6.md`](docs/history/window1.md#stitchcad-changelog-part6md) | slices 32–33, `STITCHCAD-G0-0007` … `STITCHCAD-G0-0006` | 93 lines, 8284 bytes, `sha256:3148dd0f…` |
| [`stitchcad-changelog-part7.md`](docs/history/window1.md#stitchcad-changelog-part7md) | slices 34–35, `STITCHCAD-G0-0013d` … `STITCHCAD-G0-0008` | 105 lines, 9793 bytes, `sha256:ff62d418…` |
| [`stitchcad-changelog-part8.md`](docs/history/window1.md#stitchcad-changelog-part8md) | slices 41–42, `STITCHCAD-G0-0014` … `STITCHCAD-G0-0004b` | 106 lines, 9766 bytes, `sha256:2c7ee35a…` |
| [`stitchcad-changelog-part9.md`](docs/history/window1.md#stitchcad-changelog-part9md) | the two oldest live entries, `STITCHCAD-G0-0004c` and `STITCHCAD-SPINE-0004d` — no slice range, because the earlier ranges have no producer (D51) | 90 lines, 8386 bytes, `sha256:8e4081d4…` |
| [`stitchcad-changelog-part10.md`](docs/history/window1.md#stitchcad-changelog-part10md) | two spine slices on the table convention, `STITCHCAD-SPINE-0020` and `STITCHCAD-SPINE-0015` | 73 lines, 6652 bytes, `sha256:062ccfa3…` |
| [`stitchcad-changelog-part11.md`](docs/history/window1.md#stitchcad-changelog-part11md) | the delegation-and-uncertainty slice, `STITCHCAD-G0-0014c` | 46 lines, 4507 bytes, `sha256:de34382e…` |
| [`stitchcad-changelog-part12.md`](docs/history/window1.md#stitchcad-changelog-part12md) | the dialects and formula-language slices, `STITCHCAD-G0-0010` and `STITCHCAD-G0-0009` | 84 lines, 7767 bytes, `sha256:9c61ba7c…` |
| [`stitchcad-changelog-part13.md`](docs/history/window1.md#stitchcad-changelog-part13md) | the release-contract and canvas-spike-rule slices, `STITCHCAD-G0-0012` and `STITCHCAD-G0-0011` | 75 lines, 7188 bytes, `sha256:1e52b5c9…` |
| [`stitchcad-changelog-part14.md`](docs/history/window1.md#stitchcad-changelog-part14md) | the i18n slice, `STITCHCAD-G0-0016` | 38 lines, 3631 bytes, `sha256:2c895780…` |
| [`stitchcad-changelog-part15.md`](docs/history/window1.md#stitchcad-changelog-part15md) | the recurring cleanup slice, `STITCHCAD-SPINE-0021` | 23 lines, 2123 bytes, `sha256:f93154e3…` |
| [`stitchcad-changelog-part16.md`](docs/history/window1.md#stitchcad-changelog-part16md) | the closed-defect sealing slice, `STITCHCAD-SPINE-0019a` | 27 lines, 2450 bytes, `sha256:211f9bec…` |
| [`stitchcad-changelog-part17.md`](docs/history/window1.md#stitchcad-changelog-part17md) | the command-layer contract, `STITCHCAD-G0-0017` | 38 lines, 3626 bytes, `sha256:819f240a…` |
| [`stitchcad-changelog-part18.md`](docs/history/window1.md#stitchcad-changelog-part18md) | the G0 exit review, `STITCHCAD-G0-0015` | 38 lines, 3694 bytes, `sha256:22d63ec4…` |

**Correction (D30).** part1's own descriptor says its coverage runs "through `STITCHCAD-SPINE-0004c`".
It does not: part1's newest entry is `STITCHCAD-SPINE-0004b`, and `SPINE-0004c` is sealed in part2.
Sealed segments are immutable, so the correction is recorded here and in part2's descriptor rather than
by editing part1.

The bedrock scaffold's own changelog — the provenance of this repository's discipline spine — is sealed
in [`docs/history/bedrock-scaffold-changelog.md`](docs/history/window1.md#bedrock-scaffold-changelogmd).
| [`stitchcad-changelog-part19.md`](docs/history/window1.md#stitchcad-changelog-part19md) | STITCHCAD-G1-0001 | 21 lines, 1770 bytes, `sha256:2f602e9a…` |
| [`stitchcad-changelog-part20.md`](docs/history/window1.md#stitchcad-changelog-part20md) | STITCHCAD-G1-0002 | 19 lines, 1598 bytes, `sha256:a3918baa…` |
| [`stitchcad-changelog-part21.md`](docs/history/window1.md#stitchcad-changelog-part21md) | STITCHCAD-G1-0003 | 21 lines, 1831 bytes, `sha256:ab5e04ca…` |

| [`stitchcad-changelog-part22.md`](docs/history/window1.md#stitchcad-changelog-part22md) | STITCHCAD-G1-0004 | 24 lines, 2100 bytes, `sha256:9050689c…` |

| [`stitchcad-changelog-part23.md`](docs/history/window1.md#stitchcad-changelog-part23md) | STITCHCAD-G1-0005 | 48 lines, 4079 bytes, `sha256:e74210d6…` |

| [`changelog-part24.md`](docs/history/window2.md#stitchcad-changelog-part24md) | STITCHCAD-G1-0007, STITCHCAD-G1-0006 | 39 lines, 3139 bytes, `sha256:30ff1380…` |

| [`changelog-part25.md`](docs/history/window2.md#stitchcad-changelog-part25md) | STITCHCAD-G1-0008 | 17 lines, 1371 bytes, `sha256:d57637be…` |

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

| [`stitchcad-changelog-part26.md`](docs/history/window2.md#stitchcad-changelog-part26md) | STITCHCAD-G1-0009 | 16 lines, 1371 bytes, `sha256:ff2d0b1c…` |

| [`stitchcad-changelog-part27.md`](docs/history/window2.md#stitchcad-changelog-part27md) | STITCHCAD-G1-0010 | 20 lines, 1663 bytes, `sha256:b3824493…` |

| [`stitchcad-changelog-part28.md`](docs/history/window2.md#stitchcad-changelog-part28md) | STITCHCAD-G1-0011 | 20 lines, 1689 bytes, `sha256:6900e83e…` |

| [`stitchcad-changelog-part29.md`](docs/history/window2.md#stitchcad-changelog-part29md) | STITCHCAD-G1-0012 | 15 lines, 1306 bytes, `sha256:88277bd5…` |

| [`stitchcad-changelog-part30.md`](docs/history/window2.md#stitchcad-changelog-part30md) | STITCHCAD-G1-0013 | 14 lines, 1211 bytes, `sha256:7eb41b35…` |

| [`stitchcad-changelog-part31.md`](docs/history/window2.md#stitchcad-changelog-part31md) | STITCHCAD-G1-0014 | 13 lines, 1097 bytes, `sha256:28bbb8fb…` |

| [`stitchcad-changelog-part32.md`](docs/history/window2.md#stitchcad-changelog-part32md) | STITCHCAD-G1-0015 | 13 lines, 1096 bytes, `sha256:9e00082b…` |

| [`changelog-part33.md`](docs/history/window2.md#stitchcad-changelog-part33md) | STITCHCAD-G1-0016 | 14 lines, 1199 bytes, `sha256:7fc5a9cb…` |

| [`changelog-part34.md`](docs/history/window2.md#stitchcad-changelog-part34md) | STITCHCAD-G1-0017 | 13 lines, 1094 bytes, `sha256:a8c9973a…` |

| [`changelog-part35.md`](docs/history/window2.md#stitchcad-changelog-part35md) | STITCHCAD-G1-0018 | 13 lines, 1081 bytes, `sha256:74e03b90…` |

| [`changelog-part36.md`](docs/history/window2.md#stitchcad-changelog-part36md) | STITCHCAD-G1-0020 / STITCHCAD-G1-0019 | 29 lines, 2350 bytes, `sha256:64af4d6d…` |

| [`changelog-part37.md`](docs/history/window2.md#stitchcad-changelog-part37md) | STITCHCAD-G1-0021 | 13 lines, 1082 bytes, `sha256:2feb224c…` |

| [`changelog-part38.md`](docs/history/window2.md#stitchcad-changelog-part38md) | STITCHCAD-G1-0022 | 13 lines, 1098 bytes, `sha256:fbe202b2…` |

| [`changelog-part39.md`](docs/history/window2.md#stitchcad-changelog-part39md) | STITCHCAD-G1-0023 | 15 lines, 1292 bytes, `sha256:b06df32f…` |

| [`changelog-part40.md`](docs/history/window2.md#stitchcad-changelog-part40md) | STITCHCAD-SPINE-0021a | 9 lines, 799 bytes, `sha256:156e9198…` |

| [`changelog-part41.md`](docs/history/window2.md#stitchcad-changelog-part41md) | STITCHCAD-G1-0024 | 12 lines, 1073 bytes, `sha256:5f9373df…` |

| [`changelog-part42.md`](docs/history/window2.md#stitchcad-changelog-part42md) | STITCHCAD-G1-0025 | 10 lines, 880 bytes, `sha256:d8268d8b…` |

| [`changelog-part43.md`](docs/history/window2.md#stitchcad-changelog-part43md) | STITCHCAD-G1-0026 | 13 lines, 1217 bytes, `sha256:30323afa…` |

| [`changelog-part44.md`](docs/history/window2.md#stitchcad-changelog-part44md) | STITCHCAD-G1-0027 | 6 lines, 465 bytes, `sha256:2c04058f…` |

| [`changelog-part45.md`](docs/history/window2.md#stitchcad-changelog-part45md) | STITCHCAD-SPINE-0019b | 9 lines, 809 bytes, `sha256:dfe49803…` |

| [`stitchcad-changelog-part46.md`](docs/history/window2.md#stitchcad-changelog-part46md) | G1-0028 and SPINE-0019c | 22 lines, 1861 bytes, `sha256:62f1e78a…` |

| [`stitchcad-changelog-part47.md`](docs/history/window3.md#stitchcad-changelog-part47md) | G1-0029 | 14 lines, 1170 bytes, `sha256:575d31eb…` |

| [`stitchcad-changelog-part48.md`](docs/history/window3.md#stitchcad-changelog-part48md) | G1-0030 | 14 lines, 1137 bytes, `sha256:0f58d786…` |

| [`stitchcad-changelog-part49.md`](docs/history/window3.md#stitchcad-changelog-part49md) | G1-0031 | 8 lines, 682 bytes, `sha256:ed0742e6…` |

| [`stitchcad-changelog-part50.md`](docs/history/window3.md#stitchcad-changelog-part50md) | STITCHCAD-G1-0032 | 14 lines, 1176 bytes, `sha256:32b7947f…` |

| [`stitchcad-changelog-part51.md`](docs/history/window3.md#stitchcad-changelog-part51md) | STITCHCAD-G1-0033 | 13 lines, 1094 bytes, `sha256:8f279ee4…` |

| [`stitchcad-changelog-part52.md`](docs/history/window3.md#stitchcad-changelog-part52md) | STITCHCAD-G1-0034 | 14 lines, 1219 bytes, `sha256:43a87a6f…` |

| [`stitchcad-changelog-part53.md`](docs/history/window3.md#stitchcad-changelog-part53md) | STITCHCAD-G1-0035 | 13 lines, 1094 bytes, `sha256:83494a18…` |

| [`stitchcad-changelog-part54.md`](docs/history/window3.md#stitchcad-changelog-part54md) | STITCHCAD-G1-0036 | 15 lines, 1273 bytes, `sha256:d5122987…` |

| [`stitchcad-changelog-part55.md`](docs/history/window3.md#stitchcad-changelog-part55md) | STITCHCAD-G1-0037 | 15 lines, 1284 bytes, `sha256:49280c1f…` |

| [`stitchcad-changelog-part56.md`](docs/history/window3.md#stitchcad-changelog-part56md) | STITCHCAD-G1-0038 | 14 lines, 1147 bytes, `sha256:8f2b2b4f…` |

| [`stitchcad-changelog-part57.md`](docs/history/window3.md#stitchcad-changelog-part57md) | STITCHCAD-G1-0039 | 12 lines, 958 bytes, `sha256:8ee0ecac…` |

| [`stitchcad-changelog-part58.md`](docs/history/window3.md#stitchcad-changelog-part58md) | STITCHCAD-G1-0040 | 15 lines, 1267 bytes, `sha256:5d9ffa4d…` |

| [`stitchcad-changelog-part59.md`](docs/history/window3.md#stitchcad-changelog-part59md) | STITCHCAD-G1-0041 | 13 lines, 1098 bytes, `sha256:38e6cdf2…` |

| [`stitchcad-changelog-part60.md`](docs/history/window3.md#stitchcad-changelog-part60md) | STITCHCAD-G1-0042 | 11 lines, 999 bytes, `sha256:717945df…` |

| [`stitchcad-changelog-part61.md`](docs/history/window3.md#stitchcad-changelog-part61md) | STITCHCAD-G1-0044 / STITCHCAD-G1-0043 | 28 lines, 2471 bytes, `sha256:5484241b…` |

| [`stitchcad-changelog-part62.md`](docs/history/window3.md#stitchcad-changelog-part62md) | STITCHCAD-G1-0045 | 11 lines, 970 bytes, `sha256:a334432a…` |

| [`stitchcad-changelog-part63.md`](docs/history/window3.md#stitchcad-changelog-part63md) | STITCHCAD-G1-0046 | 11 lines, 950 bytes, `sha256:939369e8…` |

| [`stitchcad-changelog-part64.md`](docs/history/window3.md#stitchcad-changelog-part64md) | STITCHCAD-G1-0047 | 11 lines, 995 bytes, `sha256:339b8cba…` |

| [`stitchcad-changelog-part65.md`](docs/history/window3.md#stitchcad-changelog-part65md) | STITCHCAD-G1-0049, STITCHCAD-G1-0048 | 15 lines, 1114 bytes, `sha256:e746de86…` |

| [`stitchcad-changelog-part66.md`](docs/history/window3.md#stitchcad-changelog-part66md) | STITCHCAD-G1-0051, STITCHCAD-G1-0050 | 18 lines, 1463 bytes, `sha256:3dcee333…` |

| [`changelog-part67.md`](docs/history/window3.md#stitchcad-changelog-part67md) | STITCHCAD-G1-0052 | 10 lines, 844 bytes, `sha256:d0d97d54…` |

| [`changelog-part68.md`](docs/history/window3.md#stitchcad-changelog-part68md) | STITCHCAD-G1-0054/0053 | 17 lines, 1299 bytes, `sha256:5a46d26d…` |

| [`changelog-part69.md`](docs/history/window3.md#stitchcad-changelog-part69md) | STITCHCAD-G1-0055 | 10 lines, 776 bytes, `sha256:e3db9c94…` |

| [`stitchcad-changelog-part70.md`](docs/history/stitchcad-changelog-part70.md) | STITCHCAD-G1-0056 | 12 lines, 951 bytes, `sha256:30ca94ac…` |

| [`changelog-part71.md`](docs/history/stitchcad-changelog-part71.md) | STITCHCAD-G1-0057 | 11 lines, 887 bytes, `sha256:e144c5ec…` |

## STITCHCAD-G1-0075 - observed third-window CI and immutability (leaf `G1-SLICE.5b.1b.0v`)

Exact pushed b595a37: both CI jobs/all steps completed success. Exclusive archive checks pass28
probes/200 CLIcontrols/191 logical reads, including newest committed catalog refusal. Local receipts
and book agree; archive/schema/caps unchanged. G1 stays5/18, defects12open/102sealed.
Next P0 SPINE.23 repairs handoff evidence; namespace review resumes after its clean completion.

## STITCHCAD-G1-0074 - third exact retained history window (leaf `G1-SLICE.5b.1b.0`)

Capacity64 blocks the next namespace-review seal. Capture62 raw full files from af98fff into window3;
all189 logical records reconstruct exactly in fresh capture/installed-input fixtures before retirement.
Prior windows and original bytes/addresses remain unchanged. Selected residue0;63 maintained links
across three live files now land on catalog headings. No reader/checker/schema/limit changed.
Full local checks and required exceptional push precede observed CI .0v, then namespace .1b resumes.
Oldest live ledger payloads remain complete/exact; G1 stays5/18, defects12open/102sealed.
D114 blind census and D115 idle CUA metadata false blocking are reproduced; SPINE.23 owns P0 repair.

## STITCHCAD-G1-0073 - independent complete static signature review (leaf `G1-SLICE.5b.1a`)

D112/D113 close: within admits exactly five tolerance names; min/max accept their documented
one-argument base case. Independently authored closed populations and4032 actual parse/infer cases
cover all eight kinds/22 names/operators/arity boundaries/conditionals/tolerance candidates/envelope
precedence, with numeric/state/geometry reads and execution trapped. Twelve compiled actual guard
faults fail body assertions; producer remains byte-identical on disk. Existing reference controls green.
New expert annex preserves progressive reading; product namespace/type/whole preflight/execution
remain pending. Review .5b.1 splits into signature .1a, namespace/preflight .1b and full closure .1c.
Oldest ledger reports retained whole; G1 stays5/18; defects10open/102sealed. Next .5b.1b.

## STITCHCAD-SPINE-0021b - due safe artifact cleanup (leaf `SPINE.21b`)

Tracked plan/apply producer verifies local ignored ownership, content/HEAD/input identity and protected
stores before removal. Fifteen controls/eleven actual compiled guard assertion reds pass without source
mutation; new standing suite makes26. Removed six output roots/1281 strays/1112101558 file bytes;
independent residue0/tracked deletion0, target1955324→1071060KB plus7172KB book removed.
Rust591/WASM3/book/full26 probes and staged gates regenerate/pass; exact ledger records preserved.
Latest cleanup record and book/tool/live navigation aligned; G1 stays5/18, defects10open/100sealed.
Return to G1-SLICE.5b.1 static review. No product scope, dependency stores or other repository changed.

## STITCHCAD-G1-0072 - syntax milestone and evaluator ownership (leaf `G1-SLICE.5a.4`)

Complete syntax/input/identity proof map closes .5a without evaluator or geometry claims. Full native
591 tests/48groups, WASM3 and all25 probe suites pass; seven coupled actual compiled assertion reds
restore all four product sources exactly, then public review5 passes. All existing Rust bytes stay exact.
D111 fixes current statement-identity status; .5b–.5g own23 pending static/numeric/irrational/replay/
operation/final acceptance children. Book52/25 APIs/1093 source/1688 rendered links and task pointers
agree. Completed node graph/prior closure and oldest ledger payloads preserved exactly; G1 stays5/18,
defects10open/100sealed. Next .5b.1 independent static review, checking cleanup due first.

## STITCHCAD-G1-0071 - coupled whole input and identity review (leaf `G1-SLICE.5a.3f.2`)

Actual17 worked bindings/four assertions produce21 exact statements/25 operands and a complete
ordered identity. Independently authored whole statement bytes agree with actual reference syntax,
with semantics trapped. All13 actual refusal examples distinguish3 syntax errors from10 deferred
semantic checks. Later branch/call input errors and combined maximum/first excess retain context.
Five new public contracts/seven actual compiled assertion reds/exact four-source restoration;
strict native591/48groups, release5/WASM3, scoped reference/language16/publication9 pass.
Book52 chapters/25 APIs/1089 source/1682 rendered links publishes the complete obligation map and
remaining static/binding/evaluation/geometry/store/MCP owners. Every existing Rust source/test exact.
Complete recipe evidence/closure moved byte-identically to bounded sibling with old anchor routes;
map orientation shortened under unchanged cap. Oldest ledger payloads independently compare exact.
G1 stays5/18, defects10open/99sealed; tree13/10 siblings. .3f/.3 input-identity scope closes;
next .5a.4 full syntax/canonical milestone and safe evaluator decomposition. No evaluation/signoff claim.

## STITCHCAD-G1-0070 - owned statement and recipe identity bytes (leaf `G1-SLICE.5a.3f.1c`)

Private typed canonical statement/recipe owners retain exact names/annotations and authored order,
using existing normalized inputs/expression identity. One ASCII space/no newline; empty recipe
retains its wrapper. Clone/Eq/explicit extraction outlive all source/arenas; Debug omits customer text.
Ten public contracts compare16 authored statements/nine recipes,55 expressions×three roles and100
independent numeric rows×three roles, actual four published examples and two typed text collisions.
Full4096×2×256/16if serialization, long names/grouping/deep/wide calls pass on64KiB stack.
Twenty-one actual compiled assertion reds/exact restoration; five negative/two runnable API docs.
Strict native586/47groups, release10/WASM3 and scoped reference/language16/publication9 pass;
book52 chapters/25 APIs/1083 source/1674 rendered links. Shared nine sources/prior records exact.
Book/API/live/task records agree; G1 stays5/18, defects10open/99sealed; next .5a.3f.2 coupled review.
No static validation, binding/evaluation, typed project hashes/storage, MCP or production approval.

## STITCHCAD-G1-0069 - complete recipe literal input normalization (leaf `G1-SLICE.5a.3f.1b`)

Private normalized statement/recipe owners retain names, closed annotations, all original global
spans/order and source borrowing after syntax drop. Shared expression conversion normalizes every
input; contextual literal errors retain operand/known1-based index/rule/span and Error source chain.
Eight contracts compare authored16 statements/nine recipes and100 independent numeric rows in
all three roles; simultaneous4096×2×256 nodes/16if conversion/Clone/drop passes on64KiB stack.
Seventeen actual compiled assertion reds/exact source restoration verify production composition.
Strict native569/46groups, release8/WASM3 and focused reference/book checks pass:52 chapters/23 APIs/
1073 source/1663 rendered links, language16/publication9. Prior sources/closure/oldest records exact.
Book/API/status/progressive routes and live docs align; G1 stays5/18, defects10open/99sealed.
Next .5a.3f.1c owned statement/recipe serializer; no binding/evaluation/storage/MCP/signoff claim.

## STITCHCAD-G1-0068 - exact statement and recipe byte contract (leaf `G1-SLICE.5a.3f.1a`)

D109 closes technically: retained bind bytes, flat named/tolerance assertion operands, explicit
ordered recipe wrapper and empty(recipe), single spaces/no final newline. Engineering delegation,
same-party author/applier, independent approval unclaimed and reversal/migration path are recorded.
Sixteen authored statement rows cover six kinds/five tolerances; nine whole sources/twelve chunks
match actual reference syntax. Nine actual renderer faults fail exact-byte assertions and restore
source; watched structural controls retain explicit non-product scope. Focused syntax20/language16/
publication9 pass. All Rust sources/tests and predecessor full syntax subtree/closure stay exact.
Book/grammar/decision/current status stay aligned; G1 remains5/18, defects10open/99sealed.
Next .5a.3f.1b complete statement/recipe input normalization, then .1c owned identity. No runtime,
project format/hash, human approval or production signoff added.

## STITCHCAD-G1-0067 - coupled statement and recipe syntax review (leaf `G1-SLICE.5a.3e.3`)

Three new public controls verify authored zero-gap/token-prefix boundaries, exact later-statement
header/operand diagnostics and simultaneous4096×2×256-node/16-if maxima on64KiB stack. Five actual
compiled assertion reds target only those controls and restore source exactly; all product Rust sources
remain byte-identical to415d577. Independent fixtures/reference and syntax-only proof map agree.
D110 diagnostic context wording is fixed; D109 exact assertion/recipe identity contract remains owned
next before serializer implementation. Strict native553/45groups, release20/WASM3 and scoped book
checks pass:51 chapters/21 API rows/1058 source/1636 rendered links, language16/publication9.
Prior proof/oldest ledgers preserve exact bytes. G1 stays5/18, defects11 open/98 sealed;
next .5a.3f.1 exact recipe byte contract and normalization. No numeric runtime/MCP/signoff claim.

## STITCHCAD-G1-0066 - ordered original-source recipe syntax (leaf `G1-SLICE.5a.3e.2`)

FormulaRecipe retains immutable authored let/assert order, empty/multiline/same-line lists and
original whole-source statement/header/node/error spans. Shared statement parsing keeps strict
standalone consumption; nested keywords remain invalid expressions. Errors identify1-based
statement indices where known; global ASCII preflight supplies no guessed ordinal. The fixed4096
bound refuses the recognized4097th keyword with exact span/bound/measured count before its body.

Eight public contracts/nine authored whole sources/14 original statements/18 operand identities,
all21 worked statements/25 existing bytes,15 new actual compiled assertion reds and the prior15
statement faults pass with exact restoration. Strict native550/release17/WASM3 and scoped checks
pass. Recipe annex/progressive links/API map/index/README/live/task pointers align; prior proof
and oldest ledgers retain exact payloads. G1 stays5/18; next .3e.3 coupled syntax/diagnostic review.
D108 archive-copy offset is fixed with exact predecessor comparison and watched ledger checks.
Complete recipe normalization/identity, static names/types/bindings and evaluation remain owned.

## STITCHCAD-G1-0065 - single immutable formula statements (leaf `G1-SLICE.5a.3e.1`)

FormulaStatement parses one complete let/assert form with six kind/five tolerance annotations,
full-source statement/name/annotation/node spans and typed operand refusals. Assertions preserve
exactly one top-level separator and both independently bounded expression arenas. Private borrowed
construction/Clone/opaque Debug retain source lifetimes without conversion, binding or execution.

Nine public contracts/fifteen independent reference header/operand rows/six refusal families and
all21 worked statements/25 expression identities pass. Fifteen actual compiled faults fail assertions
in failed-test bodies and restore exact source; D107 false assertion classification is repaired.
Strict native538/release9/WASM3 and structural/language/publication checks pass. New statement annex,
progressive links/API map/index/README/live/task records align; D106 literal decision status is fixed.
Completed subtree/proof and oldest ledgers remain exact in bounded parts. G1 stays5/18; defects10open/
96sealed. Next .3e.2 ordered recipe/4096/context; statement identity and evaluation remain later work.

## STITCHCAD-G1-0064 - coupled canonical identity review (leaf `G1-SLICE.5a.3d.3`)

Expression identity now has a complete scoped obligation map against grammar4/5 and D84/D95/D103.
All25 worked expressions match independently checked authored bytes and exact book population;
a distinct255-argument call preserves complete order on64KiB stack. Nested unit/whitespace aliases
preserve identity while raw turns and kind remain distinct. Product serializer source stays exact.

Nine public contracts/three additional actual compiled symbol/order/truncation assertion reds pass
with byte-identical restoration. Strict native525, release9/WASM3 and structural/book controls pass.
D105 stale syntax-annex/introduction status is corrected. Current grammar/decision/API status/index/
live/task pointers agree; earlier serializer proof and oldest ledger payload retain exact text in
bounded parts. .5a.3d closes for expressions only; next .3e ordered let/assert syntax. G1 stays5/18;
defects10open/94sealed. Binding/evaluation, geometry, storage and command/API/MCP remain later work.

## STITCHCAD-G1-0063 - owned canonical expression bytes (leaf `G1-SLICE.5a.3d.2`)

Normalized expressions now emit privately constructed owned ASCII identity bytes using a flat action
stack. Full-u128 literal magnitudes/kinds, source names, all operators and ordered children survive;
D103 unary/square symbols remain distinct from ordinary calls. Eq/Clone, explicit as_str/into_string
and opaque Debug preserve source-independent identity. No folding, evaluation or project hash added.

Seven public contracts/55 independent byte fixtures/nested100 Fraction rows/six book examples and
nineteen actual compiled assertion reds pass with exact restoration. Small64KiB stack covers256-node
chains/16 if levels/50000 groups/100000-byte names. Final native523, release7/WASM3 and scoped
book/reference checks pass. D104 crate overview/status numerical drift is fixed. README/progressive
learning/availability/grammar/API map/expert annex/decision/index/live/task records align; prior proof
and ledger bytes remain exact in bounded parts. G1 stays5/18; defects10open/93sealed; next .3d.3 review.
Ordered recipes, names/types/bindings/evaluation, geometry, storage and command/API/MCP remain later work.

## STITCHCAD-G1-0062 - exact canonical byte contract (leaf `G1-SLICE.5a.3d.1`)

D103 closes: director chose unary (- child) and square (^2 child) before production serialization.
Grammar/expert annex/canonical decision specify all seven expression roles, exact ASCII spacing,
no terminal newline and preservation of normalized literals/ordered unevaluated syntax. Symbolic
opcodes distinguish operators from ordinary neg/square calls; authored syntax is unchanged.

Tracked inventory checks seven actual reference roles/ten binary symbols/two call distinctions and
six authored byte examples through independent reference rendering, watched by the structural suite.
Four actual inventory-renderer assertion reds restore exact source; these are interpreter controls.
Scoped reference/language/publication and recording checks pass; Rust implementation is unchanged.
Book/index/live/task pointers align and prior history stays exact. G1 remains5/18; defects10open/
92sealed. Next .3d.2 product canonical serializer, then .3d.3 review; no execution or persistence claim.

## STITCHCAD-G1-0061 - coupled normalization review (leaf `G1-SLICE.5a.3c.4`)

Production literal/arena input normalization now has a complete scoped obligation map. Independent
Fraction176 reduction-frontier cases exercise valid large raw mantissas after unit cancellation and
located reduced-width refusals through both individual and nested whole APIs:103 accepted/73 refused.
Four actual compiled early-scale/raw-mantissa/cancellation assertion reds pass with exact restoration.
Production implementations retain exact prior identities; no language or execution behavior changes.

Strict native514, release14 public contracts, three WASM builds and reference/book controls pass.
D101 stale-variable topic index overwrite and D102 misplaced defect entry are diagnosed, owned and
fixed; publication/exact historical-record checks verify repairs. Book/grammar/decisions/live/task
pointers agree and old payloads remain exact. .5a.3c closes for input normalization; next .5a.3d canonical
identity. G1 stays5/18; defects10open/91sealed. Ordered recipes/binding/evaluation remain future work.

## STITCHCAD-G1-0060 - immutable normalized expression arenas (leaf `G1-SLICE.5a.3c.3`)

Whole syntax arenas now explicitly normalize every literal into separate immutable source-borrowing
storage. Nodes/operators/names/ordered children/spans/depth remain intact; unary minus and raw angle
turns survive. All call arguments/both branches convert; an input refusal returns its original location
and aborts atomically. Syntax stays reusable; normalized views/iterators retain private arena handles.
No operator/name/type validation, execution or canonical serialization is inferred from this stage.

Eight public contracts/24 independent reference shapes/nested100 literal rows/25 worked expressions,
three privacy-lifetime docs and seventeen actual compiled assertion reds pass with exact restoration.
Flat conversion/clone/drop passes on64KiB stack. Strict native513, release eight, WASM3, book/reference
checks pass. README/book/API map/decisions/live/task docs align; prior histories remain exact. G1 stays
5/18; next .5a.3c.4 coupled normalization review before canonical serialization.

## STITCHCAD-G1-0059 - exact typed individual literals (leaf `G1-SLICE.5a.3c.2`)

Parsed literal views now explicitly convert into private typed128-bit canonical inputs. Exact bounded
reduction precedes the rational-width check, then shared input rounding and the scalar length guard.
Wide reducible raw mantissas stay valid; huge zero padding stays accepted. Original source/unit/span,
count-versus-ratio and raw angle turns survive; unary minus remains its own syntax node. Structured
width errors report an honest>=129-bit lower bound. No whole-arena, binding or execution claim yet.

Five public contracts/100 independent Fraction rows/two privacy-lifetime docs and thirteen compiled
actual mutation reds pass, with exact source restoration. Strict native501, release five, WASM3 and
focused reference/book checks pass. README/current availability, grammar/language, indexed expert
annex/status map and live/task records align; prior histories remain exact. G1 stays5/18; next .3c.3
immutable normalized expression arena. D70 axes and later binding/geometry/API/MCP remain separately owned.

## STITCHCAD-G1-0058 - full-width unsigned rounding (leaf `G1-SLICE.5a.3c.1`)

A new sc-units unsigned128 API shares the signed half-away magnitude rule, preserving wide positive
literal children before later signed binding. Subtraction replaces doubled remainder so every
nonzero u128 ratio rounds without narrowing, overflow or wrap; zero names the public operation.
Signed i64 endpoints, signs and overflow remain unchanged. No formula conversion/execution yet.

Five public contracts/138 independent Decimal rows, nine actual debug mutation reds and one release
red pass with exact source restoration. Existing signed contracts/36 Fraction rows/five reds remain
required. Strict Rust494 (units46), release public tests, three WASM crates and focused book/reference
checks pass. Indexed expert annex and unit API/example align with task/live records; prior evidence
and oldest ledger payloads remain exact. G1 stays5/18; next .5a.3c.2 exact typed literal conversion.
