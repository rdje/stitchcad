# Sealed archive — StitchCAD dev notes, the two oldest 2026-09-30 lessons

Immutable historical segment, sealed out of `DEV_NOTES.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `SPINE.4.4` in the commit whose append crossed
the window's byte health target.

- **Sealed identity:** 62 lines, 5915 bytes, `sha256:edcd08085356f66fef5502cbde5393f77ef8958513b319927b2dd1fce184187e`
- **Coverage:** the lessons `a boundary is only real if something enumerates it, and a RED arm must
  remove the property` and `a vocabulary census is a domain-defect detector, and a green rule may be a
  vacuous one`, both dated 2026-09-30, oldest last, exactly as they stood.
- **Sealed by:** leaf `SPINE.4.4` on `2026-09-30`, after `part1` (sealed by `G0-CONTRACT.4b`).
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and it
  ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after defect
  D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.

---

## _(2026-09-30)_ — a boundary is only real if something enumerates it, and a RED arm must remove the property

- **Writing the feature matrix found a roadmap gap that reading it four times had not.** Roadmap §3.2 puts
  a classic collar and trousers inside the v1 envelope; ontology §4.7 specifies button and pocket objects;
  and no gate's exit criteria in §11 mention any of them
  (`sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` → `0`; corrected
  by D41 — a section range also catches G3's complexity note, which says "a shirt/trousers intermediate", so
  the command yields `1` and the count that supports the claim is the exit bullet alone, `sed -n '702,708p'`,
  → `0`). The
  gap was invisible while the envelope was prose, because prose does not have to enumerate. It became
  visible the moment each feature needed a disposition AND a gate, and refusing to invent one produced
  `unnamed (D32)` — five rows the census now prints on every run. General rule: **a table with a required
  column is a census of the prose it replaces.** The column nobody can fill is the finding.
- **A RED arm that mutates one instance of a property tests nothing.** Two of the matrix probe's ten arms
  passed at `exit=0` on the first run: renaming `` `ngo_costing` `` renamed its §10 declaration and its row
  together, and de-citing one of three rows citing `ontology §4.4` left the clause covered. Both reported
  "the census discriminates" while the census was never asked to. The fix was to make the mutation the only
  copy of the property. Corollary worth keeping beside D15's shadowing probe: a suite's value is
  concentrated in its RED arms, so an arm deserves the same suspicion as a green gate — **what else in the
  tree still satisfies this rule?** Promoted to `TOOLBOX.md`'s probe conventions rather than to a layer-C
  record, because that is the file a probe author is directed to read (`CLAUDE.md` step 3) and a record
  would duplicate it; `promotion: declined` is recorded in the leaf.
- **The same slice reproduced the failure mode in the instrument itself.** Replacing a `{2,4}` interval
  with `###+` "for portability" silently stopped matching `##`-level headings, so the ontology-coverage rule
  required three clauses fewer and printed `0 failure(s)`. Nothing in the output distinguished "covered"
  from "not looking". Two guards caught it: a GREEN arm over the real tree (which reports the *count* of
  required clauses, so a shrinking population is visible) and a RED arm over a single citation. Corollary:
  **a census should print the size of the population it judged, not only the number of breaches** — a
  count is the only thing that reveals a rule that quietly stopped looking.

## _(2026-09-30)_ — a vocabulary census is a domain-defect detector, and a green rule may be a vacuous one

- **The instrument found the domain defect, not the reading.** Building the glossary's token census
  required deciding what "accounted for" means, and the only honest answer — a token must be declared by
  a table in the chapter that uses it — immediately failed on the reference fixture: `19` of its `42`
  tokens were declared nowhere (§4 named its derived values in prose while its own formulas used
  `quarter_hip`, `front_dart_centre`, `sa_cb`). Forcing each one to a declaration is also what exposed
  **D27**: `sa_wb_bottom` had no declaring row because the waistband it belongs to is described two
  incompatible ways in the same chapter (§4's `2 × wb_width + … = 10.0 cm` is one band folded lengthwise;
  §6's piece list is a faced two-piece band at `6.0 cm` each, and §8 sews only the outer one). A
  prose-only fixture hides that; a fixture that must name every value cannot. Promoted to
  `docs/decisions/decision_machine-tokens-declared-where-used.md`.
- **A rule that reports `0 failures` may have checked nothing.** The census's reference rule printed
  `dead references: 0` on the first run, and the honest reading is not "the glossary is clean" but "did
  this rule ever fail?". It had two bugs: it read the *meaning* column instead of the object column, so
  no cell was ever examined; and it sliced the clause with `substr(link, RSTART + 1, …)` where `§` is
  **two bytes** and `LC_ALL=C` makes awk count bytes, so every extracted clause carried a stray `\xa7`.
  Only the `DEAD-CLAUSE` RED arm — which mutates a copy and demands a refusal — showed it: the arm failed
  to fire. Corollary worth keeping: **a probe suite's value is concentrated in the arms that fail on
  purpose; a suite of GREEN arms is a report, not a test.** This is the third time this repository has
  measured that shape (D15's shadowing probe, D20's untracked producer, D25's misclassified prose).
- **Path arithmetic deserves the same suspicion.** `resolve()` collapsed `a/b/../c.md` by deleting `/../`,
  which leaves `a/b/c.md` — the parent segment has to go with it. Every one of 155 references was reported
  dead until the pattern became `/\/[^\/]*\/\.\.\//`. Relative-link resolution is where a census silently
  becomes a complaint, because both failure directions look like content problems.
- **A rollover is a transaction, not an append.** `CHANGELOG.md` crossed its health target on this slice,
  and doing it by hand reproduced the two defects the ledger probe now checks: the window was not in
  commit order (D29 — so "seal the oldest" seals the wrong end) and part1's descriptor claimed coverage
  through an entry that never left the live window (D30). The containment doctrine already said this
  ("run link, freshness, ordering, uniqueness, retrieval … checks"); what was missing was the executable.
  The sealed-content digest is now re-computed on every `make probes`, and part1's `365` lines still
  reproduce `sha256:f4aec75a…` — so immutability is a measured property, not a policy sentence.
