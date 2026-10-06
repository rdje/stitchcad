# Sealed archive — whole authored D201 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.v.b1` on `2026-10-06` (UTC).

- **Sealed identity:** 10 lines, 1107 bytes, sha256:67f657e8bb124a529d339cda85dbd34af71b44d54bdeea9ff17532426fb22409
- **Coverage:** complete authored docs/tasks/PLANNING.md bytes 80704–81366, 662 bytes; absent from 864fafa8239d06e9d147bca6018e6f69293710e8.
- Authored original sha256:27f89796652a454b6121e49818bf8e7f6f68ec6f9b733874b516f2df5013818a; only D201 appends verified closure.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- **D201** — new auxiliary control node incorrectly placed beneath output-owner .a01.2.
  Reproduce current producer_census_contract.py rc=1, fixture-census.log: effect owner coverage.
  The exact terminal-owner census treats every .a* child as an output/delegate group. Our new
  .a01.2.b owns a profile control, leaving the static recipe TSV owner nonterminal. Source864
  validates67/29; current placement red; changing only its ID to profile lane .p.v.b1 restores
 67/29. Owner current0199 bridge child; priority P1, fix now before committing. Preserve old
  authored node scope/report whole; no census guard or table changes. Status: open, active repair.

Verified0199 closure: Source864 output-owner topology passes67/29; new auxiliary placement has named effect-owner coverage red; changing only node ID to profile .p.v.b1 restores67/29.
The unchanged exact census guard and both owner TSVs retain the copied-book .a01.2 owner; profile/runtime audit .p.v remains pending. Old authored node scope retained whole in255; current same Goal/Paths/Acceptance live under .p.v.b1. Status: fixed, verified.
