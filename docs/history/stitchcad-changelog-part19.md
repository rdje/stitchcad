# Sealed archive — StitchCAD first G1 reconciliation

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4b.2` on `2026-10-01`.

- **Sealed identity:** 21 lines, 1770 bytes, `sha256:2f602e9a1d0acf49cd6b86fb8b4b42e221609653dd41ed70c17db30332971996`
- **Coverage:** STITCHCAD-G1-0001, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0001 - the frontier pointed at a leaf whose work had already shipped (leaf `G1-SLICE.1`)

The G1 frontier named `G1-SLICE.1` (the workspace crate layout) as the next slice to take, but its every
deliverable had already shipped: `G0-CONTRACT.18` (commit `eb83f01`) retired the bedrock starter crate
"closing defect D10 ahead of `G1-SLICE.1`", created `sc-units` and `sc-core` with the workspace lints
inherited, and landed the G0 CI shape (fmt / clippy / unit+property / a real `wasm32-unknown-unknown` build).
The leaf was left `pending`, so the tree's status disagreed with the workspace — a resuming session pointed at
`.1` would have re-done finished work.

This slice is the reconciliation: it audits `eb83f01` against each of `.1`'s acceptance criteria and records
the closure rather than writing new code. Every criterion was re-derived by command — `cargo metadata
--no-deps` lists exactly `sc-core, sc-units` (the roadmap crates that exist so far; §4.3 grows the rest at
their gates); `git ls-tree HEAD crates/` shows `crates/app` gone and no crate prints the template message;
`make check` is green (21 property tests + doc-test), `make wasm` cross-builds both crates, and
`run_g0_exit_review.sh` reports `G0-17 MET` (CI) and `G0-18 MET` (the wasm build); `KNOWLEDGE_MAP.md` names
both subsystems. `make gate` stays `=== all doctrines green ===`.

The lesson is recorded in `DEV_NOTES.md` and promotion declined there: a leaf's status drifting when a sibling
leaf delivers its work early is an instance of the D34 hand-kept-state class `PLANNING.5` owns, so this slice
fixes the instance and leaves the class to its derivation. The frontier advances to `.2` (`sc-units`), whose
code likewise shipped under `eb83f01` and is reconciled next.
