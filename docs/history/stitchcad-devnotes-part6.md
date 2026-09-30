# Sealed archive — StitchCAD dev notes, two more 2026-09-30 lessons

Immutable historical segment, sealed out of `DEV_NOTES.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `G0-CONTRACT.15` in the commit whose append
crossed the window's byte health target (200 lines / 16 384 bytes).

- **Sealed identity:** 55 lines, 5359 bytes, `sha256:129d50d8fff573e5ae50a42f23afc75d103eeb5f3a54878fc01c89329f8989e7`
- **Coverage:** the lessons `a spec's tables are its test suite, and a code span is a claim` and
  `settle an argument with the artifact, and anchor an arm on the property`, both dated 2026-09-30,
  oldest last, exactly as they stood.
- **Sealed by:** leaf `G0-CONTRACT.15` on `2026-09-30`, after `part5` (sealed by `G0-CONTRACT.16`).
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.
- **Write policy:** none — sealed segments are immutable.

---

## _(2026-09-30)_ — a spec's tables are its test suite, and a code span is a claim

- **A reference evaluator that reads the chapter's own tables turns "implementable from the chapter
  alone" from an opinion into a measurement.** The formula-language census parses grammar §1, takes its
  kinds from the contract's §2, its unit ratios from grammar §2.1, its product law from §5.1 and its
  signatures from §6, then type-checks and evaluates every worked example with them — so a function
  nobody wrote down cannot be implemented, and a signature written down wrongly reddens the run instead
  of the product. The price was regularity: `sqrt` needed two rows (`area`→`length`, `ratio`→`ratio`),
  `min` a variadic marker, and the metavariables a declared notation table, because prose signatures
  ("≥ 2 values of one kind") are not parseable. **Writing the instrument first would have made the
  tables regular from the start; writing it second cost one rewrite of two of them.**
- **A code span is a claim that the span is a machine token**, and grammar notation is not one. The
  metavariable `T` collided with the `T-notch` entry's token, so "one token, one meaning" was violated by
  *notation* — invisibly, because the collision satisfied the census (the token was owned, by a notch).
  Its sibling `N` was reported undeclared, and that is the only reason the collision surfaced.
  Metavariables are now italic and carry a notation table. Generalized: a single letter in backticks
  anywhere in this book is a token somebody owns or must own, so notation stays out of code spans.
- **An instrument that scans inline code must skip fenced blocks first.** The first cut paired backticks
  across newlines; a fence's ``` is an odd number of them, so the whole file became one span and the run
  reported 125 "undeclared operators" that were EBNF nonterminals and table separators. The count was
  large enough to look like a finding and was the instrument measuring the fences. **A refusal list that
  long is a bug report about the instrument, not about the tree** — read one entry before believing the
  total. Its sibling in the same run: a link resolver that dropped the leading `/` of an absolute base
  reported every link in the book as dead, which is the same shape wearing a different rule.
- **A chapter born at 87 % of its byte ceiling is partitioned, not trimmed.** 599 lines / 37 317 B
  against a `book_collection` per-part health of 400 / 24 576: every table was normative and the prose
  that could move had already moved to the decision record, so trimming meant cutting contract. The
  containment doctrine's own remedy for a `partitioned_canonical` surface applied — contract, grammar,
  examples, at 308 / 247 / 92 lines and each inside health — with the parts table censused in both
  directions so a fourth file cannot appear unlisted. The trigger to check is not "am I over health"
  but "how much ceiling is left on the day this is born".
## _(2026-09-30)_ — settle an argument with the artifact, and anchor an arm on the property

- D22 sat for a session as a *question* because two tracked instruments disagreed about what a markdown table
  row means: the inherited arity checker self-tests that a pipe inside a code span is not a separator, while the
  doctrine doc warns that GFM drops extra cells. Neither could settle it, and reading the CommonMark/GFM text
  would have settled nothing either — so a page was rendered. `mdbook build` over a three-column row whose
  first cell carried a raw pipe in a code span came back as `A raw pipe in a code span: ` + "`x" + ` | ` + "y`" +
  ` | `2`: the code span broken open, the cells shifted, and the rightmost cell **gone**, with no diagnostic
  anywhere. **When two instruments disagree, the oracle is the artifact they both claim to describe.**
- **The first cut of that oracle asserted a count and was wrong on the truth.** From a two-column page the split
  row yields 2 cells; from a three-column page it yields 3 with the last dropped. An arm anchored on `2` goes red
  when the renderer does something *worse* than expected, which is the same trap as a RED arm that removes one
  instance instead of the property: assert the property (first cell truncated at the pipe, rightmost cell not
  the one written), never the number this morning's input happened to produce.
- **Raise the target AND trim the rows, or do neither.** Both remaining prose-derived maxline targets were
  re-derived from their binding shapes (382 B, 443 B) and the fat they had been blamed for was removed in the
  same slice — one 491 B row became bounded prose, nine index hooks went from three lines to one, four
  verification rows were tightened. Deriving a target to fit verbosity launders the verbosity; trimming rows to
  fit a guessed target launders the guess. Doing both is the only combination that leaves the axis meaning
  something.
- **A convention in `COMMIT.md` is what authors read; a gate is what holds when they do not.** The convention is
  written where authors look, and the fact that nothing in this repository mechanically refuses a row the
  renderer truncates is logged as D47 with `SPINE.20` owning the project-slot check — the inherited checker
  stays untouched, because a NEUTRAL spine file is reported upstream and never patched locally.
