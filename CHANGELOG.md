# CHANGELOG.md

Newest first: one section per completed slice, in commit order. Older slices live in sealed, immutable
segments under `docs/history/`, each named below with its identity and retrieval path.

# Sealed archive — earlier slices

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [part1](docs/history/window1.md#stitchcad-changelog-part1md) | slices 1–15, `STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004b` | 365 lines, 30452 bytes, `sha256:f4aec75a…` |
| [part2](docs/history/window1.md#stitchcad-changelog-part2md) | slices 16–20, `STITCHCAD-SPINE-0014` … `STITCHCAD-G0-0002` | 152 lines, 12811 bytes, `sha256:5783ac36…` |
| [part3](docs/history/window1.md#stitchcad-changelog-part3md) | slices 21–24, `STITCHCAD-G0-0013` … `STITCHCAD-G0-0018` | 147 lines, 12289 bytes, `sha256:14ad5278…` |
| [part4](docs/history/window1.md#stitchcad-changelog-part4md) | slices 25–29, `STITCHCAD-G0-0004` … `STITCHCAD-SPINE-0017` | 177 lines, 15440 bytes, `sha256:a8cc1de6…` |
| [part5](docs/history/window1.md#stitchcad-changelog-part5md) | slices 30–31, `STITCHCAD-G0-0005` … `STITCHCAD-G0-0013c` | 82 lines, 7505 bytes, `sha256:18548ff7…` |
| [part6](docs/history/window1.md#stitchcad-changelog-part6md) | slices 32–33, `STITCHCAD-G0-0007` … `STITCHCAD-G0-0006` | 93 lines, 8284 bytes, `sha256:3148dd0f…` |
| [part7](docs/history/window1.md#stitchcad-changelog-part7md) | slices 34–35, `STITCHCAD-G0-0013d` … `STITCHCAD-G0-0008` | 105 lines, 9793 bytes, `sha256:ff62d418…` |
| [part8](docs/history/window1.md#stitchcad-changelog-part8md) | slices 41–42, `STITCHCAD-G0-0014` … `STITCHCAD-G0-0004b` | 106 lines, 9766 bytes, `sha256:2c7ee35a…` |
| [part9](docs/history/window1.md#stitchcad-changelog-part9md) | the two oldest live entries, `STITCHCAD-G0-0004c` and `STITCHCAD-SPINE-0004d` — no slice range, because the earlier ranges have no producer (D51) | 90 lines, 8386 bytes, `sha256:8e4081d4…` |
| [part10](docs/history/window1.md#stitchcad-changelog-part10md) | two spine slices on the table convention, `STITCHCAD-SPINE-0020` and `STITCHCAD-SPINE-0015` | 73 lines, 6652 bytes, `sha256:062ccfa3…` |
| [part11](docs/history/window1.md#stitchcad-changelog-part11md) | the delegation-and-uncertainty slice, `STITCHCAD-G0-0014c` | 46 lines, 4507 bytes, `sha256:de34382e…` |
| [part12](docs/history/window1.md#stitchcad-changelog-part12md) | the dialects and formula-language slices, `STITCHCAD-G0-0010` and `STITCHCAD-G0-0009` | 84 lines, 7767 bytes, `sha256:9c61ba7c…` |
| [part13](docs/history/window1.md#stitchcad-changelog-part13md) | the release-contract and canvas-spike-rule slices, `STITCHCAD-G0-0012` and `STITCHCAD-G0-0011` | 75 lines, 7188 bytes, `sha256:1e52b5c9…` |
| [part14](docs/history/window1.md#stitchcad-changelog-part14md) | the i18n slice, `STITCHCAD-G0-0016` | 38 lines, 3631 bytes, `sha256:2c895780…` |
| [part15](docs/history/window1.md#stitchcad-changelog-part15md) | the recurring cleanup slice, `STITCHCAD-SPINE-0021` | 23 lines, 2123 bytes, `sha256:f93154e3…` |
| [part16](docs/history/window1.md#stitchcad-changelog-part16md) | the closed-defect sealing slice, `STITCHCAD-SPINE-0019a` | 27 lines, 2450 bytes, `sha256:211f9bec…` |
| [part17](docs/history/window1.md#stitchcad-changelog-part17md) | the command-layer contract, `STITCHCAD-G0-0017` | 38 lines, 3626 bytes, `sha256:819f240a…` |
| [part18](docs/history/window1.md#stitchcad-changelog-part18md) | the G0 exit review, `STITCHCAD-G0-0015` | 38 lines, 3694 bytes, `sha256:22d63ec4…` |

**Correction (D30).** part1's own descriptor says its coverage runs "through `STITCHCAD-SPINE-0004c`".
It does not: part1's newest entry is `STITCHCAD-SPINE-0004b`, and `SPINE-0004c` is sealed in part2.
Sealed segments are immutable, so the correction is recorded here and in part2's descriptor rather than
by editing part1.

The bedrock scaffold's own changelog — the provenance of this repository's discipline spine — is sealed
in [`docs/history/bedrock-scaffold-changelog.md`](docs/history/window1.md#bedrock-scaffold-changelogmd).
| [part19](docs/history/window1.md#stitchcad-changelog-part19md) | STITCHCAD-G1-0001 | 21 lines, 1770 bytes, `sha256:2f602e9a…` |
| [part20](docs/history/window1.md#stitchcad-changelog-part20md) | STITCHCAD-G1-0002 | 19 lines, 1598 bytes, `sha256:a3918baa…` |
| [part21](docs/history/window1.md#stitchcad-changelog-part21md) | STITCHCAD-G1-0003 | 21 lines, 1831 bytes, `sha256:ab5e04ca…` |
| [part22](docs/history/window1.md#stitchcad-changelog-part22md) | STITCHCAD-G1-0004 | 24 lines, 2100 bytes, `sha256:9050689c…` |
| [part23](docs/history/window1.md#stitchcad-changelog-part23md) | STITCHCAD-G1-0005 | 48 lines, 4079 bytes, `sha256:e74210d6…` |
| [`changelog-part24.md`](docs/history/window2.md#stitchcad-changelog-part24md) | STITCHCAD-G1-0007, STITCHCAD-G1-0006 | 39 lines, 3139 bytes, `sha256:30ff1380…` |
| [`changelog-part25.md`](docs/history/window2.md#stitchcad-changelog-part25md) | STITCHCAD-G1-0008 | 17 lines, 1371 bytes, `sha256:d57637be…` |

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

| [part26](docs/history/window2.md#stitchcad-changelog-part26md) | STITCHCAD-G1-0009 | 16 lines, 1371 bytes, `sha256:ff2d0b1c…` |
| [part27](docs/history/window2.md#stitchcad-changelog-part27md) | STITCHCAD-G1-0010 | 20 lines, 1663 bytes, `sha256:b3824493…` |
| [part28](docs/history/window2.md#stitchcad-changelog-part28md) | STITCHCAD-G1-0011 | 20 lines, 1689 bytes, `sha256:6900e83e…` |
| [part29](docs/history/window2.md#stitchcad-changelog-part29md) | STITCHCAD-G1-0012 | 15 lines, 1306 bytes, `sha256:88277bd5…` |
| [part30](docs/history/window2.md#stitchcad-changelog-part30md) | STITCHCAD-G1-0013 | 14 lines, 1211 bytes, `sha256:7eb41b35…` |
| [part31](docs/history/window2.md#stitchcad-changelog-part31md) | STITCHCAD-G1-0014 | 13 lines, 1097 bytes, `sha256:28bbb8fb…` |
| [part32](docs/history/window2.md#stitchcad-changelog-part32md) | STITCHCAD-G1-0015 | 13 lines, 1096 bytes, `sha256:9e00082b…` |
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
| [part46](docs/history/window2.md#stitchcad-changelog-part46md) | G1-0028 and SPINE-0019c | 22 lines, 1861 bytes, `sha256:62f1e78a…` |
| [part47](docs/history/window3.md#stitchcad-changelog-part47md) | G1-0029 | 14 lines, 1170 bytes, `sha256:575d31eb…` |
| [part48](docs/history/window3.md#stitchcad-changelog-part48md) | G1-0030 | 14 lines, 1137 bytes, `sha256:0f58d786…` |
| [part49](docs/history/window3.md#stitchcad-changelog-part49md) | G1-0031 | 8 lines, 682 bytes, `sha256:ed0742e6…` |
| [part50](docs/history/window3.md#stitchcad-changelog-part50md) | STITCHCAD-G1-0032 | 14 lines, 1176 bytes, `sha256:32b7947f…` |
| [part51](docs/history/window3.md#stitchcad-changelog-part51md) | STITCHCAD-G1-0033 | 13 lines, 1094 bytes, `sha256:8f279ee4…` |
| [part52](docs/history/window3.md#stitchcad-changelog-part52md) | STITCHCAD-G1-0034 | 14 lines, 1219 bytes, `sha256:43a87a6f…` |
| [part53](docs/history/window3.md#stitchcad-changelog-part53md) | STITCHCAD-G1-0035 | 13 lines, 1094 bytes, `sha256:83494a18…` |
| [part54](docs/history/window3.md#stitchcad-changelog-part54md) | STITCHCAD-G1-0036 | 15 lines, 1273 bytes, `sha256:d5122987…` |
| [part55](docs/history/window3.md#stitchcad-changelog-part55md) | STITCHCAD-G1-0037 | 15 lines, 1284 bytes, `sha256:49280c1f…` |
| [part56](docs/history/window3.md#stitchcad-changelog-part56md) | STITCHCAD-G1-0038 | 14 lines, 1147 bytes, `sha256:8f2b2b4f…` |
| [part57](docs/history/window3.md#stitchcad-changelog-part57md) | STITCHCAD-G1-0039 | 12 lines, 958 bytes, `sha256:8ee0ecac…` |
| [part58](docs/history/window3.md#stitchcad-changelog-part58md) | STITCHCAD-G1-0040 | 15 lines, 1267 bytes, `sha256:5d9ffa4d…` |
| [part59](docs/history/window3.md#stitchcad-changelog-part59md) | STITCHCAD-G1-0041 | 13 lines, 1098 bytes, `sha256:38e6cdf2…` |
| [part60](docs/history/window3.md#stitchcad-changelog-part60md) | STITCHCAD-G1-0042 | 11 lines, 999 bytes, `sha256:717945df…` |
| [part61](docs/history/window3.md#stitchcad-changelog-part61md) | STITCHCAD-G1-0044 / STITCHCAD-G1-0043 | 28 lines, 2471 bytes, `sha256:5484241b…` |
| [part62](docs/history/window3.md#stitchcad-changelog-part62md) | STITCHCAD-G1-0045 | 11 lines, 970 bytes, `sha256:a334432a…` |
| [part63](docs/history/window3.md#stitchcad-changelog-part63md) | STITCHCAD-G1-0046 | 11 lines, 950 bytes, `sha256:939369e8…` |
| [part64](docs/history/window3.md#stitchcad-changelog-part64md) | STITCHCAD-G1-0047 | 11 lines, 995 bytes, `sha256:339b8cba…` |
| [part65](docs/history/window3.md#stitchcad-changelog-part65md) | STITCHCAD-G1-0049, STITCHCAD-G1-0048 | 15 lines, 1114 bytes, `sha256:e746de86…` |
| [part66](docs/history/window3.md#stitchcad-changelog-part66md) | STITCHCAD-G1-0051, STITCHCAD-G1-0050 | 18 lines, 1463 bytes, `sha256:3dcee333…` |
| [`changelog-part67.md`](docs/history/window3.md#stitchcad-changelog-part67md) | STITCHCAD-G1-0052 | 10 lines, 844 bytes, `sha256:d0d97d54…` |
| [`changelog-part68.md`](docs/history/window3.md#stitchcad-changelog-part68md) | STITCHCAD-G1-0054/0053 | 17 lines, 1299 bytes, `sha256:5a46d26d…` |
| [`changelog-part69.md`](docs/history/window3.md#stitchcad-changelog-part69md) | STITCHCAD-G1-0055 | 10 lines, 776 bytes, `sha256:e3db9c94…` |
| [part70](docs/history/window4.md#stitchcad-changelog-part70md) | STITCHCAD-G1-0056 | 12 lines, 951 bytes, `sha256:30ca94ac…` |
| [`changelog-part71.md`](docs/history/window4.md#stitchcad-changelog-part71md) | STITCHCAD-G1-0057 | 11 lines, 887 bytes, `sha256:e144c5ec…` |
| [`changelog-part72.md`](docs/history/window4.md#stitchcad-changelog-part72md) | STITCHCAD-G1-0058 | 12 lines, 970 bytes, `sha256:ef59b0a1…` |
| [`changelog-part73.md`](docs/history/window4.md#stitchcad-changelog-part73md) | STITCHCAD-G1-0059 | 13 lines, 1095 bytes, `sha256:eea1fea0…` |
| [`changelog-part74.md`](docs/history/window4.md#stitchcad-changelog-part74md) | STITCHCAD-G1-0060 | 13 lines, 1090 bytes, `sha256:ab4422c6…` |
| [`changelog-part75.md`](docs/history/window4.md#stitchcad-changelog-part75md) | STITCHCAD-G1-0061 | 13 lines, 1091 bytes, `sha256:c4bbd66c…` |
| [`changelog-part76.md`](docs/history/window4.md#stitchcad-changelog-part76md) | STITCHCAD-G1-0062 | 13 lines, 1068 bytes, `sha256:a0da7975…` |
| [`changelog-part77.md`](docs/history/window4.md#stitchcad-changelog-part77md) | STITCHCAD-G1-0063 | 14 lines, 1197 bytes, `sha256:3349f7a6…` |

| [part105](docs/history/window5.md#stitchcad-changelog-part105md) | `STITCHCAD-G1-0090` | 10 lines, 853 bytes, `sha256:723f9852…` |

| [part106](docs/history/window5.md#stitchcad-changelog-part106md) | `STITCHCAD-G1-0091` | 8 lines, 613 bytes, `sha256:407ab4aa…` |

| [part107](docs/history/window5.md#stitchcad-changelog-part107md) | `STITCHCAD-G1-0092` | 8 lines, 659 bytes, `sha256:63eeecdf…` |

| [part108](docs/history/window5.md#stitchcad-changelog-part108md) | G1-0093 wanted signature ledger | 8 lines, 647 bytes, `sha256:88395c24…` |
| [part109](docs/history/window5.md#stitchcad-changelog-part109md) | G1-0094 call lookup ledger | 9 lines, 766 bytes, `sha256:801531e8…` |
| [part110](docs/history/window5.md#stitchcad-changelog-part110md) | G1-0096/G1-0095 ledger | 19 lines, 1502 bytes, `sha256:8a882c4f…` |
| [part111](docs/history/window6.md#stitchcad-changelog-part111md) | G1-0099/0098/0097/0080h | 35 lines, 2644 bytes, `sha256:8cc099ce…` |

| [part168](docs/history/window9.md#stitchcad-changelog-part168md) | whole G1-0178 ledger | 7 lines, 563 bytes, sha256:444701d5… |


















## STITCHCAD-G1-0246 — exact signed rational sums and differences

.5c.1.2.2: bounded private bigint workspace reduces before128-bit judgment, preserving cancellation
and exact typed width witnesses. Focused native target3/0; broader/platform work stays deferred.

| [207](docs/history/window11.md#stitchcad-changelog-part207md) | whole0227 | sealed |

| [206](docs/history/window11.md#stitchcad-changelog-part206md) | whole0226 | sealed |

| [205](docs/history/window11.md#stitchcad-changelog-part205md) | whole0225 | sealed |

| [204](docs/history/window11.md#stitchcad-changelog-part204md) | whole0224 | sealed |

| [203](docs/history/window11.md#stitchcad-changelog-part203md) | whole0223 | sealed |

| [202](docs/history/window11.md#stitchcad-changelog-part202md) | whole0222 | sealed |

| [201](docs/history/window11.md#stitchcad-changelog-part201md) | whole0221 | sealed |

| [199](docs/history/window11.md#stitchcad-changelog-part199md) | 0219 | sealed |

Whole0218/0217 ledgers retained in [part198](docs/history/window11.md#stitchcad-changelog-part198md).

Whole retained whole0210 ledger: [part191](docs/history/window11.md#stitchcad-changelog-part191md).

Whole retained whole0209 ledger: [part190](docs/history/window11.md#stitchcad-changelog-part190md).

Whole whole0208 through0206 ledgers retained in [stitchcad-changelog-part188](docs/history/window11.md#stitchcad-changelog-part188md).

Whole0205/0204/0203 ledgers retained in [part187](docs/history/window10.md#stitchcad-changelog-part187md).

Whole0202/0201 ledgers retained in [part185](docs/history/window10.md#stitchcad-changelog-part185md).

Whole retained record: [part186](docs/history/window10.md#stitchcad-changelog-part186md).

Whole retained whole0198 and0197 ledgers: [part189](docs/history/window11.md#stitchcad-changelog-part189md).

Whole G1-0183 ledger retained in [the task record](docs/tasks/G1-SLICE-retention-proof.md#input-review-changelog-retention---whole0183).

| [part172](docs/history/window9.md#stitchcad-changelog-part172md) | whole G1-0182 ledger | 6 lines, 397 bytes, sha256:84358beb… |

| [part171](docs/history/window9.md#stitchcad-changelog-part171md) | whole G1-0181 ledger | 7 lines, 489 bytes, sha256:2d3d625d… |

| [part170](docs/history/window9.md#stitchcad-changelog-part170md) | whole G1-0180 ledger | 7 lines, 569 bytes, sha256:61c54829… |

| [part169](docs/history/window9.md#stitchcad-changelog-part169md) | whole G1-0179 ledger | 7 lines, 573 bytes, sha256:df27ee3b… |

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

| [part78](docs/history/window4.md#stitchcad-changelog-part78md) | G1-0064 identity review | 13 lines, 1077 bytes, `sha256:c658f537…` |
| [part79](docs/history/window4.md#stitchcad-changelog-part79md) | STITCHCAD-G1-0065 | 14 lines, 1183 bytes, `sha256:aa1df64f…` |
| [part80](docs/history/window4.md#stitchcad-changelog-part80md) | G1-0066 ordered recipe syntax | 15 lines, 1243 bytes, `sha256:ffd1e428…` |
| [part81](docs/history/window4.md#stitchcad-changelog-part81md) | STITCHCAD-G1-0067 | 11 lines, 974 bytes, `sha256:aad9b836…` |
| [part82](docs/history/window4.md#stitchcad-changelog-part82md) | STITCHCAD-G1-0068 | 12 lines, 1035 bytes, `sha256:451e0450…` |
| [part83](docs/history/window4.md#stitchcad-changelog-part83md) | STITCHCAD-G1-0069 | 12 lines, 1076 bytes, `sha256:61590cec…` |
| [part84](docs/history/window4.md#stitchcad-changelog-part84md) | STITCHCAD-G1-0070 | 13 lines, 1176 bytes, `sha256:ce5bfa8a…` |
| [part85](docs/history/window4.md#stitchcad-changelog-part85md) | STITCHCAD-G1-0071 | 14 lines, 1261 bytes, `sha256:a370b662…` |
| [part86](docs/history/window4.md#stitchcad-changelog-part86md) | STITCHCAD-G1-0072 | 9 lines, 795 bytes, `sha256:480c7c1b…` |
| [part87](docs/history/window4.md#stitchcad-changelog-part87md) | STITCHCAD-SPINE-0021b | 9 lines, 773 bytes, `sha256:df9a2d7c…` |
| [part88](docs/history/window4.md#stitchcad-changelog-part88md) | STITCHCAD-G1-0073 | 10 lines, 878 bytes, `sha256:32b87b67…` |
| [part89](docs/history/window4.md#stitchcad-changelog-part89md) | STITCHCAD-G1-0075/0074 | 16 lines, 1250 bytes, `sha256:2d8487c5…` |
| [part90](docs/history/window4.md#stitchcad-changelog-part90md) | STITCHCAD-SPINE-0023 | 8 lines, 648 bytes, `sha256:fa18d0d1…` |
| [part91](docs/history/window4.md#stitchcad-changelog-part91md) | STITCHCAD-SPINE-0023v | 5 lines, 339 bytes, `sha256:fad82add…` |
| [part92](docs/history/window4.md#stitchcad-changelog-part92md) | STITCHCAD-G1-0076 | 8 lines, 648 bytes, `sha256:9138f15a…` |
| [part93](docs/history/window5.md#stitchcad-changelog-part93md) | STITCHCAD-G1-0077 | 11 lines, 976 bytes, `sha256:556627ca…` |
| [part94](docs/history/window5.md#stitchcad-changelog-part94md) | STITCHCAD-G1-0078 | 10 lines, 887 bytes, `sha256:f70a9dba…` |
| [part95](docs/history/window5.md#stitchcad-changelog-part95md) | STITCHCAD-G1-0079 | 10 lines, 859 bytes, `sha256:94ba0cb6…` |
| [part96](docs/history/window5.md#stitchcad-changelog-part96md) | STITCHCAD-G1-0080 | 11 lines, 976 bytes, `sha256:8d91c335…` |
| [part97](docs/history/window5.md#stitchcad-changelog-part97md) | STITCHCAD-G1-0081 | 10 lines, 858 bytes, `sha256:14381657…` |
| [part98](docs/history/window5.md#stitchcad-changelog-part98md) | STITCHCAD-G1-0082 | 9 lines, 763 bytes, `sha256:cd886e4d…` |
| [part99](docs/history/window5.md#stitchcad-changelog-part99md) | STITCHCAD-G1-0083 | 9 lines, 767 bytes, `sha256:ce745517…` |
| [part100](docs/history/window5.md#stitchcad-changelog-part100md) | STITCHCAD-G1-0084 | 9 lines, 755 bytes, `sha256:ffb7e24e…` |
| [part101](docs/history/window5.md#stitchcad-changelog-part101md) | STITCHCAD-G1-0085 | 10 lines, 849 bytes, `sha256:838b6b56…` |
| [part102](docs/history/window5.md#stitchcad-changelog-part102md) | STITCHCAD-G1-0086 | 10 lines, 850 bytes, `sha256:ab8d7ec9…` |
| [part103](docs/history/window5.md#stitchcad-changelog-part103md) | STITCHCAD-G1-0087 | 14 lines, 1241 bytes, `sha256:af59826f…` |
| [part104](docs/history/window5.md#stitchcad-changelog-part104md) | STITCHCAD-G1-0089/0088 | 21 lines, 1708 bytes, `sha256:84e03c0f…` |
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

| [`part136`](docs/history/window8.md#stitchcad-changelog-part136md) | whole G1-0137/0136/0135 ledgers | 24 lines, 1819 bytes, `sha256:9b764edc…` |

| [`part137`](docs/history/window8.md#stitchcad-changelog-part137md) | three whole G1-0140/0139/0138 ledgers | 18 lines, 1236 bytes, `sha256:38ef85ae…` |

| [`part138`](docs/history/window8.md#stitchcad-changelog-part138md) | whole G1-0143/0142/0141 ledgers | 19 lines, 1383 bytes, `sha256:6e08f574…` |

| [`part139`](docs/history/window8.md#stitchcad-changelog-part139md) | whole G1-0144 ledger | 7 lines, 578 bytes, `sha256:0661ecd7…` |

| [`part140`](docs/history/window8.md#stitchcad-changelog-part140md) | whole G1-0145 cleanup ledger | 7 lines, 588 bytes, `sha256:8a23c596…` |

| [`part141`](docs/history/window8.md#stitchcad-changelog-part141md) | whole G1-0147 and G1-0146 ledgers | 14 lines, 1097 bytes, `sha256:8fa23459…` |

| [`part142`](docs/history/window8.md#stitchcad-changelog-part142md) | whole G1-0148 static guard ledger | 10 lines, 861 bytes, `sha256:790a719b…` |

| [`part143`](docs/history/window8.md#stitchcad-changelog-part143md) | whole G1-0149 continuity ledger | 6 lines, 530 bytes, `sha256:d5b8a94e…` |

| [`part144`](docs/history/window8.md#stitchcad-changelog-part144md) | whole G1-0151 and G1-0150 continuity ledgers | 13 lines, 983 bytes, `sha256:4c0a2f29…` |

| [`part145`](docs/history/window8.md#stitchcad-changelog-part145md) | whole G1-0153 and G1-0152 continuity ledgers | 14 lines, 1036 bytes, `sha256:07a2dcad…` |

| [`part146`](docs/history/window8.md#stitchcad-changelog-part146md) | whole G1-0154 continuity ledger | 7 lines, 603 bytes, `sha256:136114b1…` |

| [`part147`](docs/history/window8.md#stitchcad-changelog-part147md) | whole G1-0156 and G1-0155 continuity ledgers | 11 lines, 719 bytes, `sha256:45304271…` |

| [`part148`](docs/history/window8.md#stitchcad-changelog-part148md) | whole G1-0157 continuity ledger | 6 lines, 474 bytes, `sha256:934bd04d…` |

| [`part149`](docs/history/window8.md#stitchcad-changelog-part149md) | whole G1-0158 chart continuity ledger | 8 lines, 638 bytes, `sha256:d18ad1d9…` |

| [`part150`](docs/history/window8.md#stitchcad-changelog-part150md) | whole G1-0160 and G1-0159 continuity ledgers | 15 lines, 1158 bytes, `sha256:16a21ab1…` |

| [`part151`](docs/history/window8.md#stitchcad-changelog-part151md) | whole G1-0161 collection continuity ledgers | 7 lines, 570 bytes, `sha256:44242cdb…` |

| [`part152`](docs/history/window8.md#stitchcad-changelog-part152md) | whole G1-0162 individual Ease continuity ledgers | 7 lines, 571 bytes, `sha256:2d4ad014…` |

| [`part153`](docs/history/window8.md#stitchcad-changelog-part153md) | whole G1-0163 Ease-set continuity ledger | 7 lines, 557 bytes, `sha256:f9db2b5b…` |

| [`part154`](docs/history/window8.md#stitchcad-changelog-part154md) | whole G1-0164 signed-round continuity ledger | 8 lines, 665 bytes, `sha256:a3aa5776…` |

| [`part155`](docs/history/window8.md#stitchcad-changelog-part155md) | whole G1-0165 unsigned-round continuity ledger | 8 lines, 622 bytes, `sha256:33ca42fe…` |

| [`part156`](docs/history/window8.md#stitchcad-changelog-part156md) | whole G1-0166 length-operator continuity ledger | 9 lines, 757 bytes, `sha256:1f5c635b…` |

| [`part157`](docs/history/window8.md#stitchcad-changelog-part157md) | whole G1-0167 mixed-domain continuity ledger | 9 lines, 756 bytes, `sha256:d8f4288f…` |

| [`part158`](docs/history/window9.md#stitchcad-changelog-part158md) | whole G1-0168 lexer file and macro ledger | 8 lines, 581 bytes, `sha256:a253490b…` |

| [`part159`](docs/history/window9.md#stitchcad-changelog-part159md) | whole G1-0169 semantic mode and macro ledger | 8 lines, 648 bytes, `sha256:686a8b4f…` |

| [`part160`](docs/history/window9.md#stitchcad-changelog-part160md) | whole G1-0170 guarded recipe mode and worker ledger | 4 lines, 580 bytes, `sha256:e2113b03…` |

| [`part161`](docs/history/window9.md#stitchcad-changelog-part161md) | whole G1-0171 statement modes and D177 ledger | 6 lines, 463 bytes, `sha256:5da8b958…` |

| [`part162`](docs/history/window9.md#stitchcad-changelog-part162md) | whole G1-0172 ledger and navigation tail | 166 lines, 12187 bytes, `sha256:286ab32c…` |

| [`part164`](docs/history/window9.md#stitchcad-changelog-part164md) | whole G1-0173 normalized expression ledger | 6 lines, 490 bytes, `sha256:f55b6f6f…` |

| [`part165`](docs/history/window9.md#stitchcad-changelog-part165md) | whole G1-0175 and G1-0174 ledgers | 13 lines, 969 bytes, `sha256:37dd6f4b…` |

| [`part166`](docs/history/window9.md#stitchcad-changelog-part166md) | whole G1-0176 ledger | 6 lines, 498 bytes, `sha256:b78badf1…` |

| [`part167`](docs/history/window9.md#stitchcad-changelog-part167md) | whole G1-0177 ledger | 7 lines, 564 bytes, `sha256:0840c5ba…` |

| [`part174`](docs/history/window10.md#stitchcad-changelog-part174md) | whole0185 ledger | 8 lines, 450 bytes, `sha256:c302c5c0…` |
| [part173](docs/history/window10.md#stitchcad-changelog-part173md) | whole G1-0184 ledger | 6 lines, 386 bytes, sha256:fa21b2e4… |

| [part175](docs/history/window10.md#stitchcad-changelog-part175md) | whole0186 cleanup ledger | 5 lines, 417 bytes, sha256:61cb12f1… |

| [part176](docs/history/window10.md#stitchcad-changelog-part176md) | whole0187 builtin ledger | 6 lines, 479 bytes, sha256:09460dee… |

| [part177](docs/history/window10.md#stitchcad-changelog-part177md) | whole0188 operator ledger | 6 lines, 443 bytes, sha256:fcb51477… |

| [part178](docs/history/window10.md#stitchcad-changelog-part178md) | whole0189 wanted ledger | 7 lines, 621 bytes, sha256:56671d47… |

| [part179](docs/history/window10.md#stitchcad-changelog-part179md) | whole0190 call-plan ledger | 8 lines, 561 bytes, sha256:6440b75c… |

| [part180](docs/history/window10.md#stitchcad-changelog-part180md) | whole0191 expression-plan ledger | 6 lines, 476 bytes, sha256:38123105… |

| [part181](docs/history/window10.md#stitchcad-changelog-part181md) | whole0192 reduction-plan ledger | 6 lines, 389 bytes, sha256:30f3b928… |

| [part182](docs/history/window10.md#stitchcad-changelog-part182md) | whole0193 input-plan ledger | 5 lines, 313 bytes, sha256:bed0904d… |

| [part183](docs/history/window10.md#stitchcad-changelog-part183md) | whole0194 window9 ledger | 4 lines, 216 bytes, sha256:165a4b92… |

| [part184](docs/history/window10.md#stitchcad-changelog-part184md) | whole0196/0195 ledgers | 10 lines, 507 bytes, sha256:88e5570a… |

| [part185](docs/history/window10.md#stitchcad-changelog-part185md) | whole0202/0201 ledgers | 13 lines, 702 bytes, sha256:5a62a9a9… |

| [part186](docs/history/window10.md#stitchcad-changelog-part186md) | whole0200/0199 ledgers | 12 lines, 860 bytes, sha256:95911f3c… |

| [part187](docs/history/window10.md#stitchcad-changelog-part187md) | whole0205/0204/0203 ledgers | 14 lines, 612 bytes, sha256:0d29f076… |

| [part188](docs/history/window11.md#stitchcad-changelog-part188md) | whole0208 through0206 ledgers | 20 lines, 1283 bytes, sha256:50381f4f… |

| [part189](docs/history/window11.md#stitchcad-changelog-part189md) | whole0198 and0197 ledgers | 10 lines, 605 bytes, sha256:032aed1a… |

| [part190](docs/history/window11.md#stitchcad-changelog-part190md) | whole0209 ledger | 6 lines, 466 bytes, sha256:794856d3… |

| [part191](docs/history/window11.md#stitchcad-changelog-part191md) | whole0210 ledger | 5 lines, 336 bytes, sha256:73d7dbf8… |

| [part192](docs/history/window11.md#stitchcad-changelog-part192md) | whole0211 ledger | 6 lines, 443 bytes, sha256:531f2a44… |

| [part193](docs/history/window11.md#stitchcad-changelog-part193md) | whole0212 ledger | 5 lines, 354 bytes, sha256:302f1610… |

| [part194](docs/history/window11.md#stitchcad-changelog-part194md) | whole0213 ledger | 4 lines, 237 bytes, sha256:6a806fd0… |

| [part195](docs/history/window11.md#stitchcad-changelog-part195md) | wholeSPINE.23r ledger | 4 lines, 235 bytes, sha256:b5fbaf01… |

| [part196](docs/history/window11.md#stitchcad-changelog-part196md) | whole0214 ledger | 5 lines, 275 bytes, sha256:a71a4233… |

| [part197](docs/history/window11.md#stitchcad-changelog-part197md) | whole0216/0215 ledgers | 7 lines, 249 bytes, sha256:c77b778e… |

| [part198](docs/history/window11.md#stitchcad-changelog-part198md) | whole0218/0217 ledgers | 7 lines, 293 bytes, sha256:f965b1d7… |

| [200](docs/history/window11.md#stitchcad-changelog-part200md) | whole0220 | sealed |


| [part208](docs/history/stitchcad-changelog-part208.md) | whole0234v through0228 | 49 lines, 3030 bytes, sha256:2e6922f5… |

| [part209](docs/history/stitchcad-changelog-part209.md) | whole0235 | 6 lines, 453 bytes, sha256:547359ee… |

| [part210](docs/history/stitchcad-changelog-part210.md) | whole0236 | 8 lines, 645 bytes, sha256:a79c26a7… |

| [part211](docs/history/stitchcad-changelog-part211.md) | whole0237 | 5 lines, 326 bytes, sha256:c2872be6… |

| [part212](docs/history/stitchcad-changelog-part212.md) | whole0238 | 5 lines, 375 bytes, sha256:4b8726ed… |

| [part213](docs/history/stitchcad-changelog-part213.md) | whole0239 | 5 lines, 341 bytes, sha256:b1054b78… |

| [part214](docs/history/stitchcad-changelog-part214.md) | whole0241/0240 | 11 lines, 642 bytes, sha256:87131883… |

| [part215](docs/history/stitchcad-changelog-part215.md) | whole0242 | 5 lines, 364 bytes, sha256:40c7cebb… |

| [part216](docs/history/stitchcad-changelog-part216.md) | whole0243 | 5 lines, 368 bytes, sha256:dce8d7bb… |

| [part217](docs/history/stitchcad-changelog-part217.md) | whole0244 | 4 lines, 219 bytes, sha256:33c0c860… |

| [part218](docs/history/stitchcad-changelog-part218.md) | whole0245 | 4 lines, 250 bytes, sha256:bbadee3a… |
