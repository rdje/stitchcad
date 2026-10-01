# Sealed archive — StitchCAD changelog, sealing closed defects

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2b.2` on `2026-10-01`.

- **Sealed identity:** 27 lines, 2450 bytes, `sha256:211f9bec5af2af8c9d2d66fa94f0b7b47a144491c7a7ae2782c18435049431da`
- **Coverage:** `STITCHCAD-SPINE-0019a`, copied from the oldest live entry; no hand-kept slice range.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-SPINE-0019a - the closed defects are sealed, so the open ones are what a reader meets (leaf `SPINE.19.1`)

D46 named this remedy and `SPINE.19` owns its durable half, but the deferral's premise - "the file is inside
every ceiling today" - had expired: `PLANNING.md` was at 1133 lines / 93378 bytes, 95 % of its byte ceiling,
and 50292 of those bytes belonged to defects already fixed. The next defect any slice logged would have
blocked a commit, so the seal happened at the trigger D49 declares rather than at the breach.

- **the seal** - 44 closed entries moved to `docs/history/stitchcad-defects-part1.md` (624 lines / 53323
  bytes, `sha256:1897bde0…`) in the census's own order, under the descriptor contract; the live census keeps
  the 7 open defects, a pointer, and the two `grep -c` commands that derive both counts instead of a
  hand-kept pair. `PLANNING.md` is now 515 lines / 40606 bytes, inside its health on both axes.
- **nothing lost, nothing duplicated** - 7 live + 44 sealed = the 51 that were there, and the intersection
  of the two id lists is empty. The standing `DESCRIPTOR` rule reproduces the new segment's digest:
  `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`, `REAL` at 32 segment verdicts.
- **classifying the census is still a reading, and that is D38 measured again** - three entries (`D7`, `D9`,
  `D15`) record their closure in wording no marker list anticipated, so the open/closed split was made by
  reading each owner line. D38's ask (a status token a script can read) stays open with `PLANNING.5`, and
  `SPINE.19` keeps the durable half: a ledger-agnostic verifier, a segment registry, and an arm that refuses
  a fixed defect left live - because today nothing would notice one.
- **two containment defects found in this slice's own work and fixed before committing** - the first seal
  wrote the descriptor without the content (a missing concatenation), which the `DESCRIPTOR` rule caught as a
  one-byte segment; and a changelog entry built as one unwrapped string came out at 928 bytes against a
  600-byte ceiling, the only hard breach this session produced.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `20 suite(s) green`; containment
  `OK - 17 surfaces, 15 routes, 111 files measured`; the coverage census still `10 lanes / 13 trees /
  3 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`, so a history segment is not mistaken for a task
  tree
