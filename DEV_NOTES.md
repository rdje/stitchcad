# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-09-30)_ — the artifact that certifies everything else must itself be derived

- A gate review written as prose is the one document whose acceptance rule it violates: `G0-CONTRACT.15`'s
  own criterion is "no clause is marked met on prose alone", and a paragraph asserting nineteen verdicts is
  nineteen prose claims wearing a table. So the review parses `ROADMAP.md` §11's exit bullet, requires every
  fragment to be dispositioned and every row to match a fragment, then **runs** each row's check and prints
  the verdict. Two seconds, nineteen clauses, and a reader who wrote none of the chapters can reproduce the
  whole gate. The general rule: **the certification layer gets the same treatment as the numbers** — a
  derived verdict, not a confident one.
- Its honest output is `18 met / 1 not met`, and the one is more useful than a clean sweep would have been:
  a review that reports 19 of 19 invites the reader to stop reading, while a named gap carries its cost
  (no receiver ever reads our artifacts back) and its owner. Related, and worth keeping: the review prints
  `closure unapproved` even when every check passes, because the party running it authored sixteen of the
  nineteen deliverables — a verdict and an approval are different claims, and conflating them is how a
  self-review becomes a certificate.

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

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/stitchcad-devnotes-part1.md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/stitchcad-devnotes-part2.md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/stitchcad-devnotes-part3.md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/stitchcad-devnotes-part4.md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/stitchcad-devnotes-part5.md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`devnotes-part6.md`](docs/history/stitchcad-devnotes-part6.md) | two `2026-09-30` lessons (a spec's tables are its test suite; settle it with the artifact) | 55 lines, 5359 bytes, `sha256:129d50d8…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

