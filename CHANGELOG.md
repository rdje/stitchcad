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

## STITCHCAD-G1-0024 - canonical length inputs preserve authored state (leaf `G1-SLICE.4a.1`)

Immutable core declarations retain source plus known/assumed/unknown/preference/derived state and
required provenance references. Known inventory refuses empty/duplicates; unknown and derived inputs
have no numeric field or fallback, naming the observation or formula they need. Signed values and
explicit zero remain exact. Evidence existence/scope/truth and evaluated state remain Design/recipe/G4.
Eight contracts and three compile-fail examples pass; four independent guard/fallback mutations fail
with actual assertions. Strict Rust executes 298 tests; WASM/book/focused censuses/ledger/gates green.
D64 corrects stale ontology coverage, sealed in defects-part10. Completed construction contracts/ten
checklists partition unchanged; two oldest lessons seal unchanged in devnotes-part23. G1 remains
5/18, next .4a.2 measurement metadata. D65 owns retention at 61/64 archive files before the limit blocks
required seals; the product frontier stays active until that trigger.

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
