# G6-CONFORMANCE: conformance lab & reliability (roadmap gate G6)

## Metadata

- Tree ID: `G6-CONFORMANCE`
- Status: `proposed`
- Roadmap lane: `ROADMAP.md` §11 gate **G6 — Conformance lab & reliability** (sources: §7.5 import loss
  report, §8.1 evidence, §11 G6, §13 layers of truth and fuzzing, §14 risk register)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

Compatibility stops being a claim and becomes **evidence about named receivers**: real import-filter
validation with the receiver's product, version and import settings recorded; a physical plotter and
printed patterns checked; a live factory pilot whose rejection reasons feed profile calibration; the
foreign-DXF diff loop that turns a factory's edited files into candidate profile updates; and the
reliability matrix (recovery, migrations, malformed input, performance, cancellation) at supported
scale, cross-platform, with fuzzing wherever a parser exists.

## Entry criteria

- `G5-SHELLS` closed (a factory pilot needs an application, and a UI test suite to trust).
- `G4-PROFILES` closed (import validation and the rejection taxonomy both write back into profiles).

## Non-Goals

- The production declaration and semver commitment (G7).
- Calling a manually repaired import "automated acceptance" — §11 G6 forbids it explicitly, and this
  tree's leaves record manual repair as manual repair.
- Any universal compatibility claim: every result is scoped to a receiver envelope.

## Acceptance Criteria (gate G6 exit, clause by clause)

| Roadmap G6 exit clause | Leaf |
| --- | --- |
| real import-filter validation, receiver settings recorded, manual repair documented | `.2` |
| physical plotter + printed-pattern checks | `.4` |
| factory pilot loop live, rejection-reason taxonomy feeding calibration + editor copy | `.5` |
| foreign-DXF import-diff → semantic delta → candidate profile updates | `.3` |
| recovery / migrations / malformed-input / performance / cancellation matrix at supported scale | `.6` |
| cross-platform regression matrix | `.7` |
| fuzzing at depth where parsers exist | `.8` |
| (prerequisite the gate assumes: a DXF importer with an honest loss report) | `.1` |

## Task Tree

- ID: `G6-CONFORMANCE`
  Status: `proposed`
  Goal: the conformance lab exists and its verdicts are scoped, reproducible and honest.
  Children: `.1` … `.10`

- ID: `G6-CONFORMANCE.1`
  Status: `pending`
  Goal: the DXF importer (§7.5) — an honest imported representation with **no fabricated history**,
  plus the loss report classifying every element preserved / approximated / omitted / unsupported.
  Acceptance: an imported contour is stored as explicit primitives and is distinguishable from a
  drafted one; the loss report is a first-class output, not a log line; round-trip honesty holds
  (compare `P(native)`, per §13).
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.2`
  Status: `pending`
  Goal: real import-filter validation — export into the actual target products with the receiver's
  product, version and import settings recorded, and any manual repair documented as manual.
  Acceptance: each result names receiver + version + settings + artifact hash + operator + date;
  a repaired import is never recorded as automated acceptance; results land in the per-claim evidence
  store with that scope.
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.3`
  Status: `pending`
  Goal: the foreign-DXF import-diff loop — a factory returns modified files, we compute the **semantic**
  delta (not the byte delta), and propose candidate profile updates from it.
  Acceptance: the delta is semantic and explains itself in profile terms; a candidate update never
  auto-applies (a human dispositions it); the loop is demonstrated on at least one real returned file.
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.4`
  Status: `pending`
  Goal: physical plotter and printed-pattern checks — HPGL on a real plotter with the pen map and
  calibration recorded, and printed patterns measured against the canonical geometry.
  Acceptance: the plotter model, settings and operator are recorded; measurements are inside the
  physical-acceptance tolerance class; failures produce profile-parameter candidates, not shrugs.
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.5`
  Status: `pending`
  Goal: the factory pilot loop — a live pilot with a rejection-reason taxonomy that feeds profile
  calibration and Profile Editor copy.
  Acceptance: every rejection is classified in the taxonomy; each class maps to a profile parameter or
  a documented envelope limit; the editor's wording changes where the taxonomy shows experts misreading
  it; the loop runs at least one full cycle (ship → reject/accept → calibrate).
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.6`
  Status: `pending`
  Goal: the reliability matrix at supported scale — crash recovery, schema migrations (including
  failed-migration recovery), malformed input, performance budgets, and cancellation of long operations.
  Acceptance: each row has a scale, a budget and a measured result; forced interruption leaves a
  recoverable project; a cancelled operation leaves no partial artifact.
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.7`
  Status: `pending`
  Goal: the cross-platform regression matrix — the golden and conformance suites on every supported OS,
  with the determinism policy from `G2-2D.12` enforced rather than assumed.
  Acceptance: CI runs the matrix on all supported platforms; a platform-specific byte difference is a
  defect with an owner; no suite is marked platform-optional without a recorded reason.
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.8`
  Status: `pending`
  Goal: fuzzing at depth wherever a parser exists (DXF import, `.rul`, project files, profile JSON).
  Acceptance: each parser has a fuzz target with a recorded corpus and a crash-free run length; every
  finding becomes a regression case; fuzzing runs in CI at a bounded depth and unbounded overnight.
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.9`
  Status: `pending`
  Goal: the conformance lab itself — the layers-of-truth matrix maintained as a live artifact, the
  golden-refresh policy (who may re-freeze a golden, with what evidence), and one command that runs the
  whole lab.
  Acceptance: every claimed feature maps to its oracle in the matrix; a claim without an oracle is
  listed as unclaimed; refreshing a golden requires a recorded authority.
  Verification: `pending`
  Commit: `pending`

- ID: `G6-CONFORMANCE.10`
  Status: `pending`
  Goal: G6 exit review — every clause cited against evidence, the receiver envelopes published, the
  frontier handed to `G7-RELEASE`.
  Acceptance: each clause `met` with a re-runnable check or `not met` with a named blocker.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `G6-CONFORMANCE.1` | `pending` | gated on `G5-SHELLS`; the importer is the prerequisite for `.2`, `.3` and `.8` |

## Decisions

- `2026-09-29`: the importer (`.1`) is a leaf of this tree even though the gate's exit list does not
  name it, because §13 places DXF *import* in G6 scope and three named clauses depend on it. An
  unlisted prerequisite is how a gate stalls.
- `2026-09-29`: `.5` requires a **completed** pilot cycle, not a scheduled one. A loop that has not run
  cannot calibrate anything, and the roadmap's claim is that rejection reasons feed profiles.

## Open Questions

- Which target products and versions are available for `.2` depends on the evaluation seats named at
  `G0-CONTRACT.14`; the documented fallback is a partner-run manual test, and `.2` records which happened.
- Whether the physical plotter for `.4` is owned, borrowed or partner-run — same procurement role.

## Blockers

- `.2` and `.4` may be blocked on evaluation seats / hardware (owned by `G0-CONTRACT.14`). The fallback
  is named in each leaf so the gate cannot stall silently.

## Acceptance Checklist

Filled per leaf, in a `### <leaf-id>` subsection added by the same commit as the work, and mechanically
required to be fresh in that commit by `FRESH-ACCEPTANCE-EVIDENCE`; this file carries no unticked
placeholder boxes (defect D15).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0003 (leaf PLANNING.3)` | created by the seeding leaf |
| `.1` … `.10` | `pending` | — |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.3` with 10 leaves mapped to the G6 exit clauses.
