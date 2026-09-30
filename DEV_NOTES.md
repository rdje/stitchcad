# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

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

## _(2026-09-30)_ — an arm that removes the rule along with the breach reports a green census

- A probe arm meant to prove "a declared disposition no cell carries is refused" replaced **every**
  occurrence of the token — its five matrix cells *and* the row that declares it. With the declaration gone
  the rule had nothing to compare against, the census printed `0 failure(s)`, and the arm failed only because
  it expected a refusal. The trap `TOOLBOX.md` records is an arm that removes one instance instead of the
  property; this is its mirror, and quieter, because the instrument is green. **Before mutating, ask what
  else in the file the pattern matches** — a token's uses and its declaration are usually both in scope, and
  `replace-all` is a whole-file edit wearing a probe's clothes.
- Its sibling in the same suite: an arm whose mutation quoted a sentence as it read in a draft, not as it
  wraps in the file, so the mutation refused to apply and the arm reported both "did not apply" and its own
  failure. A `mutate` helper that exits nonzero on an absent pattern is what turned a silent no-op into a
  named one; without it the arm would have passed against an unmutated copy.
- **A trigger is worth what it names.** D49 declared "the first slice that leaves a tree file within 15 % of
  its byte ceiling" as the moment to act, and this slice found the evidence sibling at exactly 1000 lines and
  96 % — so the sealing happened with room to choose what to seal, instead of in the commit that breached.
  A pressure recorded with a number and a trigger is a scheduled action; the same pressure recorded as a
  worry is a defect waiting for a worse moment.

## _(2026-09-30)_ — a synthetic input is a fixture, and it must satisfy every rule but the one under test

- Six of twelve new probe arms failed against a **correct** instrument, all for one reason: each synthetic
  result set omitted a row the applicability table declared applicable, so the refusal printed was the
  completeness rule and not the one the arm meant to test. The arms were wrong, not the tool. **When an
  instrument has a completeness rule, every synthetic input must be complete except for the single property
  the arm removes** — and the way to know is to read the refusal the arm produced rather than its exit code.
  This is the sibling of the trap `TOOLBOX.md` records (an arm that removes one instance instead of the
  property): there the arm passed for the wrong reason, here it failed for one, and only the second is
  visible without reading the output.
- **An empty data file is a verdict, not an error.** The spike instrument prints `PENDING` with `exit=0`
  while `results.tsv` is empty, because the honest state at G0 is "no measurement exists yet" — and that is
  what keeps the decision record's canvas half `proposed`. A tool that fails on absent data teaches authors
  to put data in it, which is how a protocol written to prevent an argument ends up supplying one.
- **An ADR may be recorded in two states at once.** ADR-0002 is `active` for the chrome, the dev shell and
  the TypeScript ban and `proposed` for canvas hosting, in the status line rather than in a footnote: a
  decision whose evidence does not exist yet is still a decision *structure*, and the difference between
  "settled" and "ruled, awaiting measurement" belongs where a reader cannot miss it.

## _(2026-09-30)_ — a RED arm asserts the rule's refusal, and nothing beside it

- Two of thirteen arms failed on a correct instrument, and both failed the same way: they asserted what the
  author expected the mutation to do rather than what the rule owes. One required a *second* symptom — an
  invented layer name displacing `DRAW`, when another row still used `DRAW` legitimately; one cited
  `units §9` as a dead clause when §9 exists, so its mutation was not a breach at all. **An arm's
  expectation is the refusal and only the refusal**; a second condition makes the arm a test of the
  mutation's side effects, which is the mirror of the trap `TOOLBOX.md` already records (an arm that
  removes one instance instead of the property). Both shapes print the same thing: a red suite that means
  nothing is broken.
- **The arm that matters most mutates a file the chapter never mentions.** ROADMAP-GROWS copies
  `ROADMAP.md`, adds a layer to the convention, and requires the census to refuse by number — which is the
  only demonstration that the census reads the roadmap instead of a list hardcoded beside it. A probe has no
  business editing a tracked file the director owns, so it edits a copy, and the copy is the point.
- **A wrapped bullet is a population, not a line.** The first cut of that census read the first line of the
  roadmap's layer bullet, found `3` of `17` layers, and reported `21` chapter breaches. The chapter was
  right and the instrument was measuring one line of a paragraph — the same class as the backtick scanner
  that read a whole file as one code span. When a census reports breaches in bulk against a document a human
  just checked, suspect the reader before the document.

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

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/stitchcad-devnotes-part1.md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/stitchcad-devnotes-part2.md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/stitchcad-devnotes-part3.md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/stitchcad-devnotes-part4.md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/stitchcad-devnotes-part5.md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

