# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.



## _(2026-10-04 UTC)_ — preserve ordered ambiguity sources

- Two equal origins still name two colliding declarations; retain both in binding order.
- Pair positions, recipe ordinals and absent dictionary locations have distinct provenance.
- Metadata-only arguments need no numerical/state read. Promotion declined (existing source policy).

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`part1`](docs/history/window1.md#stitchcad-devnotes-part1md) | 2026-09/bootstrap | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`part2`](docs/history/window1.md#stitchcad-devnotes-part2md) | vocabulary | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`part3`](docs/history/window1.md#stitchcad-devnotes-part3md) | two-table/garment fixture | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`part4`](docs/history/window1.md#stitchcad-devnotes-part4md) | leaf split/authority | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`part5`](docs/history/window1.md#stitchcad-devnotes-part5md) | path refusal/digest | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`part6`](docs/history/window1.md#stitchcad-devnotes-part6md) | specification-table oracle | 55 lines, 5359 bytes, `sha256:129d50d8…` |
| [`part7`](docs/history/window1.md#stitchcad-devnotes-part7md) | 2026-09-30 rule/fixture/refusal lessons | 57 lines, 5120 bytes, `sha256:13fd6c73…` |
| [`part8`](docs/history/window1.md#stitchcad-devnotes-part8md) | source layout; i18n population | 35 lines, 3196 bytes, `sha256:04ab285c…` |
| [`part9`](docs/history/window1.md#stitchcad-devnotes-part9md) | certifying-artifact | 15 lines, 1343 bytes, `sha256:bc7fae65…` |
| [`part10`](docs/history/window1.md#stitchcad-devnotes-part10md) | shipped reconciliation | 15 lines, 1380 bytes, `sha256:701d33f2…` |
| [`part11`](docs/history/window1.md#stitchcad-devnotes-part11md) | property-test framework | 15 lines, 1384 bytes, `sha256:ae04eadf…` |
| [`part12`](docs/history/window1.md#stitchcad-devnotes-part12md) | ontology slice decomposition | 16 lines, 1570 bytes, `sha256:a2f04e3d…` |
| [`part13`](docs/history/window1.md#stitchcad-devnotes-part13md) | injected identity | 18 lines, 1612 bytes, `sha256:38e83349…` |
| [`part14`](docs/history/window1.md#stitchcad-devnotes-part14md) | persistent-identity | 24 lines, 2230 bytes, `sha256:2b6aebd3…` |
| [`part15`](docs/history/window1.md#stitchcad-devnotes-part15md) | structural-piece | 15 lines, 1334 bytes, `sha256:1b362d26…` |
| [`part16`](docs/history/window1.md#stitchcad-devnotes-part16md) | interval-coverage | 13 lines, 1183 bytes, `sha256:fcf7c475…` |
| [`part17`](docs/history/window1.md#stitchcad-devnotes-part17md) | separate-pair-member | 12 lines, 1049 bytes, `sha256:140c4c41…` |
| [`part18`](docs/history/window1.md#stitchcad-devnotes-part18md) | semantic-anchor/profile-binding | 12 lines, 1102 bytes, `sha256:61a13500…` |
| [`part19`](docs/history/window1.md#stitchcad-devnotes-part19md) | physical-copy identity | 18 lines, 1663 bytes, `sha256:c0e3c442…` |
| [`part20`](docs/history/window1.md#stitchcad-devnotes-part20md) | physical sewing-interval | 15 lines, 1375 bytes, `sha256:34867dc9…` |
| [`part21`](docs/history/window1.md#stitchcad-devnotes-part21md) | directed-grainline | 13 lines, 1212 bytes, `sha256:c31c3298…` |
| [`part22`](docs/history/window1.md#stitchcad-devnotes-part22md) | edge allowance | 13 lines, 1192 bytes, `sha256:1807ae98…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

| [`part23`](docs/history/window1.md#stitchcad-devnotes-part23md) | tuck/pleat/dart | 25 lines, 2172 bytes, `sha256:ba5ee2a7…` |
| [`part24`](docs/history/window1.md#stitchcad-devnotes-part24md) | gather | 13 lines, 1160 bytes, `sha256:2a10a04e…` |
| [`part25`](docs/history/window1.md#stitchcad-devnotes-part25md) | Hem and served-layer lessons | 28 lines, 2416 bytes, `sha256:a95d8c77…` |
| [`part26`](docs/history/window2.md#stitchcad-devnotes-part26md) | closure/notion lessons | 27 lines, 2385 bytes, `sha256:b4b58e1b…` |
| [`part27`](docs/history/window2.md#stitchcad-devnotes-part27md) | buttonhole derivation | 13 lines, 1158 bytes, `sha256:319f11b3…` |
| [`part28`](docs/history/window2.md#stitchcad-devnotes-part28md) | Pocket composition | 14 lines, 1301 bytes, `sha256:764116ee…` |
| [`part29`](docs/history/window2.md#stitchcad-devnotes-part29md) | coverage probe calibration | 15 lines, 1383 bytes, `sha256:d6d266b6…` |
| [`part30`](docs/history/window2.md#stitchcad-devnotes-part30md) | numeric availability/source-truth | 15 lines, 1356 bytes, `sha256:7fa4db87…` |
| [`part31`](docs/history/window2.md#stitchcad-devnotes-part31md) | identifier grammar/binding | 14 lines, 1283 bytes, `sha256:317f385a…` |
| [`part32`](docs/history/window2.md#stitchcad-devnotes-part32md) | procedure metadata | 20 lines, 1878 bytes, `sha256:d5d201dc…` |
| [`part33`](docs/history/window2.md#stitchcad-devnotes-part33md) | archive capacity/retrieval | 18 lines, 1591 bytes, `sha256:aad7494a…` |
| [`part34`](docs/history/window2.md#stitchcad-devnotes-part34md) | measurement table binding | 18 lines, 1699 bytes, `sha256:3dc0b619…` |
| [`part35`](docs/history/window2.md#stitchcad-devnotes-part35md) | individual Ease mapping | 13 lines, 1190 bytes, `sha256:de68d50d…` |
| [`part36`](docs/history/window2.md#stitchcad-devnotes-part36md) | per-POM Ease query | 13 lines, 1196 bytes, `sha256:f0b78fd7…` |
| [`part37`](docs/history/window2.md#stitchcad-devnotes-part37md) | membership/Ease review lessons | 21 lines, 1781 bytes, `sha256:6a76a906…` |
| [`part38`](docs/history/window2.md#stitchcad-devnotes-part38md) | chart correspondence | 14 lines, 1281 bytes, `sha256:453f9677…` |
| [`part39`](docs/history/window2.md#stitchcad-devnotes-part39md) | MTM/coverage lessons | 31 lines, 2736 bytes, `sha256:c7d16877…` |
| [`part40`](docs/history/window2.md#stitchcad-devnotes-part40md) | progressive book | 18 lines, 1677 bytes, `sha256:89bc77bc…` |
| [`part41`](docs/history/window2.md#stitchcad-devnotes-part41md) | borrowed lexical source | 19 lines, 1811 bytes, `sha256:92361240…` |
| [`part42`](docs/history/window2.md#stitchcad-devnotes-part42md) | complete argument traversal | 19 lines, 1781 bytes, `sha256:f77b3cf9…` |
| [`part43`](docs/history/window2.md#stitchcad-devnotes-part43md) | source gaps and keyword roles | 15 lines, 1397 bytes, `sha256:3d7763a5…` |
| [`part44`](docs/history/window2.md#stitchcad-devnotes-part44md) | exact arithmetic results | 17 lines, 1562 bytes, `sha256:a378e4ad…` |
| [`part45`](docs/history/window2.md#stitchcad-devnotes-part45md) | semantic bounds and delimiter nesting | 19 lines, 1803 bytes, `sha256:c9686523…` |
| [`part46`](docs/history/window2.md#stitchcad-devnotes-part46md) | signed reconstruction | 17 lines, 1562 bytes, `sha256:d00c7340…` |
| [`part47`](docs/history/window2.md#stitchcad-devnotes-part47md) | literal identity | 17 lines, 1561 bytes, `sha256:1b718cca…` |
| [`part48`](docs/history/window2.md#stitchcad-devnotes-part48md) | angular conversion | 18 lines, 1649 bytes, `sha256:376d37ac…` |
| [`part49`](docs/history/window2.md#stitchcad-devnotes-part49md) | reduced-width | 18 lines, 1658 bytes, `sha256:0317bfcf…` |
| [`part50`](docs/history/window3.md#stitchcad-devnotes-part50md) | public-operator | 18 lines, 1673 bytes, `sha256:2f4b81de…` |
| [`part51`](docs/history/window3.md#stitchcad-devnotes-part51md) | domain-context | 19 lines, 1705 bytes, `sha256:6c25796f…` |
| [`part52`](docs/history/window3.md#stitchcad-devnotes-part52md) | inline-language | 18 lines, 1440 bytes, `sha256:f9fdf033…` |
| [`part53`](docs/history/window3.md#stitchcad-devnotes-part53md) | scalar-domain | 17 lines, 1495 bytes, `sha256:c5c05294…` |
| [`part54`](docs/history/window3.md#stitchcad-devnotes-part54md) | numeric binding/literal ruling | 16 lines, 1445 bytes, `sha256:7fcc4170…` |
| [`part55`](docs/history/window3.md#stitchcad-devnotes-part55md) | archive capacity and D96 | 19 lines, 1731 bytes, `sha256:12f0338c…` |
| [`part56`](docs/history/window3.md#stitchcad-devnotes-part56md) | observed CI | 12 lines, 1047 bytes, `sha256:cb01f979…` |
| [`part57`](docs/history/window3.md#stitchcad-devnotes-part57md) | scoped numeric review | 15 lines, 1350 bytes, `sha256:a196cecf…` |
| [`part58`](docs/history/window3.md#stitchcad-devnotes-part58md) | kind replay | 15 lines, 1327 bytes, `sha256:f5820b9c…` |
| [`part59`](docs/history/window3.md#stitchcad-devnotes-part59md) | principal angle | 15 lines, 1367 bytes, `sha256:19cb42c9…` |
| [`part60`](docs/history/window3.md#stitchcad-devnotes-part60md) | scoped reference review | 12 lines, 1063 bytes, `sha256:29112bc9…` |
| [`part61`](docs/history/window3.md#stitchcad-devnotes-part61md) | unsigned magnitude rounding | 15 lines, 1322 bytes, `sha256:90999a24…` |
| [`part62`](docs/history/window3.md#stitchcad-devnotes-part62md) | literal normalization | 18 lines, 1637 bytes, `sha256:69a0a9b4…` |
| [`part63`](docs/history/window3.md#stitchcad-devnotes-part63md) | coupled normalization review | 34 lines, 3069 bytes, `sha256:99a6f002…` |
| [`part64`](docs/history/window3.md#stitchcad-devnotes-part64md) | G1-0062 inspection tags | 15 lines, 1345 bytes, `sha256:5699b48e…` |
| [`part65`](docs/history/window3.md#stitchcad-devnotes-part65md) | G1-0063 expression identity | 17 lines, 1602 bytes, `sha256:377fd96c…` |
| [`part66`](docs/history/window3.md#stitchcad-devnotes-part66md) | G1-0065/0064 syntax and identity lessons | 33 lines, 2960 bytes, `sha256:01527194…` |
| [`part67`](docs/history/window3.md#stitchcad-devnotes-part67md) | G1-0066 ordered recipe boundaries | 20 lines, 1850 bytes, `sha256:934aee26…` |
| [`part68`](docs/history/window3.md#stitchcad-devnotes-part68md) | G1-0067 coupled recipe review | 18 lines, 1629 bytes, `sha256:6a16cabe…` |
| [`part69`](docs/history/window3.md#stitchcad-devnotes-part69md) | G1-0068 exact recipe bytes | 18 lines, 1651 bytes, `sha256:e352ef3b…` |
| [`part70`](docs/history/window3.md#stitchcad-devnotes-part70md) | G1-0069 whole input normalization | 20 lines, 1874 bytes, `sha256:ce41946c…` |
| [`part71`](docs/history/window3.md#stitchcad-devnotes-part71md) | G1-0070 identity | 22 lines, 2039 bytes, `sha256:364f5f56…` |
| [`part72`](docs/history/window3.md#stitchcad-devnotes-part72md) | G1-0071 coupled review | 20 lines, 1758 bytes, `sha256:0b6fd7f2…` |
| [`part73`](docs/history/window3.md#stitchcad-devnotes-part73md) | G1-0072 syntax closure | 18 lines, 1564 bytes, `sha256:e6407999…` |
| [`part74`](docs/history/window4.md#stitchcad-devnotes-part74md) | SPINE-0021b cleanup | 17 lines, 1432 bytes, `sha256:13acd888…` |
| [`part75`](docs/history/window4.md#stitchcad-devnotes-part75md) | static/archive/handoff lessons | 62 lines, 5136 bytes, `sha256:b1ff0e5f…` |
| [`part76`](docs/history/window4.md#stitchcad-devnotes-part76md) | observed handoff CI | 8 lines, 625 bytes, `sha256:f9b6ffbc…` |
| [`part77`](docs/history/window4.md#stitchcad-devnotes-part77md) | preflight/namespace | 41 lines, 3724 bytes, `sha256:898aab4a…` |
| [`part78`](docs/history/window4.md#stitchcad-devnotes-part78md) | static recognition review | 17 lines, 1566 bytes, `sha256:94655e7b…` |
| [`part79`](docs/history/window4.md#stitchcad-devnotes-part79md) | G1-0079 assertion | 13 lines, 1135 bytes, `sha256:71b5fdbc…` |
| [`part80`](docs/history/window4.md#stitchcad-devnotes-part80md) | origin and context | 16 lines, 1471 bytes, `sha256:8576788c…` |
| [`part81`](docs/history/window4.md#stitchcad-devnotes-part81md) | executed provenance | 12 lines, 1049 bytes, `sha256:de02f486…` |
| [`part82`](docs/history/window4.md#stitchcad-devnotes-part82md) | D124 grammar recognition | 11 lines, 932 bytes, `sha256:2d6e66b8…` |
| [`part83`](docs/history/window4.md#stitchcad-devnotes-part83md) | G1-0083 metadata | 13 lines, 1139 bytes, `sha256:c79e3803…` |
| [`part84`](docs/history/window4.md#stitchcad-devnotes-part84md) | G1-0084 source metadata | 14 lines, 1110 bytes, `sha256:f1ad9ba4…` |
| [`part85`](docs/history/window4.md#stitchcad-devnotes-part85md) | G1-0085 reserved diagnostic proposal | 11 lines, 903 bytes, `sha256:a1c4d338…` |
| [`part86`](docs/history/window4.md#stitchcad-devnotes-part86md) | G1-0086 delegated binding-source | 12 lines, 1018 bytes, `sha256:92241e4a…` |
| [`part87`](docs/history/window4.md#stitchcad-devnotes-part87md) | G1-0087 initial namespace | 17 lines, 1539 bytes, `sha256:06d7a8cd…` |
| [`part88`](docs/history/window4.md#stitchcad-devnotes-part88md) | G1-0088 exact name-read | 14 lines, 1220 bytes, `sha256:11170a8b…` |
| [`part89`](docs/history/window4.md#stitchcad-devnotes-part89md) | G1-0089 ordered scope | 15 lines, 1296 bytes, `sha256:ffe6d2c5…` |
| [`part90`](docs/history/window4.md#stitchcad-devnotes-part90md) | G1-0090 operator signatures | 13 lines, 1106 bytes, `sha256:b0a427bf…` |
| [`part91`](docs/history/window4.md#stitchcad-devnotes-part91md) | G1-0091 diagnostic guidance | 10 lines, 838 bytes, `sha256:a83b9679…` |
| [`part92`](docs/history/window4.md#stitchcad-devnotes-part92md) | G1-0092 symbolic-role | 9 lines, 753 bytes, `sha256:a62696a7…` |
| [`part93`](docs/history/window4.md#stitchcad-devnotes-part93md) | G1-0093 wanted-rule | 8 lines, 649 bytes, `sha256:41df0629…` |
| [`part94`](docs/history/window4.md#stitchcad-devnotes-part94md) | G1-0094 call-source | 7 lines, 500 bytes, `sha256:5d32cfcf…` |
| [`part95`](docs/history/stitchcad-devnotes-part95.md) | G1-0095 missing-kind | 5 lines, 338 bytes, `sha256:a1cf2132…` |
| [`part96`](docs/history/stitchcad-devnotes-part96.md) | G1-0096 catalog-label lesson | 6 lines, 423 bytes, `sha256:d93fba17…` |
| [`part98`](docs/history/stitchcad-devnotes-part98.md) | G1 retention/CI protocols | 86 lines, 6837 bytes, `sha256:741f466e…` |
| [`part97`](docs/history/stitchcad-devnotes-part97.md) | G1-0098 compiler lesson | 6 lines, 417 bytes, `sha256:87269597…` |
| [`part99`](docs/history/stitchcad-devnotes-part99.md) | CI locality | 6 lines, 425 bytes, `sha256:bba39276…` |
| [`part100`](docs/history/stitchcad-devnotes-part100.md) | CI observation | 4 lines, 232 bytes, `sha256:ae0a5f60…` |
| [`part101`](docs/history/stitchcad-devnotes-part101.md) | cleanup | 4 lines, 156 bytes, `sha256:cea7ce81…` |
| [`part102`](docs/history/stitchcad-devnotes-part102.md) | geometry preflight | 5 lines, 357 bytes, `sha256:7addd9c0…` |
| [`part103`](docs/history/stitchcad-devnotes-part103.md) | CI/cleanup protocols | 118 lines, 8944 bytes, `sha256:532de5ae…` |
| [`part104`](docs/history/stitchcad-devnotes-part104.md) | source positions | 5 lines, 312 bytes, `sha256:3990c390…` |
| [`part105`](docs/history/stitchcad-devnotes-part105.md) | G1-0104 prerequisite | 5 lines, 321 bytes, `sha256:28672a01…` |
| [`part106`](docs/history/stitchcad-devnotes-part106.md) | G1-0105 checker protocol | 39 lines, 3461 bytes, `sha256:6cfe304a…` |
| [`part107`](docs/history/stitchcad-devnotes-part107.md) | G1-0105 checker lesson | 9 lines, 746 bytes, `sha256:551f5302…` |
| [`part108`](docs/history/stitchcad-devnotes-part108.md) | D139 header protocol | 34 lines, 2868 bytes, `sha256:d0ca98ee…` |
| [`part109`](docs/history/stitchcad-devnotes-part109.md) | D139 header lesson | 7 lines, 543 bytes, `sha256:7cd43f55…` |
| [`part110`](docs/history/stitchcad-devnotes-part110.md) | D146 class protocol | 31 lines, 2497 bytes, `sha256:2fb7225e…` |
| [`part111`](docs/history/stitchcad-devnotes-part111.md) | D146 class lesson | 5 lines, 267 bytes, `sha256:839ac404…` |
| [`part112`](docs/history/stitchcad-devnotes-part112.md) | D145 prerequisite protocol | 23 lines, 1819 bytes, `sha256:4014eac8…` |
| [`part113`](docs/history/stitchcad-devnotes-part113.md) | G1-0108 statement protocol | 41 lines, 3566 bytes, `sha256:98c31994…` |
| [`part114`](docs/history/stitchcad-devnotes-part114.md) | G1-0108 owner lesson | 5 lines, 258 bytes, `sha256:dbc1f357…` |
