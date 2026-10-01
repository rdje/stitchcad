# Sealed archive — StitchCAD dev notes, two oldest live lessons

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2a` on `2026-10-01`.

- **Sealed identity:** 35 lines, 3196 bytes, `sha256:04ab285cf192ac917e7ca364fbdef082bed60eb25f5bb7993d827ddb928a00ae`
- **Coverage:** the two oldest live `2026-09-30` lessons (parsing source layout; deriving the i18n population), copied in original order.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-09-30)_ — an instrument that reads another document's prose must normalise it first, and must fail closed when it reads nothing

- The command-layer census parses two lists out of `ROADMAP.md`: the backticked commands in §4.4 and the
  slash-separated authority levels in §7.8. The first run parsed **five commands and zero levels**, then
  reported five invented levels in the chapter — a verdict that was entirely about the reader. The list
  wraps mid-item (`generate /` newline `approve`), and the character class did not include a newline. The
  fix is one `re.sub(r"\s+", " ", …)` before the match, and it is the third time this session an instrument
  mis-read a wrapped structure: the interchange census read one line of the roadmap's layer bullet and
  reported 21 chapter breaches, and a code-span scanner paired backticks across a fence.
  **Prose in a source document is a population with a layout; parse the layout away before parsing the
  population.**
- What made the bug visible instead of silent is the guard that refuses a zero-length population: the census
  calls `bad()` when it parses fewer items than the clause it reads is known to carry, so "I read nothing"
  is a failure and not a green run over an empty set. Every instrument written this session now carries that
  guard, and it is the cheapest line in each of them. Its mirror is the arm that proves the guard works —
  ROADMAP-GROWS and CODE-GROWS mutate the *source* document in a copy and require the refusal, so a reader
  that stops reading is caught by a probe rather than by a reviewer.

## _(2026-09-30)_ — a count you just wrote by hand is already wrong if the population has a shape you did not grep for

- The i18n chapter's message inventory was written from four chapters' diagnostic tables and one crate's
  error enum, and it was wrong twice before it ever ran: the envelope's 29th token (`geom_offset_budget`)
  had been folded into the `env_*`/`ngo_*` family, and `UnitError` was counted at four variants when it
  carries five. The fifth, `EmptyDerivation`, has **no braces** — a first grep for `Variant {` counted the
  four struct variants and reported a clean answer. **A population enumerated by one member's shape is a
  population minus the members that differ**, and the missing member is invisible precisely because the
  count looks plausible.
- The fix was not a better grep but a different rule: the census derives the population by SHAPE
  (any snake_case token in a diagnostic table's first cell) and derives the families from the chapter's own
  inventory, so a sixth prefix appears by itself. A list of prefixes kept beside the chapters is the same
  hand-kept number in a different file — and it is what the first cut had.
- **An instrument written in the same slice as the document still earns its cost.** Both errors were caught
  before the chapter landed, by a census written after it; the alternative was a reviewer reading a table of
  eight counts and believing six of them. Where a chapter publishes a count of anything, the count is derived
  in the same commit or it is marked as unverified — there is no third state where it is simply typed.
