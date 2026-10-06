# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.


## _(2026-10-06 UTC)_ — coupled static complete plans

Ordinary execution declares7 files and20 read-only inputs; mutation CLI declares244 files
and only its4 Rust sources writable. Each build label independently plans7 outputs/current
inputs before mkdir/Cargo, including the borrowed first-phase label. Cargo-reported libraries,
metadata and dependency directories receive pure checks before Rustc. Prepared stores remain.
The original26723 cases,28 Rust/six reference/three annex faults and all child arguments stay.
Actual captures retain ordinary3/mutation90/early-refusal8/borrowed3 dispatch and restoration.

The main static block supplies334 exact source-body omissions, including every initial file,
source and writable input, each per-build output and current library plans. Physical15 CLI
refusals precede reads/store activation/effects. Ordinary accepts all4 Rust inputs with3 links
and ignores an unused mutation-tail alias. Profile4 binding reds/two guard-disable controls pass.
Fresh python_producer_contract.py:1125 body reds/35 classifiers/58 captures/329 refusals, rc=0.

static_coupled_contract.py --mutations passes baseline/restored26723 and28 Rust/6 reference/3
annex reds. first_phase_contract.py --mutations passes263 public/reference cases and10 body
reds. Independent verifier checks93 frozen inputs/63Rust, original case/fault functions/data,
argv and bridge bytes, both26723 driver oracles,263 phase outputs and current library SHA.
Both current driver binaries rerun exactly. Native and verifier terminal0; no compiler noise counts.
Draft diagnostic precedence and helper-counter failures were consumed/fixed and uncredited.

Before growth, whole d96 task1436B+lesson1384B→251,0188ledger443B→177 and completed n13
scopes2573B→252 were copied/verified/read before live retirement. Prior549 identities/all40
committed archive bytes stay. Book growth preflight refused before write: Source bytes2400–6034,
3634B57L sha256:838ab2585ef2a6dfcfb9a82c9c4b548cd67f8003a54157ae67a73ac6b53f1023,
are copied whole into formula-static-signatures.md before relocation, with both navigation routes.
Final all552 reads/prior549/all40 bytes/59 Source headers55live4retained/176 pointers/three
protected proofs/93 frozen/map/caps pass0. Ledger731 real/11 arms/13 pointers pass0.
Current .n14 closes; .a01.2 copied-book fixtures next. D156, engine/grammar and400 cadence stay.

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
