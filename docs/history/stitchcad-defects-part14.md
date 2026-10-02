# Sealed archive — publication status and archive probe calibration

Immutable historical segment, sealed by leaf `G1-SLICE.4d.1` on `2026-10-02`.

- **Sealed identity:** 15 lines, 1370 bytes, `sha256:f3c0f4cf1c19c29e4b1eacd3479f2d5160621430e4d6fbf2dd06ba1191d310ab`
- **Coverage:** D71 and D72, closed and verified by STITCHCAD-G1-0036; prior logged descriptions copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D71** — the book landing page incorrectly reports G0-only status after G1 libraries landed.
  - Reproduce: `rg -n 'project is in gate|G1 —|Product code' docs/book/src/introduction.md
    LIVE_STATUS.md` shows the conflicting current-state statements; canonical sc-measure APIs exist.
  - Impact: the reader mistakes implemented structural contracts for entirely future behavior.
  - Owner: `G1-SLICE.4d.1`, next safe publication/alignment leaf after the MTM commit. Correct the
    status with explicit implemented/deferred scope, verify against code/roadmap/live records, and
    improve novice/expert navigation under the director's glossary/index/annex requirement.

- **D72** — archive resident-limit probe depends on the growing production history inventory.
  - Reproduce: full `make probes` at the publication milestone fails in resident_overflow: current
    486538 decoded bytes + 22 × 160000 = 4006538; the 4000000 decoded cap correctly fires before the
    expected resident cap. The earlier snapshot was small enough for this same fixed fixture.
  - Impact: unrelated history growth invalidates milestone proof of the distinct resident predicate.
  - Owner: `G1-SLICE.4d.1`, fix now as the small blocking verification prerequisite. Construct a
    minimal independent archive and paired passing/resident-overflow fixtures; retain all reader caps.
