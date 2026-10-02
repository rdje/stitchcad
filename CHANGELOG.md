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

| [`changelog-part24.md`](docs/history/stitchcad-changelog-part24.md) | STITCHCAD-G1-0007, STITCHCAD-G1-0006 | 39 lines, 3139 bytes, `sha256:30ff1380…` |

| [`changelog-part25.md`](docs/history/stitchcad-changelog-part25.md) | STITCHCAD-G1-0008 | 17 lines, 1371 bytes, `sha256:d57637be…` |

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

| [`stitchcad-changelog-part26.md`](docs/history/stitchcad-changelog-part26.md) | STITCHCAD-G1-0009 | 16 lines, 1371 bytes, `sha256:ff2d0b1c…` |

| [`stitchcad-changelog-part27.md`](docs/history/stitchcad-changelog-part27.md) | STITCHCAD-G1-0010 | 20 lines, 1663 bytes, `sha256:b3824493…` |

| [`stitchcad-changelog-part28.md`](docs/history/stitchcad-changelog-part28.md) | STITCHCAD-G1-0011 | 20 lines, 1689 bytes, `sha256:6900e83e…` |

| [`stitchcad-changelog-part29.md`](docs/history/stitchcad-changelog-part29.md) | STITCHCAD-G1-0012 | 15 lines, 1306 bytes, `sha256:88277bd5…` |

| [`stitchcad-changelog-part30.md`](docs/history/stitchcad-changelog-part30.md) | STITCHCAD-G1-0013 | 14 lines, 1211 bytes, `sha256:7eb41b35…` |

| [`stitchcad-changelog-part31.md`](docs/history/stitchcad-changelog-part31.md) | STITCHCAD-G1-0014 | 13 lines, 1097 bytes, `sha256:28bbb8fb…` |

| [`stitchcad-changelog-part32.md`](docs/history/stitchcad-changelog-part32.md) | STITCHCAD-G1-0015 | 13 lines, 1096 bytes, `sha256:9e00082b…` |

| [`changelog-part33.md`](docs/history/stitchcad-changelog-part33.md) | STITCHCAD-G1-0016 | 14 lines, 1199 bytes, `sha256:7fc5a9cb…` |

| [`changelog-part34.md`](docs/history/stitchcad-changelog-part34.md) | STITCHCAD-G1-0017 | 13 lines, 1094 bytes, `sha256:a8c9973a…` |

| [`changelog-part35.md`](docs/history/stitchcad-changelog-part35.md) | STITCHCAD-G1-0018 | 13 lines, 1081 bytes, `sha256:74e03b90…` |

| [`changelog-part36.md`](docs/history/stitchcad-changelog-part36.md) | STITCHCAD-G1-0020 / STITCHCAD-G1-0019 | 29 lines, 2350 bytes, `sha256:64af4d6d…` |

| [`changelog-part37.md`](docs/history/stitchcad-changelog-part37.md) | STITCHCAD-G1-0021 | 13 lines, 1082 bytes, `sha256:2feb224c…` |

| [`changelog-part38.md`](docs/history/stitchcad-changelog-part38.md) | STITCHCAD-G1-0022 | 13 lines, 1098 bytes, `sha256:fbe202b2…` |

| [`changelog-part39.md`](docs/history/stitchcad-changelog-part39.md) | STITCHCAD-G1-0023 | 15 lines, 1292 bytes, `sha256:b06df32f…` |

| [`changelog-part40.md`](docs/history/stitchcad-changelog-part40.md) | STITCHCAD-SPINE-0021a | 9 lines, 799 bytes, `sha256:156e9198…` |

| [`changelog-part41.md`](docs/history/stitchcad-changelog-part41.md) | STITCHCAD-G1-0024 | 12 lines, 1073 bytes, `sha256:5f9373df…` |

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

## STITCHCAD-G1-0042 - reference literals preserve canonical integer identity (leaf `G1-SLICE.5a.3b.1`)

D79 closes: bare decimals and all seven unit forms convert once and round once into canonical
integer literals before arithmetic. The census compares the actual node, without rounding its
presentation to hide a fraction. Sixty explicit rows/360 controls agree with an independent Decimal
oracle; six actual quantum/tie/scale/kind/conversion guard reds restore exact reference source.
Structural/input/reference/Fraction controls, language 15/publication nine and recording gates pass.
D80 duplicate units section numbering and D81 stale G1 status routing are corrected; glossary/index
and progressive learning stay intact. Completed rounding evidence/oldest ledgers preserve predecessor
bytes. D82 premature arithmetic rounding and D83 numeric-domain/stored-angle enforcement are owned
next under .5a.3b.2/.3. G1 remains 5/18, defects 12 open/70 sealed; no production numeric/evaluation claim.

## STITCHCAD-G1-0041 - total extreme-magnitude rounding (leaf `G1-SLICE.5a.3a`)

D78 closes: public i128 MIN/1 previously panicked before its checked i64 conversion. Checked unsigned
quotient narrowing and an explicit negative i64 endpoint preserve half-away rounding and return typed
Overflow for wider magnitudes. No clamp, wrap, new precondition or changed UnitError is introduced.

Four public contracts/36 independent Fraction rows and five actual guard assertion reds pass with exact
restoration. The public diagnostic now returns typed errors for all three wide magnitude cases;
controls and both i64 endpoints pass. Restored strict 476 tests, release four contracts and three-library
WASM pass; book/reference/ledger/archive/censuses/staged doctrines verify the recording commit.
Completed AST evidence and oldest live payloads preserve b681a49 bytes. D79 reference literal identity
was independently found/logged and is scheduled next before product canonicalization; its open scope
is stated in the expert annex. G1 stays 5/18; defects 11 open/67 sealed; D70 axes ruling remains pending.

## STITCHCAD-G1-0040 - production expression syntax (leaf `G1-SLICE.5a.2b.2`)

FormulaExpression parses one complete machine expression with precedence, closed literal units,
mandatory conditional branches and exact source gaps/spans. A private flat arena exposes immutable
borrowed views; explicit parser stacks avoid input recursion. Measured node 257/if level 17 refuses
unchanged limits. Unsupported exponents retain formula_unsupported; other grammar refusals are typed.

15 contracts, three privacy/lifetime doctests and eleven actual guard/order mutation reds pass with
exact restoration. Twelve independently enumerated shape/count/depth fixtures agree with the existing
reference. A 20736-input short-token corpus has no internal-structure refusals; a small-stack test
handles 50000 nested parentheses. Restored strict 472 native tests and three-library WASM pass.
Book/reference/ledger/archive/censuses and staged doctrines verify the recording commit. Scope stays
syntax: no conversion, canonical identity, recipe/name/type validation or evaluation. Book/API status
and progressive learning align; completed input evidence and oldest ledgers preserve predecessor bytes.
Next .5a.3 exact literals/canonical ordered recipes; D70 axes ruling remains pending.

## STITCHCAD-G1-0039 - reference machine-input parity (leaf `G1-SLICE.5a.2b.1`)

D76 closes: reference input now enforces ASCII/lower-snake spelling, keyword positions, original
single-space unit separators and nonempty call arguments. General ASCII whitespace stays valid;
assert splitting cannot repair invalid unit gaps. No grammar or product behavior changes.

130 direct controls, three copied-book refusals and nine actual guard assertion reds pass with exact
source restoration. Existing structural 16+2 controls, language 15 probes and 20 Rust name/lexer
contracts pass. Book/ledger/archive/censuses and staged doctrines verify the recording commit.
Completed .2a evidence and oldest lessons/ledger payloads retain exact predecessor bytes. Technical
proof stays in the annex; progressive learning, glossary and index remain intact. Next .2b.2 product
expression trees; no product parsing/evaluation certification. D70 axes ruling remains unanswered.

## STITCHCAD-G1-0038 - reference structural bounds include all function arguments (leaf `G1-SLICE.5a.2a`)

D75's reference walkers omitted call argument lists, falsely accepting 258-node/17-level fixtures.
Complete iterative semantic traversal and early conditional-depth refusal now preserve the unchanged
256-node/16-level limits. Sixteen direct controls/refusals, two copied-book refusals and four actual
guard mutations verify measured sizes and byte-identical restoration. This repairs a book oracle;
product expression parsing/evaluation remain pending, and the lexer behavior is unchanged.

D77's sibling-owner matcher now accepts section fragments while requiring the exact target; nine
coverage probes retain unlinked/wrong-target refusals. Full 25 suites/book/ledger/archive checks pass.
Completed lexical contracts/journal and older
ledger payloads preserve predecessor bytes. Book annex/reference scope, toolbox and live/task records
align. D76 malformed reference identifiers/missing unit separator is owned next by .5a.2b.1 before the
product parser; D70 axes still requires a ruling. G1 remains 5/18; no production-readiness claim.

## STITCHCAD-G1-0037 - borrowed machine-form lexing retains exact source spans (leaf `G1-SLICE.5a.1`)

The new core recipe front-end scans ASCII keywords, identifiers, numbers, operators and punctuation
without cloning source names/numbers or interpreting values. Immutable lexemes retain original text
and half-open byte spans; first error/end fuse the iterator. Shared spelling/keyword classification
preserves MachineToken behavior. Typed errors and lexer Debug do not dump customer source. Lexical
success supplies no valid-expression, numeric, canonical-identity or executable-recipe claim.

Thirteen contracts scan every worked machine example and exercise borrowing, precise refusals and
parser-owned boundaries; two privacy/lifetime doctests pass. Nine actual guard mutations fail contract
assertions, restore exact production bytes, and final strict native/WASM checks pass. D73 obsolete ADR
clause links and D74 vertical-tab handling are fixed. Book learning/status/index and the detailed syntax
annex align with code/roadmap ownership; 48 chapters/15 APIs and nine publication probes pass. Older
publication evidence and sealed histories retain exact predecessor bytes. G1 remains 5/18 top-level;
next .5a.2 expression trees. D70's required axes ruling remains unanswered.

## STITCHCAD-G1-0036 - progressive book and indexed expert annexes (leaf `G1-SLICE.4d.1`)

Five learning chapters introduce recipes, measurements, physical copies, sizes and agent workflows.
Truthful availability separates current libraries from future applications/execution. The glossary
remains reachable, a topic index covers every other registered chapter, and detailed contracts move
into Annexes navigation at preserved URLs/anchors. Roadmap §2 adopts the director's policy, with an
indexed decision; D32's older disposition is preserved exactly while keeping the unchanged baseline.

Publication checks verify 47 chapters, 14 scoped public API/requirement rows, 983 source and 1489
rendered links; eight refusal fixtures plus real-tree green pass. D71 stale G0/Ease status is fixed.
D72 isolates the archive resident probe from growing production history and adds a green control;
28 archive arms and all 24 full suites pass, with glossary/ledger/staged doctrines green. Browser
local-URL policy prevents screenshot inspection; rendered HTML content/navigation is checked.
Completed MTM and oldest ledger/lesson/defect descriptions retain exact bytes. G1 stays 5/18;
next .5a syntax is independent of D70 axes, which remains awaiting the required director ruling.

## STITCHCAD-G1-0035 - canonical MTM body/Ease inputs (leaf `G1-SLICE.4c.3c`)

Immutable custom-member-of-one charts pin canonical Ease-set/table/mapping references. Current body
and signed Ease declarations remain separate borrowed inputs with source/state; garment metadata is
not a generated result. Unknown/derived values refuse numeric fallbacks. Complete validation requires
all current Design garment POMs; selected queries do not certify other mappings, and grading refuses.

Fifteen contracts/two privacy-role docs and twelve actual source mutation reds pass; strict Rust runs
439 tests, with WASM/book and focused censuses/ledger/staged doctrines green. The main book chapter
teaches a waist example and links detailed API/currentness/verification in an annex. Completed chart
collection evidence and oldest history payloads retain exact predecessor bytes. D71 landing status is
logged/owned; next .4d.1 applies the director's incremental book/glossary/index requirement. G1 remains
5/18; D70 axes ruling pending; full SizeSet, geometry, app/MCP and release proofs remain later work.

## STITCHCAD-G1-0034 - exact current chart coverage (leaf `G1-SLICE.4c.3b`)

Immutable garment charts bind explicit POM targets and canonical member observations. Draft validation
checks authored targets without inventing missing cells; completeness checks a nonempty exact target
inventory against the entire current Design table and one observation per member/POM. Reduced targets
cannot hide Design quantities; new Design POMs invalidate prior coverage. Members and row queries retain
explicit authored order. Values/state/source/provenance remain canonical; retargeting requires replacement.

Eighteen contracts/privacy and fourteen actual production mutation reds verify identity/domain/coverage
and unresolved numeric refusal. Restored strict Rust executes 422 tests, WASM/book and focused glossary/
uncertainty/tree/feature/ledger/staged doctrine checks pass. Shared measurements need explicit member
correspondences; full structural coverage does not certify numeric/path/physical/release readiness.
Completed observation evidence retains exact predecessor bytes; rolling windows stay below health targets.
G1 remains 5/18. Next .4c.3c MTM/body; axes D70 ruling remains pending, no representation is defaulted.

## STITCHCAD-G1-0033 - current garment chart observations (leaf `G1-SLICE.4c.3a`)

Immutable authored observations pin set revision/member, named Design/chart tables and two garment
measurement bindings with correspondence provenance. Current queries refuse missing/revised members,
substitute tables, aliases and retargeted metadata; state/source/unit/procedure remain canonical and
borrowed. A logical Design base input may also serve as a chart observation without duplicated scalars;
regenerated geometry measurements and physical equivalence remain separate G3/G4 obligations.

Sixteen contracts/privacy and eight deliberate production mutations verify currentness and borrowing.
Restored strict Rust runs 403 tests; three-crate WASM, warning-free book, glossary/uncertainty/tree,
ledger and staged doctrine gates pass. Completed membership evidence retains exact predecessor bytes;
oldest live changelog/lesson seals preserve their identity. G1 stays 5/18; next .4c.3b chart coverage,
then MTM/body, breaks/composite and review. Axes D70 awaits the director; no representation defaulted.

## STITCHCAD-G1-0032 - authored size membership (leaf `G1-SLICE.4c.1`)

Immutable size membership separates stable member identities, exact human labels, authored order,
explicit system and base member from a pinned SizeSet id/Count revision. Empty/duplicate membership,
blank labels, missing base/lookups and revision overflow refuse. No sorting, label arithmetic,
quantities, chart values or axis defaults. Custom single-member ranges establish membership only.

Twelve contracts plus three privacy/quantity docs pass. Seven real production mutations fail their
regression assertions, including an introduced sort and wrapped revision fallback, then restore exact
source. Strict Rust passes 386 tests; WASM/book/glossary and focused tracking/ledger/staged gates pass.
The SizeSet family is split before implementation; D70 reproduces contradictory axes cardinality and
is owned by .4c.2 with a director ruling requested. Independent membership proceeds; all chart/break/
resolution/review scope remains tracked. G1 stays 5/18, defects 10 open/59 sealed. Completed Ease review
and oldest live records are retained unchanged; full SizeSet and production proof remain pending.

## STITCHCAD-G1-0031 - structural Ease milestone review (leaf `G1-SLICE.4b.3`)

Ontology .2.2 fields map to immutable individual/set APIs and current reference/permission contracts.
The book distinguishes structural proof from evaluation, chart/physical fit, source truth and release.
Current sc-measure tests/docs pass 65; full milestone probes pass all 23 suites; warning-free book and
staged doctrines pass. Product code remains unchanged from b4e0bc7 strict 371-test/WASM verification;
no new remote-CI claim. Set contract/checklist retained unchanged. .4b closes structurally, .4 remains
active for SizeSet .4c and combined review .4d. G1 stays 5/18, defects 9 open/59 sealed.

## STITCHCAD-G1-0030 - current per-POM Ease sets (leaf `G1-SLICE.4b.2`)

Immutable ordered sets bind unique mapping identities, machine tokens and POM identities to named
current body/garment tables. Borrowed canonical mappings retain current fit, provenance/permission and
amount state/source; retargeting or missing identities refuse. Both selected table memberships and
current Ease validate before lookup returns. Shared body/amount sources, one mixed table and empty
drafts are legal; missing POMs never acquire default mappings or zero ease.

Fourteen contracts plus privacy pass. Ten production guard removals each produce an actual assertion
failure and restore exact source. Strict Rust passes 371 tests, with WASM/book/glossary and focused
tracking/ledger/staged gates green. D69 fixes stale package discovery, verified by cargo metadata;
it seals in defects-part13. Existing D34's stale sibling example is corrected; derivation remains
owned. Prior individual Ease contract/checklist retained unchanged; oldest live entries seal unchanged
as changelog-part27/devnotes-part29. Next .4b.3 structural Ease review; G1 stays 5/18.

## STITCHCAD-G1-0029 - individual canonical Ease intent (leaf `G1-SLICE.4b.1`)

sc-measure implements immutable body-to-POM mappings with saved current bindings, a distinct signed
LengthDeclaration, ordered fit intent and explicit compression provenance. Current queries retain
canonical state/source, reject reassigned or missing metadata, and recheck authorization for negative
amounts. Unknown/derived drafts remain inspectable but numeric queries refuse; supplied evaluated
results have an explicit permission check. No quantitative fit thresholds or envelope expansion.

Thirteen contracts plus privacy pass; seven production guard mutations produce real assertion reds
and restore exact source. Strict Rust passes 356 tests; three-crate WASM/book/glossary and focused
tracking/ledger/staged gates pass. Reassignment error snapshots are boxed to satisfy strict lint.
Book examples are assumed inputs, not physical-fit proof. Table evidence moves unchanged to its
sibling; oldest live records seal unchanged as changelog-part26/devnotes-part28. .4b.2 sets and table
membership follow, then .4b.3 review; G1 remains 5/18, physical/evaluation/release proofs deferred.

## STITCHCAD-G1-0028 - named current measurement tables (leaf `G1-SLICE.4a.3`)

sc-measure adds immutable MeasurementTable identity/name and ordered unique measurement-id/token
bindings. Each captures expected metadata id/token/body-POM kind/canonical declaration; current
queries borrow records, refuse missing peers or reassignment, and retain actionable underlying
metadata errors. Same-id source/state/document edits stay visible without numeric caching. Table
names preserve authored content; empty drafts and shared declarations are legal, token uniqueness
is per table. Private representation prevents unchecked mutation. Design revision, source truth,
physical repeatability, formula/evidence policy and release certification remain separate proofs.

Sixteen table contracts plus privacy pass. Eight independent real guard mutations fail their intended
regressions; restored strict checks execute 342 tests, with WASM/book/censuses green. Milestone probes report 23 green suites; the staged doctrine gate passes. All ontology .2.1 length-input fields and table bindings
are mapped to executable APIs; .4a closes structurally, .4 stays active for Ease/SizeSet/signoff. G1
remains 5/18; next .4b per-POM Ease. Completed metadata review moves unchanged to its sibling; oldest
live records seal unchanged as changelog-part25 and devnotes-part27. README/package/book/decision and live pointers agree.

## STITCHCAD-SPINE-0019c - observed archive CI (leaf `SPINE.19.2v`)

For ebed2c5, doctrine run36931196049/job110600555955 and Rust run36931196050/job110600556738
completed success; every step successful, including Python prerequisite/enforcer and Rust/WASM.
Post-commit archive probes 27/0 include committed-window mutation. Doc-only verdict; book/censuses/
staged gate pass. Archive transition closes; next product G1 .4a.3. No new seal or status change.

## STITCHCAD-SPINE-0019b - self-contained bounded history windows (leaf `SPINE.19.2`)

D65: retained all 64 historical files byte for byte in a content-addressed window; complete manifests
and catalog preserve logical paths. Reader list/read/materialize/verify works without historical Git
objects; exact capture reconstruction is independently proved. Decoded and resident storage retain
original aggregate bounds, with finite controls/payload/decompression and immutable committed windows.
Ledger probes consume logical records, all nine arms pass. D68 fixes the coverage mutation's unrelated
false pass. New archive refusals, binary sizing, strict Rust/WASM/book/full probes and staged gates pass;
exceptional push/observed CI follow in .19.2v. Older live records seal unchanged; product remains G1 .4a.3.

## STITCHCAD-G1-0027 - observed metadata CI/signoff (leaf `G1-SLICE.4a.2c`)

Pushed bf29b03; Rust run 36921077740/job 110566989457 and doctrine run 36921077711/job
110566988221 completed success, every step successful. Metadata fields/current refusals match the
ontology; source/physical/release proof stays deferred. Parent .4a.2 closes; G1 stays 5/18, next table
.4a.3. Book/censuses/ledger/staged gates pass; .2b contract/checklist moves unchanged. No new seal.

## STITCHCAD-G1-0026 - canonical measurement metadata and current records (leaf `G1-SLICE.4a.2b`)

sc-measure adds immutable Measurement/Landmark/MeasurementProcedure with body/POM kind, entered unit,
required named/documented records and stable references. Value/state/source borrow core declarations;
current context refuses duplicate identity, missing targets, kind mismatch and reserved-input binding.
Target queries prove their own reference; full validation covers all metadata. Source truth, physical
repeatability, global Design registries and release proof retain later owners. Sixteen contracts plus
three privacy docs pass; six real guard mutations fail. Strict Rust 325 tests, three-crate WASM/book,
full 22 probes and staged gates pass. Runtime CI integration requires immediate push/observed .2c
verdict. D66 fixes README/workspace status and seals in defects-part11; Hem/layer lessons seal unchanged
in devnotes-part25. Completed token task records move unchanged. G1 stays 5/18, next .2c CI/signoff;
archive reaches 64/64 files, so D65 must precede any further required seal. D67 owns the promotion
verifier's unrelated historical-decline weakness; fresh token/metadata retrieval questions are added.

## STITCHCAD-G1-0025 - shared machine tokens retain exact identifiers (leaf `G1-SLICE.4a.2a`)

Core MachineToken validates ASCII lower-snake identifiers and refuses let/assert/if without trimming,
normalization or auto-renaming. Built-in input names remain legal references; their rebinding belongs
to metadata/recipe namespace validation. Six contracts plus privacy cover spelling, lookalikes,
keywords, collection identity and replacement; four guard mutations fail actual assertions. Restored
strict Rust executes 305 tests; WASM/book/focused censuses/ledger/staged gates pass. Grammar, input
chapter and promoted decision agree. Completed .4a.1 contract/evidence moves unchanged to a bounded
measurement sibling; oldest gather lesson seals in devnotes-part24. D66 owns stale README status in
next .4a.2b metadata/runtime integration; .2c observes CI/signoff. G1 remains 5/18.
