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

- **Run:** `2026-09-30` 21:00 CEST (19:00 UTC) by leaf `SPINE.21` — removed the doctrine and probe scratch
  trees (`target/doctrine_scratch`, `target/scratch`, `target/tmp`), cargo's incremental caches on both
  hosts (`target/debug/incremental`, `target/wasm32-unknown-unknown/debug/incremental` — 57 `.bin` files),
  the mdBook output (`docs/book/book`, 4 120 KB) and three scratch bodies the containment self-tests had
  left in `target/`; `target` went 40 648 KB → 10 808 KB, so 33 960 KB left the volume. The residue census
  found all nine paths gone, `0` stray `*.log` / `*.bin` / `*.tmp` / `*.orig` / `*.rej` / `.DS_Store`
  files anywhere outside `.git`, `0` tracked artifact-shaped files before and after, and `0` deleted
  tracked files. `make gate`, `make check`, `make book`, `make probes` and `make wasm` were all green
  afterwards, the last two regenerating exactly what was removed (`docs/book/book` back at 4 120 KB,
  `target` rebuilding to 13 460 KB).
