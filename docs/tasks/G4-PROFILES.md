# G4-PROFILES: Factory Profiles & the uncertainty workflow (roadmap gate G4)

## Metadata

- Tree ID: `G4-PROFILES`
- Status: `proposed`
- Roadmap lane: `ROADMAP.md` §11 gate **G4 — Profiles & uncertainty workflow** (sources: §6
  constraint machinery, §7.5 HPGL, §7.6 i18n copy, §8 Factory Profiles, §2.8 no-code by design,
  §12 domain review)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

The signoff loop becomes the product: a **Factory Profile** is a versioned, typed, evidence-bearing
object that a sewing expert can create without writing JSON; unknown facts stay unknown and visibly
govern what may be exported; the custom CSP solver resolves profile parameters with explanations
mapped to fields, and a differential-oracle campaign backs its verdicts; and HPGL joins DXF/PDF with
a known-good real-plotter fixture.

## Entry criteria

- `G3-GRADING` closed (profiles parameterise a graded, exported garment; without both instantiation
  paths there is nothing for a profile to govern).
- `G0-CONTRACT.8`/`.10` (license, dialects) and the §8.2 policy matrix tuned at G0.

## Non-Goals

- The full application Profile Editor and the shipped UX (G5) — this tree delivers the *minimal*
  guided editor the gate names (CLI interview or single-page form over `sc-api`).
- Real import-filter validation against factory products, the factory pilot loop and the
  rejection-reason taxonomy (G6).
- Any general "verified factory" badge: evidence is scoped per claim (§8.1), and this tree is where
  that rule becomes code.

## Acceptance Criteria (gate G4 exit, clause by clause)

| Roadmap G4 exit clause | Leaf |
| --- | --- |
| typed constraint AST + CSP solving with explained unsat mapped to fields (custom solver) | `.2`, `.3` |
| differential-oracle campaign green | `.4` |
| minimal guided Profile Editor (CLI interview or single-page form over sc-api), not the app editor | `.9` |
| artifact policy matrix (§8.2) enforced | `.7` |
| per-claim evidence store with supersession | `.6` |
| profile composition precedence | `.8` |
| HPGL writer + known-good real-plotter fixture in `conformance/` | `.10` |
| private-vs-public profile paths working | `.11` |
| usability gate: a non-programmer sewing expert creates a profile unaided in one session | `.12` |

## Task Tree

- ID: `G4-PROFILES`
  Status: `proposed`
  Goal: profiles, uncertainty and their export consequences are one enforced mechanism.
  Children: `.1` … `.14`

- ID: `G4-PROFILES.1`
  Status: `pending`
  Goal: `sc-profiles` — the Factory Profile schema v2 (§8): parameters with type, domain, value or
  default, state (known / assumed / unknown), evidence refs, artifact effect, transformation stage;
  schema version distinct from content version; validation that rejects unresolved identifiers.
  Acceptance: the §8 example profile validates; an unknown parameter name or dangling evidence ref is
  a validation error naming the field; a profile's content version changes only when its content does.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.2`
  Status: `pending`
  Goal: the typed constraint AST (§6.2) — versioned, stable rule ids, units, scope, hard/soft status,
  provenance, diagnostic templates, and the pretty-printer that renders explanations as sentences.
  Acceptance: no free-string constraints exist in the schema; each AST node round-trips through
  canonical serialization; a pretty-printed explanation names the factory-visible field, not an
  internal symbol.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.3`
  Status: `pending`
  Goal: production CSP solving over profiles (§6.1) — deterministic finite-domain propagation and
  search resolving profile parameters, with the four distinguished outcomes and unsat cores mapped
  to profile fields; bounded backtracking when a discrete-feasible candidate fails geometric
  verification.
  Acceptance: an unsatisfiable profile yields an explanation naming the conflicting fields and rules;
  repeated runs give identical results; the rejected-*candidate* case is distinguished from
  whole-space infeasibility in the diagnostic type.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.4`
  Status: `pending`
  Goal: the differential-oracle campaign (§6.1) — randomized constraint sets solved by both
  `sc-constraints` and the non-shipped `csp-z3` oracle, results compared, any divergence treated as a
  bug in ours; seeds recorded so a divergence is reproducible.
  Acceptance: the campaign runs in CI behind the non-default feature; a deliberately weakened
  propagation rule makes it fail (a control seen RED); Z3 appears in no shipped artifact
  (dependency census recorded).
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.5`
  Status: `pending`
  Goal: the topological verification pass (§6.1 pass 3) — curve continuity, minimum curvature radius
  for knife cuts, offset self-intersection, boundary boxes vs fabric width — with failures mapped back
  to parameter domains as structured diagnostics.
  Acceptance: each check names the parameter domain it implicates; a too-narrow cutting width produces
  a field-linked diagnostic rather than a geometric warning; the checks run in the release pipeline.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.6`
  Status: `pending`
  Goal: the per-claim evidence store (§8.1) — evidence scoped to target system + version + import
  settings + artifact hashes + procedure + observer + date + result; superseded and revoked evidence
  preserved, never overwritten; badges as derived views.
  Acceptance: one acceptance cannot make a whole profile "verified" (a test asserts the scope);
  revocation is visible in derived badges; evidence records are immutable once written, with
  supersession links.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.7`
  Status: `pending`
  Goal: the artifact policy matrix enforced (§8.2, §8.3) — states × artifact effects per requested
  artifact, with dependency closure computed per request so an irrelevant unknown cannot block an
  unrelated export, and no conservative default ever substituting for observation.
  Acceptance: the normative release §8 matrix is tested per artifact: cosmetic label → badge in
  previews/drafts, block in production; notch geometry → badge in preview, sidecar in draft, block
  in production; units → blocked everywhere. No default replaces an unknown. A blocked export names
  the unknown, its field and its resolve policy; symbolic G1 profile bindings must resolve declarations and expected field kinds before
  physical output. The closure computation is proved minimal by a case where an unrelated unknown
  does not block.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.8`
  Status: `pending`
  Goal: profile composition and precedence (§8.1) — hard restrictions > factory overrides >
  preferences > defaults, with conflicts visible rather than silently resolved.
  Acceptance: a conflict between two layers is reported with both sources; precedence is a tested
  property over generated layer combinations; the resolved value carries its provenance layer.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.9`
  Status: `pending`
  Goal: the minimal guided Profile Editor (§2.8, G4 exit) — a CLI interview or a single-page form over
  `sc-api` that produces the typed AST; plain-language questions; machine tokens never rendered raw;
  strings externalised per the i18n architecture chosen at G0.
  Acceptance: a user never types JSON or a constraint expression; every question maps to a parameter
  and its diagnostic template; the editor writes through the same commands the API exposes (no private
  mutation path).
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.10`
  Status: `pending`
  Goal: the HPGL/PLT writer (§7.5) — plotter-unit scale (1016 units/inch), `IP`/`SC` calibration,
  pen-per-function mapping, `LB` labels with size/qty, path optimization, notch-as-geometry vs tool
  commands — plus a known-good real-plotter `.plt` fixture committed to `conformance/`.
  Acceptance: the fixture's provenance (plotter model, settings, who produced it) is recorded; our
  output is compared against it semantically and byte-wise where the format allows; pen mapping comes
  from the profile, and an `assumed` pen map is badged as such.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.11`
  Status: `pending`
  Goal: private-vs-public profile paths (§8.1, §12) — profiles private by default; sharing is an
  explicit action with a preview of exactly what leaves the machine; the sanitized public subset is a
  separate, reviewed artifact.
  Acceptance: a share preview lists every field that will be published; no customer measurement, body
  scan or commercial secret can reach a public artifact (asserted by a redaction test over the
  diagnostics/telemetry path as well); contribution requires the two-step domain + maintainer review.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.12`
  Status: `pending`
  Goal: the usability gate (G4 exit) — a non-programmer sewing expert creates a profile unaided in one
  session, with the terminology feedback recorded and fed back into the glossary and editor copy.
  Acceptance: the session is observed, not simulated; every point of confusion is logged as a defect
  with an owner (glossary entry, editor copy, or diagnostic template); the gate is re-run after the
  fixes rather than waived.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.13`
  Status: `pending`
  Goal: profile-driven transformations applied at their declared stage — shrinkage
  (`post_grade_pre_export`), cut/sew line swap as a named export mapping, seam-allowance
  included-in-contour vs generated-downstream, notch physical representation and encoding.
  Acceptance: each transformation is asserted at its stage by a test that would fail if it moved;
  the cut/sew swap never inverts semantic meaning (a swap changes the mapping, not the object);
  allowance policy resolution is per Factory Profile as §3.1 requires.
  Verification: `pending`
  Commit: `pending`

- ID: `G4-PROFILES.14`
  Status: `pending`
  Goal: G4 exit review — every clause cited against evidence, the profile schema and policy matrix
  published in the book, the frontier handed to `G5-SHELLS`.
  Acceptance: each clause `met` with a re-runnable check or `not met` with a named blocker.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `G4-PROFILES.1` | `pending` | gated on `G3-GRADING` |

## Decisions

- `2026-09-29`: the usability gate (`.12`) is a leaf with a re-run condition, not a milestone note.
  The roadmap calls early expert feedback on terminology "a feature, not a delay"; a feature gets a
  deliverable and a verdict.
- `2026-09-29`: `.4` keeps Z3 out of every shipped artifact and proves it by dependency census rather
  than by intention, because a dev-only oracle that leaks into a release is a licensing defect
  (ADR-0001) and a WASM-size defect (§7.3) at once.

## Open Questions

- CLI interview vs single-page form for `.9`: the gate permits either. Decided at gate entry on
  whether `sc-api` and the dev shell are far enough along to host a form; the CLI interview is the
  fallback that cannot be blocked.
- Who the "non-programmer sewing expert" is (`.12`) depends on the governance roles named at
  `G0-CONTRACT.14`; if unnamed by gate entry, the leaf records the blocker instead of substituting a
  developer.

## Blockers

- `.12` needs a real domain expert (director's network / governance roles). Tracked, not assumed.

## Acceptance Checklist

Filled per leaf, in a `### <leaf-id>` subsection added by the same commit as the work, and
mechanically required to be fresh in that commit by leaf `SPINE.8`; a tree file carries no unticked
placeholder boxes (defect D15, measured by the `SPINE.7` probe).

### `G1-SLICE.3c.3a` — dependency alignment for `.7` (D58 only)

This checklist verifies the dependent test contract corrected by the G1 notch leaf. All G4
implementation leaves remain pending; this is no claim that profile policy executes yet.

- [x] **REPRODUCE / ISSUE** — `.7` asked for a notch default while its Goal and normative release
  §8 forbid defaults. D58 preserves the original contradiction and its impact in defects-part4.
- [x] **ROOT CAUSE (WHY + WHERE)** — `.7` retained obsolete example wording after G0's tuned
  matrix. `bash docs/tasks/artifacts/release_contract/run_release_contract_census.sh` →
  `8 matrix rows / 0 failure(s)`, `rc=0`: notch geometry is badge/sidecar/block, with no default.
- [x] **FIX** — align `.7` Acceptance with the canonical per-artifact matrix; require symbolic G1
  bindings to resolve declarations and expected field kinds before physical output.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/release_contract/run_release_contract_census.sh`
  → `0 failure(s)`, `rc=0`; `cargo test -p sc-core --test notch_contract` → `7 passed`, `rc=0`:
  G1 retains declarations and always exposes `DeferredToG4`, without supplying physical values.
- [x] **NO REGRESSION** — `make check` → strict clippy/all Rust tests green; `make wasm` → green;
  `make book` → warning-free; release/feature censuses → `0 failure(s)`, all `rc=0`. The normative
  policy itself is unchanged and G4's actual policy enforcement stays pending.
- [x] **LOCKSTEP** — `.7` Acceptance, G1 evidence/decision, ontology implementation examples,
  closed D58 record and the live defect pointer; no G4 leaf status advances.

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

- `2026-09-29`: Tree created by `PLANNING.2` with 14 leaves mapped to the G4 exit clauses.

- `2026-10-01`: `G1-SLICE.3c.3a` corrects `.7`'s contradictory no-default acceptance (D58).
  All G4 implementation leaves stay pending; the dependency checklist records this limited change.
