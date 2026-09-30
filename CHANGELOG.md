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

**Correction (D30).** part1's own descriptor says its coverage runs "through `STITCHCAD-SPINE-0004c`".
It does not: part1's newest entry is `STITCHCAD-SPINE-0004b`, and `SPINE-0004c` is sealed in part2.
Sealed segments are immutable, so the correction is recorded here and in part2's descriptor rather than
by editing part1.

The bedrock scaffold's own changelog — the provenance of this repository's discipline spine — is sealed
in [`docs/history/bedrock-scaffold-changelog.md`](docs/history/bedrock-scaffold-changelog.md).

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

## STITCHCAD-SPINE-0021 - the cadence runs, and the residue census proves what it took (leaf `SPINE.21`)

The cleanup cadence had no recurring owner: `SPINE.2` discharged the first run and wrote the record, but a
cadence is an obligation that returns, and this one was 23 hours from firing mid-slice with no leaf to own it.

- **the run** - nine paths removed, each named by the residue census and each found gone: both scratch trees
  (`target/doctrine_scratch`, `target/scratch`, `target/tmp`), both incremental caches
  (`target/debug/incremental`, `target/wasm32-unknown-unknown/debug/incremental`, 57 `.bin` files), the mdBook
  output (`docs/book/book`, 4 120 KB) and three scratch bodies the containment self-tests had left in
  `target/`. `target` went 40 648 KB -> 10 808 KB, so 33 960 KB left the volume counting the book.
- **nothing tracked was touched** - `git ls-files | grep -cE '^(target/|docs/book/book/)'` -> `0` before and
  after, `0` tracked artifact-shaped files, `0` deleted tracked files in `git status --porcelain`, and `0`
  stray `*.log` / `*.bin` / `*.tmp` / `*.orig` / `*.rej` / `.DS_Store` anywhere outside `.git`.
- **the removal is shown to cost rebuild time and nothing else** - `make gate` -> `=== all doctrines green ===`;
  `make check` -> `test result: ok. 1 passed; 0 failed`; `make book` -> regenerated at exactly 4 120 KB;
  `make probes` -> `20 suite(s) green` with `target/scratch` recreated by the Makefile's own rule;
  `make wasm` -> the smoketest green. `target` rebuilt to 13 460 KB.
- **D34's fourth instance removed** - the index's `SPINE` frontier cell still named `.20` as open one commit
  after it landed, while the execution-order paragraph in the SAME file had it right: two hand-kept sentences
  about one lane, drifting against each other, which is the strongest argument yet for deriving the cells.
- `SPINE.20`'s checklist moved to `SPINE-evidence.md`, as the convention requires of the slice after the one
  that landed it, bringing the tree back inside its per-part health (683 lines / 56 833 B); the changelog's own
  rollover follows in this entry (`part10`).

## STITCHCAD-G0-0016 - one message system, and an inventory nothing keeps by hand (leaf `G0-CONTRACT.16`)

Roadmap §7.6 required the choice at G0 and refused "Fluent or ICU" as an answer. The repository had neither the
choice nor the architecture, and seven references across five chapters pointed at this leaf instead of a clause.

- **the choice, on evidence** - **Fluent at both ends**: `fluent-rs` in the Rust core and `fluent.js` in the
  TypeScript chrome over one catalogue format, both Apache-2.0 and unarchived, read on `2026-09-30` from the
  GitHub and crates.io APIs and tabulated in the chapter. ICU4X is the rejected alternative and stays a named
  one - active, Unicode License V3, built for resource-constrained clients, which is this product's browser
  profile - and it loses on the boundary: ICU4X in Rust and the platform's Intl in the browser are two message
  dialects, the failure the clause exists to prevent. The bridge stays in reserve for a measured plural-rule or
  locale-data gap, which is `unverified-with-owner` with G5.
- **the architecture** - the message id IS the diagnostic token, so there is no second numbering to drift;
  arguments are typed and carry units and text is a presentation field; the termbase is a projection of the
  glossary with the ⚠ terms first and no guessed translation; the externalization lint has five exemptions
  each carrying a reason; canonical files are locale-free, so a decimal comma changes nothing stored;
  pseudolocalization runs in CI before any translator exists; RTL mirrors layout and never geometry, as a
  byte-comparison test (`i18n_geometry_mirrored`) rather than a sentence; and three review tiers make `strict`
  and `safety` absolute - so with the domain seat vacant no pack can ship today, stated rather than hidden.
- **the instrument** - `run_i18n_census.sh` derives the inventory from the envelope's §10, the formula
  language's §5.2, the dialects' §11, the release contract's §9 and `crates/sc-units/src/error.rs`:
  `8 families / 64 message ids / 0 failure(s)`. Deriving it caught two errors in the chapter's own first
  draft - the envelope's 29th token `geom_offset_budget` folded into the `env_*` family, and `UnitError`
  counted at four variants when it carries five, because the fifth has no braces and a first grep looked for
  braces. `run_i18n_probes.sh` -> `11 pass / 0 fail`, including CODE-GROWS, which adds a variant to a COPY of
  the crate's source and requires the count to be refused.
- **decisions** - `docs/decisions/decision_i18n-one-message-system-fluent.md` records the choice, the
  comparison table and six rejected alternatives, including percentages as review thresholds ("90 % of the
  safety tier" is one message in ten saying the wrong thing about a cut line).
- **glossary** - a ninth part, `localization.md`, with seven terms; the parts table, SUMMARY and the A-Z index
  updated; six cross-references repointed from this leaf to a clause: `305 terms / 9 parts / 156 tokens /
  0 failure(s)`.
- **D49's trigger fired on the tree file itself** at 91 % of its byte ceiling, so the oldest fourteen
  changelog entries were sealed into `g0-contract-changelog-part1` (84 lines / 7941 bytes, digest reproduced)
  and the tree fell to 81 811 B; the dev-notes ledger rolled over in the same commit (`devnotes-part5`,
  43 lines / 3977 bytes), `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `20 suite(s) green`; `make book` ->
  exit=0; all eight other censuses green; containment `OK - 17 surfaces, 15 routes, 105 files measured`

## STITCHCAD-G0-0012 - the release contract is the roadmap's §9, compared rather than restated (leaf `G0-CONTRACT.12`)

Roadmap §9 was the last G0 clause with a promised chapter and no chapter: the spec index carried "Release and
approval" as an unlinked row and thirteen glossary references pointed at this leaf. §9 is a list - nine
manifest fields, six acceptance states, three artifact classes - and a list realised in prose drifts, so the
chapter ships with an instrument that parses the roadmap and compares.

- **the chapter** - `docs/book/src/spec/release-contract.md` (257 lines / 19 010 B): nine manifest fields
  each with the object it comes from; identity as the digest of the canonical manifest, artifact hashes
  included, so a package is never edited and any change makes a new candidate; seven completeness checks
  against the declared construction, so a geometrically valid file missing a piece is an invalid package; the
  six acceptance states in the roadmap's order with the evidence and the granter each needs, where a state
  may not be claimed without the rung below it and does not survive a new identity; five scope axes with the
  rule that a claim's scope is the INTERSECTION of its evidence, narrowing by itself and widening only by a
  new record; approval as a human act, with a package whose approver is not a human identity invalid; and
  §8.2's policy matrix tuned in a recorded table over a closed four-word disposition vocabulary, consulted
  for the `unknown` state alone, with the dependency closure recorded per artifact and eight diagnostics.
- **the tuning is visible, not silent** - the roadmap publishes the matrix as an example and invites G0 to
  tune it, so the chapter quotes each example row beside what it became: notch geometry in a draft export is
  a provenance `sidecar` instead of "default + visible badge", because a default is a value substituted for
  an observation and §8.3 forbids it. `permit` is deliberately absent from the matrix's cells - a cell that
  simply permitted an unknown would be a cell that invented one - and the census refuses one.
- **the instrument** - `run_release_contract_census.sh` parses roadmap §9 (parenthesis-aware, because one
  field carries a nested comma), §8.2's header and example rows, and the five states ontology §5 declares,
  then compares all three with the chapter in BOTH directions: a field the roadmap names and the chapter
  lacks is a refusal, and so is a field the chapter invents wearing the roadmap's authority.
  `9 manifest fields / 6 states / 8 matrix rows / 0 failure(s)`, with `run_release_contract_probes.sh` ->
  `14 pass / 0 fail` over a swapped ladder, an invented field, a dropped artifact class, an unrecorded
  tuning, an undispositioned state, an undeclared and an unused disposition, and a control.
- **decisions** - `docs/decisions/decision_release-package-identity-and-scope.md` records the four rules and
  six rejected alternatives, including the similarity threshold for stale-ification: a threshold is a guess
  about which differences a factory cares about, made by the party that wants the approval to survive.
- **D49's trigger fired and was discharged in this commit** - the evidence sibling had reached 1000 lines and
  96 % of its byte ceiling, so ten completed checklists (`G0-CONTRACT.2` … `.4b`, 560 lines / 52 573 bytes)
  were sealed into `docs/history/stitchcad-g0-contract-evidence-part1.md` under the descriptor contract and
  the live sibling fell to 449 / 42 065; `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail` reproduces the
  digest. Four glossary terms were added, twelve entries repointed from this leaf to a clause, and one
  misrouting corrected (`spi` belongs to `G5-SHELLS.13`, not here).
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `19 suite(s) green`; `make book` ->
  exit=0; all eight other censuses green; containment `OK - 17 surfaces, 15 routes, 102 files measured`

## STITCHCAD-G0-0011 - the spike's rule is written before its measurement (leaf `G0-CONTRACT.11`)

ADR-0002 was the one ADR whose evidence does not exist yet, and nothing in the repository constrained what a
G1 spike would be allowed to conclude. The roadmap itself warns why that matters: "Custom wgpu, not DOM
canvas" is a hypothesis to test, not an axiom. So the rule was written first.

- **the record** - `docs/decisions/decision_adr-0002-ui-stack-and-canvas-spike-protocol.md` (116 lines):
  the chrome (Tauri + TypeScript/React, Slint as the named fallback, Flutter still rejected), the egui/iced
  dev shell and the TypeScript domain-logic ban are `active`; canvas hosting is `proposed`, in the status
  line rather than a footnote, because a decision whose evidence does not exist yet is still a decision
  *structure*. It carries the three topologies with the hypothesis each tests, a corpus declared before
  anybody measures it (16 pieces, 400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000
  fidelity round trips) with a re-run trigger if a G3 garment exceeds it, seven gates each naming what it
  protects, and six rules: eligibility, correctness outranking speed, scoring inside a declared margin with
  a total tiebreak order, a one-renderer preference where that is free, escalation with a bounded fallback,
  and a verdict a human may overrule only by a recorded decision naming the rows.
- **the data plane and the instrument** - `docs/tasks/artifacts/canvas_spike/` holds the gates, the
  applicability table and the rule parameters as three TSVs, plus a `results.tsv` that is empty on purpose:
  `run_spike_verdict.sh` prints `PENDING` with exit=0 until `G1-SLICE.13` measures, refuses a data set that
  cannot produce a verdict, and otherwise prints the rule that decided each profile. Tightening a threshold
  is therefore a diff a reviewer sees, which the GATES-READ arm pins by changing the verdict.
- **the probe suite** - `run_spike_verdict_probes.sh` -> `13 pass / 0 fail` over twelve synthetic result
  sets whose verdicts are known in advance: a tie broken by memory, the fastest topology losing to
  `snap_exact = no`, a faster native-only winner giving way to one renderer for both profiles (R4), a
  profile with no survivor escalating to the `dev-shell` fallback (R5), an unmeasured gate, an undeclared
  topology, a missing row, a control and a missing plane. Six arms failed first against a correct
  instrument because each omitted a row the applicability table declared - recorded as the slice's lesson.
- **the consumer** - `G1-SLICE.13`'s acceptance now names the instrument by path, the corpus by its
  declared numbers, and requires the hardware, OS versions and corpus script identity in the results file,
  because a verdict is scoped to them.
- **the dev-notes ledger rolled over** in the commit whose append crossed it (`devnotes-part4`, 42 lines /
  3706 bytes, digest reproduced by `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`).
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `18 suite(s) green`; `make book` ->
  exit=0; all seven book censuses unchanged; containment `OK - 17 surfaces, 15 routes, 99 files measured`

## STITCHCAD-G0-0010 - the dialects are a closed registry, not a format with flags (leaf `G0-CONTRACT.10`)

ADR-0004 was the last of the four ADRs with neither a record nor a chapter, and three chapters already leaned
on the missing one: sixteen glossary entries named the leaf as their specifier, the ontology pointed at "the
interchange-dialects chapter", and the instantiation paths deferred their three modes to it.

- **the chapter** - `docs/book/src/spec/interchange-dialects.md` (303 lines / 21 650 B): six axes an export
  target is a tuple over, each naming the party that resolves it; a **closed** registry of four targets, so a
  tuple nobody validated is refused naming the nearest one; the seventeen-layer table in both naming modes
  with this project's object mapping, and the named mode's loss of separation declared instead of discovered
  by a partner; cut-as-1 against sew-as-1 as a profile mapping recorded in three places and never a writer
  default; one BLOCK per piece with SST and PST mandatory on the ASTM path; one polyline-only entity set for
  both releases, arcs travelling exactly as bulges and Beziers tessellating at T2's chordal bound; three
  grading carriages, each its own artifact and validation; HPGL and PDF; and the receiver-config record that
  turns a dispute into a comparison of fields. D6673-10's withdrawal is recorded with the convention
  implemented as de-facto and no conformance claim anywhere.
- **the instrument** - `run_interchange_census.sh` reads the layer list out of `ROADMAP.md` itself, so
  `17 layers / 4 targets / 12 entities / 0 failure(s)` is a closure against the roadmap and not against a
  list kept beside it; the axes and the registry columns are checked in both directions, the entity policy's
  floor and ceiling are pinned, and every diagnostic and link resolves. `run_interchange_probes.sh` ->
  `13 pass / 0 fail`, including ROADMAP-GROWS, which adds a layer to a *copy* of the roadmap and requires
  the refusal - a probe has no business editing a file the director owns.
- **what the chapter deliberately does not specify** - SST and PST field content. The syntax is
  case-sensitive and receiver-specific, so a field table written at G0 would be a guess in a normative font;
  G6 is the only oracle, and layers 84-87 stay absent for the same reason rather than carrying placeholder
  curves no design authored.
- **decisions** - `docs/decisions/decision_adr-0004-interchange-dialects.md` records ten decisions with the
  alternative each rejected, what was read (Wikipedia's DXF article: the published specification is
  incomplete, which is why the oracle is a receiver) and what was attempted and not read.
- **glossary** - five new terms, sixteen entries repointed from this leaf to a chapter clause, the A-Z index
  re-derived: `294 terms / 8 parts / 155 tokens / 0 failure(s)`.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `17 suite(s) green`; `make book` ->
  exit=0; fixture, formula-language, matrix, standards, uncertainty and coverage censuses all green;
  containment `OK - 17 surfaces, 15 routes, 91 files measured`

## STITCHCAD-G0-0009 - the formula language, and the drafting system named with it (leaf `G0-CONTRACT.9`)

ADR-0003 had two halves and the repository held neither: roadmap §5 requires the recipe's expression language
"specified HERE, not later", and §11's G0 exit clause names it. Both are written now, and every number in the
chapter is computed by a tracked evaluator rather than typed beside it.

- **the language, in three parts** - `docs/book/src/spec/formula-language.md` carries the contract (eight
  kinds, nine name origins, declaration-order evaluation, exact rational arithmetic with two declared rounding
  points, twelve diagnostics, four structural limits, the exclusions); `formula-language/grammar.md` the
  syntax (the EBNF, literals and their seven unit tokens, the display and canonical forms, the operator,
  function and selector tables); `formula-language/examples.md` the evidence (17 bindings, 4 assertions, 13
  refusals over the reference skirt). 308 / 247 / 92 lines, each inside the `book_collection` per-part health
  of 400 / 24 576: the single file this replaced was 599 lines / 37 317 B, which is 87 % of the ceiling on the
  day it was born, so the containment doctrine's own remedy for a partitioned surface applied - and the parts
  table is censused in both directions, so a fourth file cannot appear unlisted.
- **the instrument** - `run_formula_language_census.sh` carries a reference evaluator that reads the chapter's
  OWN tables (kinds, unit ratios, the product law, the signatures) and type-checks every example with them:
  `17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`, twelve names cross-checked against the fixture
  chapter (the D27 class, derived rather than read), the fixture's four oracles holding as `assert`
  statements, and each refusal raising the token its row names. `run_formula_language_probes.sh` ->
  `15 pass / 0 fail`, with a CONTROL arm keeping an unrelated prose edit green so the RED arms are not
  vacuous.
- **the named drafting system** - Aldrich's metric pattern cutting. The four rejected candidates carry what
  was actually read on this machine: Seamly2D's repository is GPL-3.0, so its blocks would be a projection of
  GPL code and no independent oracle; Müller & Sohn was the least verifiable from here (an interstitial, no
  bibliographic record); Armstrong's record carries no metric claim; FreeSewing is an archived monorepo of
  individually authored designs. The method is adopted and the text is not, nothing has been read yet, so
  every number that will come from it is `unverified-with-owner` - and `G3-GRADING.16`, created by this slice,
  ships the blocks, because a decision with no owner is a wish.
- **decisions** - `docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md` records
  fourteen language decisions with the alternative each rejected, the 20x margin the node limit is derived
  against, and the three conditions that would re-open it.
- **glossary** - twelve new terms and two entries updated (`reference drafting` and `block (pattern)` now cite
  the decision instead of parking it in a leaf), the A-Z index re-derived: `289 terms / 8 parts / 153 tokens /
  0 failure(s)`. The census caught a collision while it was being written: the grammar's metavariable `T` was
  already the `T-notch` entry's token, so one token had two meanings and the rule that forbids it was
  satisfied. Notation is now italic and carries its own table - a code span is a claim that the span is a
  machine token.
- **D48 logged and fixed** - `docs/tasks/G3-GRADING.md` carried two `## Acceptance Checklist` headings (D15's
  class, by heading instead of by box) and a children range that still said `.14` while `.15` was in the file.
  **D36's third instance** recorded and removed in the same pass: `LIVE_STATUS.md` listed D47 open after
  `SPINE.20` had closed it, and its probe-suite count is now the command's rather than a hand-kept number.
- **both ledgers rolled over in the commit that crossed them** - `devnotes-part3` (50 lines / 4723 bytes) and
  `changelog-part8` (slices 41-42) sealed under the descriptor contract, each digest reproduced by
  `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `16 suite(s) green`; `make book` ->
  exit=0; fixture `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`; matrix `105 rows /
  29 diagnostics / 0 failure(s)`; standards `6 registered / 0 failure(s)`; uncertainty `107 markers /
  13 files / 0 unowned`; containment `OK - 17 surfaces, 15 routes, 90 files measured`

## STITCHCAD-G0-0014c - a delegated decision is bounded, and what the project does not know is derived (leaves `G0-CONTRACT.14c`, `G0-CONTRACT.19`)

The director's instruction was to decide and act on the three findings. The first was that the engineer proposed
the roadmap amendment and then applied it; the second was that three seats are empty. Both are now governed
rather than reported.

- **governance §6.1, decisions made under delegation** — five rules for the case this project actually runs in:
  record the author and the applier and say so when they are the same party; the author of a decision may never
  approve the evidence that decision requires (where no independent reviewer exists the claim stays
  **unapproved**, and is recorded as unapproved); the consequences become instruments others can run; the record
  states what would reverse it and who may; and a delegation to decide is not one to upgrade evidence. A
  disclosure decays with the conversation it was made in - a rule is read by whoever acts next
- **the rule bites in three verifiable places**: the roadmap's Appendix A v0.3 entry now states that author and
  applier were the same party (`grep -c 'same one' ROADMAP.md` -> `1`); `G3-GRADING.14`'s acceptance withholds
  the exit review from the criterion's author; and `decision_self-application-under-delegation.md` carries the
  reasoning with the v0.3 instance as its founding measurement. The bound is on CERTIFICATION, not on action -
  waiting for an approver would stall the project, and letting the author certify would make every gate a
  formality
- **the revision-aware baseline caught this slice's own roadmap edit**, which is the mechanism working rather
  than a nuisance: four lines of disclosure grew v0.3 from 947 to 951 lines and `check_live_doc_size.sh`
  refused it (`transition debt WIDENED on lines (951 > baseline 947)`). Handled by the rule - re-based to v0.3's
  final state with the authority cited in the row's notes - and the limit it exposed is recorded rather than
  hidden: `at=<revision>` binds a baseline to a revision MARKER, not to a commit, so the real control is that
  every re-base is visible in a diff beside the content that moved it
- **G0-CONTRACT.19, the uncertainty census**: `bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh`
  -> `uncertainty census: 88 markers / 10 files / 0 unowned / 0 failure(s)`, enumerating every marker in the
  book's own vocabulary (`assumed`, `unknown`, `unverified-with-owner`, `read-in-repo`,
  `cited-from-roadmap`, `read-external`, `known`, `derived`, `(proposed)`, `vacant`) per file and per resolving
  authority, and refusing a blocking marker whose verification-status section names no resolver. So the day a
  name arrives, the work it unblocks is one command away, and an edit that quietly drops an `assumed` from a
  fixture constant changes a count somebody reads
- **six probe arms** -> `probes: 6 pass / 0 fail`: an owned-claim control, an unowned synthetic chapter refused
  by name, a chapter added AFTER the census was written refused (the rule is about the population, not today's
  files), a definition left alone (U1 reads verification-status sections, because deciding that any mention is a
  claim would be a classifier guessing at meaning), and an absent book refusing with exit=2
- **building it found two defects in existing instruments, both fixed here**: the glossary census's `resolve()`
  deleted one `/x/../` per gsub pass and so reported `../../governance.md` - a file that exists - as a dead
  reference (it now normalises segment by segment -> `277 terms / 8 parts / 146 tokens / 0 failure(s)`); and
  this census's first authority list counted a bare gate id as an owner, so "frozen as a golden at G2" satisfied
  it and the UNOWNED arm passed for the wrong reason until the list was narrowed to parties that can RESOLVE a
  claim. A census whose own probe arm passes for the wrong reason is the vacuous-green class, fifth instance
- the token census fired a sixth time at authoring time: `vacant` is a status an instrument greps for, so it
  became a glossary term (`vacant seat`, ⚠, owning `vacant`) with the A-Z index re-derived
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `15 suite(s) green`; `make book` ->
  exit=0; containment `OK - 17 surfaces, 15 routes, 89 files measured`; matrix 105 rows, standards 6
  registered, fixture 20 rows / 4 checks / 5 pieces, coverage 13 trees / 3 siblings - all 0 failure(s)

