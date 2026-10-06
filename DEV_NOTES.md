# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-10-06 UTC)_ — inline context and nested classifier fixture plans

Original inline fixtures could replace their book without complete source/output plans; the writer
read/created fixtures on import and changed the classifier before validating its child's replacement.
Both original gaps fail python_inline_context_contract.py; original13/five cases and all assertions
compare exactly against6df9f5c ASTs after removing the added guards/environment adapters.
D156 remains open for later producers. This leaf closes only .a09; classifier/grammar stay exact.

The fixture validates BOOT3 on import. It plans the whole65-file book, one replacement tree and actual
selectors before activation, rejects old unexpected entries/links/hardlinks/selected file or directory
aliases, and rechecks deletion/copied inputs/child source and selectors. Pure fixture_plan(activate=False)
returns the complete plan without effects; producing calls preserve prepared stores, argv and other
environment values. Relative selectors match the child's repository working directory. Caller
selector overrides can change the authored verdicts; they are preserved, never silently rewritten.

The writer validates BOOT4 before loading the inert fixture module and adopts its Refusal identity.
Its producing plan merges the child's complete plan:69 baseline sources,65 copied files/five logs and
one writable classifier. Child preflight runs before activation or source changes, then each source
change/child/log gets a fresh plan. Both restoration paths validate only that writable source (D207).
An unexpected child entry refuses before effects; disabling the actual child old-entry guard admits
that same entry through the writer's pure planner, with the classifier/sentinel unchanged.

Current controls3.14/OS3.9, target/inline-plan-control/control39.log, rc=0: nine captures,20 actual guard
reds/two original gaps; full13-copy calls perform858 real copied writes/13 intercepted children,
writer16 modeled writes/five children, failure path restores the source. Fourteen physical refusals,
unused-sidebar inert-import positive,14 CLI/eight repeated borrowed refusals, actual directory alias
and profile planner omissions discriminate. Real mirrored late-log failure produces CLI2; full-plan
restoration variants fail the source-byte assertion, source-only guards restore; mirrors/actual source
exact. A draft symlink fixture expected the generic message instead of the earlier copied-book
refusal; corrected from observed output before signoff. The initial retention draft's old receipt
suffix assertion failed before any write; the actual6df source boundary supplied the correct suffix.

Exclusive real3.14/3.9 runs, target/inline-plan-real/real39.log, rc=0: all13 independently authored
verdicts and five original classifier assertion reds. Terminal traceback frames name actual contract
assertions; no SyntaxError or profile refusal counts as a red. All188 frozen inputs/118Rust63
production stay byte-exact after each child. Captured dispatch and real reference evidence are distinct.

Whole0207 receipt/lesson274 and oldest0205/0204/0203 ledgers187 are exact6df intervals, complete FILE,
fresh single-inode copies, prior590/all81 physical controls and public reads before live retirement.
Current592/64 working Markdown; no sealed edit or ceiling increase. Three protected proof siblings,
closed Source headers, all prior records/navigation and current library remain separately checked.

Focused closure: publication45, language16, census9/9/4 plus235 Git paths/126 file rows, coverage9,
archive28+611 CLI/all592 reads, ledger781 real verdicts/11 arms plus13 pointer controls, all terminal
rc=0 in target/inline-plan-{publication,language,census,coverage,archive,ledger,census-cli,continuity}.log.
Continuity: prior590/all81 physical controls,70 original closed headers,186 CH pointers, navigation
11675/four routes, three proof siblings/current library/caps exact; pressure64/21887L/1604763 logicalB/
778854 residentB. Known P3 ls/head SIGPIPE noise recurred; existing .b.s ownership remains. No new defect.

Promotion declined: applies existing boundary, nested-effect, calibrated counterfactual, source-only
restoration and whole-record retention rules. The new control is watched by the environment runner.

Whole0207 task/lesson retained in [part274](docs/history/stitchcad-devnotes-part274.md).

Whole0205 task/lesson/scope retained in [part270](docs/history/stitchcad-devnotes-part270.md).

Whole0204 task/lesson/scope retained in [part269](docs/history/stitchcad-devnotes-part269.md).

Whole0203 task and geometry/call lesson retained in [part267](docs/history/stitchcad-devnotes-part267.md).

Whole0202 task and pure payload lesson retained in [part265](docs/history/stitchcad-devnotes-part265.md).

Whole0201 task/lesson prefix retained in [part262](docs/history/stitchcad-devnotes-part262.md).

Complete0201 lesson tail retained in [part263](docs/history/stitchcad-devnotes-part263.md); prefix in part262.

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


Whole0206 task/lesson retained in [part271](docs/history/stitchcad-devnotes-part271.md).

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

| [part262](docs/history/stitchcad-devnotes-part262.md) | whole0201 task and public-loader lesson | 40 lines, 3990 bytes, sha256:a2ca4f23… |

| [part263](docs/history/stitchcad-devnotes-part263.md) | complete0201 lesson tail/D205 boundary repair | 12 lines, 988 bytes, sha256:0e2987e2… |

| [part264](docs/history/stitchcad-devnotes-part264.md) | whole public-loader/declaration/cleanup scopes | 20 lines, 2188 bytes, sha256:bc63bced… |

| [part127](docs/history/stitchcad-defects-part127.md) | D205 whole authored boundary report/verified closure | 3 lines, 786 bytes, sha256:ae7842f1… |

| [part265](docs/history/stitchcad-devnotes-part265.md) | whole0202 task and pure payload lesson | 46 lines, 4285 bytes, sha256:9c8393f1… |

| [part266](docs/history/stitchcad-devnotes-part266.md) | whole completed payload-library scope | 8 lines, 870 bytes, sha256:93c7f85b… |

| [part267](docs/history/stitchcad-devnotes-part267.md) | whole0203 task and geometry/call lesson | 43 lines, 3882 bytes, sha256:cda30653… |

| [part268](docs/history/stitchcad-devnotes-part268.md) | whole completed geometry/call scope | 7 lines, 692 bytes, sha256:523c9127… |

| [part269](docs/history/stitchcad-devnotes-part269.md) | whole0204 task/lesson/scope | 42 lines, 3893 bytes, sha256:fae39ec6… |

| [part128](docs/history/stitchcad-defects-part128.md) | D206 report/verified closure | 7 lines, 712 bytes, sha256:d12bedb6… |

| [part270](docs/history/stitchcad-devnotes-part270.md) | whole0205 task/lesson/scope | 44 lines, 3852 bytes, sha256:5aa39fa7… |

| [part271](docs/history/stitchcad-devnotes-part271.md) | whole0206 task/lesson | 65 lines, 5113 bytes, sha256:a271aff2… |

| [part272](docs/history/stitchcad-devnotes-part272.md) | whole original Ease receipt | 10 lines, 710 bytes, sha256:c9a97e5b… |
| [part273](docs/history/stitchcad-devnotes-part273.md) | whole original serializer checklist | 22 lines, 2058 bytes, sha256:14b7d6df… |

| [part274](docs/history/stitchcad-devnotes-part274.md) | whole0207 task/lesson | 61 lines, 5997 bytes, sha256:b45c9d9f… |
