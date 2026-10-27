# Historical archive reader

Records keep stable logical names after packing. Python3.9+ validates the complete archive before
list/read/materialize using only its standard library; ordinary retrieval needs no historical Git.

```bash
bash scripts/history_archive.sh list
bash scripts/history_archive.sh read docs/history/stitchcad-changelog-part1.md
bash scripts/history_archive.sh verify
```

Read emits complete original bytes, including the sealed descriptor. Materialize writes all logical
records in sorted order beneath a fresh destination; repeating the command refuses overwrite:

```bash
bash scripts/history_archive.sh materialize target/history-review
# ARCHIVE-RETENTION: REFUSED — destination exists; overwrite refused
```

Destinations stay below `target/` on the repository volume. `HISTORY_ARCHIVE_ROOT` selects the project
or a fixture below project `target/`; the destination is relative to that selected root. Symlinks,
nested Git boundaries, other volumes and invalid destination kinds refuse.

## Materialization authority

Before effects, materialization declares code/profile/physical inputs and every output file/directory,
including implicit parents. Prepared stores/caller settings survive. Store overlap with the archive
or output tree refuses before activation, preventing unplanned entries.

Creation and exclusive opens recheck authority; bytes are never rewritten and existing destinations
are never overwritten. An I/O failure may leave a partial fresh review copy. Normal artifact cleanup
owns disposable copies. Imported validation and CLI list/read/verify remain read-only: no producer
profile load, store activation, directories or children. Invalid unused stores remain irrelevant.

## Committed-history evidence

```bash
bash scripts/history_archive.sh verify-retention
bash scripts/history_archive.sh prove-source window11
```

These proofs need local Git objects. Git children validate inputs, clear repository/config overrides
and disable optional locks, without store activation or caller changes. Committed windows retain exact
Git bytes. Initial absence requires a successful independent HEAD-tree query: failed observation,
unavailable/present-unreadable metadata refuses. Initial source reconstruction remains required.

## Bounded generations (version3)

An ordered index pins each generation's control and a domain-separated digest of every indexed
physical file. A changed manifest, payload or catalog cannot preserve that tree identity. Generation1
keeps the exact old v2 master, original windows,16 legacy catalogs and33 opaque backing files. Old
compressed evidence is never recursively activated. Logical addresses and public inventory shape stay
stable; list/read/materialize work without historical Git or network access.

At most eight generations are admitted, each independently bounded:

| Generation bound | Value |
| --- | --- |
| Decoded bytes / lines | 4,000,000 / 60,000 |
| Resident bytes | 4,000,000 |
| Windows / members per window / working Markdown | 16 / 256 / 64 |
| Record bytes / lines / line bytes | 160,000 / 2,000 / 400 |
| Compressed payload / decoded tar bytes | 1,048,576 / 8,388,608 |
| Control bytes / lines | 262,144 / 20,000 |

The index is at most 8,192 bytes. Derived totals are 32,000,000 decoded bytes/480,000 lines and 32,008,192
resident bytes. These bound storage/decode, not observed process-memory or unlimited growth.

All but the last generation are sealed. New raw drafts belong only to the active group and remain
unsealed until capture; all committed raw records still retain exact bytes. Orphans, shared ownership,
duplicate identities, links, malformed indices and reordered/removed seals refuse. Earlier v1/v2
schemas retain their original4MB/60000-line rules. Another retention decision is required before the
finite generation envelope is exhausted. Active-generation authoring and resource/current-client
acceptance remain the immediate continuation; read/cutover support alone does not close that work.

## Independent controls

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py --real
python3 -I -B docs/tasks/artifacts/history_archive/v3_reader_contract.py
```

The v3 fixture reconstructs its frozen cohort through logical bytes, preserving predicates as history
grows. Native tar/tree-hash oracles and full-current retention checks remain separate.

## Refusal fixture maintenance

Refusal producers declare copied inputs, fault payloads, resident fixtures and recovered outputs before
effects. Original sources/populations stay fixed; valid prepared stores survive and overlapping stores
refuse before activation. Creation/copy/write/unlink/child/removal boundaries recheck authority. A known
local-payload symlink fixture has exact lexical removal authority and a nonfollowing remover; other
links/unknown entries refuse. Protected cleanup covers interrupted setup; Git context stays project-scoped.

```bash
python3 -I -B docs/tasks/artifacts/history_archive/legacy_refusals.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py
```

The native entry preserves28 original calibrated cases. The companion captures plans/effects and
simulates child responses, proving dispatch separately. Its --deep and further physical/body
calibration remain .f.fixture-verification under the [checking policy](checking-policy.md); no full
campaign pass is claimed. The activation-omission parser has a focused regression. Historical traps
use capture-pinned v1 backings: only deliberate raw-read faults receive exact logical substitution;
the original missing-public-read assertion remains. Legacy catalogs/non-changelog rows stay valid.

## Guarded migration tooling

The installer pins HEAD, formatter, installer, both reader dependencies and environment profile. It
checks exact source/candidate identities, including byte/line counts. A flushed journal binds the
complete preparation audit before the first rename. Individual directory renames are atomic; keep
readers idle until installation/recovery finishes. This proves process interruption, not power loss.

Use a fresh workspace and choose a preparation matching the source schema. Owner ids name an actual
leaf (1–80 characters). Append requires committed raw records within bounds. Guards survive Python -O.

```bash
# v1 to v2:
python3 -I -B docs/tasks/artifacts/history_archive/v2_migration.py prepare target/history-transition
# v2 raw capture:
python3 -I -B docs/tasks/artifacts/history_archive/v2_migration.py append target/history-transition --owner G1-SLICE.6.4.5.3.d.2.a
# One-time v2 to v3:
python3 -I -B docs/tasks/artifacts/history_archive/v2_migration.py generations target/history-transition --owner G1-SLICE.6.4.5.capacity.apply.cutover
# After the chosen preparation:
python3 -I -B docs/tasks/artifacts/history_archive/v2_install.py install target/history-transition
# After interruption, before archive use:
python3 -I -B docs/tasks/artifacts/history_archive/v2_install.py recover target/history-transition
```

Recovery classifies both trees before effects, accepting only exact original/staged/installed states.
Source, candidate, code, audit, journal or HEAD drift refuses, including later records/commits. Original
trees remain backed up; the candidate survives rollback. Incomplete staging has not changed live storage.
Append preserves complete committed raw bytes and old windows. Headings navigate; manifests retain
full identities. Packing grants no capacity: stop before an oversized retirement.
