# One message system: Fluent at both ends, because the boundary is where a second system appears

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute); roadmap §7.6 requires the choice at G0 and refuses "Fluent or ICU" as
  an answer
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.16`, from roadmap §7.6, §2.9, §11 G0 and G5; the normative text is
  `docs/book/src/spec/i18n-architecture.md`

answers: "which message system does StitchCAD use?" · "why not ICU4X?" · "what is a message id?" · "what must be reviewed before a language pack ships?" · "may geometry mirror in an RTL locale?" · "what would reopen the i18n choice?"

## The decision

**The one message system is Fluent, at both ends** — `fluent-rs` in the Rust core and `fluent.js` in the
TypeScript chrome, over one catalogue format. Four rules follow and are normative in the chapter:

1. **The message id is the diagnostic's stable token.** No numeric ids, no slugs derived from English
   sentences, no second numbering to keep in step. A token change is an API break, not a copy edit.
2. **Arguments are typed and carry units; text is a presentation field.** A message never receives a
   pre-formatted string, so a locale or a unit change re-renders rather than re-derives, and an agent reads
   tokens and arguments and never a sentence.
3. **Review is tiered and the two tiers that reach fabric are absolute.** `strict` (units, construction
   geometry, approval and release) and `safety` (notch types, the cut/sew family, fold, allowance
   ownership) ship only at 100 % translated *and* reviewed — the first by two reviewers, the second by the
   domain seat. `standard` ships fully translated with review allowed to follow, and the pack's report names
   what is unreviewed.
4. **RTL mirrors layout and never geometry**, and that is a test rather than an intention: an RTL render and
   an LTR render of one design produce one canonical byte string.

## Why Fluent, and what ICU4X loses on

The evidence read on `2026-09-30` is tabulated in the chapter's §2 (GitHub's repository API, the crates.io
API, and ICU4X's own LICENSE file): both Fluent implementations are Apache-2.0 and unarchived, ICU4X is
Unicode License V3, unarchived, pushed the day it was read, and describes itself as solving i18n for
client-side and resource-constrained environments — which is this product's browser profile. So the
licences are all compatible with ADR-0001 and both projects are alive. The decision is therefore not about
quality; it is about **the boundary**:

| Consideration | Fluent | ICU4X |
| --- | --- | --- |
| one catalogue format across the Rust core and the TS chrome | yes — `fluent-rs` and `fluent.js` read one FTL catalogue | no in practice — ICU4X in Rust, the platform's Intl in the browser, two dialects |
| licence against ADR-0001 | Apache-2.0 | Unicode License V3, permissive |
| selectors for plurals and gender | in the format | in the library |
| id space | flat and textual, so a token is an id | message ids plus a formatting layer |
| locale-data and plural-rule completeness | **not established here** — `unverified-with-owner`, G5 owns it | CLDR-derived, its strongest claim |

§7.6's own warning is the tie-break: "if both ends are needed, a designed bridge". Choosing ICU4X would
need exactly that bridge on day one, for the product's own chrome; choosing Fluent needs none, and keeps
the bridge in reserve for the one thing Fluent's coverage has not been shown to do.

## Rejected alternatives

| Rejected | Why |
| --- | --- |
| ICU4X as the message system | two message dialects across the Rust/TS boundary, which is the failure §7.6 exists to prevent |
| "Fluent for messages, ICU4X for formatting", adopted now | it is a bridge, and a bridge is what you build when a measured gap demands it — not what you build because both libraries are good |
| a numeric or slug-based message id | a second numbering to keep in step with the tokens, and a slug derived from English copy changes when the copy does |
| passing pre-formatted strings into messages | a string cannot be re-formatted for another locale or unit, and it hides the value's kind from the type check |
| shipping a pack with an unreviewed safety term and a warning badge | a badge is presentation; the wrong word for "net line" reaches a cutting room either way |
| review thresholds as percentages | a 90 % reviewed `safety` tier means one message in ten may say the wrong thing about a cut line, and the tier exists because that is unacceptable |

## How to apply

- G5 builds the review queue (or substitutes the GitHub workflow §9 permits), ships the first pack against
  §9's thresholds, and reports the tier of every message it ships.
- The lint of §5 is a CI gate from the first UI string, with its five exemptions exercised by cases that
  must pass — an exemption nobody tests is an exemption that grows.
- The inventory of §10 is derived: a new diagnostic in any chapter is a new message id, and the census
  refuses an id the inventory does not cover. Do not keep the counts by hand.
- Pseudolocalization runs on every surface in CI, before any translator exists, because the four defect
  classes it catches are cheap to fix then and expensive after a pack ships.

## What would reverse it

Two conditions, each needing a recorded decision: (1) the first language pack exposes a plural-rule or
locale-data gap in the Fluent Rust stack that a fix upstream will not close, in which case the designed
bridge is built — ICU4X for locale data behind Fluent messages, with the messages staying one system;
(2) the chrome's front-end constraints make `fluent.js` unusable there, which would put a second dialect on
the other side of the boundary and re-open the choice on the same criterion that made it.
