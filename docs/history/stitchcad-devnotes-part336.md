# Sealed archive — completed twelfth-window packing scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.1` on `2026-10-25` (UTC).

- **Sealed identity:** 13 lines, 1051 bytes, sha256:de5c14c22f3a386b6a8c4c5aa3c0bfba857dc6466b3cee7d303762065edadb70
- **Coverage:** whole committed source interval from d5ca0b72f691a538bb73ea816dab31b03cc7aee2.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 28314–29365, 1051 bytes, sha256:de5c14c22f3a386b6a8c4c5aa3c0bfba857dc6466b3cee7d303762065edadb70
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.1.2.2.h`
  Status: `done`
  Goal:0259 pack window12; retain closed scopes/oldest journal/0258 ledger under fixed caps.
  Paths: docs/history/;.doctrine/history_archive/;DEV_NOTES.md;docs/tasks/artifacts/history_archive/capture_window12.py
- [x] REPRODUCE: capture_window12.py observes64 Markdown at f168eec, rc=0.
- [x] ROOT CAUSE: same producer verifies53 raw records plus11 catalogs; working ceiling64, rc=0.
- [x] FIX: deterministic payload/catalog/manifest; copy/verify/use before exactly53 checked removals.
- [x] ADDRESSED: python3 -I -B docs/tasks/artifacts/history_archive/capture_window12.py;
  53 Git/materialize/public reads exact,11 prior windows unchanged, rc=0.
- [x] NO REGRESSION: history_archive.py --root . prove-source window12:53 exact; verify-retention
  archive: OK, rc=0. Broad controls deferred to integration/pre-push; no product Rust changed.
- [x] LOCKSTEP:332–335/231 whole Git/public intervals, rc=0; live/book align.
  Promotion declined: history-window decision already owns the unchanged contract.
