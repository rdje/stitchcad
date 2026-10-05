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
| [`stitchcad-changelog-part111.md`](docs/history/window6.md#stitchcad-changelog-part111md) | G1-0099/0098/0097/0080h | 35 lines, 2644 bytes, `sha256:8cc099ce…` |

## STITCHCAD-G1-0175 - canonical recipe modes close D178 (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n07.4`)

- Eight inputs/22 native logs/one mutable; classifier-only zero effects;21 cases/full10 exact.
- Shared458 reds/35 classifiers/27 captures/157 refusals0; real21/frozen31/current full10 artifact0.
- Whole task/lesson/ledger/D178 retained;467 public reads/prior464/all62/seven windows exact.
- D178 all four repaired/replayed/closed; D156 .n08 next. Rust/grammar/400 cadence unchanged.

## STITCHCAD-G1-0174 - normalized recipe modes and thread evidence (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n07.3`)

- Seven inputs/18 native logs/three mutable; classifier-only zero effects;17 cases/bindings exact.
- Shared433 reds/35 classifiers/25 captures/139 refusals0; real17/frozen27/current full8 artifact0.
- Whole task/lesson/ledger retained;464 public reads/prior462/all60/seven windows exact.
- D178 three fixed/canonical P1 next; D156 open. Rust/grammar/director400 cadence stay.

## STITCHCAD-G1-0173 - normalized expression modes and thread evidence (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n07.2`)

- Five inputs/18 native logs/one mutable; classifier-only zero native effects;17 cases exact.
- Shared408 reds/35 classifiers/23 captures/123 refusals0; real17/frozen23/current full8 artifact0.
- Whole task/lesson/ledger retained;462 public reads/prior460/all58/seven windows exact.
- D178 literal/expression fixed/2 recipe P1 remain; D156 open. Rust/grammar/400 cadence stay.

## STITCHCAD-G1-0172 - literal modes and thread evidence (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n07.1`)

- Four inputs/14 native logs/one mutable; classifier-only zero native effects;13 original cases exact.
- Shared389 reds/35 classifiers/21 captures/111 refusals0; real13/frozen22/current full6 artifact0.
- Whole task/lesson/two ledgers retained;460 public reads/prior458/all56/seven windows exact.
- D178 literal fixed/3 P1 remain; D156 continues. Rust/grammar/400 cadence unchanged.

## STITCHCAD-G1-0171 - statement modes close D177 (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n06.2`)

- Five inputs/16 native logs/one mutable; classifier-only zero effects; original15/full10 exact.
- Shared372 reds/35 classifiers/19 captures/101 refusals0; actual15/frozen21/full10 artifact0.
- Whole recipe/lexer/chart/D177 records retained;458 public reads/prior455/all53 exact.
- D17715 entries/16 predicates closed; D156 continues. Rust/grammar/400 cadence unchanged.

## STITCHCAD-G1-0170 - guarded recipe modes and worker assertions (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n06.1`)

- Seven inputs/16 ordinary or6 coupled files/two mutables/classifier zero writes; original15+5 cases exact.351 body reds/34 classifiers/17 captures/89 refusals0. Two test count guards fix reproduced extraction-only failures.
- Twenty real macro faults/22 frozen inputs/full11 artifact/full fmt-Clippy-native703 gate0. Whole receipts/lesson/ledger retained; prior453/all51/public455 exact. Book/live route .n06.2/parent active/D17714 fixed1. Grammar/400 cadence unchanged.

## STITCHCAD-G1-0169 - semantic mode-specific files and current macros (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n05.2`)

- Guard three inputs; ordinary11 logs/one mutable, classifier-only zero files/mutable/native.
- Shared312 body reds/33 classifiers/fourteen captures/64 physical refusals; final receipts in leaf.
- Ten real macro faults/exact failed-test sets;13 inputs/six Rust files/full5 artifact exact0.
- Retain whole domain receipt/length lesson/two ledgers; prior451/all49/public453 exact.
- Parent lexer/semantic completes; D17713 fixed2 remain, .n06 next after checks.
- Book/live/focused proof in leaf; grammar/metadata/400 cadence unchanged.

## STITCHCAD-G1-0168 - guarded lexer files and current macro sites (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n05.1`)

- Plan10 logs/four inputs/two mutable sources; preserve9 cases and imported examples.
- Shared296 body reds/32 classifiers/twelve captures/56 physical refusals pass0.
- Nine real macro reds;12 frozen inputs/five Rust HEAD files/full13 public artifact exact0.
- Retain whole length receipt/oldest ledger; prior450/all48/public451 exact.
- Book/domain prose/live/focused proof in leaf; D17712 fixed3 remain, .n05.2 next.
- Rust/grammar/400 cadence unchanged; no early push.

## STITCHCAD-G1-0167 - guarded mixed domain-context files and panic sites (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n04.4`)

- Plan16 logs/six inputs/five mutable sources; preserve14 cases/two consumer restorations.
- Shared283 body reds/31 classifiers/eleven captures/50 physical refusals pass0.
- Fourteen real current macro reds;17 frozen inputs/ten Rust HEAD files/exact failed-test sets0.
- Current Cargo-selected artifacts pass all5 public unit/all3 focused private core tests0.
- Retain whole unsigned receipt/two lessons/two ledgers; prior448/all46/public450 exact.
- Numeric family complete; D177 eleven fixed/4 P1 remain; .n05 next; focused/staged receipts in leaf.
- Rust/API/grammar/400 cadence unchanged; private guards certify totality, not geometry.

## STITCHCAD-G1-0166 - guarded length-operator files and panic sites (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n04.3`)

- Plan7 logs/two inputs/one mutable source; preserve six faults/two selected/full4 public tests.
- Shared264 body reds/30 classifiers/ten captures/42 physical refusals pass0.
- Six real current macro reds; nine frozen inputs/three Rust HEAD files/full4 public artifact exact0.
- Retain whole signed-round receipt/two lessons/two ledgers; prior446/all44/public448 exact.
- Fix owned PDF-point prose: exact25400/72 ≈352.78µm, independently checked; production unchanged.
- Focused/staged receipts in owning leaf; D177 ten fixed/5 P1 remain; domain .n04.4 next.
- Rust/API/grammar/400 cadence unchanged; no early push or unobserved runner claim.

## STITCHCAD-G1-0165 - guarded unsigned-round files and both profiles (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n04.2`)

- Plan12 logs/three inputs/one mutable source; preserve original CALL-derived9 faults.
- Explicit totality assertion; shared256 body reds/29 classifiers/nine captures/38 refusals0.
- Nine real debug/one release macro reds; ten frozen inputs/both full5 public artifacts exact0.
- Retain whole Ease receipts/two lessons/oldest ledger; prior444/all42/public446 exact.
- Full Rust/focused/staged receipts in owning leaf; production/grammar/400 cadence unchanged.
- D177 nine fixed/6 P1 remain; length .n04.3 next.

## STITCHCAD-G1-0164 - guarded signed-round files and totality sites (leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n04.1`)

- Split four numeric owners; plan6 logs/three inputs/one mutable source before reads.
- Replace labelled expect evidence with an explicit totality assertion; five cases/four tests intact.
- Shared240 body reds/28 classifiers/eight captures/33 physical refusals0; five real macro reds.
- Ten frozen inputs/production exact/full4 public artifact0; full fmt/Clippy/703 Rust tests pass.
- Retain two whole records/prior442/all40/public444 exact; final checks in owning leaf.
- D177 eight fixed/7 P1 remain; production rounding/grammar/400 cadence unchanged.






















| [`part132`](docs/history/window7.md#stitchcad-changelog-part132md) | whole G1-0130 ledger | 8 lines, 682 bytes, `sha256:6e3506f0…` |

| [`part131`](docs/history/window7.md#stitchcad-changelog-part131md) | whole G1-0129 ledger | 7 lines, 586 bytes, `sha256:016b79cb…` |

| [`part130`](docs/history/window7.md#stitchcad-changelog-part130md) | G1-0128 lexer guards | 6 lines, 463 bytes, `sha256:21deb59d…` |

| [`part129`](docs/history/window7.md#stitchcad-changelog-part129md) | G1-0127 unit guards | 6 lines, 490 bytes, `sha256:ae6bb38b…` |

| [`part128`](docs/history/window7.md#stitchcad-changelog-part128md) | G1-0126 continuity | 6 lines, 482 bytes, `sha256:a25c438d…` |

| [`part127`](docs/history/window7.md#stitchcad-changelog-part127md) | G1-0125 chart guards | 6 lines, 488 bytes, `sha256:d7e282a6…` |

| [`part126`](docs/history/window7.md#stitchcad-changelog-part126md) | G1-0124 continuity | 5 lines, 394 bytes, `sha256:d1cbcbfa…` |

| [`part125`](docs/history/window7.md#stitchcad-changelog-part125md) | G1-0123 Ease guards | 6 lines, 479 bytes, `sha256:a51ba574…` |

| [`part124`](docs/history/window7.md#stitchcad-changelog-part124md) | G1-0122/0121 | 21 lines, 1470 bytes, `sha256:8fa452b9…` |

| [`part123`](docs/history/window7.md#stitchcad-changelog-part123md) | G1-0120 census | 6 lines, 473 bytes, `sha256:897e1536…` |

| [`part122`](docs/history/window7.md#stitchcad-changelog-part122md) | G1-0119/0118 | 12 lines, 834 bytes, `sha256:ad285cfc…` |

| [`part121`](docs/history/window7.md#stitchcad-changelog-part121md) | G1-0117 producer profile | 8 lines, 683 bytes, `sha256:03c915a1…` |

| [`part120`](docs/history/window7.md#stitchcad-changelog-part120md) | G1-0116 exact CI | 6 lines, 460 bytes, `sha256:31a25724…` |











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
| [`part112`](docs/history/window6.md#stitchcad-changelog-part112md) | G1-0104 through G1-0100 | 37 lines, 2713 bytes, `sha256:b9ab801e…` |

| [`part113`](docs/history/window6.md#stitchcad-changelog-part113md) | G1-0109 through G1-0105 | 53 lines, 4156 bytes, `sha256:feeb6e30…` |
| [`part114`](docs/history/window6.md#stitchcad-changelog-part114md) | G1-0110 whole-source preflight | 9 lines, 761 bytes, `sha256:dd7f339a…` |
| [`part115`](docs/history/window6.md#stitchcad-changelog-part115md) | G1-0111 static contract | 11 lines, 955 bytes, `sha256:fde1010c…` |
| [`part116`](docs/history/window6.md#stitchcad-changelog-part116md) | G1-0112 syntax and literal phases | 9 lines, 774 bytes, `sha256:3c01839a…` |
| [`part117`](docs/history/window6.md#stitchcad-changelog-part117md) | complete verification continuity | 12 lines, 1064 bytes, `sha256:86e927ea…` |
| [`part118`](docs/history/window6.md#stitchcad-changelog-part118md) | complete verification continuity | 6 lines, 573 bytes, `sha256:c225854a…` |
| [`part119`](docs/history/window6.md#stitchcad-changelog-part119md) | complete closed verification | 9 lines, 745 bytes, `sha256:833b325f…` |

| [`part133`](docs/history/window7.md#stitchcad-changelog-part133md) | G1-0131 ledger | 8 lines, 700 bytes, `sha256:534079f9…` |

| [`part134`](docs/history/window7.md#stitchcad-changelog-part134md) | G1-0132 ledger | 8 lines, 680 bytes, `sha256:6617db21…` |

| [`part135`](docs/history/window7.md#stitchcad-changelog-part135md) | whole G1-0134/0133 ledgers | 17 lines, 1365 bytes, `sha256:b27ceffd…` |

| [`part136`](docs/history/stitchcad-changelog-part136.md) | whole G1-0137/0136/0135 ledgers | 24 lines, 1819 bytes, `sha256:9b764edc…` |

| [`part137`](docs/history/stitchcad-changelog-part137.md) | three whole G1-0140/0139/0138 ledgers | 18 lines, 1236 bytes, `sha256:38ef85ae…` |

| [`part138`](docs/history/stitchcad-changelog-part138.md) | whole G1-0143/0142/0141 ledgers | 19 lines, 1383 bytes, `sha256:6e08f574…` |

| [`part139`](docs/history/stitchcad-changelog-part139.md) | whole G1-0144 ledger | 7 lines, 578 bytes, `sha256:0661ecd7…` |

| [`part140`](docs/history/stitchcad-changelog-part140.md) | whole G1-0145 cleanup ledger | 7 lines, 588 bytes, `sha256:8a23c596…` |

| [`part141`](docs/history/stitchcad-changelog-part141.md) | whole G1-0147 and G1-0146 ledgers | 14 lines, 1097 bytes, `sha256:8fa23459…` |

| [`part142`](docs/history/stitchcad-changelog-part142.md) | whole G1-0148 static guard ledger | 10 lines, 861 bytes, `sha256:790a719b…` |

| [`part143`](docs/history/stitchcad-changelog-part143.md) | whole G1-0149 continuity ledger | 6 lines, 530 bytes, `sha256:d5b8a94e…` |

| [`part144`](docs/history/stitchcad-changelog-part144.md) | whole G1-0151 and G1-0150 continuity ledgers | 13 lines, 983 bytes, `sha256:4c0a2f29…` |

| [`part145`](docs/history/stitchcad-changelog-part145.md) | whole G1-0153 and G1-0152 continuity ledgers | 14 lines, 1036 bytes, `sha256:07a2dcad…` |

| [`part146`](docs/history/stitchcad-changelog-part146.md) | whole G1-0154 continuity ledger | 7 lines, 603 bytes, `sha256:136114b1…` |

| [`part147`](docs/history/stitchcad-changelog-part147.md) | whole G1-0156 and G1-0155 continuity ledgers | 11 lines, 719 bytes, `sha256:45304271…` |

| [`part148`](docs/history/stitchcad-changelog-part148.md) | whole G1-0157 continuity ledger | 6 lines, 474 bytes, `sha256:934bd04d…` |

| [`part149`](docs/history/stitchcad-changelog-part149.md) | whole G1-0158 chart continuity ledger | 8 lines, 638 bytes, `sha256:d18ad1d9…` |

| [`part150`](docs/history/stitchcad-changelog-part150.md) | whole G1-0160 and G1-0159 continuity ledgers | 15 lines, 1158 bytes, `sha256:16a21ab1…` |

| [`part151`](docs/history/stitchcad-changelog-part151.md) | whole G1-0161 collection continuity ledgers | 7 lines, 570 bytes, `sha256:44242cdb…` |

| [`part152`](docs/history/stitchcad-changelog-part152.md) | whole G1-0162 individual Ease continuity ledgers | 7 lines, 571 bytes, `sha256:2d4ad014…` |

| [`part153`](docs/history/stitchcad-changelog-part153.md) | whole G1-0163 Ease-set continuity ledger | 7 lines, 557 bytes, `sha256:f9db2b5b…` |
