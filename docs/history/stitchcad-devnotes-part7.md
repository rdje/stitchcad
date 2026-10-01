# Sealed archive — StitchCAD dev notes, three more 2026-09-30 lessons

Immutable historical segment, sealed out of `DEV_NOTES.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `G1-SLICE.3b` in the commit whose append
crossed the window's byte health target (200 lines / 16 384 bytes).

- **Sealed identity:** 57 lines, 5120 bytes, `sha256:13fd6c73d2bf400145816ff6c5a234b888dcfbc5ce5eaf47b99c96223c803475`
- **Coverage:** the lessons `an arm that removes the rule along with the breach reports a green census`, `a synthetic input is a fixture, and it must satisfy every rule but the one under test` and `a RED arm asserts the rule's refusal, and nothing beside it`, all dated 2026-09-30, oldest last, exactly as they stood.
- **Sealed by:** leaf `G1-SLICE.3b` on `2026-10-01`, after `part6` (sealed by `G0-CONTRACT.15`).
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.
- **Write policy:** none — sealed segments are immutable.

---

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
