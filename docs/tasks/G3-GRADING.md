# G3-GRADING: construction & grading (roadmap gate G3)

## Metadata

- Tree ID: `G3-GRADING`
- Status: `proposed`
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
  Status: `proposed`
  Goal: construction operations and both instantiation paths work, and their divergence is measured.
  Children: `.1` … `.16`

- ID: `G3-GRADING.1`
  Status: `pending`
  Goal: closure semantics — Dart / Tuck / Pleat / Gather as objects with intake, direction and
  apex, plus slash-and-spread as a drafting operation on the recipe (§3.1).
  Acceptance: a dart's intake is queryable and conserved across the operation that closes it;
  slash-and-spread is replayable from the recipe (not a baked geometry edit); unsupported closures
  produce the diagnostics named in the `G0-CONTRACT.4` feature matrix.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.2`
  Status: `pending`
  Goal: `walk` and `true` as first-class editing operations on the SewingGraph (§3.1) — a sleeve cap
  walked around an armscye, balance notches planted, seam lengths trued with the ease declared.
  Acceptance: walking reports the length differential per span and attributes it to declared ease or
  to a defect; truing never silently changes a piece's semantics; both operations are bus commands
  with undo.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.3`
  Status: `pending`
  Goal: the identity contract under edit (§4.1) — referenced-edge split, merge, reverse, delete and
  offset-fragmentation either preserve references or produce a visible unresolved-reference repair
  task; no silent reassignment.
  Acceptance: a property test enumerates the edit operations and asserts reference survival or a
  repair task for each; notches, grade points and seam spans are the reference carriers tested.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.4`
  Status: `pending`
  Goal: the bodice + set-in sleeve garment (G3 exit) with declared ease — armscye, cap ease, balance
  notches, dart placement — drafted through the recipe and evaluated per size.
  Acceptance: cap ease is a declared quantity in the Ease model and appears in diagnostics as
  intentional; the garment's piece list, sewing graph and notch scheme are complete against the
  `G0-CONTRACT.3` ontology; the reference skirt's goldens stay green.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.5`
  Status: `pending` (**required**, not optional, since roadmap v0.3 — the envelope-coverage exit criterion
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
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.15`
  Status: `pending` (created by the roadmap v0.3 envelope-coverage amendment, which named the classic collar
  an exit criterion; defect **D32** is what left it unproved until then)
  Goal: the **classic collar** — a stand, a fall and a roll line — attached to the `.4` bodice's neckline,
  drafted through the recipe, graded, and exported, so the envelope's fourth garment is proved by the gate
  that owns construction.
  Acceptance: the roll line is an internal construction line and never exports as a cut line (the ⚠ hazard
  the glossary marks); the collar's inner and outer edges differ by the declared roll, and the difference is
  a recipe quantity rather than a fudge; the stand's height and the fall's depth are graded, not fixed;
  walking the collar-to-neckline span reports a differential inside the declared ease; the feature matrix's
  `classic collar` row cites this leaf's evidence at `.14`.
  Verification: `pending`
  Commit: `pending`

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
  Status: `pending`
  Goal: instantiation path A — measurement-driven regeneration: re-evaluate the construction recipe
  per size / per body (§3.3), MTM-ready.
  Acceptance: a size set evaluates to a graded family with no hand edits; regeneration is
  deterministic; per-size diagnostics name the formulas that drove each point.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.7`
  Status: `pending`
  Goal: instantiation path B — grade-rule instantiation: base size + size breaks + per-point X/Y delta
  rule tables, `.rul` serialization, incremental vs cumulative, stack-point / fixed-perimeter /
  smoothing attributes (§3.3).
  Acceptance: grade points carry stable identifiers (§4.1); both rule conventions round-trip; the
  attributes are honoured rather than ignored, and an unsupported attribute is a typed rejection.
  Verification: `pending`
  Commit: `pending`

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
  Status: `pending`
  Goal: extreme sizes reconstructed and measured **after target-system reconstruction**, not just at
  the base size (§3.3, G3 exit).
  Acceptance: the smallest and largest sizes of the declared size system are reconstructed through
  the target path and measured against path A within the declared equivalence tolerance; a failure
  is reported as a bounded equivalence, not hidden.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.10`
  Status: `pending`
  Goal: the grading interchange modes validated separately (ADR-0004): base-size DXF + `.rul`,
  grade rules embedded in DXF (ASTM mode), and all-contours-graded (GradedNest-style).
  Acceptance: each mode has its own golden suite and its own receiver record; no mode is described
  as universal; the loss of each mode is stated.
  Verification: `pending`
  Commit: `pending`

- ID: `G3-GRADING.11`
  Status: `pending`
  Goal: path-equivalence validation (§3.3) — the declared tolerances within which regeneration and
  grade rules must agree, and the report when they do not.
  Acceptance: the tolerance is derived from a stated downstream requirement (not chosen to pass);
  the equivalence report is a first-class output of the grading command.
  Verification: `pending`
  Commit: `pending`

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
