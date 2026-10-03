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

| [`stitchcad-changelog-part70.md`](docs/history/window4.md#stitchcad-changelog-part70md) | STITCHCAD-G1-0056 | 12 lines, 951 bytes, `sha256:30ca94ac…` |

| [`changelog-part71.md`](docs/history/window4.md#stitchcad-changelog-part71md) | STITCHCAD-G1-0057 | 11 lines, 887 bytes, `sha256:e144c5ec…` |

| [`changelog-part72.md`](docs/history/window4.md#stitchcad-changelog-part72md) | STITCHCAD-G1-0058 | 12 lines, 970 bytes, `sha256:ef59b0a1…` |

| [`changelog-part73.md`](docs/history/window4.md#stitchcad-changelog-part73md) | STITCHCAD-G1-0059 | 13 lines, 1095 bytes, `sha256:eea1fea0…` |

| [`changelog-part74.md`](docs/history/window4.md#stitchcad-changelog-part74md) | STITCHCAD-G1-0060 | 13 lines, 1090 bytes, `sha256:ab4422c6…` |

| [`changelog-part75.md`](docs/history/window4.md#stitchcad-changelog-part75md) | STITCHCAD-G1-0061 | 13 lines, 1091 bytes, `sha256:c4bbd66c…` |

| [`changelog-part76.md`](docs/history/window4.md#stitchcad-changelog-part76md) | STITCHCAD-G1-0062 | 13 lines, 1068 bytes, `sha256:a0da7975…` |

| [`changelog-part77.md`](docs/history/window4.md#stitchcad-changelog-part77md) | STITCHCAD-G1-0063 | 14 lines, 1197 bytes, `sha256:3349f7a6…` |

## STITCHCAD-G1-0099 - checkout-local CI stores (leaf `G1-SLICE.5b.3c.2b.h1.r2`)

Rust CI prepares Cargo/Rustup/target/scratch stores from checkout root before public stable Rustup
installation with --no-self-update, then verifies effective paths/components/devices before builds.
19 runtime controls/six actual compiled guard reds/workflow order pass0; required GitHub environment
transport is documented, with foreign Git/symlink refusals and no shared cache deletion.
Full checks/probe/publication/ledger receipts recorded in leaf before exceptional push; D142/D143
remain open until repaired-head CI .v. Complete prior task/ledger/lesson payloads retained; book/
live/map/tool/task/resume agree; product grammar unchanged, G1 stays5/18,14open/128sealed.

## STITCHCAD-G1-0098 - exact Rust1.99 lint repair (leaf `G1-SLICE.5b.3c.2b.h1.r1`)

Project-local Rust1.99 reproduces the actual CI double_must_use failure101. Remove only the
redundant declarations() annotation; iterator signature/body, lexical order and immutable sources
stay exact, with no lint waiver/toolchain pin/grammar change. Exact1.99 strict663tests/56groups
pass0; public namespace/read/order contracts included. Runner proof remains .h1.v after D143
local-store workflow; D142 stays open pending that verdict. Oldest complete ledger/lesson retained,
book/live/task/resume align; G1 stays5/18 and14open/128sealed, no repaired remote verdict claimed.

## STITCHCAD-G1-0097 - observed archive CI verdicts (leaf `G1-SLICE.5b.3c.2b.h1`)

Clean9fab7ec pushed after read-only remote/authority proof resolved automatic review. Exact-head
jobs/steps observed: doctrine succeeded; Rust1.99 Clippy failed redundant iterator must_use and
skipped tests/WASM. Local1.95/1.98 success is not a runner verdict. D142 lint/D143 CI locality
repairs logged and scheduled immediately, then .v observes repaired CI before D140.
Post-commit CLI266/252 includes newest immutable catalog refusal; oldest complete G1-0078 payload
retained. Book/live/task/resume scope agrees; G1 stays5/18,14open/128sealed, no Rust CI success claimed.

## STITCHCAD-G1-0096 - fourth exact retained window (leaf `G1-SLICE.5b.3c.2b.h0`)

Window4 retains60 whole source files from2bdcd31;249 logical records independently reconstruct
exactly through read/materialize/Git, with deterministic payload and60 redirected links. Working
history drops63→4 before new seals; existing windows/reader/schemas/limits unchanged. D141 legacy
window3 heading is superseded in the live book and independent label controls; no archive rewrite.
Full local checks/27 probe suites/gate pass0; publication/ledger receipts are in the leaf.
Exceptional push/observed CI .h1 follows; no remote verdict claimed.
G1 stays5/18; exact oldest ledger/lesson/original D141 report retained, product D140 repair next.

## STITCHCAD-G1-0095 - complete expression dimension arguments (leaf `G1-SLICE.5b.3c.2b.1`)

D138 reference refusals retain all actual kinds/roles and complete wanted signatures. Known children
resolve left-to-right before dimension checks; callee priority and grammar remain unchanged.
4023 cases/3814 complete refusals/15 actual compiled reds pass; old4032/14 and call166/12 controls,
full reference/language16/publication10/focused product15/ledger9+13/coverage/retention pass0.
Older structural probes load real catalogs; no guessed fallback. Canonical/book/decision/live scope
align; exact prior/oldest records retained. G1 stays5/18,12open/127sealed; Rust bytes unchanged.
P0 D140 geometry reference kind loss is next; D139 header payloads have a separate owner.

## STITCHCAD-G1-0094 - source-bearing call refusals (leaf `G1-SLICE.5b.3c.2a`)

Exact callee lookup distinguishes envelope/catalog sources from data origins and retains actual
requests/alternatives before argument access. D136/D137 reference payloads repaired; grammar unchanged.
Five public contracts/two negative examples/18 compiled body reds and166 reference cases/12 compiled/
three normative-set reds pass with exact restoration. Strict663/56 groups/WASM3/reference/language16/
publication10/ledger9+13/coverage/retention pass0. Decision/book/API/live scope align; no expression or
execution proof claimed. Exact prior/oldest records retained; G1 stays5/18,11open/126sealed.
Next D138 dimension payloads .5b.3c.2b.1, then bounded accepted expression checking.

## STITCHCAD-G1-0093 - typed wanted-kind catalogs (leaf `G1-SLICE.5b.3c.1`)

Immutable static rows expose exact/generic/class operand requirements and positional results.
Ten public contracts/681358 admission cases/exact descriptors and19 actual compiled body reds
verify closed alternatives; a branch-position fault is caught despite unchanged kind acceptance.
Strict656/55 groups/WASM3/reference/language16/book10/ledger9+13/coverage/retention pass0.
Bounded book/API/live scope and exact prior/oldest records align; G1 stays5/18,11open/124sealed.
D136 missing call arguments owned for .5b.3c.2a repair next; grammar/prior query code unchanged.

## STITCHCAD-G1-0092 - built-in and selector kind signatures (leaf `G1-SLICE.5b.3b`)

Closed22-name metadata preserves ordered arities, arithmetic T, conditional branches and symbolic
within classes; selectors create no geometry. Four public contracts/680702 tuples/24 actual rows/
22 unchanged canonical names and21 actual compiled body reds verify closure and restore source.
Strict651 tests/55 groups/WASM3/reference/language16/book10/ledger9+13 pass0;14 operator faults
rerun green. Bounded book/API/live scope and exact prior/oldest retention align; G1 stays5/18,
10open/124sealed. Next .5b.3c typed contextual checking, then .4 atomic graph acceptance.

## STITCHCAD-G1-0091 - reference product guidance and review status (leaf `G1-SLICE.5b.3a.1`)

D134 confines arc_length advice to refused angle×length products; the independent matrix checks
presence and absence, catching actual quotient-regression and missing-product faults. D135 fixes
review status to reflect implemented metadata and precise pending expression/graph proof.
Static4032 cases/14 actual reds, structure/language16/publication10/ledger9+13/coverage/retention
pass0; grammar unchanged. Exact prior/oldest records retained; G1 stays5/18,10open/124sealed.
Next .5b.3b built-in/selector signatures.

## STITCHCAD-G1-0090 - closed formula operator kind signatures (leaf `G1-SLICE.5b.3a`)

Pure unary/binary metadata preserves every arithmetic kind rule, directed quotient, commutative
product and exact angle×length hint. Four contracts check656 kind cases and actual normative rows;
7 unary/71 binary cases accepted,17 products/14 quotients/2 hints, all12 canonical symbols match
unchanged serialization. Fourteen actual compiled body reds restore source exactly. Numeric values,
contextual errors and whole expression/recipe acceptance remain later owners. Book/API/live/task
scope align; exact prior/oldest records retained. Strict native647/54 groups/WASM three libraries,
reference/language16/publication10/ledger9+13 pass0;57 chapters/40 APIs/1161 source/1811 render links.
G1 stays5/18;10open/122sealed; built-in/selector signatures .5b.3b next.

## STITCHCAD-G1-0089 - ordered formula name scopes (leaf `G1-SLICE.5b.2d.2`)

Actual recipe cursor exposes initial/prior metadata in authored order; current/future names and
assertion labels never bind early. Header refusal leaves position/prefix unchanged; both actual
let indices/spans and initial/reserved source identities remain available without invented context.
Eight public contracts/five negative examples/18 actual body reds verify ordering/source/privacy
and4096-let/assertion boundaries on64KiB stack. Metadata staging grants no expression/graph/value
acceptance. Book annex/API/live scope align; exact prior evidence/oldest records retained.
Strict native643/53 groups/WASM three libraries/reference/language16/publication10/ledger9+13
pass0;56 chapters/39 APIs/1156 source/1798 rendered links. G1 stays5/18,10open/122sealed; .5b.3 next.

## STITCHCAD-G1-0088 - exact initial formula name reads (leaf `G1-SLICE.5b.2d.1`)

Validated queries resolve exact existing declarations, preserving original source lifetimes and
canonical record/geometry identities. Opaque absent-name errors retain exact query and nine searched
origins; no alias, value read, fallback or fictional recipe context. Seven public read contracts/nine
actual compiled body reds restore source exactly; all17 prior collision faults still fail assertions.
Book examples/API/status/live scope align; exact prior namespace evidence moved to bounded sibling,
oldest ledger/lesson sealed unchanged. Strict native630/52 groups, WASM three libraries and
reference/language16/publication10/ledger9+13 pass0;55 chapters/37 APIs/1143 source/1777 render links.
G1 stays5/18;10open/122sealed defects; ordered binding scope .5b.2d.2 next.

## STITCHCAD-G1-0087 - checked initial formula namespace (leaf `G1-SLICE.5b.2c.2`)

Typed initial sources exclude recipe/reserved injection; immutable namespace seeds eight reserved
metadata entries without context values and refuses the first authored collision before insertion.
Errors retain earlier/reserved and attempted sources, including equal origins, without invented
recipe indices; opaque Debug/token-only Display preserve payload privacy. Collision pair boxed
once on failure to keep error compact; canonical records stay borrowed, no cached values/state.
Ten public contracts/17 actual compiled body assertion reds verify admission/context/source/order/
privacy and exact restoration. Draft API-name/unwrap-only/large-error controls corrected, no waiver.
Book examples/status/API/live scope and exact prior task/ledger/lesson retention align.
Strict native621 tests/51 result groups, WASM three libraries, reference/language16/publication10/
ledger9+13 controls pass0. D132/D133 fixed: actual unclosed-tag warnings now refuse publication,
repaired generic renders exactly. Book55 chapters/36 APIs/1143 source/1775 rendered links.
G1 stays5/18;10open/122sealed defects. Next .5b.2d checked reads/prior bindings, then type/graph.

## STITCHCAD-G1-0086 - source-aware binding refusal contract (leaf `G1-SLICE.5b.2c.1b`)

Delegated D131 decision retains formula_rebinding, discriminating reserved attempts from repeated
recipe lets. Sources carry fixed reserved metadata and actual initial pair/recipe locations;
detached checks invent no ordinal, whole preflight preserves both real indices/global spans.
3624 independently authored argument cases/19 actual compiled assertion reds pass0, source exact;
state/value/availability/execution trapped. Canonical contract/ADR/book disclose source/approval
scope and reversal; grammar and product Rust unchanged. Prior task/oldest ledger/lesson retained.
Structural/language16/publication9/ledger9/13 pointer controls pass0;55 chapters/33 APIs/
1140 source/1770 rendered links. G1 stays5/18;10open/120sealed defects; namespace .2c.2 next.

## STITCHCAD-G1-0085 - reserved-name diagnostic proposal (leaf `G1-SLICE.5b.2c.1a`)

D131 reproduces120 reserved-name refusals and one ordinary rebinding against the actual reference.
Three compiled assertion reds detect token changes/invented index; producer bytes remain exact.
Canonical rebinding row requires two recipe indices that reserved metadata/initial inputs lack.
ADR/book record a concrete source-aware reserved-case proposal; director ruling precedes repair
and product namespace. Grammar and product Rust remain unchanged; D131 stays open and owned.
Exact predecessor task/oldest ledger/lesson records retained within unchanged ceilings.
Structural/language16/publication9/ledger9/13 pointer controls pass0; book55 chapters/33 APIs/
1138 source/1767 rendered links. G1 remains5/18;11open/119sealed defects. Next .2c.1b ruling/repair.

## STITCHCAD-G1-0084 - immutable sourced formula declarations (leaf `G1-SLICE.5b.2b`)

Declarations preserve distinct metadata/value IDs, canonical length borrows, point/edge refs and
actual normalized let ordinals/names/spans/annotations without value reads or binding authority.
Generic scalar domains are three; measurement/Ease stay forced-length canonical adapters (D130).
Seven public/five negative construction/private/lifetime contracts,19 actual compiled body reds/
one widened API-negative red and strict native608/50groups pass. Reference/language16/publication9
verify55 chapters/33 APIs; source remains exact after faults. Book/live/task/README align; exact
prior payloads retained. G1 stays5/18;10open/119sealed; checked namespace .2c next.

## STITCHCAD-G1-0083 - closed product semantic metadata (leaf `G1-SLICE.5b.2a`)

Product FormulaKind/Origin/ReservedName/ReservedContext enumerate8 kinds/six binding kinds/nine
origins/eight names/four context classes independently of value/state/availability. Five public
contracts compare canonical rows in both directions; ten actual compiled assertion reds restore
exact source. Strict native596/49groups, WASM3, full reference/language16/publication9 pass0.
Metadata annex/examples/navigation/API map and live/task/resume records align; D129 stale review
status fixed, exact task/ledger/lesson/report payloads retained. G1 stays5/18;10open/118sealed defects.
Next .5b.2b sourced declarations; namespace/type/whole static acceptance and execution remain pending.

## STITCHCAD-G1-0082 - approved current-grammar recognition (leaf `G1-SLICE.5b.1c.2`)

Director's D124 ruling preserves v1 grammar and its three keywords. Exclusion diagnostics now
follow actual recognition: unknown calls unbound-name, malformed definitions parse, recognized
non-square exponents unsupported. Five ordinary identifiers remain valid scalar names/let headers.
100 metadata-only cases/seven actual source guard reds/three loaded documentation reds and full
reference/language16/publication9 pass. Complete static reference review .5b.1 closes; product
namespace/type/graph remains .5b.2–.4. Book/ADR/live/task/frontier align, original task and oldest
ledger/lesson/D124 report payloads retained. G1 stays5/18;10open/117sealed defects; .5b.2 next.

## STITCHCAD-G1-0081 - reference contribution provenance (leaf `G1-SLICE.5e.3b`)

The reference retains approximation-call sources through executed operators/conditions, numeric
and Boolean bindings/reads, actual book publication and precise lazy geometry/cache coordinates.
Named T1 assertion/within comparisons now raise formula_domain under the director's ruling;
T2-or-looser keeps its threshold and exact/untaken/independent-coordinate T1 controls still pass.
425 independent controls/26 actual compiled body assertion reds and actual copied-book bound-value
refusal pass; full reference/language16/publication/ledger/gate checks recorded in the leaf.
Runtime/diagnostic/ADR/live/task records agree; oldest complete ledger/lesson/D121 report and task
receipts retain predecessor payloads. G1 stays5/18; product evaluation/real geometry/release pending.

## STITCHCAD-G1-0080h - handoff observation and ownership (leaf `G1-SLICE.5e.1a.h`)

Post-commit handoff first refuses malformed lsof name evidence2. Captured fresh actual census
completes0 with0blocking/11advisory; canonical retry0. First failed record was not retained, so exact
cause remains unconfirmed and P1 capture/reproduction is owned by SPINE.23r on recurrence.
Current product frontier stays D121 .5e.3b; book/live/task/resume records agree. No source guard
relaxed, no verified defect classification added; G1 remains5/18 and defects12open/115sealed.



| [`stitchcad-changelog-part78.md`](docs/history/window4.md#stitchcad-changelog-part78md) | G1-0064 identity review | 13 lines, 1077 bytes, `sha256:c658f537…` |

| [`stitchcad-changelog-part79.md`](docs/history/window4.md#stitchcad-changelog-part79md) | STITCHCAD-G1-0065 | 14 lines, 1183 bytes, `sha256:aa1df64f…` |

| [`stitchcad-changelog-part80.md`](docs/history/window4.md#stitchcad-changelog-part80md) | G1-0066 ordered recipe syntax | 15 lines, 1243 bytes, `sha256:ffd1e428…` |

| [`stitchcad-changelog-part81.md`](docs/history/window4.md#stitchcad-changelog-part81md) | STITCHCAD-G1-0067 | 11 lines, 974 bytes, `sha256:aad9b836…` |

| [`stitchcad-changelog-part82.md`](docs/history/window4.md#stitchcad-changelog-part82md) | STITCHCAD-G1-0068 | 12 lines, 1035 bytes, `sha256:451e0450…` |

| [`stitchcad-changelog-part83.md`](docs/history/window4.md#stitchcad-changelog-part83md) | STITCHCAD-G1-0069 | 12 lines, 1076 bytes, `sha256:61590cec…` |

| [`stitchcad-changelog-part84.md`](docs/history/window4.md#stitchcad-changelog-part84md) | STITCHCAD-G1-0070 | 13 lines, 1176 bytes, `sha256:ce5bfa8a…` |

| [`stitchcad-changelog-part85.md`](docs/history/window4.md#stitchcad-changelog-part85md) | STITCHCAD-G1-0071 | 14 lines, 1261 bytes, `sha256:a370b662…` |

| [`stitchcad-changelog-part86.md`](docs/history/window4.md#stitchcad-changelog-part86md) | STITCHCAD-G1-0072 | 9 lines, 795 bytes, `sha256:480c7c1b…` |

| [`stitchcad-changelog-part87.md`](docs/history/window4.md#stitchcad-changelog-part87md) | STITCHCAD-SPINE-0021b | 9 lines, 773 bytes, `sha256:df9a2d7c…` |

| [`stitchcad-changelog-part88.md`](docs/history/window4.md#stitchcad-changelog-part88md) | STITCHCAD-G1-0073 | 10 lines, 878 bytes, `sha256:32b87b67…` |

| [`stitchcad-changelog-part89.md`](docs/history/window4.md#stitchcad-changelog-part89md) | STITCHCAD-G1-0075/0074 | 16 lines, 1250 bytes, `sha256:2d8487c5…` |

| [`stitchcad-changelog-part90.md`](docs/history/window4.md#stitchcad-changelog-part90md) | STITCHCAD-SPINE-0023 | 8 lines, 648 bytes, `sha256:fa18d0d1…` |

| [`stitchcad-changelog-part91.md`](docs/history/window4.md#stitchcad-changelog-part91md) | STITCHCAD-SPINE-0023v | 5 lines, 339 bytes, `sha256:fad82add…` |

| [`stitchcad-changelog-part92.md`](docs/history/window4.md#stitchcad-changelog-part92md) | STITCHCAD-G1-0076 | 8 lines, 648 bytes, `sha256:9138f15a…` |

| [`stitchcad-changelog-part93.md`](docs/history/stitchcad-changelog-part93.md) | STITCHCAD-G1-0077 | 11 lines, 976 bytes, `sha256:556627ca…` |

| [`stitchcad-changelog-part94.md`](docs/history/stitchcad-changelog-part94.md) | STITCHCAD-G1-0078 | 10 lines, 887 bytes, `sha256:f70a9dba…` |

| [`stitchcad-changelog-part95.md`](docs/history/stitchcad-changelog-part95.md) | STITCHCAD-G1-0079 | 10 lines, 859 bytes, `sha256:94ba0cb6…` |

| [`stitchcad-changelog-part96.md`](docs/history/stitchcad-changelog-part96.md) | STITCHCAD-G1-0080 | 11 lines, 976 bytes, `sha256:8d91c335…` |
