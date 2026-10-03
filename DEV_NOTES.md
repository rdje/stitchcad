# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.


## _(2026-10-03 UTC)_ — call sources differ from data origins

- D136/D137 had correct tokens but empty arguments. Callee checks retain query,
  actual envelope/catalog searches and truthful alternatives; data origins never grant callability.
- Reference and actual product faults verify payloads and argument priority.
  Syntax/input validation precedes static checking; context is never fabricated.
- Promotion: `docs/decisions/decision_call-lookup.md`; strict checks pass, D138 dimensions next.

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/window1.md#stitchcad-devnotes-part1md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/window1.md#stitchcad-devnotes-part2md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/window1.md#stitchcad-devnotes-part3md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/window1.md#stitchcad-devnotes-part4md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/window1.md#stitchcad-devnotes-part5md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`devnotes-part6.md`](docs/history/window1.md#stitchcad-devnotes-part6md) | two `2026-09-30` lessons (a spec's tables are its test suite; settle it with the artifact) | 55 lines, 5359 bytes, `sha256:129d50d8…` |
| [`devnotes-part7.md`](docs/history/window1.md#stitchcad-devnotes-part7md) | three `2026-09-30` lessons (an arm that removes the rule; a synthetic input is a fixture; a RED arm asserts the refusal) | 57 lines, 5120 bytes, `sha256:13fd6c73…` |
| [`devnotes-part8.md`](docs/history/window1.md#stitchcad-devnotes-part8md) | two `2026-09-30` lessons (source layout; i18n population) | 35 lines, 3196 bytes, `sha256:04ab285c…` |
| [`devnotes-part9.md`](docs/history/window1.md#stitchcad-devnotes-part9md) | the `2026-09-30` certifying-artifact lesson | 15 lines, 1343 bytes, `sha256:bc7fae65…` |
| [`devnotes-part10.md`](docs/history/window1.md#stitchcad-devnotes-part10md) | the `2026-09-30` shipped-work reconciliation lesson | 15 lines, 1380 bytes, `sha256:701d33f2…` |
| [`devnotes-part11.md`](docs/history/window1.md#stitchcad-devnotes-part11md) | the `2026-09-30` property-test framework lesson | 15 lines, 1384 bytes, `sha256:ae04eadf…` |
| [`devnotes-part12.md`](docs/history/window1.md#stitchcad-devnotes-part12md) | ontology slice decomposition | 16 lines, 1570 bytes, `sha256:a2f04e3d…` |
| [`devnotes-part13.md`](docs/history/window1.md#stitchcad-devnotes-part13md) | injected identity lesson | 18 lines, 1612 bytes, `sha256:38e83349…` |
| [`devnotes-part14.md`](docs/history/window1.md#stitchcad-devnotes-part14md) | persistent-identity lesson | 24 lines, 2230 bytes, `sha256:2b6aebd3…` |
| [`devnotes-part15.md`](docs/history/window1.md#stitchcad-devnotes-part15md) | structural-piece lesson | 15 lines, 1334 bytes, `sha256:1b362d26…` |
| [`devnotes-part16.md`](docs/history/window1.md#stitchcad-devnotes-part16md) | interval-coverage lesson | 13 lines, 1183 bytes, `sha256:fcf7c475…` |
| [`devnotes-part17.md`](docs/history/window1.md#stitchcad-devnotes-part17md) | separate-pair-member lesson | 12 lines, 1049 bytes, `sha256:140c4c41…` |
| [`devnotes-part18.md`](docs/history/window1.md#stitchcad-devnotes-part18md) | semantic-anchor/profile-binding lesson | 12 lines, 1102 bytes, `sha256:61a13500…` |
| [`devnotes-part19.md`](docs/history/window1.md#stitchcad-devnotes-part19md) | physical-copy identity lesson | 18 lines, 1663 bytes, `sha256:c0e3c442…` |
| [`devnotes-part20.md`](docs/history/window1.md#stitchcad-devnotes-part20md) | physical sewing-interval lesson | 15 lines, 1375 bytes, `sha256:34867dc9…` |

| [`devnotes-part21.md`](docs/history/window1.md#stitchcad-devnotes-part21md) | directed-grainline lesson | 13 lines, 1212 bytes, `sha256:c31c3298…` |

| [`devnotes-part22.md`](docs/history/window1.md#stitchcad-devnotes-part22md) | per-edge allowance lesson | 13 lines, 1192 bytes, `sha256:1807ae98…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

| [`devnotes-part23.md`](docs/history/window1.md#stitchcad-devnotes-part23md) | tuck/pleat and dart lessons | 25 lines, 2172 bytes, `sha256:ba5ee2a7…` |

| [`devnotes-part24.md`](docs/history/window1.md#stitchcad-devnotes-part24md) | canonical gather lesson | 13 lines, 1160 bytes, `sha256:2a10a04e…` |

| [`devnotes-part25.md`](docs/history/window1.md#stitchcad-devnotes-part25md) | Hem and served-layer lessons | 28 lines, 2416 bytes, `sha256:a95d8c77…` |

| [`devnotes-part26.md`](docs/history/window2.md#stitchcad-devnotes-part26md) | closure and notion-placement lessons | 27 lines, 2385 bytes, `sha256:b4b58e1b…` |

| [`devnotes-part27.md`](docs/history/window2.md#stitchcad-devnotes-part27md) | canonical buttonhole derivation lesson | 13 lines, 1158 bytes, `sha256:319f11b3…` |

| [`stitchcad-devnotes-part28.md`](docs/history/window2.md#stitchcad-devnotes-part28md) | Pocket composition lesson | 14 lines, 1301 bytes, `sha256:764116ee…` |

| [`stitchcad-devnotes-part29.md`](docs/history/window2.md#stitchcad-devnotes-part29md) | coverage probe calibration lesson | 15 lines, 1383 bytes, `sha256:d6d266b6…` |

| [`stitchcad-devnotes-part30.md`](docs/history/window2.md#stitchcad-devnotes-part30md) | numeric availability/source-truth lesson | 15 lines, 1356 bytes, `sha256:7fa4db87…` |

| [`stitchcad-devnotes-part31.md`](docs/history/window2.md#stitchcad-devnotes-part31md) | identifier grammar/binding lesson | 14 lines, 1283 bytes, `sha256:317f385a…` |

| [`stitchcad-devnotes-part32.md`](docs/history/window2.md#stitchcad-devnotes-part32md) | canonical procedure metadata lesson | 20 lines, 1878 bytes, `sha256:d5d201dc…` |

| [`stitchcad-devnotes-part33.md`](docs/history/window2.md#stitchcad-devnotes-part33md) | archive capacity/retrieval lesson | 18 lines, 1591 bytes, `sha256:aad7494a…` |

| [`stitchcad-devnotes-part34.md`](docs/history/window2.md#stitchcad-devnotes-part34md) | measurement table binding lesson | 18 lines, 1699 bytes, `sha256:3dc0b619…` |

| [`devnotes-part35.md`](docs/history/window2.md#stitchcad-devnotes-part35md) | individual Ease mapping lesson | 13 lines, 1190 bytes, `sha256:de68d50d…` |

| [`devnotes-part36.md`](docs/history/window2.md#stitchcad-devnotes-part36md) | per-POM Ease query lesson | 13 lines, 1196 bytes, `sha256:f0b78fd7…` |

| [`devnotes-part37.md`](docs/history/window2.md#stitchcad-devnotes-part37md) | membership/Ease review lessons | 21 lines, 1781 bytes, `sha256:6a76a906…` |
| [`devnotes-part38.md`](docs/history/window2.md#stitchcad-devnotes-part38md) | chart correspondence lesson | 14 lines, 1281 bytes, `sha256:453f9677…` |
| [`devnotes-part39.md`](docs/history/window2.md#stitchcad-devnotes-part39md) | MTM/coverage lessons | 31 lines, 2736 bytes, `sha256:c7d16877…` |
| [`devnotes-part40.md`](docs/history/window2.md#stitchcad-devnotes-part40md) | progressive book lesson | 18 lines, 1677 bytes, `sha256:89bc77bc…` |

| [`devnotes-part41.md`](docs/history/window2.md#stitchcad-devnotes-part41md) | borrowed lexical source lesson | 19 lines, 1811 bytes, `sha256:92361240…` |

| [`devnotes-part42.md`](docs/history/window2.md#stitchcad-devnotes-part42md) | complete argument traversal lesson | 19 lines, 1781 bytes, `sha256:f77b3cf9…` |

| [`devnotes-part43.md`](docs/history/window2.md#stitchcad-devnotes-part43md) | source gaps and keyword roles | 15 lines, 1397 bytes, `sha256:3d7763a5…` |

| [`devnotes-part44.md`](docs/history/window2.md#stitchcad-devnotes-part44md) | exact arithmetic results | 17 lines, 1562 bytes, `sha256:a378e4ad…` |

| [`devnotes-part45.md`](docs/history/window2.md#stitchcad-devnotes-part45md) | semantic bounds and delimiter nesting | 19 lines, 1803 bytes, `sha256:c9686523…` |

| [`devnotes-part46.md`](docs/history/window2.md#stitchcad-devnotes-part46md) | signed reconstruction lesson | 17 lines, 1562 bytes, `sha256:d00c7340…` |

| [`devnotes-part47.md`](docs/history/window2.md#stitchcad-devnotes-part47md) | literal identity lesson | 17 lines, 1561 bytes, `sha256:1b718cca…` |

| [`devnotes-part48.md`](docs/history/window2.md#stitchcad-devnotes-part48md) | angular conversion lesson | 18 lines, 1649 bytes, `sha256:376d37ac…` |

| [`stitchcad-devnotes-part49.md`](docs/history/window2.md#stitchcad-devnotes-part49md) | reduced-width lesson | 18 lines, 1658 bytes, `sha256:0317bfcf…` |

| [`stitchcad-devnotes-part50.md`](docs/history/window3.md#stitchcad-devnotes-part50md) | public-operator lesson | 18 lines, 1673 bytes, `sha256:2f4b81de…` |

| [`stitchcad-devnotes-part51.md`](docs/history/window3.md#stitchcad-devnotes-part51md) | domain-context lesson | 19 lines, 1705 bytes, `sha256:6c25796f…` |

| [`stitchcad-devnotes-part52.md`](docs/history/window3.md#stitchcad-devnotes-part52md) | inline-language lesson | 18 lines, 1440 bytes, `sha256:f9fdf033…` |

| [`devnotes-part53.md`](docs/history/window3.md#stitchcad-devnotes-part53md) | scalar-domain lesson | 17 lines, 1495 bytes, `sha256:c5c05294…` |

| [`devnotes-part54.md`](docs/history/window3.md#stitchcad-devnotes-part54md) | numeric binding/literal ruling lesson | 16 lines, 1445 bytes, `sha256:7fcc4170…` |

| [`devnotes-part55.md`](docs/history/window3.md#stitchcad-devnotes-part55md) | archive capacity and D96 lesson | 19 lines, 1731 bytes, `sha256:12f0338c…` |

| [`devnotes-part56.md`](docs/history/window3.md#stitchcad-devnotes-part56md) | observed CI lesson | 12 lines, 1047 bytes, `sha256:cb01f979…` |

| [`stitchcad-devnotes-part57.md`](docs/history/window3.md#stitchcad-devnotes-part57md) | scoped numeric review lesson | 15 lines, 1350 bytes, `sha256:a196cecf…` |

| [`stitchcad-devnotes-part58.md`](docs/history/window3.md#stitchcad-devnotes-part58md) | six-kind replay lesson | 15 lines, 1327 bytes, `sha256:f5820b9c…` |

| [`stitchcad-devnotes-part59.md`](docs/history/window3.md#stitchcad-devnotes-part59md) | signed principal angle lesson | 15 lines, 1367 bytes, `sha256:19cb42c9…` |

| [`stitchcad-devnotes-part60.md`](docs/history/window3.md#stitchcad-devnotes-part60md) | scoped reference review lesson | 12 lines, 1063 bytes, `sha256:29112bc9…` |

| [`stitchcad-devnotes-part61.md`](docs/history/window3.md#stitchcad-devnotes-part61md) | unsigned magnitude rounding lesson | 15 lines, 1322 bytes, `sha256:90999a24…` |

| [`stitchcad-devnotes-part62.md`](docs/history/window3.md#stitchcad-devnotes-part62md) | individual literal normalization lesson | 18 lines, 1637 bytes, `sha256:69a0a9b4…` |

| [`stitchcad-devnotes-part63.md`](docs/history/window3.md#stitchcad-devnotes-part63md) | coupled normalization review lesson | 34 lines, 3069 bytes, `sha256:99a6f002…` |

| [`stitchcad-devnotes-part64.md`](docs/history/window3.md#stitchcad-devnotes-part64md) | G1-0062 inspection tags | 15 lines, 1345 bytes, `sha256:5699b48e…` |

| [`stitchcad-devnotes-part65.md`](docs/history/window3.md#stitchcad-devnotes-part65md) | G1-0063 expression identity | 17 lines, 1602 bytes, `sha256:377fd96c…` |

| [`stitchcad-devnotes-part66.md`](docs/history/window3.md#stitchcad-devnotes-part66md) | G1-0065/0064 syntax and identity lessons | 33 lines, 2960 bytes, `sha256:01527194…` |

| [`stitchcad-devnotes-part67.md`](docs/history/window3.md#stitchcad-devnotes-part67md) | G1-0066 ordered recipe boundaries | 20 lines, 1850 bytes, `sha256:934aee26…` |

| [`stitchcad-devnotes-part68.md`](docs/history/window3.md#stitchcad-devnotes-part68md) | G1-0067 coupled recipe review | 18 lines, 1629 bytes, `sha256:6a16cabe…` |
| [`stitchcad-devnotes-part69.md`](docs/history/window3.md#stitchcad-devnotes-part69md) | G1-0068 exact recipe bytes | 18 lines, 1651 bytes, `sha256:e352ef3b…` |
| [`stitchcad-devnotes-part70.md`](docs/history/window3.md#stitchcad-devnotes-part70md) | G1-0069 whole input normalization | 20 lines, 1874 bytes, `sha256:ce41946c…` |
| [`stitchcad-devnotes-part71.md`](docs/history/window3.md#stitchcad-devnotes-part71md) | G1-0070 identity lesson | 22 lines, 2039 bytes, `sha256:364f5f56…` |
| [`stitchcad-devnotes-part72.md`](docs/history/window3.md#stitchcad-devnotes-part72md) | G1-0071 coupled review lesson | 20 lines, 1758 bytes, `sha256:0b6fd7f2…` |

| [`devnotes-part73.md`](docs/history/window3.md#stitchcad-devnotes-part73md) | G1-0072 syntax closure | 18 lines, 1564 bytes, `sha256:e6407999…` |

| [`stitchcad-devnotes-part74.md`](docs/history/stitchcad-devnotes-part74.md) | SPINE-0021b cleanup lesson | 17 lines, 1432 bytes, `sha256:13acd888…` |

| [`devnotes-part75.md`](docs/history/stitchcad-devnotes-part75.md) | static/archive/handoff lessons | 62 lines, 5136 bytes, `sha256:b1ff0e5f…` |

| [`devnotes-part76.md`](docs/history/stitchcad-devnotes-part76.md) | observed handoff CI lesson | 8 lines, 625 bytes, `sha256:f9b6ffbc…` |
| [`devnotes-part77.md`](docs/history/stitchcad-devnotes-part77.md) | whole preflight/namespace lessons | 41 lines, 3724 bytes, `sha256:898aab4a…` |
| [`devnotes-part78.md`](docs/history/stitchcad-devnotes-part78.md) | static recognition review lesson | 17 lines, 1566 bytes, `sha256:94655e7b…` |

| [`stitchcad-devnotes-part79.md`](docs/history/stitchcad-devnotes-part79.md) | G1-0079 assertion lesson | 13 lines, 1135 bytes, `sha256:71b5fdbc…` |

| [`stitchcad-devnotes-part80.md`](docs/history/stitchcad-devnotes-part80.md) | origin and context lesson | 16 lines, 1471 bytes, `sha256:8576788c…` |

| [`stitchcad-devnotes-part81.md`](docs/history/stitchcad-devnotes-part81.md) | executed provenance lesson | 12 lines, 1049 bytes, `sha256:de02f486…` |

| [`stitchcad-devnotes-part82.md`](docs/history/stitchcad-devnotes-part82.md) | D124 grammar recognition lesson | 11 lines, 932 bytes, `sha256:2d6e66b8…` |

| [`stitchcad-devnotes-part83.md`](docs/history/stitchcad-devnotes-part83.md) | G1-0083 metadata lesson | 13 lines, 1139 bytes, `sha256:c79e3803…` |

| [`stitchcad-devnotes-part84.md`](docs/history/stitchcad-devnotes-part84.md) | G1-0084 source metadata lesson | 14 lines, 1110 bytes, `sha256:f1ad9ba4…` |

| [`stitchcad-devnotes-part85.md`](docs/history/stitchcad-devnotes-part85.md) | G1-0085 reserved diagnostic proposal lesson | 11 lines, 903 bytes, `sha256:a1c4d338…` |

| [`stitchcad-devnotes-part86.md`](docs/history/stitchcad-devnotes-part86.md) | G1-0086 delegated binding-source lesson | 12 lines, 1018 bytes, `sha256:92241e4a…` |

| [`stitchcad-devnotes-part87.md`](docs/history/stitchcad-devnotes-part87.md) | G1-0087 initial namespace lesson | 17 lines, 1539 bytes, `sha256:06d7a8cd…` |

| [`stitchcad-devnotes-part88.md`](docs/history/stitchcad-devnotes-part88.md) | G1-0088 exact name-read lesson | 14 lines, 1220 bytes, `sha256:11170a8b…` |

| [`stitchcad-devnotes-part89.md`](docs/history/stitchcad-devnotes-part89.md) | G1-0089 ordered name-scope lesson | 15 lines, 1296 bytes, `sha256:ffe6d2c5…` |

| [`stitchcad-devnotes-part90.md`](docs/history/stitchcad-devnotes-part90.md) | G1-0090 operator signatures lesson | 13 lines, 1106 bytes, `sha256:b0a427bf…` |

| [`stitchcad-devnotes-part91.md`](docs/history/stitchcad-devnotes-part91.md) | G1-0091 diagnostic guidance lesson | 10 lines, 838 bytes, `sha256:a83b9679…` |

| [`stitchcad-devnotes-part92.md`](docs/history/stitchcad-devnotes-part92.md) | G1-0092 symbolic-role lesson | 9 lines, 753 bytes, `sha256:a62696a7…` |

| [`stitchcad-devnotes-part93.md`](docs/history/stitchcad-devnotes-part93.md) | G1-0093 wanted-rule lesson | 8 lines, 649 bytes, `sha256:41df0629…` |
