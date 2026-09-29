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

- **Run:** `2026-09-29` (UTC) by leaf `SPINE.2` — removed the doctrine/probe scratch trees
  (`target/doctrine_scratch`, `target/scratch`), cargo's incremental caches
  (`target/debug/incremental`, 896 KB of `.bin` files) and the mdBook output (`docs/book/book`,
  1 068 KB); `target` went 2.1 MB → 1.2 MB, the residue census found all four paths gone and
  `0` stray `*.log` / `*.bin` / `.DS_Store` files remained, `0` tracked artifact-shaped files existed
  before or after, and `make gate`, `make check`, `make book` and `make probes` were all green
  afterwards (the last two regenerating exactly what was removed).
