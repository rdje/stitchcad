# Sealed archive — StitchCAD changelog, slices 25–29

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the live
window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 177 lines, 15440 bytes, `sha256:a8cc1de6a52f30e21e1b2ff57e025e55dad36719c4497bd309c7356e3ade08ae`
- **Sealed by:** leaf `G0-CONTRACT.8` on `2026-09-30` — the append that crossed the rollover milestone
  performed the rollover, as the doctrine requires. The window was already in commit order, so "seal the
  oldest" sealed the oldest (defect D29 is what made that worth saying).
- **Coverage:** slices 25–29 — `STITCHCAD-G0-0004` … `STITCHCAD-SPINE-0017`, newest first, exactly as they stood.
- **Predecessors:** the earlier `stitchcad-changelog-part*.md` segments. part1's own coverage line
  overstates its range by one slice; the correction is recorded in part2's descriptor and in the live
  pointer, because a sealed segment is never edited (defect D30).
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`. The sealed content is everything below the
  `---` rule and the blank line after it; `shasum -a 256` over exactly those bytes reproduces the digest
  above, and `bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves it together with
  the ordering, uniqueness, coverage and pointer rules.
- **Write policy:** none — sealed segments are immutable.

---

## STITCHCAD-G0-0004 - the release claim gets a boundary, and the boundary is derived (leaf `G0-CONTRACT.4`)

docs/book/src/spec/feature-matrix.md dispositions **105 rows** - 76 supported, 19 rejected, 10 deferred -
each with a reason that cites its source explicitly (`ontology §4.3`, `roadmap §3.2`), the gate whose exit
criteria prove it, and for every refusal the diagnostic token it must produce. §10 declares those 29
tokens with the arguments each carries, because an undeclared token has no defined meaning.

- three rules make the table normative rather than descriptive: **no silent approximation** (a refused
  construction produces its diagnostic and NO geometry - a knit block drafted as woven, a NURBS curve
  flattened without saying so, a pocket drawn as internal lines are all defects); **a supported row names
  its proof** (a gate's exit criteria, not a hope); and **modelled is not supported** (an object nobody
  scheduled a proof for is `deferred` to G7, whose exit is an envelope statement with named limitations)
- the acceptance clause "nothing in the ontology is silently unlisted" is derived, not asserted:
  `run_feature_matrix_census.sh` -> "feature-matrix census: 105 rows / 29 diagnostics / 0 failure(s)",
  exit=0 - all 16 required ontology object clauses cited, 8 of 8 roadmap §1.3 non-goals matched to a
  rejected row with the mapping printed, 5 of 5 §3.2 envelope garments supported and 3 of 3 named refusals
  rejected, every gate cell resolving to a real roadmap §11 gate or to `unnamed (D32)`, and every declared
  diagnostic used by exactly the rows that raise it
- **D32, found by writing the rows**: roadmap §3.2 puts a classic collar and trousers inside the v1
  envelope and ontology §4.7 specifies button and pocket objects, yet no gate's exit criteria mention any
  of them - `sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` -> `0`.
  G3's note *permits* a shirt/trousers intermediate without promising one. Rather than borrow a gate, those
  five rows (collar, trousers, buttons, pockets, fly) say `unnamed (D32)` and the census prints them on
  every run as advisory A1, so `G0-CONTRACT.15` must put the assignment to the director instead of the gap
  closing by being forgotten
- the glossary absorbed the 26 terms the matrix introduces (239 -> **265**), index re-derived with
  `--emit-index`; `run_glossary_census.sh` -> "265 terms / 8 parts / 139 tokens / 0 failure(s)", exit=0.
  The machine-token convention paid for itself immediately: the matrix's 29 diagnostic tokens are declared
  by its own §10 table, so the glossary census's coverage rule needed no exemption
- **two of the ten new probe arms passed for the wrong reason, and the census was right** - measured, not
  assumed. One renamed a diagnostic token everywhere, which renames a declaration and its use together and
  changes nothing a census can see; the other de-cited one of three rows citing `ontology §4.4`, leaving
  the clause covered. A RED arm must remove the PROPERTY, not one instance of it, and the rule is now in
  TOOLBOX.md's probe conventions where a probe author reads it. A third bug was mine: swapping a `{2,4}`
  interval for `###+` "for portability" silently dropped every `##`-level heading, so the coverage rule
  required three clauses fewer and still printed 0 failures - which is why a census prints the size of the
  population it judged, not only its breaches
- gates: `make gate` -> "=== all doctrines green ==="; `make probes` -> "10 suite(s) green" (72 arms,
  0 fail); `make check` -> five "test result: ok" lines, no product code touched; `make book` -> exit=0 with
  6 spec pages; containment -> "OK - 17 surfaces, 15 routes, 64 files measured", with the matrix part at
  28 646 B against a 24 576 B per-part health (117%, ceiling 40 960) recorded in the leaf rather than paid
  for in vaguer rows

## STITCHCAD-G0-0001 — the glossary: one meaning per term, one owner per token (leaf `G0-CONTRACT.1`)

**The G0 exit clause "glossary of construction terms" is met, and its completeness is derived rather
than declared.** `docs/book/src/spec/glossary.md` plus eight domain parts carry **239 terms**, each with
a plain-language meaning, the canonical object that specifies it, the synonyms factories and other CADs
use, and the machine token — and a ⚠ on the ones whose mistranslation causes a wrong cut.

- **Partitioned because a termbase is not a chapter.** One file would have been ~70 KB of five-column
  rows against a `book_collection` per-part health of 24 576 bytes, so the glossary is eight parts by
  domain (29–48 lines, 4.3–7.5 KB each) behind an index chapter that carries the rules and the derived
  A–Z list. A term sits next to the terms it is confused with, and no part breaches its ceiling.
- **The machine-token rule is normative** (and is now a layer-C record,
  `decision_machine-tokens-declared-where-used.md`): a token is never rendered raw to a human; one token
  has one meaning, so where four terms are all a `Closure`, one entry owns the token and three write
  `→ Closure`; tokens are ASCII and locale-independent; and where code exists the token is quoted from
  code — `Micrometre`, `MICRODEGREES_PER_DEGREE`, `UnitError::DomainExceeded`, the six `ToleranceClass`
  variants — not invented prettier than the crate.
- **Safety-relevant terms are a shipping rule, not a styling one.** Roadmap §7.6 names notch types and
  the sew/cut line aliases; §8.3 names units and allowance ownership. 22 entries are required to carry
  the ⚠ (72 do), and the census refuses any of the 22 that loses it, plus any entry owning a `type`
  token — so a new notch shape inherits the requirement instead of being forgotten.
- **Its own claims are re-derived:** `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` →
  `glossary census: 239 terms / 8 parts / 138 tokens / 0 failure(s)`, `exit=0`. Ten rules: entry
  structure, parts inventory, term uniqueness, token ownership, cross-references, token shape, every
  canonical reference resolving to a real chapter clause, roadmap clause, ADR or task-tree leaf (155
  references, 0 dead), the safety marks, index↔parts equality both directions, and the coverage claim —
  every machine token the specification uses is owned here, declared by the chapter that uses it, or
  exempted with a written reason (`82` used, `0` unaccounted).
- **The probes found three defects in the census before it found any in the glossary**, which is why
  they exist: `R1` read the *meaning* column instead of the object column, so it checked nothing and
  reported `dead references: 0` vacantly; the `§` clause slice was one byte off, because `§` is two bytes
  and `LC_ALL=C` makes awk count bytes; and `resolve()` stripped `/../` without the parent segment, so
  every `../ontology.md` resolved to a path that does not exist. All three were caught by RED arms
  (`DEAD-CLAUSE` failing to fire), never by reading the code. `probes: 10 pass / 0 fail`.
- **A second instrument came with it**, because the rollover this entry triggers is exactly the
  operation D29/D30 corrupted: `run_changelog_ledger_probes.sh` checks the live window's order against
  commit order, live-vs-sealed uniqueness, each segment's declared `sha256` against its content, each
  descriptor's coverage claim, and the pointer↔segment closure — the five checks the containment
  doctrine's rollover protocol step 6 requires. It verified part1's digest (`365` lines reproduce
  `sha256:f4aec75a…`) and then proved part1's coverage line wrong (D30).
- **Rollover performed by this append:** the live window crossed its health target, so the five oldest
  entries are sealed into `docs/history/stitchcad-changelog-part2.md` with their own descriptor, the
  window is reordered into commit order (D29), the stale "_Inherited spine history_ divider" sentence is
  corrected (D31 — that divider left this file in `SPINE-0014`), and part1's coverage claim is corrected
  by superseding record rather than by editing an immutable segment (D30).
- Validation: `make book` → `exit=0` with all nine glossary pages rendered; `make probes` →
  `9 suite(s) green`; `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok`;
  containment → the widest glossary row is `268` B against a `320` B ceiling (over the `200` B
  prose-shaped health target, recorded in the leaf rather than trimmed away).

## STITCHCAD-G0-0013b — the reference fixture declares its own tokens (leaf `G0-CONTRACT.13b`)

**Found by building the glossary's token census, not by reading.** The fixture every G2 golden, mutation
suite and agent gate will be frozen over used **19 machine tokens that no table declared** — and forcing
each one to a declaration exposed a contradiction in the garment itself.

- **D26, fixed.** §4 named its 17 derived values in prose ("quarter hip", "front dart centre") while §5's
  recipe and §4's own formulas referred to them as `quarter_hip` and `front_dart_centre`; `ease_waist`,
  `ease_hip`, `sa_cb`, `sa_waist` and `sa_wb_bottom` appeared inside formulas with no declaring table at
  all. All three tables now lead with a **Token** column, §7 declares `sa_side`/`sa_cb`/`sa_waist`/
  `sa_hem`/`sa_wb_bottom`, and §4's formulas are written over tokens only — no prose word survives inside
  one, which is what makes them machine-checkable. Measured: tokens used-and-undeclared `19` → `6`, and
  the six remaining are glossary vocabulary (`SeamAllowance`, `assumed`, `close`, `known`, `semi`, `walk`),
  not fixture names. **No derived value changed**: `allocation_balance` still closes `28.0 = 28.0`.
- **D27, logged and owned, not guessed at.** §4's `waistband_cut_width = 2 × wb_width + sa_waist +
  sa_wb_bottom = 10.0 cm` is the cut width of ONE band folded lengthwise; §6's piece list carries
  `waistband_outer`, `waistband_inner` **and** `waistband_interfacing` — a faced two-piece band whose
  pieces would each be cut at `4.0 + 1.0 + 1.0 = 6.0 cm` (`python3 -c "print(2*4.0+1.0+1.0, 4.0+1.0+1.0)"`
  → `10.0 6.0`). §8 compounds it: the `waist` span sews only the outer band, so the inner band has no span
  and §12's count of 6 pieces is the faced reading's. Both are real skirt constructions and they are
  different garments, so choosing is a domain decision: **`G0-CONTRACT.14`** owns it, with the expert it
  names. The contradiction is now recorded in the chapter (§6 and §11) with both readings and their
  arithmetic, so no reader and no golden can take a waistband number as settled while it is open.
- **D28, fixed.** §5 step 7 sent the reader to "allowances (§6)"; allowances are §7 and §6 is the piece
  list. `G0-CONTRACT.1`'s glossary census carries an `R1` rule that resolves every canonical-object
  reference against real headings, which is that class instrumented rather than eyeballed.
- Validation: `make book` → `exit=0`; `make gate` → `=== all doctrines green ===`; the token census over
  the fixture at `HEAD` against the working tree → the numbers above.

## STITCHCAD-SPINE-0018 — the push-due trigger means "CI must re-verify this" (leaf `SPINE.18`)

- `SPINE.17` globbed `scripts/check_*.sh`, so the helper matched its own trigger: committing it reported
  `EXCEPTIONAL PUSH DUE — 1 unpushed file(s) … scripts/check_push_due.sh`, `exit=1`, for a file no CI job
  executes. A standing false obligation is worse than no instrument — the honest response to a warning
  that always fires is to stop reading it.
- The registered checks are now **derived** from the two registries (`registry_checks()` reads the paths
  cited in `scripts/check_doctrines.sh` and `scripts/check_doctrines.project.sh`, plus the drivers):
  17 paths, including both project doctrines, excluding this helper. A newly registered check becomes a
  trigger with no edit here — the property a glob cannot have.
- Arms re-observed: not-owed → `no push due (1 < 400, no CI/doctrine paths touched)`, `exit=0`; owed →
  `EXCEPTIONAL PUSH DUE — 6 unpushed file(s)`, naming the workflow, both project-slot files and the three
  `.doctrine/` seams, `exit=1`; bogus base → `REFUSED`, `exit=2`.
- Validation: `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 3 passed;
  0 failed`; `make probes` → `7 suite(s) green`; `bash -n` clean. No push is owed by this slice, so the
  400-commit cadence holds.

## STITCHCAD-SPINE-0017 — the push-cadence exception is derived, not remembered (leaf `SPINE.17`)

- **Director-approved rule change.** The cadence stays 400 commits, with one exception: a commit that
  touches `.github/workflows/`, a `scripts/check_*.sh` doctrine check, the `.doctrine/` seams those
  checks read, or `.githooks/` owes a push immediately — because layer E4 is the un-bypassable backstop
  and such a change is *unverified until a runner executes it*. `COMMIT.md` carries the rule and the
  deriving command; `make push-due` (`scripts/check_push_due.sh`) reports the state and exits 1 when a
  push is owed, listing the triggering files.
- **It is not hypothetical here.** `eb83f01` rewrote `.github/workflows/rust.yml` to add the
  `wasm32-unknown-unknown` target and its smoketest step, and that workflow had never executed anywhere
  until the exceptional push. And the platform risk is measured: this machine's BSD awk lacks `\b` and
  `{n}`, and its `PATH` shadows BSD userland with GNU coreutils — a gate can be green here and behave
  differently on the ubuntu runner.
- **Three arms observed:** not-owed (`0 unpushed commit(s)`, `exit=0`); owed
  (`PUSH_DUE_BASE=051a075 …` → `EXCEPTIONAL PUSH DUE — 6 unpushed file(s) … unverified by CI`, `exit=1`);
  refused (`PUSH_DUE_BASE=nope-not-a-ref` → `exit=2`). The owed arm is testable because the comparison
  base is overridable, so it needs no invented commits.
- **The RED arm caught a defect in the first cut:** the trigger list used the pathspec
  `scripts/check_`, which matches *nothing* — a git pathspec matches whole path components unless it
  carries a wildcard — so every doctrine check script was silently missed. Now `scripts/check_*.sh`, with
  the lesson recorded in the file header. Only the arm that compared against a revision *known* to
  contain those files could reveal it.
- **The exceptional push was made and CI observed, not assumed:** `git push origin main` →
  `051a075..119946b`, and the Actions API for that head sha reports `runs: 2` with **`rust`
  completed `success`** (the first execution of the new WASM smoketest step) and **`doctrines`
  completed `success`**. The verdict is recorded in `G0-CONTRACT.18`'s Verification Log.
- Known over-breadth, stated rather than hidden: the helper matches its own trigger pattern although no
  runner executes it, so landing it owes one further push. Flagged to the director rather than pushed
  unilaterally, since the authorisation was for one exceptional push.
- Validation: `make gate` → `=== all doctrines green ===`; `make check` → `test result: ok. 3 passed;
  0 failed`; `make probes` → `7 suite(s) green`; `bash -n` clean.

---

Older StitchCAD slices are in `stitchcad-changelog-part3.md`
`stitchcad-changelog-part2.md`
`stitchcad-changelog-part1.md`, and the bedrock scaffold's own changelog in
`bedrock-scaffold-changelog.md`. Newer slices are live in `CHANGELOG.md`, newest first.
