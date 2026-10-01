# CHANGELOG.md

Newest first: one section per completed slice, in commit order. Older slices live in sealed, immutable
segments under `docs/history/`, each named below with its identity and retrieval path.

# Sealed archive — earlier slices

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`part1.md`](docs/history/stitchcad-changelog-part1.md) | slices 1–15, `STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004b` | 365 lines, 30452 bytes, `sha256:f4aec75a…` |
| [`part2.md`](docs/history/stitchcad-changelog-part2.md) | slices 16–20, `STITCHCAD-SPINE-0014` … `STITCHCAD-G0-0002` | 152 lines, 12811 bytes, `sha256:5783ac36…` |
| [`part3.md`](docs/history/stitchcad-changelog-part3.md) | slices 21–24, `STITCHCAD-G0-0013` … `STITCHCAD-G0-0018` | 147 lines, 12289 bytes, `sha256:14ad5278…` |
| [`part4.md`](docs/history/stitchcad-changelog-part4.md) | slices 25–29, `STITCHCAD-G0-0004` … `STITCHCAD-SPINE-0017` | 177 lines, 15440 bytes, `sha256:a8cc1de6…` |
| [`part5.md`](docs/history/stitchcad-changelog-part5.md) | slices 30–31, `STITCHCAD-G0-0005` … `STITCHCAD-G0-0013c` | 82 lines, 7505 bytes, `sha256:18548ff7…` |
| [`part6.md`](docs/history/stitchcad-changelog-part6.md) | slices 32–33, `STITCHCAD-G0-0007` … `STITCHCAD-G0-0006` | 93 lines, 8284 bytes, `sha256:3148dd0f…` |
| [`part7.md`](docs/history/stitchcad-changelog-part7.md) | slices 34–35, `STITCHCAD-G0-0013d` … `STITCHCAD-G0-0008` | 105 lines, 9793 bytes, `sha256:ff62d418…` |
| [`part8.md`](docs/history/stitchcad-changelog-part8.md) | slices 41–42, `STITCHCAD-G0-0014` … `STITCHCAD-G0-0004b` | 106 lines, 9766 bytes, `sha256:2c7ee35a…` |
| [`part9.md`](docs/history/stitchcad-changelog-part9.md) | the two oldest live entries, `STITCHCAD-G0-0004c` and `STITCHCAD-SPINE-0004d` — no slice range, because the earlier ranges have no producer (D51) | 90 lines, 8386 bytes, `sha256:8e4081d4…` |
| [`part10.md`](docs/history/stitchcad-changelog-part10.md) | two spine slices on the table convention, `STITCHCAD-SPINE-0020` and `STITCHCAD-SPINE-0015` | 73 lines, 6652 bytes, `sha256:062ccfa3…` |
| [`part11.md`](docs/history/stitchcad-changelog-part11.md) | the delegation-and-uncertainty slice, `STITCHCAD-G0-0014c` | 46 lines, 4507 bytes, `sha256:de34382e…` |
| [`part12.md`](docs/history/stitchcad-changelog-part12.md) | the dialects and formula-language slices, `STITCHCAD-G0-0010` and `STITCHCAD-G0-0009` | 84 lines, 7767 bytes, `sha256:9c61ba7c…` |
| [`part13.md`](docs/history/stitchcad-changelog-part13.md) | the release-contract and canvas-spike-rule slices, `STITCHCAD-G0-0012` and `STITCHCAD-G0-0011` | 75 lines, 7188 bytes, `sha256:1e52b5c9…` |
| [`part14.md`](docs/history/stitchcad-changelog-part14.md) | the i18n slice, `STITCHCAD-G0-0016` | 38 lines, 3631 bytes, `sha256:2c895780…` |
| [`part15.md`](docs/history/stitchcad-changelog-part15.md) | the recurring cleanup slice, `STITCHCAD-SPINE-0021` | 23 lines, 2123 bytes, `sha256:f93154e3…` |
| [`part16.md`](docs/history/stitchcad-changelog-part16.md) | the closed-defect sealing slice, `STITCHCAD-SPINE-0019a` | 27 lines, 2450 bytes, `sha256:211f9bec…` |
| [`part17.md`](docs/history/stitchcad-changelog-part17.md) | the command-layer contract, `STITCHCAD-G0-0017` | 38 lines, 3626 bytes, `sha256:819f240a…` |
| [`part18.md`](docs/history/stitchcad-changelog-part18.md) | the G0 exit review, `STITCHCAD-G0-0015` | 38 lines, 3694 bytes, `sha256:22d63ec4…` |

**Correction (D30).** part1's own descriptor says its coverage runs "through `STITCHCAD-SPINE-0004c`".
It does not: part1's newest entry is `STITCHCAD-SPINE-0004b`, and `SPINE-0004c` is sealed in part2.
Sealed segments are immutable, so the correction is recorded here and in part2's descriptor rather than
by editing part1.

The bedrock scaffold's own changelog — the provenance of this repository's discipline spine — is sealed
in [`docs/history/bedrock-scaffold-changelog.md`](docs/history/bedrock-scaffold-changelog.md).
| [`changelog-part19.md`](docs/history/stitchcad-changelog-part19.md) | STITCHCAD-G1-0001 | 21 lines, 1770 bytes, `sha256:2f602e9a…` |
| [`changelog-part20.md`](docs/history/stitchcad-changelog-part20.md) | STITCHCAD-G1-0002 | 19 lines, 1598 bytes, `sha256:a3918baa…` |
| [`changelog-part21.md`](docs/history/stitchcad-changelog-part21.md) | STITCHCAD-G1-0003 | 21 lines, 1831 bytes, `sha256:ab5e04ca…` |

| [`changelog-part22.md`](docs/history/stitchcad-changelog-part22.md) | STITCHCAD-G1-0004 | 24 lines, 2100 bytes, `sha256:9050689c…` |

| [`changelog-part23.md`](docs/history/stitchcad-changelog-part23.md) | STITCHCAD-G1-0005 | 48 lines, 4079 bytes, `sha256:e74210d6…` |

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

## STITCHCAD-SPINE-0021a - recurring artifact cleanup preserves the product frontier (leaf `SPINE.21a`)

Before the next product slice crosses the 24-hour mark, removed six ignored scratch/incremental/book
roots and 255 safe stray artifacts. Target went 663084 → 328736 KB; book removal adds 4832 KB,
for 339180 KB reclaimed. Independent residue census found zero remaining artifacts and no tracked
deletion; release/deps bin/log scans were zero. Shared stores, other repositories and built dependency
outputs remain untouched. Strict Rust's 287 tests, WASM, book, all 22 probe suites and staged gates
pass after regeneration. Prior cleanup checklist moves unchanged, and oldest changelog entry seals
to part23. Latest-run record and bounded book upkeep align; G1 .4 remains the product frontier.

## STITCHCAD-G1-0023 - all four structural ontology families pass milestone review (leaf `G1-SLICE.3c.4d.2`)

Review accounts for sixteen ontology §4 objects with immutable content and current reference evidence.
Four families close structurally; geometry, construction execution, profile values, Design registries
and production-release review retain their later owners. Fourteen object/support suites execute 149
contracts; full strict Rust executes 287 tests including 19 doc-tests, with WASM/book and all 22 probe
suites green. G1 becomes 5/18 leaves; measurement/ease/size modelling is next.

Two signoff defects close: D62 attributed the original workspace's 30 tests to sc-units, whose count
is 26 regular + 1 doc. D63's negative feature probe matched obsolete prose and changed no fixture;
it now targets the gate cell by feature identity and independently proves the mutation. Missing/
duplicate targets refuse setup; a no-op writer mutation makes the suite red. Census predicates stay
unchanged. Defects seal in part9; 8 open / 54 sealed. Pocket/button evidence relocates unchanged.
Director reaffirmed SOTA/signoff/production-grade and comprehensive external-agent MCP/API control;
book and .6/.9/G5 acceptance retain discoverability, recovery, parity and independent evaluation.

## STITCHCAD-G1-0022 - Pocket retains physical composition and owned placement intent (leaf `G1-SLICE.3c.4d.1`)

Immutable Pocket binds served/component physical copies to explicit source Pieces, owned position
and directed orientation, and a required logical opening. Nonempty unique components, unambiguous
current Piece contexts and copy/source guards refuse silent substitution. Borrowed component metadata
is canonical; its contour repairs remain separate Design/G2 obligations. Opening resolution and
supported execution remain G3, without invented vocabulary/defaults or a scope approval.

Eleven contracts + privacy pass. Independent copy-source, orientation-ownership and nonempty-list
mutations fail red; restored strict Rust/WASM/book, fixture/feature/glossary/tree, ledger and staged
gates pass. Book/live records and pre-code decision align. Oldest committed changelog/lesson seal
unchanged to part22/part21. Director reaffirmed the SOTA/signoff/production-grade bar; object-family
signoff now re-derives the structural evidence. G1 remains 4/18 leaves, 3/4 families; next `.3c.4d.2`.

## STITCHCAD-G1-0021 - buttonhole length has one canonical button/operation source (leaf `G1-SLICE.3c.4c.2`)

ButtonAndButtonhole joins the distinct Closure kinds and existing stable instance/count/target
rules. Required button-size binding and recipe operation provide one borrowed canonical hole-length
source, without a separately authored or cached length. Recipe/Design must validate the typed
operation dependency; G3 executes it. DeferredToG3 remains distinct from geometry/profile checks.

Fifteen Closure contracts pass, including five new button tests. Substituting button-size or operation
source independently makes a regression red; restored strict Rust, wasm, warning-free book,
fixture/feature/glossary/tree censuses, ledger and staged gates pass. Compile-fail coverage refuses
a second length field. Completed closure evidence and task changelog relocate unchanged with
committed-content oracles. Book/live records align; no physical formula/default is invented.
Closure parent closes structurally; G1 stays 4/18 leaves, 3/4 families. Next `.3c.4d` pockets/signoff.

## STITCHCAD-G1-0020 - zipper/hook-bar instances borrow current placements and refuse fly scope (leaf `G1-SLICE.3c.4c.1b`)

Immutable Closure retains distinct centred-zipper/hook-bar intent, required size origins and stable
physical instances. Typed Count derives from nonempty instances; duplicate ids/placement reuse,
ambiguous contexts, missing targets and current repairs are refused. Borrowed placements remain
canonical. Zipper length is explicitly positive or symbolic, hardware sizes remain logical bindings.
Fly requests refuse env_fly with requested Closure, trousers gap and G7 before geometry validation.

Ten contracts, Count-domain unit and privacy pass. Scope/reuse/length/current-target mutations each
fail red; restored strict Rust, wasm, warning-free book, fixture/feature/glossary/tree censuses,
ledger and staged gates pass. Nested target evidence is boxed for the strict error-size lint.
Placement evidence and commit history relocate unchanged with independent committed-content oracles
and retrieval pointers. Book/live records align; no hardware geometry is claimed. G1 stays 4/18
leaves, 3/4 families; next `.3c.4c.2` button/buttonhole derivation.

## STITCHCAD-G1-0019 - physical notion placements preserve stable copy bindings (leaf `G1-SLICE.3c.4c.1a`)

Immutable NotionPlacement retains stable id/copy, source-frame anchor/orientation and original
Piece binding. Current validation refuses missing/reassigned copies, lost or foreign anchors and
incomplete orientation intervals. It accepts uniquely resolved historical anchors while preserving
current choices/repairs. The shared current-anchor validator keeps original birth errors and
Notch/TurnPoint behavior. Reflection remains separate G2/V1 geometry, without copied coordinates.

Ten contracts + privacy pass; copy binding, range ownership, historical resolution and current-anchor
ownership mutations each fail red. Restored strict Rust, wasm, warning-free book, fixture/feature/
glossary/tree censuses, ledger and staged gates pass. A bounded closure chapter documents the API;
closure kinds/counts/sizes still follow. Completed Hem evidence and verification table move unchanged
with committed-payload oracles and staged revalidation. G1 stays 4/18 top-level leaves, 3/4 families;
next `.3c.4c.1b` zipper/hook-bar/fly, then `.4c.2` button/buttonhole derivation.

## STITCHCAD-G1-0018 - Hem retains depth/fold intent and validates current Facing targets (leaf `G1-SLICE.3c.4b.2`)

Immutable Hem retains an owned finish edge, explicit/formula/profile depth, required fold-type
binding and turned/faced method. Faced composition borrows its original current Facing after
checking identity, served Piece and current layer sources. Removed/replaced/reassigned targets and
lost interiors are typed refusals, even with live endpoints. No layer data or solved state is copied;
physical folding remains G2/G3 and declaration/profile validation remains `.5`/`.6`/G4.

Ten contracts + privacy pass. Identity, current-source and ownership mutations each fail red;
restored strict Rust, wasm, warning-free book, fixture/feature/glossary/tree censuses, ledger and
staged gates pass. Gather/layer checklists relocate unchanged with an independent committed-content
oracle and staged revalidation. Book examples distinguish hem fold binding from allowance corners.
Hem/layer parent closes; G1 stays 4/18 top-level leaves, 3/4 families. Next `.3c.4c` closures.

## STITCHCAD-G1-0017 - served layers retain recipe and material intent with explicit lining scope (leaf `G1-SLICE.3c.4b.1`)

Immutable Facing/Lining/Interfacing types retain served Piece, recipe offset operation and directed
sources, plus material assignment. Complete owned sources and unique endpoints are required;
blank unresolved-material reasons, wrong owners and duplicate sources are typed refusals. Recipe
operations own dimensions; G1 generates no contour. Lining can be inspected but execution refuses
`env_lining` with served Piece and proving gate G7; `.6` must enforce this boundary before execution.

Nine contracts and three privacy checks pass. Disabling scope and ownership refusals independently
makes their regressions red; restored strict Rust, wasm, warning-free book, fixture/feature/glossary/
tree censuses, ledger and staged gates pass. Piece's extracted shared material invariant preserves
its original contracts. The fold checklist moves unchanged to the evidence sibling; docs and
frontier align. G1 remains 4/18 top-level leaves, 3/4 families; next `.3c.4b.2` Hem.

## STITCHCAD-G1-0016 - gathers bind physical span sides and borrow canonical ease (leaf `G1-SLICE.3c.4a.2b`)

Immutable Gather binds graph/span/side/physical-copy ids, direction and closing operation. Attachment
and signed intake/allocation come from the canonical sewing span; no second authored distribution or
cached parameter state exists. Explicit ease sign must fit the selected gathered side. Current target
queries expose missing/reassigned copies and changed span-side bindings without transferring intent.
Owned whole intervals and unique endpoints are structural checks; executed conservation remains G2/G3.

Eleven contracts + privacy pass. Disabling copy-binding and interval-ownership refusals separately
makes their regressions red; restored strict Rust, wasm, warning-free book, fixture/feature/glossary/tree
censuses, ledger and staged gates pass. Three recent mark/dart checklists move unchanged to the evidence
sibling; every staged checklist is revalidated. Oldest dev-note lesson seals to part15. No goldens change.
Intake parents close: all four kinds have structural APIs. G1 remains 4/18 top-level leaves, 3/4 object
families. Next `.3c.4b` implements hem/layer descriptors.

## STITCHCAD-G1-0015 - distinct tucks and pleats retain owned fold intent (leaf `G1-SLICE.3c.4a.2a`)

Separate immutable Tuck/Pleat types share structural validation of intake provenance, nonempty
directed fold ranges, explicit direction and closing-operation identity. Negative explicit intake,
empty/duplicate held intervals and unresolved/foreign ranges are typed refusals. Symbols supply no
values/defaults; physical fold shape, count rules and conserved intake remain G2/G3 obligations.

Nine contracts exercise both types and two privacy doctests pass. Disabling ownership refusal makes
the foreign-middle merge regression red; restored strict Rust, wasm, warning-free book, fixture,
feature/glossary/tree censuses, ledger and staged doctrines pass. Directed queries retain reversals,
fragment order and interior repairs without mutation. Construction book examples and live records
stay aligned. Oldest CHANGELOG/DEV_NOTES entries seal to part18/part14 before their health targets.
G1 stays 4/18 top-level leaves, 3/4 object families. Next `.3c.4a.2b` links gather intent to sewing spans.

## STITCHCAD-G1-0014 - semantic darts retain intake and closing-operation intent (leaf `G1-SLICE.3c.4a.1`)

Dart is immutable structural content: intake origin, apex on owned internal construction geometry,
two directed legs, explicit direction reference and closing-operation identity. Born references need
owned full intervals and unique endpoints; identical held legs, negative authored intake and invalid
apices are typed refusals. Symbols provide no values/defaults. Physical coincidence and executed
intake conservation remain explicit G2/G3 obligations; registries must validate parameters/operations.

Nine contracts + privacy pass. Ownership refusal disabled makes the merged foreign-middle regression
red; restored strict Rust, wasm, warning-free book, fixture/feature/glossary/tree censuses, ledger and
staged doctrines pass. Construction objects now have safe owned child slices; a bounded construction
book companion preserves examples and physical limits. No goldens change. G1 remains 4/18 top-level
leaves, 3/4 object families; next `.3c.4a.2` implements tuck/pleat/gather intent.

## STITCHCAD-G1-0013 - per-edge allowances retain width origin and symbolic target policy (leaf `G1-SLICE.3c.3c`)

Immutable SeamAllowance descriptors retain whole owned source edges, authored width parameter/value
or formula/profile declaration, explicit five-way corner intent and mandatory logical profile inclusion.
Born references require full interval coverage and unique endpoints; symbolic widths/policies supply no
values or defaults. Explicit zero is authored content; negative explicit width is a typed refusal.
Offsets, error budgets and target policy resolution remain visibly G2/G4 obligations.

Eight contracts + privacy doctest pass. Disabling ownership refusal makes the foreign-middle merge
regression red despite live owned endpoints; restored strict Rust, wasm, warning-free book, fixture,
feature/glossary censuses, ledger probes and staged gates pass. Earlier object checklists move unchanged
to the existing evidence sibling before the parent reaches 1000 lines; every staged checklist passes.
The oldest dev-note lesson seals to part13. No fixture golden changes. Marks/allowances closes;
G1 remains 4/18 top-level leaves and advances to 3/4 object families. Next `.3c.4` constructions.

## STITCHCAD-G1-0012 - directed grainlines retain independent print references (leaf `G1-SLICE.3c.3b`)

Immutable Grainline content holds a directed arrow, explicit parallel/angle intent and independent
optional stripe/plaid ranges. All born references require complete owned intervals and unique endpoints.
Directed queries preserve raw evidence, reverse fragment order for reverse traversal and compose journal
direction; repairs stay visible in traversal order. Explicit 45°/180° and symbolic angle declarations
supply intent without geometry proof or default values. G2/G4 retain physical/value validation.

Nine grain contracts and privacy doctest pass. Disabling reverse fragment order makes the independent
regression red; restored strict Rust, wasm, warning-free book and feature/glossary censuses pass.
D61 closes with unchanged census predicates and ten green/red glossary probes. Existing sewing/notch
suites pass after sharing whole-interval ownership. Ledger probes and staged doctrines pass.
Ontology §10 becomes a bounded index; its 231-line executable body moves unchanged to the linked
implementation chapter, which gains grain examples. Oldest live changelog/dev-note entries seal.
Next `.3c.3c` implements allowance descriptors; G1 remains 4/18 top-level leaves, 2/4 object families.

## STITCHCAD-G1-0011 - sewing spans address copies and permit disjoint self-seams (leaf `G1-SLICE.3c.2b.2`)

Immutable `SewingGraph`/`SeamSpan` content now names physical CutCopy identities and exact partial
ranges, with explicit endpoint correspondence, signed ease source/distribution and semantic stops.
Construction validates the complete cut plan, owned whole intervals and unique endpoints; stops
resolve to Notches or born-valid TurnPoints on their side. Symbolic amounts supply no defaults.
Geometry, walking/realized ease and profile/recipe value resolution remain explicit later obligations.

D35 closes: disjoint same-copy ranges may sew together, touching endpoints are legal, and overlapping
material intervals are refused after current-frame resolution. D57 closes: two copies of one Piece
have different neighbours, and removed targets remain missing rather than transferring their seams.
The fixture records the rule while retaining its existing edge-finish procedure until G2 constructs
actual folded-end ranges. Neither the five physical cuts nor its arithmetic goldens change.

Eighteen sewing contract tests + the graph privacy doctest pass; disabling either self-overlap or
whole-interval ownership refusal makes its regression red. Strict `make check`, wasm, warning-free
book, fixture/feature/glossary censuses, ledger probes and staged doctrines pass. The copy milestone's
full `make probes` passed all 22 suites. Shared anchor validation retains notch behavior and the
NotchError alias; original notch tests pass. Closed D35/D57 seal together in defects-part7.
Oldest CHANGELOG/DEV_NOTES entries roll over atomically. Next `.3c.3b` implements directed grainlines.

## STITCHCAD-G1-0010 - physical cut copies have explicit stable identities (leaf `G1-SLICE.3c.2b.1`)

Per the director's D57 ruling, `CutPlan` carries immutable `CutCopy` identities separately from the
pattern Piece. Callers supply copy ids, Piece ids and authored/reflected orientation. Validation checks
unique/disjoint identities, existing Pieces, exact quantities and equal mirrored-pair populations;
Single and separate L/R members retain authored orientation. List order supplies no identity and a
removed copy never transfers its id to a replacement. Geometry transforms remain deferred to G2/V1.

Nine contract tests and the privacy doctest pass; disabling quantity refusal makes its regression red.
Strict `make check`, wasm, warning-free book, feature/glossary/tree censuses, ledger probes and doctrine
gate pass. D59 is fixed: the glossary census reproduced 15 undeclared API terms against `6abfac3`;
a meaningful local API table plus the two new glossary concepts brings the census to zero failures.
The new terms are indexed from the producer. D57 stays open until the sewing graph exercises copy ids.

Completed `.1`, `.2`, `.3a` and `.3b` checklists move to `G1-SLICE-evidence.md` before
this append would take the parent past 1000 lines. Coverage confirms a fourth legitimate sibling. D60 closes after the staged gate exposes the
historical doc-only ROOT CAUSE bullets: `.1`/`.2` retain their prose plus re-derived delivery evidence,
and all four moved checklists are audited separately.
The copy-identity decision records replacement/orientation rules and the required token census.
Next `.3c.2b.2` lands sewing spans and settles D35 explicitly.

## STITCHCAD-G1-0009 - semantic notch anchors with symbolic profile bindings (leaf `G1-SLICE.3c.3a`)

`Notch` is immutable semantic content: a born-live, uniquely resolved point on a surviving interval
of its named Piece, plus logical target-profile declarations for style, sample/production depth and
width, and encoding. Construction refuses absent/foreign anchors, including a merged edge's foreign
remainder. Edits expose split choices, exact recomputation and repair tasks without rewriting anchors.
Bindings always report `DeferredToG4`; no physical values, profile pin or defaults are supplied.

Seven contract tests and the privacy doctest pass. Disabling ownership refusal makes the partial-merge
regression fail. Strict `make check`, wasm, warning-free book, feature/release censuses, ledger probes
and doctrine gate pass. The book documents examples and the unimplemented physical-export boundary.
D58 closes: G4's contradictory default-plus-sidecar acceptance now matches release §8's no-default
matrix; G4 still owns enforcement. The symbolic-binding decision is recorded and promoted.
The oldest changelog and dev-note entries roll into sealed segments in the same commit.
The director answered D57 before commit: each physical cut copy has a stable identity so seams can
differ. Next `.3c.2b` implements that contract; D57 closes after verified delivery.

## STITCHCAD-G1-0008 - separate cut-once L/R members are explicit piece content (leaf `G1-SLICE.3c.1a`)

Fixed D56: the canonical skirt's separate left/right back members, each cut once, were not representable
by the even-total mirrored-pair mode. `Mirroring::PairMember { handedness, companion }` now carries the
member's own L/R label and distinct companion Piece identity; quantity counts this member's copies.
Self-companions are refused. Existing even-total pair requests retain their original meaning.
The book documents both forms; collection-level reciprocity and equal quantities are owned by `.6`,
and geometric mirroring remains G2's obligation.

Validation: 14 piece-contract tests (including the canonical fixture-shaped pair and self-companion
refusal), strict `make check`, `make wasm`, `make book`, reference-fixture derivation, feature census,
doctrine gate and ledger probes green. Applying the even-total rule to all pair modes makes the fixture
regression fail. D56 closes in defects-part3; the decision records the two pairing forms.

D57 is logged and owned by `.3c.2b`: the spec does not decide how a sewing side names physical cut copies.
The director was asked to choose stable copy identities or pattern-level references with later expansion.
No answer is inferred; independent marks/allowances (`.3c.3`) proceed while that decision is pending.

## STITCHCAD-G1-0007 - whole-interval reference resolution keeps lost interiors visible (leaf `G1-SLICE.3c.2a`)

`IdentityLedger::resolve_range(EdgeRange)` now folds the complete positive-length interval through
split/merge/reverse/delete/offset. `RangeResolution` preserves ordered directed live fragments and
`RangeRepairTask`s for deleted or trimmed portions, missing source edges and exact-arithmetic refusal.
Endpoint point queries stay separate, so a full-coverage interval can still carry a boundary choice.
No query rewrites the stored reference or claims geometric validity or approval. Piece full-edge
queries consume this contract; the point-only registration verdict keeps its original scope.

D55 is fixed: deleting a middle fragment produces a visible range repair while both endpoints resolve.
The regression test goes red if the range-delete arm is removed. Thirteen range tests cover exact
partial bounds, traversal order, narrow gaps, arithmetic refusal and a recorded-seed differential
comparison with the existing point resolver. `make check`, `make wasm`, `make book`, doctrine gate,
feature-matrix census and ledger probes pass. The pre-code decision records the coverage/point/geometry
boundary; next `.3c.2b` implements sewing spans and resolves D35.

The same append rolls CHANGELOG's oldest entry into `part14` and DEV_NOTES' oldest two lessons into
`devnotes-part8`; closed D55 moves to immutable `defects-part2`. Their content identities are re-derived
by the ledger probes. The new Knowledge Map record fits after its interchange orientation entry is
tightened; D53's durable generator remedy remains separately owned.

## STITCHCAD-G1-0006 - immutable structural pieces, with geometry visibly deferred (leaf `G1-SLICE.3c.1`)

`sc_core::ontology::piece` now builds immutable `Piece` objects from editable `PieceDefinition` input.
Directed cyclic cut loops must be nonempty, distinct and live in the identity ledger; construction lines
must also exist. Cut quantities, mirrored pairs, fold-edge declarations, material explanations and complete
print text are checked with typed `PieceError` diagnostics. Quantity, pair and fold print fields derive
from the cut plan. Every piece reports `GeometricValidation::DeferredToG2`; no winding, simplicity,
containment or geometric-closure claim is made.

The object-type leaf `.3c` now has four independently committed children. Piece endpoint queries expose
repair tasks after edits without rewriting authored content. A tracked counterexample proves endpoints
cannot certify an entire fragmented edge (D55); the next child `.3c.2` owns full-range resolution before
sewing spans use it. Ontology §10 documents the implemented API and its limits.

Validation: `make check` (fmt, strict clippy, unit/property suites and private-content compile-fail test),
`cargo test -p sc-core --test piece_contract` (12 contract tests), `make wasm`, `make book`, doctrine gate
and changelog-ledger probes, all green. Startup compared the neutral README, claim-verification and
containment policy bodies with their read-only sources: no differences. Cleanup remained within 24 hours.
