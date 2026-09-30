# Sealed archive — StitchCAD changelog, slices 32–33

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the live
window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 93 lines, 8284 bytes, `sha256:3148dd0fb5f6308809dd0b868d7ed6f67f2c6df6476c15537472ceb144773963`
- **Sealed by:** leaf `SPINE.4.4` on `2026-09-30` — the append that crossed the rollover milestone performed
  the rollover in the same commit, as the doctrine requires and as `G0-CONTRACT.1`, `.8` and `.14` did before.
- **Coverage:** slices 32–33 — `STITCHCAD-G0-0007`, `STITCHCAD-G0-0006`, newest first, exactly as they stood.
- **Predecessors:** `stitchcad-changelog-part1.md` … `part5.md`. part1's own coverage line overstates its
  range by one slice; the correction is recorded in part2's descriptor and in the live pointer, because a
  sealed segment is never edited (defect D30).
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`. The sealed content is everything below the
  `---` rule and the blank line after it; `shasum -a 256` over exactly those bytes reproduces the digest
  above, and the `DESCRIPTOR` rule of `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` runs
  that comparison on every `make probes`.

---

## STITCHCAD-G0-0007 - the standards registry, and the discipline that keeps a citation honest (leaf `G0-CONTRACT.7`)

docs/book/src/spec/standards.md registers every external standard the model draws on - six designations,
each with its role, what is adopted, what is deliberately not, a status from a closed three-word
vocabulary and a named owner. All six are `cited-from-roadmap`, so this book quotes no clause of any
standard, and that is the rule rather than a gap.

- **three chapters had already routed their standards claims to a chapter that did not exist** (ontology
  §8, reference skirt §11, size sets §8), which is the shape a claim takes while it waits to be asserted by
  nobody. §4 is now a deferral ledger: all five deferrals - including the glossary's synonyms and the
  feature matrix's `supported` mapping rows - carry a disposition, and one stays honestly open (the
  fixture's body measurements are declared constants of this repository, copied from no standard, and stay
  `assumed` until somebody reads one)
- **two rules make the chapter normative**: no clause number, table number or quoted definition of a
  standard appears anywhere in the book unless its status is `read-in-repo` (none is, so none does); and a
  standard is DATA with provenance, never authority - "ISO 8559 says so" cannot satisfy an `unknown`
- **§3 states what the model needs from any standard**, so reading one later has something to satisfy:
  landmark identity, procedure repeatability, designation mapping (a label never supplies a measurement),
  and tolerance semantics - where a standard's tolerance is the physical-acceptance class, the factory's
  number, and is never silently adopted as a numerical or geometric one
- **the verification plan names its real dependency** rather than implying that reading is a formality:
  `.14` names the reviewer and the procurement owner (it is blocked on the director naming humans),
  procurement obtains the texts, and a read changes exactly three things - clause-level citations for
  landmarks and procedures, the fixture's `assumed` measurements, and the size-set designation data. A read
  that contradicts the book is a defect like any other
- **the registry claim is derived**: `run_standards_census.sh` -> "standards census: 6 registered / 6
  designations used / 0 failure(s)", exit=0, with a per-designation list of every file that uses it;
  `run_standards_probes.sh` -> "probes: 6 pass / 0 fail", including an arm that smuggles "per ISO 4915"
  into a chapter and requires the census to name it, and one that turns a row `read-in-repo` with no
  citation behind it
- **the containment ceiling refused the first draft**: the six-column registry table had rows of 410 B
  against a 320 B maxline ceiling and the deferral ledger 398 B. The registry became four columns plus one
  bounded subsection per standard, and the ledger became bounded prose entries - the remedy
  G0-CONTRACT.2 recorded when the same ceiling caught its tolerance table. The chapter now measures 204
  lines / 13 849 B, widest line 187
- **the glossary census caught two prose spans wearing token formatting** before the chapter shipped:
  `blocked` (a task-tree status) and `AAMA` inside a sentence describing the census's match shapes. The fix
  was to de-tokenize the prose, not to exempt it - a backticked span is an identifier a program reads
- D34's immediate half is corrected here: docs/TASK_TREE.md's frontier cell and execution-order line had
  gone stale at `.4` while `.5` and `.6` landed, because COMMIT.md updates that file "only if the frontier
  changes" and nothing derived whether it did. The durable half is PLANNING.5, deferred behind product work
- gates: make gate -> "=== all doctrines green ==="; make probes -> "11 suite(s) green"; make book ->
  exit=0 with 9 spec pages; glossary census -> "275 terms / 8 parts / 144 tokens / 0 failure(s)";
  feature-matrix census -> "105 rows / 29 diagnostics / 0 failure(s)"; containment -> "OK - 17 surfaces,
  15 routes, 70 files measured"; no product code touched
- LIVE_STATUS: 33 defects logged / 29 closed; the G0 frontier moves to .8, ADR-0001

## STITCHCAD-G0-0006 - size-set ownership decided, and a label is not a token (leaf `G0-CONTRACT.6`)

Roadmap §3.4 left one question open at G0 - "whether [the SizeSet] lives in the Design, the Factory
Profile, or a third Order object" - and ontology §2.3 pointed at this chapter for the answer. Both now
exist: `docs/decisions/decision_size-set-ownership.md` and `docs/book/src/spec/size-sets.md` (187 lines /
12 812 bytes).

- **the decision**: a `SizeSet` is its own object with identity and revision; a `Design` REFERENCES it (by
  id plus the revision it was authored against, so re-opening a design pins what it was built with); a
  Factory Profile may override it, and an override is a typed transformation that produces a *resolved*
  size set naming the design's reference, the profile's revision, the transformation and its evidence -
  never an in-place relabeling, so a release package can always say whose sizes it cut
- **quantities stay out**: roadmap §7.5 puts size-run quantities in an Order object, not in the reusable
  design, so a size set carries none and a tech pack that needs them records an unresolved input rather
  than inventing a ratio - a ratio baked into a design would make a commercial change stale-ify approvals
  that geometry never touched
- **the three candidate owners are argued, not listed**: design-only cannot serve a factory's house chart;
  profile-only makes a design unsized until a factory exists, which breaks the headless CLI, the WASM
  viewer and the reference fixture; an Order object owns quantities but must not own sizes, because the
  recipe has to be evaluable with no order in the loop. Precedence follows roadmap §8: a hard restriction
  outranks a factory override, and the conflict is visible rather than merged
- **the chapter is normative about the object**: ten fields with types and requiredness; labels are names
  and `members` is an AUTHORED order that no code path re-sorts (an alphanumeric sort of "XS, S, M, L, XL,
  2XL" puts 2XL first); exactly one base size which SHALL be a member and which is what cumulative and
  incremental breaks are measured from; breaks per adjacent pair so an uneven range is expressible, and a
  set with no breaks serves regeneration while refusing grading with a diagnostic; multi-dimensional
  systems as named axes where a member is a point and an absent combination is absent, never interpolated
- **EN 13402 and ASTM D5585 are named, not quoted**: §8 states what the model must EXPRESS for each of the
  five designation systems and marks the standards' content as cited-from-the-roadmap with
  `G0-CONTRACT.7` as the owner that reads them. Asserting a standard's tables without having read it is
  the exact failure the claim-verification policy exists to stop
- **MTM is not a special case**: a made-to-measure instance is a `custom` set of one whose chart comes from
  body measurements, whose base is its only member and whose breaks are empty - so path 2 refuses and
  path 1 is the only route
- **the glossary census found a convention violation in the new chapter before it shipped**: three example
  size labels were written in backticks, so C1 reported `S`, `M` and `L` as undeclared machine tokens. The
  fix was not an exemption but the rule - a label is prose a factory reads, a token is an identifier a
  program reads, so labels are quoted and §3 of the chapter now says so. Glossary: 270 -> **275 terms**
  (`size system`, `axis`, `resolved size set`, `size-set transformation`, `order object`), index re-derived,
  `275 terms / 8 parts / 144 tokens / 0 failure(s)`
- **the matrix census grew a link rule (M7)** because this slice added cross-chapter links to the matrix:
  every markdown link in a normative chapter must resolve, and a cited clause must exist in its target.
  Its RED arm mutates one link in a copy and requires the refusal -> `probes: 11 pass / 0 fail`
- gates: `make gate` -> "=== all doctrines green ==="; `make book` -> exit=0 with 8 spec pages;
  feature-matrix census -> "105 rows / 29 diagnostics / 0 failure(s)"; containment -> OK, 67 files
  measured; no product code touched
