# Sealed archive — StitchCAD dev notes, deriving the certifying artifact

Immutable historical segment, sealed by leaf `G1-SLICE.3c.3a` on `2026-10-01`.

- **Sealed identity:** 15 lines, 1343 bytes, `sha256:bc7fae65a4f5691ad9d90b34c2990cad630ff5089acbea64d6404f2f36fb2ccb`
- **Coverage:** the oldest live `2026-09-30` lesson (the certifying artifact), copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

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
