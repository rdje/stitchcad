# V2-SIM: physically validated simulation (parallel V-track, uncapped)

## Metadata

- Tree ID: `V2-SIM`
- Status: `proposed`
- Roadmap lane: `ROADMAP.md` §11 track **V2 — Physically validated simulation (parallel, uncapped)**
  (sources: §7.7 4.2 assembly/stitch-and-smooth and 4.3 XPBD research track, §1.3 non-goals, §14 risk
  register "fit claims outrun simulation evidence")
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

A cloth-simulation research track whose **only** exit is evidence: quantitative comparison against
physical garments and material tests that supports specific, named observables. Until that evidence
exists, nothing in the product makes a fit, drape, pressure or strain claim — and every approximate
visual is labelled as approximate in the UI and on any output.

The track is deliberately uncapped and out of the default build. It reports progress, it does not
promise dates.

## Entry criteria

- `V1-ASSEMBLY` has meshing, seam correspondence and arrangement surfaces — a simulation without them
  has nothing to simulate.
- Physical test access (garments, fabrics, a measuring protocol) arranged through the governance roles
  named at `G0-CONTRACT.14`. Without it the track cannot exit, and saying so early is the point.

## Non-Goals

- Any fit-accuracy, pressure or strain claim before physical validation (§7.7 4.2).
- Photorealistic or quantitatively validated drape in v1 — an explicit §1.3 non-goal.
- Living in the default workspace or build: `sc-sim` is out of the default build (§4.3, §7.7 4.3).
- Letting this track block a G-gate or `V1-ASSEMBLY`.

## Acceptance Criteria (track V2 exit)

| Roadmap V2 exit clause | Leaf |
| --- | --- |
| quantitative comparison against physical garments/material tests supports specific claimed observables | `.4`, `.5`, `.6` |
| no fit claims before that evidence exists | `.3`, `.6` |
| XPBD research track unbounded, out of the default workspace, progress reports not promises | `.1`, `.2` |

## Task Tree

- ID: `V2-SIM`
  Status: `proposed`
  Goal: simulation claims that are earned by physical evidence, and nothing that is not.
  Children: `.1` … `.6`

- ID: `V2-SIM.1`
  Status: `pending`
  Goal: `sc-sim` as a research crate **outside the default workspace and build** — opt-in feature or
  separate workspace member, so no shipped artifact, no WASM bundle and no CI gate depends on it.
  Acceptance: a default `cargo build --all` and the WASM smoketest do not compile `sc-sim`; the
  dependency census proves it is absent from shipped artifacts; the crate states its research status in
  its own docs.
  Verification: `pending`
  Commit: `pending`

- ID: `V2-SIM.2`
  Status: `pending`
  Goal: the XPBD solver research — constraint model, iteration scheme, stability and convergence
  behaviour on garment-scale inputs, reported as progress rather than promised as a feature.
  Acceptance: each progress report states what was measured (energy drift, constraint residual, step
  size stability) on which inputs; a negative result is reported with the same rigour as a positive one;
  no report claims product capability.
  Verification: `pending`
  Commit: `pending`

- ID: `V2-SIM.3`
  Status: `pending`
  Goal: labelled approximation everywhere — in the UI and on any output — with the three capabilities
  kept separate: assembly/topology inspection, qualitative drape preview, quantitatively validated fit
  prediction.
  Acceptance: a mechanical check proves the label is present on every simulated visual and every export
  that contains one; the three capability names are distinct in code, docs and UI; no surface conflates
  a preview with a prediction.
  Verification: `pending`
  Commit: `pending`

- ID: `V2-SIM.4`
  Status: `pending`
  Goal: the calibration protocol against physical toiles — which garments, which fabrics, which
  measurements, which operator, which environment, and how a calibration result is recorded and scoped.
  Acceptance: the protocol is written before data is collected; a calibration record names its scope
  (garment, fabric, size, conditions) and cannot be cited outside it; the protocol is reviewable by a
  domain expert.
  Verification: `pending`
  Commit: `pending`

- ID: `V2-SIM.5`
  Status: `pending`
  Goal: the quantitative comparison protocol and the **observables register** — the specific quantities
  a claim may be made about (for example a measured drape coefficient or a hem-level deviation), each
  with its measurement method, tolerance and physical counterpart.
  Acceptance: an observable not in the register cannot be claimed; each register entry names how the
  physical value is obtained; the comparison statistic and its sample size are declared in advance.
  Verification: `pending`
  Commit: `pending`

- ID: `V2-SIM.6`
  Status: `pending`
  Goal: the V2 exit gate — for each claimed observable, the physical evidence, the comparison result and
  the scope, reviewed independently; and an explicit statement of what remains unclaimed.
  Acceptance: every claim traces to a register entry and a physical measurement; the review is
  independent of the implementer; unclaimed observables are listed as limitations, and the product's
  user-visible language matches the evidence exactly.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `V2-SIM.1` | `pending` | gated on `V1-ASSEMBLY`; uncapped track, progress reports not promises |

## Decisions

- `2026-09-29`: the protocol leaves (`.4`, `.5`) precede any data collection. Declaring the statistic,
  the tolerance and the sample size after seeing the data is how a simulation acquires a fit claim it
  cannot support — the exact risk §14 names.
- `2026-09-29`: `.1` is a leaf with a mechanical acceptance (the dependency census proves `sc-sim` is
  absent from shipped artifacts), because "out of the default build" is a property that silently rots
  the first time someone adds a convenience dependency.

## Open Questions

- What physical test access exists (toiles, fabric kits, a measuring lab, a partner factory) — owned by
  the governance/procurement roles from `G0-CONTRACT.14`. Without an answer, `.4`–`.6` are blocked and
  the track stays a research report, which is a legitimate state and is recorded as such.
- Whether a qualitative drape preview ships from `V1-ASSEMBLY` using this track's solver or a simpler
  one — decided at `V1-ASSEMBLY`'s frontier, not here.

## Blockers

- `.4`, `.5`, `.6` are blocked on physical test access until the governance roles are named. Recorded
  here so the track's uncapped nature is not mistaken for progress.

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
| `.1` … `.6` | `pending` | — |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.3` with 6 leaves mapped to the V2 exit clauses.
