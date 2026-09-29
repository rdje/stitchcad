# G7-RELEASE: scoped production declaration (roadmap gate G7)

## Metadata

- Tree ID: `G7-RELEASE`
- Status: `proposed`
- Roadmap lane: `ROADMAP.md` §11 gate **G7 — Scoped production declaration** (sources: §9 signoff and
  release contract, §7.8 semver commitment, §12 governance, §14 risk register)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

The project says, in public and with evidence, **exactly what it is good for**: an independent review of
the evidence, a supported-envelope statement with named limitations, a committed semver and schema/API
policy, install/reopen/upgrade/rollback evidence, release channels, and governance in force. The
declaration is scoped — one factory's acceptance is evidence for that envelope, never a general
certificate.

## Entry criteria

- `G6-CONFORMANCE` closed: there is evidence to review. A declaration without a conformance lab is
  marketing.
- `G0-CONTRACT.14`'s governance roles named (the declaration needs an accountable owner and a review
  path that is not the author).

## Non-Goals

- Widening the supported envelope beyond what G6 evidenced.
- Any fit, drape or physical-behaviour claim — those belong to `V2-SIM` and only with physical evidence.
- Declaring compatibility with a receiver that was not tested at the recorded version and settings.

## Acceptance Criteria (gate G7 exit, clause by clause)

| Roadmap G7 exit clause | Leaf |
| --- | --- |
| independent evidence review | `.1` |
| supported-envelope statement with named limitations | `.2` |
| semver + schema/API policy committed | `.3` |
| install/reopen/upgrade/rollback evidence | `.4` |
| release channels | `.5` |
| governance in force | `.6` |

## Task Tree

- ID: `G7-RELEASE`
  Status: `proposed`
  Goal: a scoped, evidence-backed production declaration, and the policies that keep it true.
  Children: `.1` … `.7`

- ID: `G7-RELEASE.1`
  Status: `pending`
  Goal: the independent evidence review — a reviewer who is not the author walks the conformance matrix,
  the evidence store and the release manifests, and records what holds and what does not.
  Acceptance: the reviewer is named and independent; every reviewed claim carries its scope; findings
  are defects with owners, and the declaration cannot land while one is open.
  Verification: `pending`
  Commit: `pending`

- ID: `G7-RELEASE.2`
  Status: `pending`
  Goal: the supported-envelope statement — garment families, construction operations, size ranges,
  artifact dialects and modes, receiver products/versions/settings, and the named limitations beside
  each claim.
  Acceptance: every claim traces to G6 evidence; every limitation is stated in the same document as the
  claim it bounds; the statement is published in the book, not buried in a release note.
  Verification: `pending`
  Commit: `pending`

- ID: `G7-RELEASE.3`
  Status: `pending`
  Goal: the semver and schema/API policy committed — what a major/minor/patch change means for the
  project format, `sc-api` and the MCP surface, and how deprecation and migration are announced.
  Acceptance: the policy is in the repository and the book; the experimental-to-stable transition of
  `sc-api` (§7.8) is recorded; schema versioning and migration guarantees match `sc-store`'s behaviour.
  Verification: `pending`
  Commit: `pending`

- ID: `G7-RELEASE.4`
  Status: `pending`
  Goal: install / reopen / upgrade / rollback evidence on every supported platform — including reopening
  a project written by an older version (profile versions pinned) and rolling back after an upgrade.
  Acceptance: each path is demonstrated with recorded commands and observed results; a rollback leaves
  a readable project; an upgrade never silently rewrites a pinned profile version.
  Verification: `pending`
  Commit: `pending`

- ID: `G7-RELEASE.5`
  Status: `pending`
  Goal: release channels — stable/pre-release, how artifacts are published and signed, and how a user
  or agent discovers which channel and version they hold.
  Acceptance: the channel definitions are published; a build is reproducible from its published artifact
  hash; the version and channel are visible in the application and the CLI.
  Verification: `pending`
  Commit: `pending`

- ID: `G7-RELEASE.6`
  Status: `pending`
  Goal: governance in force — the drafted model from `G0-CONTRACT.14` is operating: named owners, the
  sewist-vs-programmer review paths, two-step domain review for profile changes that alter exported
  bytes, golden-file approval ownership.
  Acceptance: at least one real change has gone through each review path; the governance document names
  people, not roles-in-the-abstract; conflict resolution has been used or exercised in a dry run.
  Verification: `pending`
  Commit: `pending`

- ID: `G7-RELEASE.7`
  Status: `pending`
  Goal: G7 exit review — the production declaration itself, scoped and signed by the accountable owner,
  with every clause cited against its evidence.
  Acceptance: each clause `met` with a re-runnable check or `not met` with a named blocker; the
  declaration states its scope and its date of evidence, and names what it does not cover.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `G7-RELEASE.1` | `pending` | gated on `G6-CONFORMANCE`; there must be evidence before a review of it |

## Decisions

- `2026-09-29`: the independence requirement in `.1` is a leaf-level acceptance criterion, not a
  preference. A self-review of one's own conformance matrix is the shared-classifier failure the claim
  verification standard names: the check and the thing checked share a parent.
- `2026-09-29`: `.6` requires governance to have been *used*, not written. A governance document with no
  change routed through it is a draft, and G0 already owns drafts.

## Open Questions

- Who performs the independent review (`.1`) — an external domain expert, a partner factory's
  patternmaker, or a reviewer from the community once it exists. Depends on `G0-CONTRACT.14`'s named
  roles; the leaf records the choice and why the reviewer qualifies as independent.
- Signing and distribution infrastructure for `.5` (which host, which key custody) — decided at gate
  entry with the release-channel requirements in hand.

## Blockers

- `.1` and `.6` depend on named humans (independence, governance ownership). Tracked, not assumed.

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
| `.1` … `.7` | `pending` | — |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.3` with 7 leaves mapped to the G7 exit clauses.
