# ARTIFACT_CLEANUP — the cleanup cadence record

One entry: the **latest** cleanup. Overwrite it each run; never append history (that is git's job).
Read this at session startup — if the recorded run is more than 24 hours old, clean during the session.

Cleanup is a normal change: it is owned by a task-tree leaf, recorded here, and committed.

## What is safe to remove (regenerable, gitignored, project-owned)

- `target/doctrine_scratch/`, `target/scratch/` — doctrine self-test and probe scratch, including the
  throwaway git repositories the probe suites build.
- `target/debug/incremental/` — cargo's incremental caches (`.bin` files); `make check` rebuilds them.
- `docs/book/book/` — mdBook output; `make book` regenerates it.
- Stray `*.log`, `*.bin`, `*.tmp`, `*.orig`, `*.rej`, `.DS_Store` outside `.git`.

## What is never removed

- Any tracked file (`git ls-files` is the authority; the census below re-proves it each run).
- `target/debug/deps` and the built binaries — current build cache, cheap to keep and slow to rebuild.
- Any shared/global cache outside the repository (cargo's registry, homebrew, system temp): populate a
  project-local cache instead, and remove only records provably owned by this project.

## Latest run

- **Run:** `2026-10-01` 18:56 UTC by leaf `SPINE.21a`.
- **Summary:** removed six ignored scratch/incremental/book trees and 255 safe stray artifacts;
  339180 KB reclaimed; residue 0, tracked deletions 0. Rust/WASM/book/probes/gates passed after regeneration.
