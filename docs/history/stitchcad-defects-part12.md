# Sealed archive — archive capacity and coverage calibration defects

Immutable historical segment, sealed by leaf `SPINE.19.2` on `2026-10-01`.

- **Sealed identity:** 20 lines, 1830 bytes, `sha256:0693a24114d9fd4ce319a751b90bde717e266e1fef984e916fab3dec5fe1e3c5`
- **Coverage:** D65 and D68, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D65** — sealed-history file count is approaching its enforced 64-file ceiling; normal
  per-slice retention will soon block product commits even though each segment is small.
  - Reproduce: `git ls-files -- 'docs/history/*.md' | wc -l` → 59 at `2db924b`; the two seals
    owned by G1-SLICE.4a.1 bring it to 61. surfaces.tsv declares 64; check_live_doc_size.sh
    compares measured collection cardinality against that ceiling, with no archive transition.
  - Impact: three ordinary seals consume the remaining capacity; nesting files cannot remedy it
    because git's glob crosses directories. Raising the ceiling would merely defer unbounded growth.
  - Owner/schedule: `SPINE.19.2`, before a required product seal would exceed the 64-file limit.
    Preserve exact historical bytes, order, identifiers and complete portable retrieval; bound both
    archive descriptors and retained storage under the adopted archive contract. No ceiling increase.
  - Fixed by SPINE.19.2: all 64 original full files reconstruct exactly from a self-contained window;
    stable logical paths, finite manifests/payloads and decoded/resident bounds checked. Source copies
    retired only after independent git-show reconstruction. Nine ledger arms remain meaningful.

- **D68** — COVERAGE-RED passed on unrelated D30, leaving its actual mutation untested.
  - Reproduce original run_changelog_ledger_probes.sh: append second Coverage in part2; grep -m1
    still reports part2 PASS, while unexempted part1 reports FAIL for its known immutable D30 error.
  - Impact: removing the intended coverage predicate could leave this arm green.
  - Owner/schedule: SPINE.19.2, fixed now during consumer integration; replace first part2 declaration
    and require part2 to be the sole refusal with the D30 exemption preserved. Nine arms green.
