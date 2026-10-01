# Sealed archive — StitchCAD changelog, recurring cleanup ownership

Immutable historical segment, sealed by leaf `G1-SLICE.3c.3a` on `2026-10-01`.

- **Sealed identity:** 23 lines, 2123 bytes, `sha256:f93154e3129cd5b3714c90fd47893ffd6ecce3255baf8d8c285bdd1db4dc36f4`
- **Coverage:** `STITCHCAD-SPINE-0021`, copied from the oldest live entry; no hand-kept slice range.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-SPINE-0021 - the cadence runs, and the residue census proves what it took (leaf `SPINE.21`)

The cleanup cadence had no recurring owner: `SPINE.2` discharged the first run and wrote the record, but a
cadence is an obligation that returns, and this one was 23 hours from firing mid-slice with no leaf to own it.

- **the run** - nine paths removed, each named by the residue census and each found gone: both scratch trees
  (`target/doctrine_scratch`, `target/scratch`, `target/tmp`), both incremental caches
  (`target/debug/incremental`, `target/wasm32-unknown-unknown/debug/incremental`, 57 `.bin` files), the mdBook
  output (`docs/book/book`, 4 120 KB) and three scratch bodies the containment self-tests had left in
  `target/`. `target` went 40 648 KB -> 10 808 KB, so 33 960 KB left the volume counting the book.
- **nothing tracked was touched** - `git ls-files | grep -cE '^(target/|docs/book/book/)'` -> `0` before and
  after, `0` tracked artifact-shaped files, `0` deleted tracked files in `git status --porcelain`, and `0`
  stray `*.log` / `*.bin` / `*.tmp` / `*.orig` / `*.rej` / `.DS_Store` anywhere outside `.git`.
- **the removal is shown to cost rebuild time and nothing else** - `make gate` -> `=== all doctrines green ===`;
  `make check` -> `test result: ok. 1 passed; 0 failed`; `make book` -> regenerated at exactly 4 120 KB;
  `make probes` -> `20 suite(s) green` with `target/scratch` recreated by the Makefile's own rule;
  `make wasm` -> the smoketest green. `target` rebuilt to 13 460 KB.
- **D34's fourth instance removed** - the index's `SPINE` frontier cell still named `.20` as open one commit
  after it landed, while the execution-order paragraph in the SAME file had it right: two hand-kept sentences
  about one lane, drifting against each other, which is the strongest argument yet for deriving the cells.
- `SPINE.20`'s checklist moved to `SPINE-evidence.md`, as the convention requires of the slice after the one
  that landed it, bringing the tree back inside its per-part health (683 lines / 56 833 B); the changelog's own
  rollover follows in this entry (`part10`).
