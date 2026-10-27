# Historical archive reader

Historical records keep stable logical names when their physical Markdown files are packed.
The reader validates the complete backing archive before listing, reading or materializing records.
It uses Python3.9+ and the standard library; ordinary retrieval needs no historical Git objects.

```bash
bash scripts/history_archive.sh list
bash scripts/history_archive.sh read docs/history/stitchcad-changelog-part1.md
bash scripts/history_archive.sh materialize target/history-review
bash scripts/history_archive.sh verify
```

Reading emits the exact complete record bytes, including its sealed descriptor. Materialization
writes the complete logical population in sorted order beneath a fresh destination. For example,
the first command below creates the review copy; repeating it refuses overwrite.

```bash
bash scripts/history_archive.sh materialize target/history-review
# ARCHIVE-RETENTION: REFUSED — destination exists; overwrite refused
```

Use a destination below `target/` on the repository volume. A borrowed archive root may be selected
through `HISTORY_ARCHIVE_ROOT` when it is the project root or a local fixture below project `target/`.
Its materialized destination is relative to that selected archive root. Symlink components, nested
Git boundaries, other volumes and invalid destination kinds refuse.

## Materialization authority

Before activating stores or creating the destination, the producing entry declares its own source,
the local environment profile, every physical backing file, every output directory and every logical
output filename. The directory plan includes implicit parent creation. Local prepared stores and
unrelated caller settings survive. A store inside the output tree or backing archive refuses before
activation, so preparation cannot add unplanned archive or review-copy entries.

Each destination creation, parent creation and exclusive file open rechecks its relevant authority.
Files are created exclusively and retain original bytes. A later I/O failure can leave a partial
fresh review copy; the command never overwrites a caller's existing destination. Normal artifact
cleanup owns disposable review copies once they are no longer needed.

The imported validation functions remain read-only. CLI list/read/verify modes do not load the
producer profile, activate stores, create directories or launch children. Invalid unused store
settings do not affect those modes.

## Committed-history evidence

```bash
bash scripts/history_archive.sh verify-retention
bash scripts/history_archive.sh prove-source window11
```

These modes need the relevant local Git objects. Their Git children validate complete physical
inputs and run with repository/config overrides removed and optional locking disabled. They do
not activate stores; caller state remains unchanged.

Committed manifests, payloads and catalogs must match their recorded Git bytes. D224 repaired an
exception that accepted any failed Git observation as an initial archive transition. The exemption
now requires a successful independent HEAD-tree query proving the master control was absent.
Unavailable metadata and a present-but-unreadable control refuse. A genuine initial transition
retains its original source-reconstruction obligation.

## Independent controls

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py --real
```

## Refusal fixture maintenance

The archive refusal producer plans its copied backing files, deterministic fault payload names,
resident-bound fixtures and every recovered logical output before effects. Sources are validated
before reader loading; original source bytes and input populations remain fixed during the run.
Valid prepared stores survive, and a store inside its managed workspace refuses before activation.
Creation, copy, write, unlink, child and removal boundaries recheck their declared authority.

One filesystem-symlink case deliberately creates the known payload link to the original local
payload. Removal has explicit authority for that exact lexical path and target, and uses a
nonfollowing tree remover. Other links and unknown workspace entries refuse. Protected cleanup
covers interrupted setup and copying. Child Git context is scoped to the project.

```bash
python3 -I -B docs/tasks/artifacts/history_archive/legacy_refusals.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py
```

The native entry retains the original 28 calibrated cases. The companion's default mode captures
plans/effects and simulates child responses; it proves dispatch separately from the native run.
Its optional `--deep` campaign and further physical/original-body calibration are deferred under
the [feature-delivery policy](checking-policy.md), owned by .f.fixture-verification. No complete
deep-campaign pass is claimed. The activation-omission parser control has a focused regression.

## Guarded migration tooling

The v2 installer verifies source HEAD, producer identities and exact source/candidate bytes before
staging both trees on the repository volume. A flushed journal precedes the first rename.
Each directory rename is atomic; keep readers idle until installation or recovery finishes.

```bash
python3 -I -B docs/tasks/artifacts/history_archive/v2_migration.py prepare target/history-v2-migration
python3 -I -B docs/tasks/artifacts/history_archive/v2_install.py install target/history-v2-migration
# After interruption, before any archive use:
python3 -I -B docs/tasks/artifacts/history_archive/v2_install.py recover target/history-v2-migration
```

Recovery classifies both directory trees before changing either. It accepts only exact original,
staged and installed bytes from the prepared transaction; source, candidate, journal or HEAD drift
refuses. Original source directories remain in the transaction backup, and the verified candidate
remains available after rollback. An incomplete staging phase has not touched live storage.
This protocol handles interrupted processes; no machine power-loss guarantee is claimed.

The original v1 refusal producer runs unchanged in an exact retained-v1 fixture, with Git observations
pinned to its capture. Historical reader traps use v1 data; current materialization uses v2. Reader/client
native controls and fixture capture preserve original provenance through retained backing bytes.
D246 feeds only deliberate raw-read faults exact logical bytes; the original child-dispatch assertion
still rejects the missing public read. Legacy catalogs and non-changelog ledger rows remain valid.

The commands above describe the one-time v1 transition: preparation requires a v1 source and a fresh
workspace. Recovery requires its original source HEAD and recognized archive bytes. Later records
or commits intentionally refuse that rollback path; retained original trees remain evidence.

For a v2 append, commit all raw source records first and keep the source within the working bounds.
Use a fresh workspace and the actual owning leaf:

```bash
python3 -I -B docs/tasks/artifacts/history_archive/v2_migration.py append target/history-v2-append --owner G1-SLICE.6.4.5.3.d.2.a
python3 -I -B docs/tasks/artifacts/history_archive/v2_install.py install target/history-v2-append
```

Preparation packs the complete committed raw snapshot and preserves every old window byte.
The same idle-reader, recovery and retained-backup rules apply. Catalog headings link records;
full identities remain in manifests. Packing grants no additional aggregate history capacity.
