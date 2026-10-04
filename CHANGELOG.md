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

| [`stitchcad-changelog-part105.md`](docs/history/window5.md#stitchcad-changelog-part105md) | `STITCHCAD-G1-0090` | 10 lines, 853 bytes, `sha256:723f9852…` |

| [`stitchcad-changelog-part106.md`](docs/history/window5.md#stitchcad-changelog-part106md) | `STITCHCAD-G1-0091` | 8 lines, 613 bytes, `sha256:407ab4aa…` |

| [`stitchcad-changelog-part107.md`](docs/history/window5.md#stitchcad-changelog-part107md) | `STITCHCAD-G1-0092` | 8 lines, 659 bytes, `sha256:63eeecdf…` |

| [`stitchcad-changelog-part108.md`](docs/history/window5.md#stitchcad-changelog-part108md) | G1-0093 wanted signature ledger | 8 lines, 647 bytes, `sha256:88395c24…` |
| [`stitchcad-changelog-part109.md`](docs/history/window5.md#stitchcad-changelog-part109md) | G1-0094 call lookup ledger | 9 lines, 766 bytes, `sha256:801531e8…` |
| [`stitchcad-changelog-part110.md`](docs/history/window5.md#stitchcad-changelog-part110md) | G1-0096/G1-0095 ledger | 19 lines, 1502 bytes, `sha256:8a882c4f…` |
| [`stitchcad-changelog-part111.md`](docs/history/stitchcad-changelog-part111.md) | G1-0099/0098/0097/0080h | 35 lines, 2644 bytes, `sha256:8cc099ce…` |

## STITCHCAD-G1-0117 - guarded primary producer stores (leaf `G1-SLICE.5b.4c.h2.a`)

- Shared local profile/defaults/overrides/refusals; outer Make launcher prepares before platform
  dispatch, fresh driver/G0 entry guards, coupled Make temp, explicit local channel setup/no self-update.
- Actual56 runtime/13 compiled/2 Make/1 launcher reds, exec7/readonly inspect/18 review children;
  absent-export stable1.99/native703/59groups/WASM3/full28 suites pass0. Command declarations repair
  measured glossary refusal; book62/publication10/glossary17 pass0. Exact complete lessons sealed125.
- D156 direct-producer audit remains open; required exact-head CI .h2.v then .b precede D154.

## STITCHCAD-G1-0116 - observed window5 CI (leaf `G1-SLICE.5b.4c.h1`)

Exact pushed d1198ab doctrine8/Rust11 steps all completed/success. Actual Rust1.99 strict703tests/
59groups/WASM3/four checkout-local stores verified; no aggregate inference. Post-commit archive
CLI328 controls/313 reads includes newest committed-catalog refusal. Book/task/live scope agrees;
G1 stays5/18,12siblings,12open/143sealed. D156 producer defaults then D154 known ordinals next.


## STITCHCAD-G1-0115 - D154 blocking retention (leaf `G1-SLICE.5b.4c.h0`)

Window5 retains60 exact c91cdf5 files; isolated source/read/materialization proof reconstructs309
prior logical records. Only verified raw copies retire;69 maintained destinations now use catalog
headings. Complete task evidence moves into the existing sibling, and whole oldest ledger/closed
receipts are sealed. D155 seven undeclared fields/example names repaired with17 glossary controls/
seven actual named reds; G0 again18met/1human-act-unmet. Fixed limits/four earlier windows stay
immutable; D154 stays open. Full28 suites/strict703/WASM3/final gates pass0; runner observation
follows. G1 remains5/18,12open/143sealed,12siblings; D156 defaults owned before D154.


## STITCHCAD-G1-0114 - coupled whole static review (leaf `G1-SLICE.5b.4b`)

26723 shared cases exercise13967 actual whole factories, complete source-derived graphs and62
refusal contexts;6reference/28Rust/3text body reds restore sources/artifact. D154 reference ordinals follow
before .5b closure/arithmetic. Crates unchanged; strict703/59groups, reference/language16/publication10
(61chapters/67API/1231source/1933rendered links), ledger9+13/census/retention and13 staged gates pass. Full evidence: docs/tasks/G1-SLICE-checked-recipes.md. Grammar unchanged; G1 stays5/18.

## STITCHCAD-G1-0113 - immutable whole recipe proofs (leaf `G1-SLICE.5b.4a`)

The actual normalized owner and checked initial namespace now produce a complete immutable static
proof only after every statement passes before metadata advance. Typed first refusals retain actual
owner/ordinal/spans; dependencies stream original class/operand roles, repetitions and real sources.
D152 stale book status and D153 single-line map-comment data loss repaired; grammar unchanged.
Seven public contracts (96binding/640assertion cases/max64KiB stack), five precise compiler guards,
four executable book examples and19 whole/22statement/18scope/16expression compiled body reds pass.
Strict703tests/59groups/WASM3/full reference and13149-case coupled review/language16/publication10
(61chapters/67API/1230source/1932rendered links), ledger9+13/coverage/archive/gates pass0.
Two complete records retained byte-exact;309logical/64working MD under fixed limits. G1 stays5/18,
12 evidence siblings,10open/142sealed. Coupled whole factory review .5b.4b follows before .5b closure.

## STITCHCAD-G1-0112 - complete syntax before literal inputs (leaf `G1-SLICE.5b.3d.d`)

D150 whole reference syntax/structure now completes before all literal conversions, then static
checking; raw spelling/units, original spans and normalized owners survive. Detached defaults,
grammar, bounds and closed tokens stay fixed. D151 status distinguishes reference/product proofs.
263 actual public/reference phase/ordinal cases, eight detached controls and10 compiled body reds;
prior584/11, canonical12 and13149 shared/6reference/9Rust/3text controls pass, rc=0. Full reference,
language16/publication10/ledger9+13 pass0; exact archive307/62working records. Coupled .3 closes;
whole immutable graph .4 next. G1 stays5/18; independent10open/140sealed; no native source change.

## STITCHCAD-G1-0111 - shared static contract and phase counterexamples (leaf `G1-SLICE.5b.3d.c`)

Independent13149 shared cases compare actual public Rust and reference kinds, complete dimension
arguments, call/child priority, ordered dependency sources, borrowed owners and worked21/refusal13
populations. Six reference/nine Rust actual compiled body faults restore all sources/artifact.
D149's two stale prerequisite clauses are fixed with actual copied-text assertion controls.
Strict native690/58,WASM3,full reference/publication10 pass, rc=0. Original protocol/lesson/old
navigation/ledger/closure/report retained exactly; bounds/caps/grammar unchanged.
Competing early over-width literal versus later syntax exposes D150: reference domain vs public
syntax refusal. Reproducer is deliberately failing and not yet in the standing runner; owned .3d.d
immediately before final coupled closure and .4. G1 stays5/18;11open/138sealed; final signoff open.

## STITCHCAD-G1-0110 - whole-source input before static checking (leaf `G1-SLICE.5b.3d.b`)

Reference preflight completes syntax/input for every identified statement before ordered static
inference or prior-binding publication. It reuses the actual parsed operand tuples with original
locations; detached statement behavior, grammar and limits remain unchanged. No prefix escapes.
Independent584 controls/11 compiled body fault assertions verify later input versus earlier static
errors, phase traces, parsed identity, one parse per statement and source order. Full reference and
publication10 pass, rc=0; original HEAD protocol/lesson/ledger/report retained byte-exact.
G1 remains5/18;10open/137sealed. Coupled review .5b.3d precedes whole immutable graph .4.

## STITCHCAD-G1-0109 - truthful ambiguity arguments (leaf `G1-SLICE.5b.3d.a`)

Reference formula_ambiguous_name now retains the name, both origins in binding order and actual
prior/attempted metadata sources. Ordered initial pairs retain genuine declaration positions;
recipe attempts retain genuine spans/whole ordinals. Detached dictionaries invent no positions.
Grammar, stable token, metadata identity and numerical/runtime behavior unchanged.
Independent4192 exact payloads and16 actual compiled body fault assertions verify the repair;
full reference regression and publication10 pass, rc=0. Three original HEAD records and D147's
pre-repair report retained exactly. G1 stays5/18;11open/136sealed; D148 phase repair next.

## STITCHCAD-G1-0108 - actual scoped statement kind proofs (leaf `G1-SLICE.5b.3c.3b`)

Only current scope checks its original let/assert operands and genuine ordinal. Let RHS must match
its annotation; both assertion operands precede complete equality checking. Immutable proofs/errors
retain exact normalized owners and ordered sourced dependencies, surviving cursor advancement while
recipe/record borrows remain pinned. Typed header/comparison/nested refusals expose actual context;
no combined AST, separator span, runtime value or whole-recipe acceptance is invented.
Seven public contracts verify48let/320assert pairs, sources/priority/privacy/limits;22new/18scope/
16expression actual compiled body faults restore sources exactly. Strict1.99 native690/58groups,
WASM3, five precise Cargo-current privacy/borrow guards, full reference/publication10/60chapters/
62APIs/1211source/1902rendered links and ledger9+13 pass0. Four original HEAD records retained
byte-exact; archive286 verified. G1 stays5/18;10open/135sealed exceptD18. Coupled .3d then .4 next.

## STITCHCAD-G1-0107 - invalid assertion class is syntax (leaf `G1-SLICE.5b.3c.3a.t1`)

D146 UnknownTolerance now uses formula_parse with its original rule/span and genuine recipe
ordinal; the five accepted classes and grammar are unchanged. Missing valid runtime context keeps
formula_tolerance_unbound. Actual prior mapping reproduces body red101;144invalid/10valid public
syntax cases and15actual compiled statement body faults pass0, source exact. Strict1.99 native677/
57groups/WASM3/full reference/publication10/ledger9+13 pass0. Book examples and live/task/resume
align; three original records retained byte-exact, archive282 verified. Scoped history confirms the
reference repair missed product mapping/fixtures. G1 stays5/18;10open/135sealed exceptD18.
Next .5b.3c.3b actual scope-bound statement checking, then full .4 graph.

## STITCHCAD-G1-0106 - truthful binding-header diagnostics (leaf `G1-SLICE.5b.3c.3a`)

D139 raw annotations carry original spelling/span and six wanted kinds; valid mismatch carries
actual declared/expression kinds. Genuine whole offsets/ordinals retained, no absent RHS/canonical
identity invented. Contract documented before reference repair; grammar/tokens unchanged.
Header264cases/232exact payloads/13actual compiled body reds, full structural/reference/language16,
publication10/60chapters/57APIs/1204source/1894rendered links and ledger9+13 pass0. Original report/
oldest ledger/lesson/completed checker protocol retained byte-exact; archive279 verified.
G1 stays5/18; independent11open/134sealed exceptD18. D146 public unknown-class token owned next.

## STITCHCAD-G1-0105 - bounded expression kinds and dependencies (leaf `G1-SLICE.5b.3c.2b.2`)

Initial-scope check_kinds certifies the exact normalized expression, complete child kinds and
ordered sourced dependencies, including untaken branches. Refusals retain actual spans/owner,
existing name/call arguments and typed complete dimensions/wanted rows/arc_length guidance.
No numeric/state/provider query or invented ordinal; grammar unchanged. Public contracts check
656 operator/52156 call tuples plus arity4/priority/source/limits/privacy, with16 actual compiled
body fault controls. Strict native/WASM, structural/book/ledger/gate receipts retained in task.
Whole prior ledger/lesson retained; G1 stays5/18,11open/133sealed; D139 headers/statement checks next.

## STITCHCAD-G1-0104 - current prerequisite documentation (leaf `G1-SLICE.5b.3c.2b.2.0`)

Wanted-rule annex now reflects verified D140 reference geometry argument checking, linking its
proof and retaining product operation/graph/geometry boundaries. Source census/geometry97/75/12,
publication10 and ledger/gate receipts verify D145 closure; grammar/API unchanged.
Original report/whole ledger/lesson retained; G1 stays5/18,11open/133sealed; checker follows.

## STITCHCAD-G1-0103 - truthful reference source locations (leaf `G1-SLICE.5f.3a.t1`)

Preserve the shell-header offset for original compiled reference code; mark changed in-memory
fault sources as virtual.86 independent original positions/real traceback text/variant frames and
2 actual compiled helper faults pass0, with unchanged source bytes and formula behavior/grammar.
Full reference/language/publication/ledger/gate receipts in owner; original D144/ledger/lesson and
complete prior CI/cleanup protocols retained exactly. G1 stays5/18,11open/132sealed; checker next.

## STITCHCAD-G1-0102 - geometry arguments before values (leaf `G1-SLICE.5f.3a`)

Reference point x/y and edge len parse/infer completely before numerical work; every kind must be
Length. Truthful scoped payloads and no static-failure cache/result/source mutation are verified by
97 cases/75 complete refusals/12 actual compiled reds; exact values/per-coordinate sources replay.
All26 existing provenance reds retained; reference/language/publication/ledger/gate receipts in owner.
Original D140/ledger/lesson retained, book/live/task/resume agree; G1 stays5/18,12open/131sealed, including owned D144 source-location repair.
Product operation/geometry/full graph proof remains owned; grammar unchanged; expression checker next.

## STITCHCAD-G1-0101 - guarded daily cleanup (leaf `G1-SLICE.5b.3c.2b.h1.c`)

Frozen plan removed6 output trees/991 strays,15647 files/1819917894B; independent residue0/tracked equality.
Protected stores/other repositories untouched; native663/56/WASM3 rebuilt. Full probe/gate receipts
in owner; latest cleanup/book/live/task/resume agree. Original ledger/lesson retained exactly.
G1 stays5/18,12open/130sealed; P0 D140 resumes, no independent product approval inferred.

## STITCHCAD-G1-0100 - repaired CI observed (leaf `G1-SLICE.5b.3c.2b.h1.v`)

Exact d5dd11f jobs/steps succeeded: doctrine8, Rust11 including strictClippy1.99/native663/56/WASM3.
Actual effective stores/locality checks passed; D142/D143 close with original reports retained.
Full prior ledger/lesson retained exactly; book/live/task/resume agree, G1 stays5/18,12open/130sealed.
Focused publication/ledger/gate receipts in owner; cleanup then D140, no product approval inferred.

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
| [`stitchcad-changelog-part93.md`](docs/history/window5.md#stitchcad-changelog-part93md) | STITCHCAD-G1-0077 | 11 lines, 976 bytes, `sha256:556627ca…` |
| [`stitchcad-changelog-part94.md`](docs/history/window5.md#stitchcad-changelog-part94md) | STITCHCAD-G1-0078 | 10 lines, 887 bytes, `sha256:f70a9dba…` |
| [`stitchcad-changelog-part95.md`](docs/history/window5.md#stitchcad-changelog-part95md) | STITCHCAD-G1-0079 | 10 lines, 859 bytes, `sha256:94ba0cb6…` |
| [`stitchcad-changelog-part96.md`](docs/history/window5.md#stitchcad-changelog-part96md) | STITCHCAD-G1-0080 | 11 lines, 976 bytes, `sha256:8d91c335…` |
| [`stitchcad-changelog-part97.md`](docs/history/window5.md#stitchcad-changelog-part97md) | STITCHCAD-G1-0081 | 10 lines, 858 bytes, `sha256:14381657…` |
| [`stitchcad-changelog-part98.md`](docs/history/window5.md#stitchcad-changelog-part98md) | STITCHCAD-G1-0082 | 9 lines, 763 bytes, `sha256:cd886e4d…` |
| [`stitchcad-changelog-part99.md`](docs/history/window5.md#stitchcad-changelog-part99md) | STITCHCAD-G1-0083 | 9 lines, 767 bytes, `sha256:ce745517…` |
| [`stitchcad-changelog-part100.md`](docs/history/window5.md#stitchcad-changelog-part100md) | STITCHCAD-G1-0084 | 9 lines, 755 bytes, `sha256:ffb7e24e…` |
| [`stitchcad-changelog-part101.md`](docs/history/window5.md#stitchcad-changelog-part101md) | STITCHCAD-G1-0085 | 10 lines, 849 bytes, `sha256:838b6b56…` |
| [`stitchcad-changelog-part102.md`](docs/history/window5.md#stitchcad-changelog-part102md) | STITCHCAD-G1-0086 | 10 lines, 850 bytes, `sha256:ab8d7ec9…` |
| [`stitchcad-changelog-part103.md`](docs/history/window5.md#stitchcad-changelog-part103md) | STITCHCAD-G1-0087 | 14 lines, 1241 bytes, `sha256:af59826f…` |
| [`stitchcad-changelog-part104.md`](docs/history/window5.md#stitchcad-changelog-part104md) | STITCHCAD-G1-0089/0088 | 21 lines, 1708 bytes, `sha256:84e03c0f…` |
