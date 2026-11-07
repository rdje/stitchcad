# G3-GRADING: construction & grading (roadmap gate G3)

## Metadata

- Tree ID: `G3-GRADING`
- Status: `active`
- Roadmap lane: `ROADMAP.md` §11 gate **G3 — Construction & grading** (sources: §3.1 darts /
  walking / truing, §3.3 two instantiation paths, §3.4 size systems, §4.1 identity contract,
  §15.12 notch types, ADR-0004 grading modes)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

The domain jump from a skirt to a **darted bodice with a set-in sleeve** is made honestly — cap
ease declared, not tolerated as an invariant violation — and **both** instantiation paths exist and
are separately validated: measurement-driven regeneration and grade-rule instantiation with `.rul`
interchange that an independent engine can re-import, extreme sizes included.

## Entry criteria

- `G2-2D` closed: the geometry kernel, offset engine, canonicalizer and golden harness exist.
- `G0-CONTRACT.5` (both paths specified) and `.6` (SizeSet ownership) recorded.

## Non-Goals

- Factory Profiles, the artifact policy matrix and the Profile Editor (G4).
- 3D assembly, meshing and drape (V1/V2) — although the SewingGraph this tree completes is the
  prerequisite V1 consumes.
- Claiming that graded sizes are exactly reconstructible from base + rules: roadmap §3.3 records
  the information loss, and this tree's job is to bound it, not to deny it.

## Acceptance Criteria (gate G3 exit, clause by clause)

| Roadmap G3 exit clause | Leaf |
| --- | --- |
| bodice + set-in sleeve with declared ease (cap ease intentional) | `.4` |
| darts, folds, walking/truing operations | `.1`, `.2` |
| stable references survive edit/split/mirror (or visible repair tasks) | `.3` |
| both instantiation paths | `.6`, `.7` |
| `.rul` with grade-point identifiers re-imported into an independent engine | `.8` |
| extreme sizes reconstructed and measured | `.9` |
| grading modes (base+rules / embedded / all-contours) validated separately | `.10` |
| offset + grading golden suites green | `.12` |
| **envelope coverage** — every §3.2 garment drafts, grades and exports (roadmap v0.3) | `.4` (bodice, sleeve), `.5` (trousers + pocket + derived buttonhole), `.15` (classic collar) |

## Task Tree

- ID: `G3-GRADING`
  Status: `in_progress`
  Goal: construction operations and both instantiation paths work, and their divergence is measured.
  Children: `.1` … `.16`

- ID: `G3-GRADING.1`
  Status: `done`
  Goal: closure semantics — Dart / Tuck / Pleat / Gather as objects with intake, direction and
  apex, plus slash-and-spread as a drafting operation on the recipe (§3.1).
  Acceptance: a dart's intake is queryable and conserved across the operation that closes it;
  slash-and-spread is replayable from the recipe (not a baked geometry edit); unsupported closures
  produce the diagnostics named in the `G0-CONTRACT.4` feature matrix.
  Paths:crates/sc-artifacts/**,crates/sc-cli/**,crates/sc-api/src/lib.rs,crates/sc-viewer/src/shell.rs,crates/sc-mcp/tests/agent_gate.rs,conformance/reference-skirt/**,conformance/goldens/reference-skirt/**,Cargo.lock,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-02 asks for darts and folds as operations. WHERE a dart was two internal lines,and the core's CloseDart returns HandlerUnavailable because the core holds topology and no coordinates;git grep -c closure_reports 6bf8cfcd -- crates →0,rc=1.
  - [x] ADDRESSED:cargo test -p sc-cli --test closures exit status →rc=0:13 tests. A dart's intake is measured from the drawn line,a disagreeing declaration is refused by name,and turning the far leg about the apex lands it on the near one. Intake holds at each width tried. Slash and spread is redone from the recipe,and what lay in the turned part moves with it.
  - [x] THE SKIRT:its darts are now dart objects. The DXF golden moved no byte and the canonical form gained only the closures,so the two agent sessions still golden-match with closures set aside. Their recordings predate closures.
  - [x] THE GAP,STATED:the drafting layer executes these,and sc-core's Dart,Tuck,Pleat and Gather are structural and stay unconnected,so there are two models of a closure until a later leaf binds them. The chapter says so. No garment uses a tuck,pleat or gather yet.
  - [x] NO REGRESSION:gauge.sh --checks →rc=0;cargo test -p sc-artifacts -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1012.

- ID: `G3-GRADING.2`
  Status: `done`
  Goal: `walk` and `true` as first-class editing operations on the SewingGraph (§3.1) — a sleeve cap
  walked around an armscye, balance notches planted, seam lengths trued with the ease declared.
  Acceptance: walking reports the length differential per span and attributes it to declared ease or
  to a defect; truing never silently changes a piece's semantics; both operations are bus commands
  with undo.
  Paths:crates/sc-core/**,crates/sc-measure/tests/**,crates/sc-artifacts/**,crates/sc-cli/**,crates/sc-api/src/lib.rs,crates/sc-viewer/src/shell.rs,crates/sc-mcp/tests/agent_gate.rs,conformance/reference-skirt/**,conformance/goldens/reference-skirt/**,Cargo.lock,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-02 asks for walk and true as operations. WHERE the sewing graph existed only in the judge's fixture table,and the drafting description sat outside the Design,so no edit to it had a revision or an undo;git grep -c ChangeDrafting 4f4cd2b5 -- crates →0,rc=1.
  - [x] ADDRESSED,WALK:cargo test -p sc-cli --test walk exit status →rc=0:13 tests. A difference is declared_ease only when somebody declared it,else defect with the amount. A dart on a seam is sewn away first. All seven of the reference skirt's seams walk to a match.
  - [x] ADDRESSED,BUS:the drafting document is now part of the Design,and ChangeDrafting is a real bus command,with a revision,an actor,an audit entry and an undo from the bus's snapshots. A caller without commit is refused by the bus. cargo test -p sc-core -p sc-measure →rc=0:every existing core and bus suite holds with the new field and command.
  - [x] TRUING:moves a straight end only,within a limit the caller must state,leaves declared ease,reports before and after,and is refused past its limit with nothing changed. It is an operation in the document,so a changed measurement is trued again when drawn.
  - [x] NOT SHOWN,STATED:no garment has a real eased seam yet,which is `.4`. A side names its ends by position,which `.3` replaces. A balance notch does not account for a closure before it. Nobody who sews has reviewed any of it.
  - [x] NO REGRESSION:gauge.sh --checks →rc=0;cargo test -p sc-artifacts -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1013.

- ID: `G3-GRADING.3`
  Status: `done`
  Goal: the identity contract under edit (§4.1) — referenced-edge split, merge, reverse, delete and
  offset-fragmentation either preserve references or produce a visible unresolved-reference repair
  task; no silent reassignment.
  Acceptance: a property test enumerates the edit operations and asserts reference survival or a
  repair task for each; notches, grade points and seam spans are the reference carriers tested.
  Paths:crates/sc-geometry/src/curve.rs,crates/sc-artifacts/**,crates/sc-cli/**,crates/sc-api/src/lib.rs,crates/sc-viewer/src/shell.rs,conformance/reference-skirt/session.json,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-03 asks that references survive an edit. WHERE the core's journal had the whole contract and nothing drawn used it:a drafted notch or seam end named a position in crates/sc-cli/src/draft.rs;git grep -c IdentityLedger 9e231ddd -- crates/sc-cli/src/draft.rs →0,rc=1.
  - [x] ADDRESSED,DRAFTING:every drafted edge is declared in the core's IdentityLedger,an edit changes the journal and the drawing together,and where a reference lands is the fold's answer. cargo test -p sc-cli --test references →rc=0:split,merge,reverse,delete,trim,mirror and measurement each against notches,grade points and seam ends,with stated seeds.
  - [x] ADDRESSED,RELEASE:a repair task is shown by name,with the edit and candidates,and a DXF or PDF is refused while one is open;the canonical form lists them. A deleted edge's stand-in is a new identity nothing lands on.
  - [x] ADDRESSED,TRIM:an allowance trimming part of an edge is the contract's fifth row,using the ledger's offset and the engine's own cut line,so a notch there is a task and not moved to where the cut line happens to be.
  - [x] ALSO FOUND:a reference holds a fraction and a true moves corners,so a true on a reference mid-edge or on a mirror would move nothing and say it had;refused by name,D404,before it shipped. Positions in older sessions keep working and are stated not to be references.
  - [x] NOT SHOWN,STATED:a closure's ends are still positions. Grade rules are `.7`. Nobody who sews has reviewed a notch's place. The core's structural Piece stays unbound.
  - [x] NO REGRESSION:gauge.sh --checks →rc=0;cargo test -p sc-geometry -p sc-artifacts -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1014.

- ID: `G3-GRADING.4`
  Status: `done`
  Goal: the bodice + set-in sleeve garment (G3 exit) with declared ease — armscye, cap ease, balance
  notches, dart placement — drafted through the recipe and evaluated per size.
  Acceptance: cap ease is a declared quantity in the Ease model and appears in diagnostics as
  intentional; the garment's piece list, sewing graph and notch scheme are complete against the
  `G0-CONTRACT.3` ontology; the reference skirt's goldens stay green.
  Paths:crates/sc-cli/**,crates/sc-artifacts/**,crates/sc-api/src/lib.rs,crates/sc-mcp/tests/admission.rs,conformance/reference-bodice/**,conformance/goldens/reference-bodice/**,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-01 needs a seam that differs on purpose and the skirt's all declare zero. WHERE nothing could make a cap meet a declared ease:an armhole's length is known only once drawn and the recipe cannot ask for it;git grep -c '"fits"' 5e7e9760 -- crates/sc-cli/src/ →rc=1.
  - [x] ADDRESSED,FIT:a draft's fits solve an input by whole-number false position until a seam walks to what it declares,drawing only the two pieces,with asserts holding at the answer. cargo test -p sc-cli --test bodice →rc=0:21 tests,five sizes,each measurement moved 20 mm both ways.
  - [x] ADDRESSED,DECLARED:cap_ease_front and cap_ease_back walk as declared_ease;with the declaration removed the same cloth is a defect by exactly that amount. Each curve is also measured by Gauss-Legendre from its control points,so the check is not the walk that it checks.
  - [x] ADDRESSED,MARKS:single,double and drill notches,and before_end. The graph is held against every edge in both directions,with free edges named in the book.
  - [x] ALSO FOUND:my first bust dart had unequal legs and the closure report said so,gap 2985 µm,conserved false;it now sits level with the bust point,and a test holds every dart's gap at zero. D405:draft_pattern's advertised schema refused seams.
  - [x] NOT SHOWN,STATED:nobody has cut it in cloth. All constants are assumed. Sizes are re-drafted,not graded. The cap and armhole are one cubic each. Does not fit one A0 sheet,so it tiles,and it is not shrunk.
  - [x] NO REGRESSION:gauge.sh --checks →rc=0;cargo test -p sc-artifacts -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1015.

- ID: `G3-GRADING.5`
  Status: `done` (**required**, not optional, since roadmap v0.3 — the envelope-coverage exit criterion
  names trousers, so this leaf is a gate clause and not a discretionary intermediate)
  Goal: the **trousers** garment the envelope-coverage criterion requires (roadmap §11 G3, §3.2): drafted
  through the recipe, graded, and exported — carrying at least one **pocket** (position, orientation, opening
  type and piece composition per ontology §4.7) and at least one closure whose **buttonhole length is derived
  from its button** (ontology §4.7), so the two envelope features the matrix assigns to this gate are proved
  by a garment rather than by a unit test. A shirt may be substituted for the upper body if the domain
  evidence calls for it; the trousers may not be dropped, because §3.2 names them.
  Acceptance: the crotch curve, inseam and waistband draft deterministically and grade without artifact;
  the pocket's pieces are composed by the recipe and appear in the piece list with their own labels; the
  buttonhole's length is computed from its button and never entered twice (a mutation test asserts that
  editing the button changes the hole); the feature matrix's `trousers`, `pocket` and `button and buttonhole`
  rows cite this leaf's evidence at the G3 exit review (`.14`); any ontology gap the garment exposes is fixed
  and recorded, which was the old discretionary form of this leaf and remains part of it.
  Paths:crates/sc-cli/**,crates/sc-artifacts/**,crates/sc-api/src/lib.rs,crates/sc-mcp/tests/admission.rs,conformance/reference-trousers/**,conformance/goldens/reference-trousers/**,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-11 names trousers with a pocket and a derived buttonhole,proved by a garment. WHERE a drawing had no such objects:git grep -c 'pockets' 3eef4f76 -- crates/sc-cli/src/draft.rs crates/sc-artifacts/src/pattern.rs →rc=1;a pocket as lines is the substitution the envelope forbids.
  - [x] ADDRESSED,GARMENT:front,back,band and two in-seam pockets through the recipe;a fit drops the back's crotch point so the inseam meets. cargo test -p sc-cli --test trousers →rc=0:16 tests,five sizes,each measurement moved 20 mm both ways,graph against every edge,goldens.
  - [x] ADDRESSED,POCKET:an object in the canonical form with position,orientation,opening and pieces,held by reference so an edit keeps it and a delete blocks a cutting file. An opening other than in_seam is refused by name.
  - [x] ADDRESSED,BUTTONHOLE:length is the size plus the policy and has no other source;a draft giving one is refused,and the test changes the button and reads the new hole from the drawing. A button past the extension is the recipe's own refusal.
  - [x] NOT SHOWN,STATED:nobody has cut it. Constants assumed. A fly is env_fly and refused,so it closes at CB. Sizes re-drafted,and grade rules are `.7`. A bag's mouth agrees with its curved seam in length,not in shape. The matrix cites this at `.14`. The core's Pocket and ButtonAndButtonhole stay unbound.
  - [x] NO REGRESSION:gauge.sh --checks →rc=0;cargo test -p sc-artifacts -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1017.

- ID: `G3-GRADING.15`
  Status: `done`
  Goal: the **classic collar** — a stand, a fall and a roll line — attached to the `.4` bodice's neckline,
  drafted through the recipe, graded, and exported, so the envelope's fourth garment is proved by the gate
  that owns construction.
  Acceptance: the roll line is an internal construction line and never exports as a cut line (the ⚠ hazard
  the glossary marks); the collar's inner and outer edges differ by the declared roll, and the difference is
  a recipe quantity rather than a fudge; the stand's height and the fall's depth are graded, not fixed;
  walking the collar-to-neckline span reports a differential inside the declared ease; the feature matrix's
  `classic collar` row cites this leaf's evidence at `.14`.
  Paths:crates/sc-cli/**,crates/sc-api/src/lib.rs,conformance/reference-bodice/**,conformance/goldens/reference-bodice/**,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-10 needs a collar,and the bodice's neck was a free edge waiting for one. WHERE an edge and an internal line could be straight or a cubic,and two cubics differ by whatever they come to;git grep -c '"through"' 2b413c7a -- crates/sc-cli/src/draft/edges.rs →rc=1.
  - [x] ADDRESSED,ROLL:an edge and an internal line can now be an arc. The collar's edges are concentric arcs,so the outer less the neck is collar_roll by construction;measured from the drawn line at 15 and 25 mm,and held by two recipe asserts. cargo test -p sc-cli --test bodice →rc=0:26 tests.
  - [x] ADDRESSED,ROLL LINE:drawn on INTCUT and read back from the written file,with no point of it on CUT or on the cut line.
  - [x] ADDRESSED,NECK:two more fits solve the collar's two spans from the back and front necks,so four seams walk to a match,and left unfitted they are a defect by how much. Stand and fall change by size,and the collar's depth and roll line follow.
  - [x] NOT SHOWN,STATED:re-drafted at five sizes,and grade rules are `.7`. All constants assumed,and nobody has cut it. The front edge is a straight line,with no point. INTCUT carries internal lines and holes alike,and what a receiver does with it is untested. The feature matrix cites this at `.14`.
  - [x] NO REGRESSION:gauge.sh --checks →rc=0;cargo test -p sc-artifacts -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1016.

- ID: `G3-GRADING.16`
  Status: `pending` (created by `G0-CONTRACT.9`, which named the drafting system ADR-0003 requires and
  would otherwise have left the blocks with no owner — a decision with no owner is a wish)
  Goal: ship the **reference block set**: the blocks of the named system (Aldrich's metric pattern cutting,
  `docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md`) transcribed into
  construction recipes — skirt, darted bodice, set-in sleeve, classic collar and trousers — each a recipe in
  the formula language and each an independent subject for the gates that consume it.
  Acceptance: every construction number carries a citation to edition and page and enters as `read-external`,
  or stays `assumed` with the domain seat named, and is never `known` on this repository's authority; no text,
  table or illustration of the source is reproduced; each block drafts, grades and exports, so `.14`'s
  envelope-coverage review can cite it as the reference the garments were checked against; a block whose source
  has not been procured is recorded `not met` with the procurement seat named rather than drafted from
  recollection; the reference skirt stays independent of the transcription, as `G0-CONTRACT.9` requires.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.6`
  Status: `done`
  Goal: instantiation path A — measurement-driven regeneration: re-evaluate the construction recipe
  per size / per body (§3.3), MTM-ready.
  Acceptance: a size set evaluates to a graded family with no hand edits; regeneration is
  deterministic; per-size diagnostics name the formulas that drove each point.
  Paths:crates/sc-cli/**,crates/sc-api/src/lib.rs,crates/sc-viewer/src/shell.rs,conformance/reference-*/session.json,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-04 needs regeneration as a thing the product does,and only a test changed measurements one at a time. WHERE nothing named a size set or drew one:git grep -c regenerate 29e0d7fa -- crates/sc-cli/src/ →rc=1.
  - [x] ADDRESSED,FAMILY:a draft's sizes,and a regenerate step drawing each with its own fits and closures,changing nothing in the Design. cargo test -p sc-cli --test family →rc=0:the sample is the golden to the byte for three garments,and five digests differ.
  - [x] ADDRESSED,DIAGNOSTICS:each size names for every boundary point the length,value,formula and inputs behind each axis.
  - [x] ADDRESSED,MEASURED:each size is measured from its drawing against what it asked for,which the instantiation chapter calls the check that matters,and each session's table is held to its chapter.
  - [x] NOT SHOWN,STATED:MTM-ready means a size is any set of inputs,and no body has been drafted this way. The steps are an engineer's and from no standard. Nobody has fitted any size. Size sets here are the draft's,and the core's SizeSet stays unbound.
  - [x] NO REGRESSION:skirt,bodice and trousers goldens byte-identical with labels now written {size};gauge.sh --checks →rc=0;cargo test -p sc-artifacts -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1018.

- ID: `G3-GRADING.7`
  Status: `done`
  Goal: instantiation path B — grade-rule instantiation: base size + size breaks + per-point X/Y delta
  rule tables, `.rul` serialization, incremental vs cumulative, stack-point / fixed-perimeter /
  smoothing attributes (§3.3).
  Acceptance: grade points carry stable identifiers (§4.1); both rule conventions round-trip; the
  attributes are honoured rather than ignored, and an unsupported attribute is a typed rejection.
  Paths:crates/sc-grading/**,crates/sc-cli/**,crates/sc-units/tests/determinism.rs,crates/sc-api/src/lib.rs,crates/sc-viewer/src/shell.rs,conformance/reference-*/session.json,docs/book/src/**,docs/tasks/artifacts/ci_environment/producer_entries.tsv.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-04 needs a second path that moves points and leaves the recipe out. WHERE no rule table,no sc-grading crate:git ls-tree 732e4af9 crates/sc-grading →empty,rc=0.
  - [x] ADDRESSED,TABLE:sc-grading holds a table keyed by a stable point name,exact integers,either convention. cargo test -p sc-grading --test table →rc=0:200 seeded round trips each way;a file written elsewhere,published in a guide,reads as its author wrote it.
  - [x] ADDRESSED,REBUILD:reconstruct rewrites the draft to moved points and draws it with the same engine,so allowances,notches and seams are worked out rather than graded. cargo test -p sc-cli --test rules →rc=0:every corner of three garments lands exactly where the recipe does.
  - [x] ADDRESSED,ATTRIBUTES:a stack point moving is refused;a fixed contour that changes is a conflict naming it and the size,and blocks the files;smoothing is refused by name rather than ignored.
  - [x] ALSO FOUND:the book said the file holds these attributes and a cubic's approximation is bounded by a class. The format's first pages and a guide show it holds neither,and the bodice measures 4.5 mm between corners. Corrected,with the numbers,in the chapter.
  - [x] NOT SHOWN,STATED:no receiver has read a file. A place by fraction leaves a waist seam 0.8 mm out and a pocket's mouth 7.9 mm,and the cap's ease drifts 2.6 mm,reported and not reconciled. Control points have no rules. A true is refused.
  - [x] NO REGRESSION:goldens byte-identical;gauge.sh --checks →rc=0;cargo test -p sc-units -p sc-geometry -p sc-artifacts -p sc-grading -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1019.

- ID: `G3-GRADING.8`
  Status: `pending`
  Goal: `.rul` with grade-point identifiers re-imported into an **independent** engine (G3 exit) —
  the oracle is not our own writer.
  Acceptance: the independent engine is named with its version; the reconstructed points are compared
  against ours within a declared tolerance; divergences are classified (our bug / their convention /
  documented ambiguity) and recorded.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.9`
  Status: `done`
  Goal: extreme sizes reconstructed and measured **after target-system reconstruction**, not just at
  the base size (§3.3, G3 exit).
  Acceptance: the smallest and largest sizes of the declared size system are reconstructed through
  the target path and measured against path A within the declared equivalence tolerance; a failure
  is reported as a bounded equivalence, not hidden.
  Paths:crates/sc-cli/**,crates/sc-grading/src/table.rs,crates/sc-api/src/lib.rs,crates/sc-viewer/src/shell.rs,conformance/reference-*/session.json,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-06 asks that sizes far from the sample be checked after the file,not before. WHERE nothing read a table back from its file or ran §7's checks:git grep -c check_extremes 611e32aa -- crates/sc-cli/src/session.rs →rc=1.
  - [x] ADDRESSED,THROUGH THE FILE:a table is written as .rul,read back,given back its point names,and the two ends rebuilt from that,judging each point at the file's own quantum. cargo test -p sc-cli --test extremes →rc=0.
  - [x] ADDRESSED,CHECKS:the unit domain,crossing allowances,tightest bend,fabric width,seams against ease,notch spacing,dart intake,and finished dimensions read off the drawing,each in its own words,each shown failing with its limit tightened,and no limit made up when none is declared.
  - [x] FOUND:all three garments hold everywhere except their seams,by a stated amount. That is the bounded equivalence the leaf asks for and it stays visible.
  - [x] NOT SHOWN,STATED:what reads the file is this project's own engine,because no target system is available,and G3-05 stays open for that. The four limits are assumptions,because no factory profile exists. Nobody has cut an extreme size.
  - [x] NO REGRESSION:goldens byte-identical;gauge.sh --checks →rc=0;cargo test -p sc-units -p sc-geometry -p sc-artifacts -p sc-grading -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1021.

- ID: `G3-GRADING.10`
  Status: `done`
  Goal: the grading interchange modes validated separately (ADR-0004): base-size DXF + `.rul`,
  grade rules embedded in DXF (ASTM mode), and all-contours-graded (GradedNest-style).
  Acceptance: each mode has its own golden suite and its own receiver record; no mode is described
  as universal; the loss of each mode is stated.
  Paths:crates/sc-artifacts/**,crates/sc-geometry/src/curve.rs,crates/sc-cli/**,crates/sc-api/src/lib.rs,crates/sc-viewer/src/shell.rs,conformance/goldens/grading/**,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-07 asks for three modes validated apart,since none is universal. WHERE only one file carried one size:git grep -c export_graded 130d733e -- crates/sc-cli/src/session.rs →rc=1.
  - [x] ADDRESSED,MODES:export_graded writes base_plus_rules,embedded_rules or all_contours,each its own files with a record naming what it loses and that no receiver has read it. cargo test -p sc-cli --test modes →rc=0:30 files frozen for three garments.
  - [x] ADDRESSED,AGREEMENT:a reader in the test finds every identifier on a point on its own layer,and moves each corner by the file's rule to where the nest has it at each size,so two files not written from each other agree.
  - [x] ADDRESSED,CORRESPONDENCE:a nest's sizes have the same points in the same order,as the standard says,by evenly spacing them on each curve,held within the 100 µm a polyline target allows.
  - [x] ALSO FOUND:the book said embedded rules carry the deltas inside the DXF. The pages read hold no place for one,only identifiers,so that mode needs the table it names. Corrected,and it is the mode's first stated loss.
  - [x] NOT SHOWN,STATED:no receiver has read any mode. Only sew_as_1 exists,so allowances are generated downstream. Turn and curve point layers are not written. The standard was read in part.
  - [x] NO REGRESSION:single-size goldens byte-identical;gauge.sh --checks →rc=0;cargo test -p sc-units -p sc-geometry -p sc-artifacts -p sc-grading -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1022.

- ID: `G3-GRADING.11`
  Status: `done`
  Goal: path-equivalence validation (§3.3) — the declared tolerances within which regeneration and
  grade rules must agree, and the report when they do not.
  Acceptance: the tolerance is derived from a stated downstream requirement (not chosen to pass);
  the equivalence report is a first-class output of the grading command.
  Paths:crates/sc-cli/**,crates/sc-api/src/lib.rs,docs/book/src/**.
  - [x] ROOT CAUSE (WHY + WHERE):WHY G3-04 needs the two paths compared,and until now only a test measured the gap. WHERE grade reported no comparison:git grep -c equivalence 32077fae -- crates/sc-cli/src/session.rs →rc=1.
  - [x] ADDRESSED,REPORT:grade always reports equivalence;each quantity names both values,the difference,its class and verdict,and a finding names its owner and where the table came from. cargo test -p sc-cli --test equivalence →rc=0:a wrong corner injected into a received table is found by name,size and amount.
  - [x] ADDRESSED,CLASSES:T1 is one quantum,T2 the tightest consumer,which is a walk,and seam is twice that,each with where the number comes from in the report. T4 and T5 have no bound,because a receiver's and a factory's are theirs. These do not all pass,and the armhole is a finding at every size but the sample.
  - [x] ADDRESSED,LOCKSTEP:the book's table of differences is read against what the report measures,so it cannot go stale.
  - [x] NOT SHOWN,STATED:a finding says the two paths differ by more than this project can call the same. It does not say a garment is unusable,which only a factory's tolerance can say,and none exists.
  - [x] NO REGRESSION:gauge.sh --checks →rc=0;cargo test -p sc-units -p sc-geometry -p sc-artifacts -p sc-grading -p sc-cli -p sc-api -p sc-mcp -p sc-viewer →rc=0;clippy --workspace --all-targets -D warnings →rc=0;fmt --check →rc=0;doctests →rc=0;BOOK-CENSUSES →rc=0.
  promotion: declined (no new lesson).
  Commit:STITCHCAD-G1-1020.

- ID: `G3-GRADING.12`
  Status: `pending`
  Goal: offset + grading golden suites green (G3 exit) — the union of the G2 offset corpus and the
  new graded geometry, frozen through the canonicalizer.
  Acceptance: one command runs both suites; a mutation in either the offset engine or the grading
  rules turns the suite red (proved by an injected mutation).
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.13`
  Status: `pending`
  Goal: the full typed notch set (§15.12: single/double + drill at v1; V/I/T/U/castle by G4 export) —
  semantic mark with physical representation resolved at export.
  Acceptance: each notch type has a semantic definition, an export representation per dialect and a
  golden; encoding (coded point + direction/depth/type vs drawn geometry) is a profile-resolved
  parameter with a documented default.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.14`
  Status: `pending`
  Goal: G3 exit review — every clause cited against evidence, **including the envelope-coverage criterion**
  (each §3.2 garment named with the leaf that drafted, graded and exported it), the equivalence bounds
  published, the frontier handed to `G4-PROFILES`.
  Acceptance: each clause `met` with a re-runnable check or `not met` with a named blocker; a §3.2 garment
  with no leaf's evidence behind it fails the review, because that is exactly the gap roadmap v0.3 closed; and
  the review is not signed by the party that proposed the criterion — governance §6.1 rule 2 withholds
  approval of a decision's evidence from its author, so a self-applied envelope amendment is certified by
  somebody else or stays unapproved (`decision_self-application-under-delegation.md`).
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `G3-GRADING.1` | `pending` | gated on `G2-2D` |

## Decisions

- `2026-09-30`: **roadmap v0.3 turned `.5` from discretionary into a gate clause and added `.15`.** The
  amendment's source is defect D32 — §3.2 had promised a classic collar and trousers, and the ontology had
  modelled buttons and pockets, while no gate's exit criteria proved any of them, so G7's supported-envelope
  statement would have had to declare four of the envelope's own garments untested. `.5`'s old form ("an
  intermediate may be inserted without shame") was permission, and permission proves nothing at a gate
  review; the criterion is now written over §3.2's whole garment list, so a future envelope addition
  inherits a proof requirement instead of needing its own amendment. Recorded in
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`, applied to the roadmap in its
  Appendix A disposition log.

- `2026-09-29`: `.5` (the intermediate garment) is a real leaf with a real closure condition, not a
  contingency note — the roadmap explicitly permits it, and permission that is not tracked is
  permission that is either ignored or abused.
- `2026-09-29`: `.8` requires an independent engine because a `.rul` writer validated only by our own
  reader proves transcription, not interchange (claim-verification leg 2: prefer an oracle you did
  not build).

## Open Questions

- Which independent engine is available for `.8` (a commercial system's importer, an open reader, or
  a partner run) — depends on the procurement/governance roles named at `G0-CONTRACT.14`; if no seat
  exists, the roadmap's documented fallback is a partner-run manual test, and `.8` records that.
- Whether trouser-specific constructions enter the v1 envelope via `.5` or are deferred with a
  diagnostic — decided at gate entry against the `G0-CONTRACT.4` matrix.

## Blockers

- `.8` may be blocked on evaluation-seat access (owned by `G0-CONTRACT.14`); the fallback is named
  above so the gate does not stall silently.

## Acceptance Checklist

Filled per leaf, in a `### <leaf-id>` subsection added by the same commit as the work, and
mechanically required to be fresh in that commit by leaf `SPINE.8`; a tree file carries no unticked
placeholder boxes (defect D15, measured by the `SPINE.7` probe).

### `G3-GRADING.5` / `.15` — the envelope-coverage criterion arrives as leaves, not as prose

This tree had no completed leaf, so it carried no acceptance boxes — and a staged `docs/tasks/*.md` file with
no ticked box is refused by `scripts/check_task_acceptance.sh` whenever the same commit stages code. These boxes
are the evidence for the change roadmap v0.3 made to this tree, added in the commit that made it.

- [x] **REPRODUCE / ISSUE** — before roadmap v0.3 this tree owned no envelope garment beyond the bodice and the
  sleeve: `grep -ci 'collar\|trousers\|pocket\|button' docs/tasks/G3-GRADING.md` over the committed file
  matched only `.5`'s *optional* intermediate ("a shirt or trousers may be inserted") —
  `git show HEAD:docs/tasks/G3-GRADING.md | grep -ci 'collar\|trousers\|pocket\|buttonhole'` → `1`, `rc=0`,
  against `15` leaf declarations in the same revision — while
  `docs/book/src/spec/feature-matrix.md` had four rows whose proof no gate accepted (defect D32).
- [x] **ROOT CAUSE (WHY + WHERE)** — the tree was seeded from G3's exit criteria clause by clause
  (`PLANNING.1`–`.3`), and those clauses named a bodice and a set-in sleeve; §3.2's envelope named two more
  garments and two more features that no clause carried, and the count is re-derivable rather than recalled:
  `git show HEAD:ROADMAP.md | sed -n '/^### G3 /,/^### G4 /p' | grep -ci 'collar\|trousers\|button\|pocket'`
  → `1`, `rc=0`, that one hit being the note that *permitted* an intermediate. A tree seeded from a gate
  inherits that gate's gaps exactly, which is why the amendment had to change the gate before it could change
  the tree.
- [x] **ADDRESSED (verified)** — `.5` is now **required** and named the envelope's trousers, carrying the two
  features the matrix assigns to this gate: at least one pocket (position, orientation, opening type, piece
  composition per ontology §4.7) and at least one closure whose buttonhole length is derived from its button,
  with a mutation test asserting that editing the button changes the hole. `.15` is created for the classic
  collar — stand, fall and roll line, the roll exported as an internal construction line and never as a cut
  line. `.14`'s exit review now cites the coverage clause and fails if a §3.2 garment has no leaf's evidence.
  The tree's acceptance-criteria table gained the clause row, and the capture census still derives:
  `bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh` →
  `census: 10 lanes / 13 trees / 2 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `exit=0`; its
  advisory clause-versus-leaf table shows this tree at `9` clause rows and `15` leaves against G3's `12`
  semicolon-split roadmap clauses (`bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh |
  sed -n '/=== 4./,$p'`), which is the expected direction — a tree may split one clause into several leaves.
- [x] **NO REGRESSION** — no leaf in this tree changed status (all remain `pending`, gated on `G2-2D`), so no
  commitment moved earlier and nothing was claimed done; the frontier row is unchanged; `make gate` →
  `=== all doctrines green ===`, `exit=0`, with the `TABLE-ARITY-RATCHET` accepting the new acceptance-criteria
  row; `bash scripts/check_live_doc_size.sh` → `OK`, `exit=0`, this file at `314` lines / `17 855` B against a
  `tasks_collection` per-part health of `800` / `65 536`.
- [x] **FIX** — rewrote `.5` from a discretionary intermediate into a gate clause with the two features it must
  carry and the substitution it may not make (a shirt may replace the upper body; the trousers may not be
  dropped, because §3.2 names them); added `.15`; extended `.14`'s goal and acceptance; added the clause row to
  the acceptance table and a Decisions entry recording that the change came from roadmap v0.3 and why
  permission is not a criterion.
- [x] **LOCKSTEP** — the matrix's four cells cite this tree's leaves (`G3-GRADING.5`, `.15`) and no longer say
  `(proposed)`; `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md` is marked applied;
  `ROADMAP.md` v0.3 carries the criterion and its Appendix A disposition entry; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and `docs/TASK_TREE.md` updated in the same commit.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | created by the seeding leaf |
| `.1` … `.16` | `pending` | — |

## Changelog

- `2026-09-30`: `.16` added by `G0-CONTRACT.9`, which decided ADR-0003's drafting system and arrived with
  an owner for the blocks. The same slice removed a duplicate `## Acceptance Checklist` heading this file
  carried since `G0-CONTRACT.4c` added the boxes below it, and corrected the children range, which still
  said `.14` while `.15` existed (defect **D48**).
- `2026-09-29`: Tree created by `PLANNING.2` with 14 leaves mapped to the G3 exit clauses.
