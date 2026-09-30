# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

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

## _(2026-09-30)_ — a rule whose only compliant path is "don't change the file" gets bypassed

- Applying the roadmap amendment was blocked by the containment gate, and blocked *correctly*: the `roadmap`
  row's transition debt was measured at exactly the file's size (`lines=919;bytes=50821` = `wc -lc`), so any
  growth read as a widened baseline. The rule is right; what was missing was a legitimate path, and the only
  one available was editing the number — the silent widening the rule exists to prevent. **When a gate's only
  passing move is to falsify its own input, the gate is incomplete, not the change.** The fix was to give the
  baseline a revision identity (`at=v0.3`) that the checker executes: a revision may re-base its baseline, but
  only in the commit that revises, which is where the authority has to be anyway.
- **An exit criterion must arrive with owners.** The same commit that gave G3 the envelope-coverage criterion
  made `G3-GRADING.5` required, created `.15` and made `.14`'s review fail without a leaf's evidence — because
  a roadmap clause no leaf owns is defect D32 one level up, and the tree was seeded from the gate's old text so
  it inherited the gate's gap exactly.
- **A census nobody runs is a claim, and this one had gone red for two committed slices.** The tree-coverage
  census defined a tree by *filename*, so the evidence siblings the containment registry prescribes looked like
  lane-less orphans — and `make probes` globs `run_*probe*.sh`, so nothing re-derived it. Three copies of the
  same `ls ${lane}-*.md | head -1` assumption existed; the third was in an *advisory* table, where it silently
  replaced `G0-CONTRACT` with its sibling and no exit code could reveal it. Reading a tool's output, not just
  its status, is part of running it.
- **Office can be held acting; competence cannot.** Two of the three empty governance seats are the director's
  authority already, so acting costs nothing and adds a record. The third — the sewing/factory expert — is
  knowledge nobody here has, so it is recorded as **vacant** with four explicit prohibitions, and the first G2
  golden stays gated on a real name. The tempting move (let the engineer act as reviewer) satisfies the wording
  of the two-step rule and destroys its meaning.

## _(2026-09-30)_ — a digest is a contract about BYTES, so the bytes have to be written down

- Sealing the third changelog segment of the day produced a refusal that read as the worst thing an archive
  can report — "content hashes to `3148dd0f…`, its descriptor declares `8553accc…`" — and the cause was one
  newline. The verifier hashes `content=$(sed -n 'rule+2,$p' f)` followed by `printf '%s\n'`, and bash command
  substitution strips EVERY trailing newline, so a segment whose sealed content ends with a blank line can
  never reproduce a raw-byte digest. Measured: `tail -c 12` showed `…touched\n\n` on the new segment and
  `…touched\n` on the one sealed an hour earlier.
- **The general shape: two honest implementations of "the sealed content" disagreed, and nothing said which
  one was the contract.** A digest proves identity only when both sides mean the same bytes, so the byte rule
  belongs in the instrument's header, not in the author's head. Fixed by normalizing the segment, recomputing
  its descriptor with the verifier's own method, teaching the rule to refuse a trailing blank line BY NAME,
  and pinning that with an arm — because a mismatch that looks like drift in an immutable file invites the one
  edit the archive forbids.
- **Corollary for the next rollover:** seal with the verifier's method, not with the language you happen to be
  scripting in. The check is one command (`sed | shasum`), so running it before writing the descriptor is
  cheaper than diagnosing the difference afterwards. Promoted, with the day's other containment derivation, to
  `docs/decisions/decision_maxline-health-derived-from-the-cell-budget.md`.

## _(2026-09-30)_ — a blocked leaf splits into what can be drafted and what must be named

- `.14` sat blocked for a session because it needed three named humans. The director's ruling separated the
  two halves — draft everything, name nobody — and the whole chapter turned out to be draftable: a role is
  the **decision it may make**, not the person holding it, so each seat could be specified with its authority,
  its agent-eligibility and what waits for it. What was blocking was the *coupling* of the two halves, not the
  missing names. General shape: before calling a leaf blocked, ask which of its clauses actually needs the
  missing input; usually one does and the rest were waiting beside it.
- **Governance written while the room is empty is a contract; written after the first conflict it is a
  negotiation.** Roadmap §14's mitigation is literally "governance doc at G0, while the room is empty", and
  the reason shows up in the drafting: every rule that had to be invented (which changes take the domain
  path, what makes a reviewer independent, who signs a golden) is cheap to state now and expensive to state
  after somebody has a position. Promoted to
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md`.
- **A fallback that does not state its cost is how a slip becomes a silent downgrade.** The eval-seat fallback
  (a partner runs the test) is roadmap-mandated, so the only honest addition was what it costs: layer-4
  evidence with recorded product, version and settings — slower, fewer targets. Same shape as a claim with a
  missing leg: name the leg, do not hide it.
- **The empty seat with a schedule behind it is the one to name loudly.** The domain expert gates the fixture's
  `assumed` constants, which gate the first G2 golden, so the chapter says so where a plan will hit it rather
  than where a reader will sympathise.

## _(2026-09-30)_ — permission is not a criterion, and a proposal must be as visible as the gap it replaces

- D32's five rows were not a documentation gap but a **roadmap** gap: §3.2 promises a collar and trousers,
  ontology §4.7 models buttons and pockets, and no gate's exit criteria proved any of them. G3's note that
  an intermediate "may be inserted without shame" is a licence, and a licence proves nothing at a gate
  review. The amendment this slice proposes closes the *class* — every garment §3.2 names drafts, grades
  and exports — rather than naming four features, so the next envelope addition inherits a proof
  requirement instead of needing its own amendment. Promoted to
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`.
- **A provisional commitment that reads like a settled one is the same defect in better clothes.** So the
  cells say `(proposed)`, the census gained an A3 advisory printing every proposed cell on every run, and a
  probe arm removes the markers and requires the printed count to fall — an advisory that reads nothing
  prints the same number either way, which is the vacuous-green shape the glossary census shipped once as
  its R1 bug.
- **The engineer prepares the amendment; the director applies it.** The ruling reserved `ROADMAP.md`, so the
  record quotes the current text with its line numbers beside the proposed text, states what happens on
  approval and on rejection, and the matrix acts consistently with a *proposal*. A reservation is not a
  blocker: the work lands, and the one thing that is not the engineer's to do is named rather than done.
- The token census fired a fifth time at authoring time (`` `lining` `` and `` `proposed` `` wearing token
  formatting in prose): both were de-tokenized, not exempted.

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/stitchcad-devnotes-part1.md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/stitchcad-devnotes-part2.md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/stitchcad-devnotes-part3.md) | the two next-oldest `2026-09-30` lessons (two tables describing two garments, and a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

