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

| [`stitchcad-changelog-part47.md`](docs/history/stitchcad-changelog-part47.md) | G1-0029 | 14 lines, 1170 bytes, `sha256:575d31eb…` |

| [`stitchcad-changelog-part48.md`](docs/history/stitchcad-changelog-part48.md) | G1-0030 | 14 lines, 1137 bytes, `sha256:0f58d786…` |

| [`stitchcad-changelog-part49.md`](docs/history/stitchcad-changelog-part49.md) | G1-0031 | 8 lines, 682 bytes, `sha256:ed0742e6…` |

| [`stitchcad-changelog-part50.md`](docs/history/stitchcad-changelog-part50.md) | STITCHCAD-G1-0032 | 14 lines, 1176 bytes, `sha256:32b7947f…` |

| [`stitchcad-changelog-part51.md`](docs/history/stitchcad-changelog-part51.md) | STITCHCAD-G1-0033 | 13 lines, 1094 bytes, `sha256:8f279ee4…` |

| [`stitchcad-changelog-part52.md`](docs/history/stitchcad-changelog-part52.md) | STITCHCAD-G1-0034 | 14 lines, 1219 bytes, `sha256:43a87a6f…` |

| [`stitchcad-changelog-part53.md`](docs/history/stitchcad-changelog-part53.md) | STITCHCAD-G1-0035 | 13 lines, 1094 bytes, `sha256:83494a18…` |

| [`stitchcad-changelog-part54.md`](docs/history/stitchcad-changelog-part54.md) | STITCHCAD-G1-0036 | 15 lines, 1273 bytes, `sha256:d5122987…` |

| [`stitchcad-changelog-part55.md`](docs/history/stitchcad-changelog-part55.md) | STITCHCAD-G1-0037 | 15 lines, 1284 bytes, `sha256:49280c1f…` |

| [`stitchcad-changelog-part56.md`](docs/history/stitchcad-changelog-part56.md) | STITCHCAD-G1-0038 | 14 lines, 1147 bytes, `sha256:8f2b2b4f…` |

| [`stitchcad-changelog-part57.md`](docs/history/stitchcad-changelog-part57.md) | STITCHCAD-G1-0039 | 12 lines, 958 bytes, `sha256:8ee0ecac…` |

| [`stitchcad-changelog-part58.md`](docs/history/stitchcad-changelog-part58.md) | STITCHCAD-G1-0040 | 15 lines, 1267 bytes, `sha256:5d9ffa4d…` |

| [`stitchcad-changelog-part59.md`](docs/history/stitchcad-changelog-part59.md) | STITCHCAD-G1-0041 | 13 lines, 1098 bytes, `sha256:38e6cdf2…` |

| [`stitchcad-changelog-part60.md`](docs/history/stitchcad-changelog-part60.md) | STITCHCAD-G1-0042 | 11 lines, 999 bytes, `sha256:717945df…` |

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

## STITCHCAD-G1-0057 - complete scoped reference review (leaf `G1-SLICE.5a.3b.3c.3`)

D84 received contract/reference obligations are verified: signed principal angles, raw binding/
equality/full turns, signed/fractional arcs and explicit normalized dir. The annex maps every
obligation to independent controls and actual faults. D100's adjacent parent label now distinguishes
128-bit canonical literals from i64 numeric bindings; original records/parent/evidence remain exact.

Signed90/fifteen actual reds and angular72/math42/seven reds pass with exact restoration; all existing
reference/language/publication/recording checks remain green. Reference parents close and product
normalization .5a.3c is next. G1 stays5/18; defects10open/89sealed. Production evaluator, geometry,
entity integration, arbitrary-input transcendental/cross-platform and API/MCP/release proof remain future.

## STITCHCAD-G1-0056 - signed principal formula angles (leaf `G1-SLICE.5a.3b.3c.2`)

Reference atan/atan2 retain signed principal results at the nearest microdegree; dir stays normalized.
Bindings preserve full/signed/multi-turn values and compare raw quantities, so360 degrees differs
from zero and produces a full-turn arc. Principal branches/rounded endpoints now have explicit rules.

Ninety independent controls/fifteen compiled actual reds verify principal/binding/equality/sweep/book
replay. Existing42-row math oracle/72 angular controls/seven reds, binding80/twelve reds and replay19/
nine reds remain green, with exact restoration. Book/units/decision/task/live records agree and prior
payloads retain exact bytes. D84 final review follows; G1 stays5/18, defects12open/87sealed.
D100’s stale adjacent parent label is logged/owned by the next review.
No production evaluator or general transcendental/cross-platform certificate is claimed.

## STITCHCAD-G1-0055 - complete reference binding replay (leaf `G1-SLICE.5a.3b.3c.1`)

D99's actual book consumer now accepts all six normative bindable kinds. Derived Area displays in
cm² with fixed decimals; Boolean displays true/false and replays its kind/value in subsequent
statements. Invalid state/units/text refuse. Area/Boolean source literal syntax stays unchanged.

Nineteen independent consumer verdicts and nine compiled actual assertion reds verify the repair;
existing binding80/twelve reds, full reference/language and publication checks remain green. All
seventeen original worked rows and prior task/oldest ledger payloads retain exact bytes. Book/live/
task pointers agree; D99 closes, D84 signed-angle proof is next. G1 stays5/18, defects11open/87sealed.

## STITCHCAD-G1-0054 - complete scoped numeric boundary review (leaf `G1-SLICE.5a.3b.3b.3c.2`)

D83 closes: reduced128-bit input/results, exact signed scalar/Count domains, once-rounded signed64
bindings and D95 wide literal/unary identities map to61/57/80/146 independent controls. Re-run
12/11/12/12 actual assertion reds with exact source restoration; existing language16 stays green.
D97 corrects the live parent’s canonical-i64 goal; D98 distinguishes checker1/Make0 push verdicts.
Earlier prerequisites/source/task and
oldest ledger payloads stay exact; annex maps each obligation. G1 stays5/18, defects11/86; next
D84 signed inverse-trig/equality. No production evaluator or general transcendental proof inferred.

## STITCHCAD-G1-0053 - second-window CI observed (leaf `G1-SLICE.5a.3b.3b.3c.1v`)

At pushed f876913, doctrine job110782133989 and Rust job110782134441 completed successfully with
all seven/nine steps successful. Archive prerequisite/enforcer and fmt/clippy/tests/real WASM pass.
Post-commit archive controls140 pass; newest committed catalog edit refuses by immutability, rc=1.
Remote/local heads agree; previous ledger/task bytes stay exact. Book/live/pointers reflect observed
proof, not an inferred run status. G1 stays5/18, defects12/83; next .3c.2 D83 complete review.

## STITCHCAD-G1-0052 - retained history frees the next review seal (leaf `G1-SLICE.5a.3b.3b.3c.1`)

Window2 retains63 full source files/122566 bytes at372033f; installed reader independently
reconstructs every byte before retirement. Previous window and127 logical records remain exact;
working catalogs replace raw duplicates, maintained pointers retarget, residue0. New watched CLI
contracts verify all records and newest-window digest/member/catalog/cross-window refusals.
D96 resolves actual index targets instead of filename text:13 independent verdicts/four actual reds.
Oldest payloads stay exact; book/live/tasks agree. Full local Rust/WASM/book/probes/gate precede
required exception push; observed CI .1v next. G1 stays5/18, defects12/83;
D83 review remains .2. No reader, scope or aggregate-cap change; no domain/production signoff.

## STITCHCAD-G1-0051 - canonical literals retain wide exact identity (leaf `G1-SLICE.5a.3b.3b.3b`)

D95 closes:146 independent node/Decimal controls verify128-bit canonical magnitudes, unary identity,
i64 binding distinction, aliases/quanta/scalar bounds and129-bit refusal. Twelve compiled actual
faults produce assertion reds/exact restoration. Existing reference/binding controls pass; no runtime
behavior changes. Book/grammar/decision/live pointers and preserved prior records agree. G1 stays5/18;
defects12 open/82 sealed; D83 full review .3c next. No production normalization/serializer signoff.

## STITCHCAD-G1-0050 - reference bindings store bounded integers (leaf `G1-SLICE.5a.3b.3b.3a`)

Numeric let rounds once, checks declared signed storage/scalar domains and returns the stored integer;
L2 consumes it directly. Exact temporaries remain wider. Binding80 independent controls/twelve
compiled actual reds verify endpoints/ties/context/Boolean/environment/replay. Existing numeric
families retain focused controls and literal6/arith9/angle7/rational12/scalar11/inline5 actual reds.
D95 director ruling: exact128-bit canonical literal nodes, i64 numeric bindings, unary identity kept.
Book/grammar/roadmap/decision and live task records agree; prior task/oldest ledger payloads preserved.
G1 stays5/18, defects13 open/81 sealed; next canonical proof .3b and complete review .3c. No new Rust,
production evaluator, real geometry, MCP or release claim.

## STITCHCAD-G1-0049 - reference scalar domains refuse exact excess (leaf `G1-SLICE.5a.3b.3b.2`)

D83 signed length/area and nonnegative Count domains now consume normative declarations. Rounded
canonical inputs and exact completed results check at their own boundaries; no extra operator
rounding. Scalar57/eleven actual reds and rational61/twelve reds pass; literal6/arith9/angle7/inline5
reds restore exact sources. D94 quiet context loading separates each test family’s assertions.
Book semantics/annex/live records and retained history agree; G1 stays5/18;
defects12 open/81 sealed. Next D83 i64 storage .3 then D84. No new production evaluator/MCP claim.

## STITCHCAD-G1-0048 - inline code language context stays local (leaf `G1-SLICE.5a.3b.3b.1c`)

D91 accepts one explicit Rust span; normative/adjacent formulas remain checked. Context13/five actual
reds/exact restore and language16 pass. D93 annex matches12 rational reds; D34 frontier corrected,
derivation owned. Book/task/history synchronized; checks pass. G1 stays5/18, defects12/80;
next D83 scalar/i64 then D84. No new product or release claim.

## STITCHCAD-G1-0047 - domain errors retain operation context (leaf `G1-SLICE.5a.3b.3b.1b`)

D90 DomainExceeded now carries the actual producing operation across direct/forwarded constructors,
checked arithmetic, Ratio scaling and core bridges. D92 display reports signed values and limits
without inventing a conversion cause or false upper-bound relation; numeric limits are unchanged.
Five public/three private guard contracts and fourteen actual compiled assertion reds verify context
and truthful rendering; six D89 operator reds remain green as discriminators, with exact restoration.
Strict native488 including docs, release5 and all three WASM libraries pass. Book API migration/proof
boundaries and live/task records agree; earlier task evidence and oldest ledgers preserve exact bytes.
G1 stays5/18; sc-units40; defects13 open/78 sealed. Next D91 .3b.1c, then D83 domains/i64 and D84;
D70 axes decision remains pending. No evaluator, MCP, geometry or production-release certification.

## STITCHCAD-G1-0046 - public length operators preserve the domain (leaf `G1-SLICE.5a.3b.3b.1a`)

D89 closes: Length + / - return Result using checked arithmetic, so valid operands cannot construct
an invalid length. Use `(left + right)?` / `(left - right)?`; no silent
clamp or saturation. Four public contracts include signed boundaries, inclusive endpoints, ordinary
values/cancellation, Result typing and i128 pair expectations. Six actual production
bypass/operation/saturation mutations compile and fail assertions, with exact source restoration.
Strict native/release/WASM and focused book/recording checks verify the restored candidate.
D90 missing operation context is owned immediately next; D83 scalar/reference i64 and D84 signed-angle
proof remain owned. D91 language context follows D90; prior evidence/history preserves exact bytes.
G1 remains 5/18; defects14 open/76 sealed; next .5a.3b.3b.1b operation context, D70 decision pending.

## STITCHCAD-G1-0045 - reference rational widths refuse oversized values (leaf `G1-SLICE.5a.3b.3a.2`)

D83 rational width checks now raise typed formula_domain at converted exact input and completed
numeric results, using reduced numerator/denominator widths in result-kind internal units. Input
rounding cannot hide an oversized fraction; unscaled temporaries and untaken computed branches do
not acquire false limits. Sixty-one independent Fraction controls and twelve actual mutations pass;
restored structural/language and earlier six/nine/seven mutation suites are green. D88 fixes a
masked pole-guard test by requiring its mathematical-domain reason, not just an error token.
Exact predecessor angular evidence/journals and oldest live records are preserved. Scalar domains/
i64 and D84 signed-angle verification remain owned; no new Rust/production evaluator claim.
G1 remains 5/18; defects 12 open/75 sealed; next .5a.3b.3b scalar domains, D70 decision pending.

## STITCHCAD-G1-0044 - reference angular conversion and guards agree (leaf `G1-SLICE.5a.3b.3a.1`)

D85/D86/D87 close: microdegrees convert directly to radians at the correct scale, dir rounds before
normalization, and exact odd-quarter tangent poles raise formula_domain. Signed/multi-turn/fractional
sweeps remain intact. Forty-two explicit rows/72 controls agree with an independent standard-library
math oracle on defined curated arguments; seven actual conversion/precision/direction/pole mutations
require assertion reds and exact restoration. Restored structural/input/literal/arithmetic/angle,
language 15/publication nine and recording checks pass. The diagnostic now reports full-turn radius1
arc length6, sin90 ratio1000000, dir80537678 and typed tangent pole refusal.
Normative tan domain and expert annex align; no complete transcendental/production/geometry proof
is claimed. The director resolves D84: formula angles retain sign/turns; entity directions normalize.
The decision and specifications align; D84 binding/equality/inverse-trig proof stays owned by .3c.
D83 rational/scalar repairs continue independently. Prior arithmetic/history payloads preserve predecessor
bytes. G1 remains 5/18; defects 12 open/74 sealed; next .5a.3b.3a.2 rational bounds, D70 also pending.

## STITCHCAD-G1-0043 - exact reference arithmetic retains sub-quantum results (leaf `G1-SLICE.5a.3b.2`)

D82 closes: square/product/quotient and the length-only rational selector return exact reduced
Fractions in result-kind internal units, without rounding before binding. Explicit round_to and
irrational-call rounding remain; dimension signatures, zero division and lazy branches are unchanged.
Twenty-four explicit rows/100 independent Fraction parameter cases supply 162 controls, including
re-association, binding ties and selector model scope. Nine actual precision/scale/refusal/branch/
quantization mutations require assertion reds and exact source restoration. The corrected contracts
also fail the committed predecessor on its rounded 1 um / 2 result, then restore the candidate.
Restored reference/input/literal/structural and language 15/publication nine/recording checks pass.
Expert annex and rounding contract state exact arithmetic and explicit quantization boundaries;
previous literal evidence/oldest ledgers preserve predecessor text. D83 numeric-domain/stored-angle
handling is owned next before product normalization. G1 stays 5/18; defects 11 open/71 sealed.
