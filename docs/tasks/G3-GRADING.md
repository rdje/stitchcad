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

## Task Tree

- ID: `G3-GRADING`
  Status: `proposed`
  Goal: construction operations and both instantiation paths work, and their divergence is measured.
  Children: `.1` … `.14`

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
  Status: `pending`
  Goal: the optional intermediate garment (roadmap G3 note: a shirt or trousers may be inserted
  "without shame") if the skirt→bodice+sleeve jump shows the recipe language or the ontology is
  missing something.
  Acceptance: either the leaf is closed with the evidence that no intermediate was needed, or the
  intermediate garment is drafted and the ontology gaps it exposed are fixed and recorded.
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
  Goal: G3 exit review — every clause cited against evidence, the equivalence bounds published, the
  frontier handed to `G4-PROFILES`.
  Acceptance: each clause `met` with a re-runnable check or `not met` with a named blocker.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `G3-GRADING.1` | `pending` | gated on `G2-2D` |

## Decisions

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

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | created by the seeding leaf |
| `.1` … `.14` | `pending` | — |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.2` with 14 leaves mapped to the G3 exit clauses.
