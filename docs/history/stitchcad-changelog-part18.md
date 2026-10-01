# Sealed archive — StitchCAD changelog, G0 exit review

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4a.2a` on `2026-10-01`.

- **Sealed identity:** 38 lines, 3694 bytes, `sha256:22d63ec4bc103c87fc7f908100ddc30f3bc530fb4608986dfee1df59ee9914dd`
- **Coverage:** `STITCHCAD-G0-0015`, oldest live entry copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G0-0015 - the gate review is a command, and it reports one clause this repository cannot close (leaf `G0-CONTRACT.15`)

Gate G0 had nineteen obligations, twenty leaves marked done, and no verdict for any of them: the gate's state
existed only as an impression. The leaf's acceptance forbids marking a clause met on prose alone - and a review
WRITTEN as prose is exactly that - so the review parses the roadmap and runs the checks.

- **the review** - `run_g0_exit_review.sh` reads §11's `**Exit:**` bullet, splits it into fragments (plus the
  `Fixture:` bullet), requires every fragment to be dispositioned by a row of `g0_exit_clauses.tsv` and every
  row's key to appear in a fragment, then RUNS each row's check: `G0 EXIT: 18 met / 1 not met / 19 clauses`,
  exit=0, in two seconds. Eleven of the checks are censuses; two are `cargo test -p sc-units` and `make wasm`.
- **the one open clause** - G0-12, evaluation-seat procurement, is `not met` and accepted open by the
  director's ruling of `2026-09-30`, with the cost named rather than hidden: no target system reads our
  artifacts back, so the interchange claims stay `cited-from-roadmap` and G6's receiver validation falls to
  the roadmap's partner-run fallback or does not happen. G0-13 (governance) is met as drafted with its
  qualification printed: the model and both review paths exist, the project owner is the director acting, the
  domain seat is vacant.
- **the closure is unapproved, and that is recorded** - governance §6.1 rule 2 withholds approval of a
  decision's evidence from its author, and the reviewing party authored sixteen of the nineteen deliverables.
  The mitigation §6.1 prescribes is in place: every verdict is a command's exit status, so independence is
  available to whoever reads next instead of being held by anyone now. The roadmap's status line is unchanged,
  which is the honest outcome - line 9 says DRAFT until the exit criteria are met, and one is not.
- **the ruling, recorded when it was made** - `decision_director-ruling-2026-09-30-no-seats-proceed-unapproved.md`
  carries the director's words, the boundary table of what proceeds on engineering evidence alone versus what
  needs a seat and when, and the amendment that the residual dependency is **a measurement, not a
  credential**: knowledge is substitutable by sourced reading (D27 was settled that way), judgement under
  disagreement is substitutable if a synthesized default stays `assumed` and cited, and physical truth is not
  substitutable at all - but it needs a machine, a printer and a ruler, not a hire. `G2-2D.15` was created to
  own that protocol, because a ruling that names a cost without an owner is a wish.
- **the probe suite** - `run_g0_exit_review_probes.sh` -> `10 pass / 0 fail`, including ROADMAP-GROWS (a clause
  added to a COPY of §11 is refused by name), CHECK-FAILS (a failing instrument reads as an unmet clause and
  `GATE FAILS`, not as a broken review), NO-BLOCKER and CONTROL. Two parser bugs were fixed on the way, both
  the session's recurring class: the bullet matcher looked for `**Exit:**` after emphasis had been stripped,
  and one roadmap clause spans two `;`-separated fragments, so a row needed a key list.
- **the tree's own ledger sealed again** - adding the review pushed `G0-CONTRACT.md` to 99 136 bytes against a
  98 304 ceiling, a breach rather than a warning, so four changelog entries went to
  `g0-contract-changelog-part2` (48 lines / 4651 bytes, digest reproduced) and the tree fell to 94 923.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `22 suite(s) green`; the review re-runs
  every clause's own census green; containment `OK`
