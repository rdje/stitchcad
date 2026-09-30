# Internationalization

> **Status:** normative specification, gate **G0** (roadmap §7.6: "one message system, chosen at G0",
> and §11's G0 exit clause: "one message system chosen (§7.6) + externalization architecture").
> The architecture is normative now; the first shipped language pack is G5's, with the review thresholds
> §9 declares. Terms are defined in the [glossary](glossary.md).

Roadmap §7.6 asks for two things at G0 and defers a third: **one** message system, chosen; the
externalization architecture, written; and the shipping — a review queue, full RTL, a first non-English
pack — staged to G5. This chapter does the first two and fixes the thresholds the third is measured
against, because a staged deliverable with no threshold is a deliverable nobody can fail.

The governing idea is the one §7.6 states and the whole model depends on: **the API returns a stable
diagnostic code with typed arguments and units, and localized prose is a presentation field.** An agent
never parses a translated sentence, a golden file never contains one, and a translation can never change
what a diagnostic means.

## 1. What is externalized, and what is not

| Externalized | Never externalized |
| --- | --- |
| every sentence a human reads: diagnostics, labels, panel text, the tech pack's prose, this book's future UI strings | a machine token — a diagnostic code, a kind, a layer name, a state, a piece id |
| the human-facing name of a measurement, a piece, a notch type | the canonical project format, in any field (units §7) |
| number, date and unit *presentation* (units §2.2) | the stored value of anything, which is locale-independent (§6) |
| the termbase's translations (§4) | a formula, a token table, a file name, a layer number |

The rule that makes the boundary checkable: **a string is externalized if and only if a human reads it.**
A machine token rendered raw to a user is a defect the glossary already names; a translated string inside
a canonical file is a defect this chapter names (§6), and both are lint refusals (§5).

## 2. The one message system

**The message system is Fluent**, at both ends: `fluent-rs` in the Rust core and `fluent.js` in the
TypeScript chrome, over one catalogue format (FTL). The choice is recorded with its evidence in
`docs/decisions/decision_i18n-one-message-system-fluent.md`, and the short form is:

- one catalogue format serving both ends is what §7.6 means by *not* needing a bridge — two systems, or
  one system with a dialect per end, is the failure the clause exists to prevent;
- both implementations are permissively licensed (`fluent-rs` and `fluent.js` are Apache-2.0), so
  ADR-0001's rule is satisfied without a feature flag;
- the message format carries selectors, which is what a plural or a gendered noun needs, and the id space
  is flat and textual, so a diagnostic token *is* a message id (§3) with no mapping table to drift.

**ICU (ICU4X) is the rejected alternative and stays a named one**: it is active, it is licensed under the
Unicode License V3, and it describes itself as solving i18n for client-side and resource-constrained
environments, which is this product's browser profile. It loses on the boundary, not on quality: the Rust
side would be ICU4X and the browser side would be the platform's Intl API, which is two message dialects in
practice. If the plural-rule or locale-data coverage of the Fluent stack proves insufficient when the first
pack ships, the remedy is a **designed bridge** — ICU4X for locale data behind Fluent messages — recorded
as a decision, not an ad-hoc second system (§12).

What was read, on `2026-09-30`, from the GitHub repository API, the crates.io API and the licence file
itself — the evidence for the two licence claims and for "active", and none of it a claim about coverage:

| Crate or repository | Licence | As read |
| --- | --- | --- |
| `fluent` (crate) | Apache-2.0 | `0.17.0`, released `2025-05-22` |
| `fluent-bundle` (crate) | Apache-2.0 | `0.16.0`, released `2025-05-22` |
| `fluent-rs` (repository) | Apache-2.0 | not archived, pushed `2026-09-27` |
| `fluent.js` (repository) | Apache-2.0 | not archived, pushed `2026-05-22` |
| `icu` (crate) | Unicode License V3 | `2.3.1`, released `2026-08-20` |
| `icu4x` (repository) | Unicode License V3 | not archived, pushed `2026-09-30` |

## 3. Message identity

- **The message id is the diagnostic's stable token.** `env_notch_type` is the id; there is no numeric id
  behind it, no slug derived from an English sentence, and no second numbering to keep in step. A token
  that changes is a breaking change to the API contract, not a copy edit.
- **Arguments are typed and carry units.** A length argument is a length in internal units and is
  formatted by the units chapter's rules at presentation time; a message never receives a pre-formatted
  string, because a string cannot be re-formatted for another locale or another unit.
- **A message's text is a presentation field.** The canonical record of an event carries the token and its
  arguments; the sentence is rendered on demand. Two renders of one event in two locales are the same event.
- **The id space is closed and derived.** §10 inventories every id this product can emit, grouped by family
  with the source table each family comes from, and the census compares the inventory with those tables —
  so a new diagnostic in any chapter is a new message id, and an id nobody declared cannot be shipped.

## 4. The termbase per language

The [glossary](glossary.md) is the source termbase: one meaning per term, its synonyms (including the
factory and other-CAD aliases and the German forms it already records), its machine token, and its ⚠ mark
where mistranslation causes a wrong cut or a rejection.

- **A shipped termbase is a projection of the glossary, not a second list.** For each target language: the
  term, its translation, the ⚠ flag, the machine token it must never be confused with, and the reviewer who
  approved it. The glossary's census already derives the token side; the termbase adds the language.
- **Safety-relevant terms come first**, in the order §7.6 names them: notch types, then the sew/cut line
  aliases factories use in rejection emails, then units and construction vocabulary. A pack that ships with
  an unreviewed ⚠ term does not ship.
- **A term with no approved translation is not translated.** It stays in the source language and is listed
  in the pack's own report, because a guessed translation of "net line" is how a cutting room receives a
  pattern it reads as a cut line.
- **Review is the domain seat's, not the maintainer's** ([governance §1](../governance.md), §3): a termbase
  entry is domain knowledge, and the seat that owes it is **vacant** ([governance §8.1](../governance.md)),
  so no ⚠ term can be approved today and §9's thresholds say what that blocks.

## 5. The externalization lint

A CI lint fails the build on an inline user-facing string. Its rule and its exemptions are declared here
because a lint with undeclared exemptions teaches authors to add one:

| Exemption | Why it is not a user-facing string |
| --- | --- |
| a machine token, a diagnostic code, a kind or state name | never rendered raw to a user (§1); the glossary owns it |
| canonical project content and any serialized field | locale-independent by rule (§6) |
| a test fixture, a golden file, a probe's expectation | an expected byte string is data, not copy |
| a developer-facing log line at debug or trace level | not part of any user surface; the release build does not emit it |
| a file name, path, URL, or a format's own keyword | the format's vocabulary, not ours |

Three rules make the lint worth running: a string literal in a UI, CLI or MCP surface that is not a message
id is a refusal; a message id used with no catalogue entry is a refusal; and a catalogue entry no code
references is reported, because a dead message is a translation somebody will pay for.

## 6. Locale-independent canonical files

**A decimal comma in input does not change the stored meaning of a value.** The boundary is exact:

- **Input** is parsed by the locale the user declares, and `2,5 cm` in a decimal-comma locale is
  two and a half centimetres — one conversion, one rounding, into internal units (units §2).
- **Storage** is locale-independent: internal integers, canonical key order, no localized separators, no
  localized names, no locale tag inside a value (ontology §7).
- **Output** for a human is formatted per locale; **output for a machine** — a canonical file, an artifact,
  a manifest, a golden — is formatted per the units chapter and never per locale.
- **A file carries no implicit locale.** Where a locale matters to a reader (a printed label, a tech pack),
  it is an explicit field of the artifact, recorded in the receiver-config record
  ([interchange §10](interchange-dialects.md)) so a dispute is about data.

The consequence for tests: two users in two locales producing the same design produce byte-identical
canonical files, and a save/load round trip in a different locale than the save changes nothing.

## 7. Pseudolocalization

Pseudolocalization is a build mode, not a language: every message is rendered with an accented, expanded
substitution and every argument with a marker, so four classes of defect appear without a translator.

| It catches | How |
| --- | --- |
| a hardcoded string | it stays in English while everything around it is pseudolocalized |
| truncation | the substitution is ~1.5× the source length, so a clipped label is visible |
| concatenation | two adjacent ids render as two bracketed groups where one sentence was meant |
| an argument rendered as text | the marker shows a value that should have been typed |
| a direction assumption | the mode has an RTL variant that mirrors layout only (§8) |

It runs in CI on every surface, and a pseudolocalization failure is a build failure: the point of running
it before a translator exists is that the fixes are cheap then and expensive after a pack ships.

## 8. RTL: mirrored layout, never mirrored geometry

- **Layout mirrors.** Reading order, panel order, text alignment, icon asymmetry that implies direction,
  and the sweep of a slider follow the locale's direction.
- **Geometry never mirrors.** Grain, winding, orientation, the meaning of left and right on a piece, the
  CCW boundary rule (ontology §4.1), a mirrored pair's L/R labels and a notch's position are properties of
  fabric and of the piece's own frame, not of a reading direction.
- **The rule is checkable, and it is checked:** a render in an RTL locale and the same render in an LTR
  locale produce identical canonical geometry — one byte string — and differ only in presentation. A
  difference in geometry is `i18n_geometry_mirrored`, which is the defect class this section exists for.
- **A piece label keeps its own vocabulary.** "Place on fold" is translated; `skirt_back_left` is not, and
  the ⚠ terms the glossary marks are the ones a reviewer confirms before a pack ships (§4).

## 9. Shipping stages and review thresholds

Staged as §7.6 requires, with the threshold that makes each stage pass or fail:

| Tier | Messages | Threshold to ship a pack |
| --- | --- | --- |
| `strict` | units and conversion, construction geometry, approval and release | 100 % translated **and** reviewed by two reviewers — the domain seat and the maintainer |
| `safety` | notch types, the cut/sew line family, fold and place-on-fold, allowance ownership | 100 % translated **and** reviewed by the domain seat |
| `standard` | everything else a user reads | 100 % translated, review may follow, and the pack's report names every unreviewed message |

Three rules bound the staging:

- **A pack with an unreviewed `strict` or `safety` message does not ship**, and the refusal names the
  message. Today that is every pack: the domain seat is **vacant** ([governance §8.1](../governance.md)),
  so the first non-English pack is blocked on a name, not on translation work — which is why §12 records
  the blockage as a status and not as a plan.
- **The review queue, full RTL and the first shipped pack are G5 deliverables**; a GitHub-based review
  workflow may substitute for a custom portal early, and substituting it changes nothing about the
  thresholds.
- **MCP locale-awareness is explicitly not a gate** (§7.6): an agent receives tokens and arguments, which
  are locale-free by construction (§3), so there is nothing to localize and nothing to gate.

## 10. The message inventory

Every id this product can emit, by family, with the table it comes from. The counts are not kept by hand:
`docs/tasks/artifacts/i18n/run_i18n_census.sh` re-derives them from those tables and from
`crates/sc-units/src/error.rs`, and refuses an id any table declares that this inventory does not cover.

| Family | Ids | Source of the ids | Review tier |
| --- | --- | --- | --- |
| `env_*` | 21 | [envelope §10](feature-matrix.md) | `safety` where the row is a notch, a cut/sew or an allowance refusal, else `standard` |
| `ngo_*` | 7 | [envelope §10](feature-matrix.md) | `standard` |
| `geom_*` | 1 | [envelope §10](feature-matrix.md) | `standard` |
| `formula_*` | 12 | [formula language §5.2](formula-language.md) | `strict` |
| `dialect_*` | 5 | [interchange §11](interchange-dialects.md) | `safety` for the line-semantics rows, else `standard` |
| `release_*` | 8 | [release §9](release-contract.md) | `strict` |
| `unit_*` | 5 | `crates/sc-units/src/error.rs`, the `UnitError` variants | `strict` |
| `i18n_*` | 5 | §11 below | `standard` |

A family is a prefix, and every id belongs to exactly one. Two consequences of counting rather than
guessing: the envelope's §10 declares 29 tokens in **three** families — 21 `env_*`, 7 `ngo_*` and the one
`geom_offset_budget` — and the two `env_*` tokens that [interchange §11](interchange-dialects.md) re-raises
are the envelope's, which is why that chapter's family counts 5 and not 7.

## 11. Diagnostics of this layer

| Token | Raised when | Required arguments |
| --- | --- | --- |
| `i18n_message_missing` | a message id has no catalogue entry in the active locale and no fallback declared | the id, the locale, the fallback chain |
| `i18n_argument_type` | a message is given an argument whose kind its declaration does not accept | the id, the argument, the kind wanted, the kind given |
| `i18n_locale_unsupported` | a locale no pack covers is requested | the locale, the packs that exist, the fallback used |
| `i18n_geometry_mirrored` | an RTL render changed canonical geometry | the locale pair, the entity, the differing field |
| `i18n_term_unreviewed` | a `strict` or `safety` term would ship without its review | the term, its tier, the seat that owes the review |

## 12. Verification status of the claims in this chapter

- **The message-system choice is `read-external`**, read on `2026-09-30` and tabulated in §2: the two
  Fluent repositories and their crates are Apache-2.0 and not archived, ICU4X is Unicode License V3 (read
  from its LICENSE file) and describes itself as solving i18n for client-side and resource-constrained
  environments. What those facts do **not** establish is the plural-rule and locale-data
  coverage of the Fluent Rust stack, which is `unverified-with-owner`: `G5-SHELLS` owns it, because the
  first language pack is the first thing that would expose a gap, and the remedy is the designed bridge §2
  names.
- **The inventory counts in §10 are `derived`** from the tables and the code they cite, re-derived by the
  census named there rather than read; `run_i18n_probes.sh` is its ground truth.
- **The review tiers, their thresholds, the lint's exemptions, the pseudolocalization classes and every
  rule in §3–§9 are project decisions**, recorded in
  `docs/decisions/decision_i18n-one-message-system-fluent.md`. The tiers come from §7.6's own ordering
  ("units/construction/approval warnings strictest") and from the glossary's ⚠ marks; the thresholds are
  this chapter's and are deliberately absolute for the two tiers where a mistranslation reaches fabric.
- **The first non-English pack is blocked on a seat, not on work.** The domain expert seat that reviews a
  `safety` or `strict` term is **vacant** ([governance §8.1](../governance.md)), so §9's threshold cannot be
  met today by anybody; the population waiting on that seat is derived by
  `docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh`.

## 13. What must be true in tests

- Every id in §10's inventory resolves in the source catalogue, and every id a source table declares is in
  the inventory — both directions, on every build.
- The lint fails on an inline user-facing string in a UI, CLI or MCP surface, and each exemption is
  exercised by a case that must pass.
- Pseudolocalization renders every message and catches a hardcoded string, a truncation, a concatenation
  and an untyped argument — each by a deliberately broken fixture, not by inspection.
- Two locales producing one design produce byte-identical canonical files, and an RTL render changes no
  canonical geometry (§8).
- A decimal-comma input parses to the same internal integer as its decimal-point equivalent, and a
  save/load round trip across locales changes nothing (units §2, §6 above).
- A message given an argument of the wrong kind raises `i18n_argument_type` and never renders a sentence
  with a hole in it.
- A pack containing an unreviewed `strict` or `safety` message is refused, naming the message and the seat
  that owes the review.
