# Sealed archive — StitchCAD changelog, the i18n slice

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2a` on `2026-10-01`.

- **Sealed identity:** 38 lines, 3631 bytes, `sha256:2c8957804dd5b9919ed20f8bd85814cb8a04b4da5b4c9a1faa9794929e0c7a6e`
- **Coverage:** `STITCHCAD-G0-0016`, copied from the oldest live entry; no hand-kept slice range.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

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
