# Sealed archive — StitchCAD defect census, closed D61

Immutable historical segment, sealed by leaf `G1-SLICE.3c.3b` on `2026-10-01`.

- **Sealed identity:** 13 lines, 1214 bytes, `sha256:af043224b63067096ec9d8a6004582f2bae360efb1b3094db5655127b5908dcb`
- **Coverage:** `D61`; diagnostic scope and observed refusal probes.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D61** — the glossary census's C1 failure says a token is not declared by a table "in that
  chapter", but the predicate aggregates declared tokens from all spec tables. Found while routing
  growing implementation docs to a dedicated chapter (`2026-10-01`).
  - Reproduce: `sed -n '388,426p' run_glossary_census.sh` shows the per-file loop appending to one
    global DECLARED_TOKENS set, and C1's message still claiming per-chapter absence.
  - Impact: the diagnostic misstates its checked scope and can lead authors to duplicate canonical
    declarations across chapters. Token ownership is global; the predicate is intentionally so.
  - Owner/schedule: **`G1-SLICE.3c.3b`, now**, correct only the scope wording, run the glossary
    census/probes, and retain the meaningful canonical API vocabulary in its linked chapter.
  - **Closed:** `.3c.3b` changes only C1's scope wording to "no spec table declares it".
    `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `0 failure(s)`, `rc=0`;
    `run_glossary_probes.sh` → `10 pass / 0 fail`, `rc=0`, including undeclared-token refusal.
    The predicate and exemptions are unchanged; the linked implementation chapter retains API meanings.
