# ARTIFACT_CLEANUP — the cleanup cadence record

One entry: the **latest** cleanup. Overwrite it each run; never append history (that is git's job).
Read this at session startup — if the recorded run is more than 24 hours old, clean during the session.

Cleanup is a normal change: it is owned by a task-tree leaf, recorded here, and committed.

## What is safe to remove (regenerable, gitignored, project-owned)

- `target/doctrine_scratch/`, `target/scratch/` — doctrine self-test and probe scratch, including the
  regenerable fixtures. Nested repositories, symlinks and foreign-device entries are refused.
- `target/debug/incremental/` — cargo's incremental caches (`.bin` files); `make check` rebuilds them.
- `docs/book/book/` — mdBook output; `make book` regenerates it.
- Ignored target strays: `*.log`, `*.bin`, `*.tmp`, `*.orig`, `*.rej`, `.DS_Store`.
  Inspect artifact-shaped files elsewhere; retain any whose ownership/regeneration is uncertain.

## What is never removed

- Any tracked file (`git ls-files` is the authority; the census below re-proves it each run).
- `target/debug/deps` and the built binaries — current build cache, cheap to keep and slow to rebuild.
- Project package stores, scaffold backups and cleanup audit manifests.
- Any shared/global cache outside the repository (cargo's registry, homebrew, system temp): populate a
  project-local cache instead, and remove only records provably owned by this project.

## Reproducible cleanup

Run python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py with plan or apply and
one run directory: target/artifact_cleanup_audit/<run>. Inspect plan.json before apply.
The frozen plan covers candidate content, tracked inputs, HEAD and producer identity; any drift
refuses. No tracked input, protected store, link, special file, Git boundary or other volume is deleted.
Apply rechecks each path, verifies no live project job, checks residue and tracked content, then
records result.json. Probe controls run through make probes; generated audit data stays ignored.
Run the immediate read-only residue census without activating the local-environment launcher:
activation recreates target/scratch. Rebuild only after that census, since regeneration recreates
selected roots.

## Latest run

- **Run:** `2026-10-05` 22:28 UTC by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.cleanup2`.
- **Summary:** removed6 ignored trees/1248 logs (1058866659B);1254 selected paths absent/tracked bytes unchanged before regeneration.
