# Historical archive reader

Historical records keep stable logical names when their physical Markdown files are packed.
The reader validates the complete backing archive before listing, reading or materializing records.
It uses Python3.9+ and the standard library; ordinary retrieval needs no historical Git objects.

Window14 retains50 complete committed records from b49128e, packed before command-catalog work
exceeded the fixed64-file working bound. Source comparison, exact materialization and every public
read passed; prior13 windows remain unchanged. Retrieval commands and capacity limits are unchanged.

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

The original first producing effect is trapped with stores absent. The original pure function
bodies and complete CLI body remain exact through declared planning, root-validation and child-
environment adapters. Successful committed-history comparison predicates also remain exact.
Captured materialization checks every planned file and directory, fresh effect checks, prepared
stores and exact output bytes. Fourteen physical refusals and eight actual compiled body faults
test source/output/store/parent authority, activation, rechecks and HEAD evidence. Device observations
are simulated in private fixtures.

The native mode separately executes public materialization, overwrite refusal, committed retention
and window11 source reconstruction. Every logical output byte, backing file and producer source
must match; owned fixtures are removed. Both Python3.9 and3.14 are checked. Full schema, decompression,
membership and capacity controls remain in the [governance contract](../governance.md#12-historical-records-and-bounded-retention).

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
python3 -I -B docs/tasks/artifacts/history_archive/history_archive_probes.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py
```

The native entry retains the original 28 calibrated cases. The companion's default mode captures
plans/effects and simulates child responses; it proves dispatch separately from the native run.
Its optional `--deep` campaign and further physical/original-body calibration are deferred under
the [feature-delivery policy](checking-policy.md), owned by .f.fixture-verification. No complete
deep-campaign pass is claimed. The activation-omission parser control has a focused regression.
