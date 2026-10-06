# Annex: guarded artifact cleanup

Cleanup removes regenerable, ignored project outputs using a frozen plan. It preserves tracked
inputs, package stores, current dependency binaries, scaffold backups and audit records. The
[producer profile](../build-and-checks.md#local-producer-profile) keeps stores, scratch and output
on the repository volume; all persisted candidate paths are relative to the current repository.

## Plan, review, then apply

Choose a fresh run name under target/artifact_cleanup_audit. Both modes require exactly the
mode and run directory shown below. A plan freezes the producer hash, Git HEAD, complete tracked
content hash, candidate fingerprints and skipped paths. Each candidate records its relative path,
file/tree kind, file count, byte count and SHA-256 fingerprint.

```bash
python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py plan target/artifact_cleanup_audit/daily-example
# Inspect plan.json in that run directory before applying it.
python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py apply target/artifact_cleanup_audit/daily-example
```

The example name is illustrative: use a new name for each actual run. Plan and result JSON files
are created exclusively, with sorted keys, two-space indentation and one terminal newline. An
existing plan is never overwritten. An already applied run refuses. Any change to HEAD, producer,
tracked content, candidates or skipped entries refuses with plan drift; create a fresh plan.

The entry validates its own source, local profile and strict process checker sources before use.
It checks the selected audit directory and plan/result file before metadata delegation, then
validates every Git-listed tracked input before creating stores or audit directories. Apply also
checks plan.json as a read-only input. All Git/process children receive the local environment;
valid prepared stores and unrelated values survive. Read-only tracked inputs may have hard links.

Every audit write has a fresh exact-file plan. Result aliases, hard links, directories or invalid
parents refuse before deletion. Borrowed save(path, value) validates its selected output under
target before its exclusive write. Borrowed apply(directory) validates the same audit authority
as the CLI; its directory must be under target/artifact_cleanup_audit with one run component.

## What can be selected

The census checks doctrine/scratch/tmp, Rust incremental and rendered-book trees, plus ignored
.log, .bin, .tmp, .orig, .rej and .DS_Store strays under target. Protected stores and dependency
binaries are pruned. Unsafe entries appear in skipped with a reason; they are never followed or
removed. Nested Git markers, symlinks, foreign devices, special files, nonignored paths and tracked
descendants refuse. A regular deletion file must have exactly one hard link. Directory link counts
retain their normal semantics; a linked read-only input is distinct from a deletion candidate.

Fingerprinting reads bounded chunks and includes each path, kind, file size and content digest.
Apply rechecks the complete frozen census, then each candidate immediately before removal. A
current tracked-content digest must still agree afterward. It verifies all selected paths absent
and records removed tree/stray/file/byte totals, zero residue, unchanged tracked inputs and skips.
An error stops the operation; it supplies no rollback for artifacts already removed.

## Process evidence is required

Before deleting anything, apply runs the project-owned scripts/check_handoff.sh. The canonical
checker requires successful, nonempty, structurally complete ps/lsof evidence and caller ancestry.
An unavailable or malformed census refuses2; a live project job blocks1. Both statuses abort
cleanup before removal. A complete clear census permits cleanup. The inherited neutral checker
is retained as provenance and is not used to authorize deletion.

If OS evidence is unavailable, obtain the necessary OS-visible execution access and rerun; do
not treat an empty census as proof of idleness. The checker uses its default strict mode and does
not infer that CUA is idle. Cleanup's original plan/apply argument interface is unchanged.

After a real run, inspect selected-path residue without activating the producer launcher, which
would recreate scratch. Update the single latest cadence entry in docs/ARTIFACT_CLEANUP.md and
commit its owned task. Rebuild native/WASM/book outputs and run the appropriate controls only
after the immediate residue proof. All roots and examples remain relocatable.

## Verification scope

Actual captures execute both mode bodies: creation, two exclusive JSON writes, process dispatch
and removal of isolated controlled artifacts. Independent omissions and unsafe physical audit,
input and store fixtures calibrate the plans. Complete original policy bodies and constants remain
exact after the declared planning, child-environment, process and unique-link adapters. The original
fifteen cleanup cases and eleven compiled guard-fault assertions retain their refusal families.

D210's real legacy checker returned success on unavailable lsof, and its actual apply body reached
a trapped deletion. The canonical checker refuses that evidence; actual blocked/clear process
fixtures test statuses1/0 separately. D211's multiply-linked candidate was admitted by the old
inspector; the new guard refuses and its omission fails an independent assertion. Linked tracked
inputs remain readable. Real project plan runs preserve every tracked byte and perform no deletion;
controlled process fixtures are not a certificate of an unobserved host's current process state.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py --real
```

Complete authority for the existing cleanup fixture producer follows in .a22.2; running its original
cases here verifies compatibility and does not close that separate producer audit.
