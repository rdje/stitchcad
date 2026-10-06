# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-10-06 UTC)_ — read-only public reference plans

D156's three static libraries loaded public helpers and compiled book references without complete
nonmutating plans. Their actual effects are reads and in-memory compilation. Imports must stay pure;
whole-book fixture preparation or store activation would change the borrowed library contract.
Bootstrap lists contain3/4/5 sources; fresh reference lists contain8/9/10, including the five normative
book inputs. Namespace/review reuse the signature provider's exact Refusal class, preserving nested
borrowed failures without catching unrelated ValueError/assertion exceptions. Standalone guarded
failures print the existing local-environment refusal and exit2; permissive mutation argv remains.

The watched actual control passes on Python3.14 and OS Python3.9.6:17 import/contract/CLI/loader/quantum/
exception captures, ten actual guard omissions,18 physical CLI refusals and12 borrowed refusals.
Three actual planner-body disables admit the same invalid plans in owned isolated mirrors. Per-call
frames prove fresh complete source plans before reads/delegation; original functions, classes, authored
populations/faults and CLI bodies are AST-exact after removing only added guards/refusal wrappers.
Prepared stores, unrelated values, normal/exception argv identity and tracked inputs stay exact.
The D126 quantum proof retains intentional reference numerical work; static contracts retain traps.

Public caller inspection found reference plans before the coupled build, after the first-phase native
work, and after recipe pure preflight. Closed adapters assert each stage/list/sequence, retaining all
original producing guards. Bridge40 coupled+nine body reds, recipe six captures/twelve reds/fourteen
physical refusals, and shared1125 reds/58 complete captures/329 physical refusals pass. Eleven actual
public callers retain their original normal results and faults; their own future locality audit remains.

Tracked python_static_loader_contract.py --native passes263/ten reference faults and coupled26723
cases separately from captures; the probe runner watches frozen inputs and current library identity.
All194 frozen inputs remain exact, including118 tracked Rust files and63 production src files; current
library15981408B has sha256:168cf69628c5c229f30cac663354ffe3a47c2e35a7e2105253031eaf20b7125f.
No native source or grammar change. OS interpreter/stdlib are read-only dependencies; generated
fixtures, logs and capture stores stay on the repository volume. Draft failures are excluded: wrong
Git-boundary diagnostic wording, incomplete pre/post-native reference adapters, an authoring function
name mismatch, and a Rust census that omitted the established /src/ filter. Tools resolved each.

Whole0200 task1458+lesson3428→260;0191 ledger476→180;completed scopes6451→261 were fully sealed,
copied, inventoried and publicly read before retirement. Own-only dedup preserves canonical fields;
prior565 logical/all56 physical bytes remain exact. Fixed caps stay; no existing seal changes.
D204 was found by independent continuity, then the actual ledger gate failed9 pass/2 fail. The
retention helper's stale part179 selector routed the exact CH180 index row into DEV instead of CH.
Whole historical bytes were intact. Report/ownership preceded moving only that row and selecting
by destination kind; the real public pointer set then matched all179 segments. Whole authored report
and verified closure were copied/read in126 before live retirement. Existing pointer gates watch it.

External policy hashes unchanged at2026-10-06 06:58UTC; cleanup is not due until22:28UTC.
Promotion declined: existing pure-call, public-boundary, independent-control and continuity rules apply.

Whole0200 task and copied-book lesson retained in [part260](docs/history/stitchcad-devnotes-part260.md).



Whole0199 task and bridge lesson retained in [part256](docs/history/stitchcad-devnotes-part256.md).

Whole0198 task and coupled static lesson retained in [part253](docs/history/stitchcad-devnotes-part253.md).

Owner compiler lesson retained in [part181](docs/history/window7.md#stitchcad-devnotes-part181md).

Window6 retention lesson retained in [part174](docs/history/window7.md#stitchcad-devnotes-part174md).

D163 declaration guard lesson retained in [part176](docs/history/window7.md#stitchcad-devnotes-part176md).

D162 namespace guard lesson retained in [part178](docs/history/window7.md#stitchcad-devnotes-part178md).

D161 statement and whole proof lesson retained in [part180](docs/history/window7.md#stitchcad-devnotes-part180md).

Whole0197 task and statement compiler lesson retained in [part251](docs/history/stitchcad-devnotes-part251.md).

Whole0196 task and recipe compiler lesson retained in [part249](docs/history/stitchcad-devnotes-part249.md).

Whole0195 receipts and original bridge inspection retained in [part247](docs/history/stitchcad-devnotes-part247.md).

Whole0194 task and continuity lesson retained in [part246](docs/history/stitchcad-devnotes-part246.md).

Whole input modes and D197/D198 lesson retained in [part244](docs/history/stitchcad-devnotes-part244.md).

Whole reduction dated lesson retained in [part243](docs/history/window9.md#stitchcad-devnotes-part243md).

Whole expression dated lesson retained in [part240](docs/history/window9.md#stitchcad-devnotes-part240md).

Whole call dated lesson retained in [part238](docs/history/window9.md#stitchcad-devnotes-part238md).

Whole wanted dated lesson and links retained in [part233](docs/history/window9.md#stitchcad-devnotes-part233md).

Whole operator and builtin dated lessons retained in [part229](docs/history/window9.md#stitchcad-devnotes-part229md).

Whole ordered-name replay: [part225](docs/history/window9.md#stitchcad-devnotes-part225md). Seal date correction: UTC2026-10-05; immutable header says October6.

Whole daily-cleanup and exact-name lessons retained in [part226](docs/history/window9.md#stitchcad-devnotes-part226md).

Whole declaration plans and negative construction lesson retained in [part223](docs/history/window9.md#stitchcad-devnotes-part223md).

Whole ordered-name plans and guarded advances lesson retained in [part221](docs/history/window9.md#stitchcad-devnotes-part221md).

Whole namespace plans and thread lesson retained in [part219](docs/history/window9.md#stitchcad-devnotes-part219md).

Whole window8 post-recording guard lesson retained in [part218](docs/history/window9.md#stitchcad-devnotes-part218md).

Whole window8 continuity and protected history lesson retained in [part222](docs/history/window9.md#stitchcad-devnotes-part222md).

Whole checked-recipe baseline and extraction lesson retained in [part217](docs/history/window9.md#stitchcad-devnotes-part217md).

Whole checked-statement modes and extraction lesson retained in [part216](docs/history/window9.md#stitchcad-devnotes-part216md).

Whole checked-expression modes and panic lesson retained in [part214](docs/history/window9.md#stitchcad-devnotes-part214md).

Whole canonical expression modes and thread lesson retained in [part213](docs/history/window8.md#stitchcad-devnotes-part213md).

Whole canonical recipe modes and thread lesson retained in [part212](docs/history/window8.md#stitchcad-devnotes-part212md).

Whole normalized recipe modes and thread lesson retained in [part211](docs/history/window8.md#stitchcad-devnotes-part211md).

Whole normalized expression modes and thread lesson retained in [part210](docs/history/window8.md#stitchcad-devnotes-part210md).

Whole literal modes and thread lesson retained in [part209](docs/history/window8.md#stitchcad-devnotes-part209md).

Whole statement modes and thread lesson retained in [part208](docs/history/window8.md#stitchcad-devnotes-part208md).

Whole recipe modes and worker lesson retained in [part207](docs/history/window8.md#stitchcad-devnotes-part207md).

Whole semantic mode and macro lesson retained in [part206](docs/history/window8.md#stitchcad-devnotes-part206md).

Whole lexer file and macro lesson retained in [part205](docs/history/window8.md#stitchcad-devnotes-part205md).

Whole mixed domain-context lesson retained in [part204](docs/history/window8.md#stitchcad-devnotes-part204md).

Whole length-operator lesson retained in [part203](docs/history/window8.md#stitchcad-devnotes-part203md).

Whole unsigned/signed rounding lessons retained in [part202](docs/history/window8.md#stitchcad-devnotes-part202md).

Whole Ease-set/individual lessons retained in [part201](docs/history/window8.md#stitchcad-devnotes-part201md).

Whole collection/MTM lessons retained in [part200](docs/history/window8.md#stitchcad-devnotes-part200md).

Whole table effect/panic lesson retained in [part199](docs/history/window8.md#stitchcad-devnotes-part199md).

Whole chart effect/panic lesson retained in [part198](docs/history/window8.md#stitchcad-devnotes-part198md).

Whole membership lesson retained in [part197](docs/history/window8.md#stitchcad-devnotes-part197md).

Whole mutable/window lessons retained in [part196](docs/history/window8.md#stitchcad-devnotes-part196md).

Whole bridge/effect ownership lessons retained in [part192](docs/history/window8.md#stitchcad-devnotes-part192md).

Navigation/continuity lessons retained in [part188](docs/history/window8.md#stitchcad-devnotes-part188md).


Static guard/D172–D175 lesson retained in [part184](docs/history/window7.md#stitchcad-devnotes-part184md).


D160 lesson retained in [part182](docs/history/window7.md#stitchcad-devnotes-part182md); UTC seal date correction:2026-10-05 (immutable header says Oct4).

Normalization/assertion/recipe lessons retained in [part187](docs/history/window7.md#stitchcad-devnotes-part187md).

Older formula and producer lessons retained in [part186](docs/history/window7.md#stitchcad-devnotes-part186md).


## Retained engineering continuity

Complete retrieval: `bash scripts/history_archive.sh list` enumerates every logical record;
`read <repository-relative-identity>` returns its complete verified bytes.
[Exact prior navigation](docs/history/stitchcad-devnotes-part248.md) preserves the whole
committed table and original links. Recent records follow; older identities remain in the public list.

| Segment | Coverage | Sealed identity |
| --- | --- | --- |

| [part247](docs/history/stitchcad-devnotes-part247.md) | whole0195 receipts and original bridge inspection | 40 lines, 2822 bytes, sha256:f32c681c… |

| [part248](docs/history/stitchcad-devnotes-part248.md) | whole prior navigation | 113 lines, 10385 bytes, sha256:bb5fb0dd… |

| [part249](docs/history/stitchcad-devnotes-part249.md) | whole0196 task and recipe compiler lesson | 38 lines, 2822 bytes, sha256:bff80147… |

| [part250](docs/history/stitchcad-devnotes-part250.md) | whole completed n12 scopes | 26 lines, 2383 bytes, sha256:9b2c64f3… |

| [part251](docs/history/stitchcad-devnotes-part251.md) | whole0197 task and statement compiler lesson | 37 lines, 2820 bytes, sha256:2cba3977… |

| [part252](docs/history/stitchcad-devnotes-part252.md) | whole completed n13 scopes | 32 lines, 2573 bytes, sha256:ba323f3c… |

| [part253](docs/history/stitchcad-devnotes-part253.md) | whole0198 task and coupled static lesson | 38 lines, 4279 bytes, sha256:ce1b9a12… |

| [part254](docs/history/stitchcad-devnotes-part254.md) | whole completed n14 scope | 8 lines, 1069 bytes, sha256:03b0a1ef… |

| [part255](docs/history/stitchcad-devnotes-part255.md) | whole authored old control node | 7 lines, 700 bytes, sha256:1c20d098… |

| [part256](docs/history/stitchcad-devnotes-part256.md) | whole0199 task and bridge control lesson | 37 lines, 3810 bytes, sha256:c05fe3a8… |

| [part257](docs/history/stitchcad-devnotes-part257.md) | whole completed profile control scope | 9 lines, 796 bytes, sha256:ed2cb337… |

| [part258](docs/history/stitchcad-devnotes-part258.md) | whole completed builtin family scopes | 37 lines, 3692 bytes, sha256:d2e8489b… |

| [part259](docs/history/stitchcad-devnotes-part259.md) | whole completed namespace and declaration scopes | 48 lines, 4638 bytes, sha256:2f985e2b… |

| [part260](docs/history/stitchcad-devnotes-part260.md) | whole0200 task and copied-book lesson | 49 lines, 4886 bytes, sha256:3ca9c660… |


| [part261](docs/history/stitchcad-devnotes-part261.md) | whole copied-book and signature-family scopes | 73 lines, 6451 bytes, sha256:bea84f20… |
