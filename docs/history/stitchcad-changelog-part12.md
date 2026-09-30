# Sealed archive — StitchCAD changelog, the dialects and formula-language slices

Immutable historical segment, sealed out of `CHANGELOG.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `G0-CONTRACT.15` in the commit whose append
crossed the window's health target.

- **Sealed identity:** 84 lines, 7767 bytes, `sha256:9c61ba7c3f5435238fc6184dafba35570bc772fc0fbe2e8fecc9faaacb03e549`
- **Coverage:** `STITCHCAD-G0-0010` and `STITCHCAD-G0-0009`, newest first, exactly as they stood in `CHANGELOG.md`. No slice range is declared, for the reason `stitchcad-changelog-part9.md` records (defect D51).
- **Sealed by:** leaf `G0-CONTRACT.15` on `2026-09-30`, after `stitchcad-changelog-part11.md`.
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.
- **Write policy:** none — sealed segments are immutable.

---

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
