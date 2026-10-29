# Historical archive reader

Records keep stable logical names after packing. Python3.9+ validates the complete archive before
list/read/materialize with its standard library; retrieval needs no historical Git or network.

```bash
bash scripts/history_archive.sh list
bash scripts/history_archive.sh read docs/history/stitchcad-changelog-part1.md
bash scripts/history_archive.sh verify
```

Read emits complete original bytes, including the seal. Materialize writes all records in sorted
order beneath a fresh destination; repeating it refuses overwrite:

```bash
bash scripts/history_archive.sh materialize target/history-review
# ARCHIVE-RETENTION: REFUSED — destination exists; overwrite refused
```

`HISTORY_ARCHIVE_ROOT` selects the project or a fixture below project `target/`. Destinations stay in
that root's `target/` on the repository volume. Symlinks, nested Git boundaries, other volumes and
invalid kinds refuse.

## Materialization authority

Materialization declares code/profile/physical inputs and every output file/directory, including
implicit parents, before effects. Prepared stores/caller settings survive. Store overlap with archive
or output refuses before activation.

Creation/exclusive opens recheck authority and refuse overwrite.
I/O failure may leave a partial review copy for artifact cleanup. Imported validation and CLI
list/read/verify load no producer profile, activate no stores and create no directories/children.
Invalid unused stores remain irrelevant.

## Committed-history evidence

```bash
bash scripts/history_archive.sh verify-retention
bash scripts/history_archive.sh prove-source window11
```

Proofs need local Git objects. Children validate inputs, clear repository/config overrides and disable
optional locks, preserving stores/caller state. Committed windows retain exact Git bytes. Initial absence
needs an independent successful HEAD-tree query; failed observation/unavailable/unreadable metadata
refuses. Initial source reconstruction remains required.

## Bounded generations (version3)

An ordered index pins each generation's control and a domain-separated digest of every indexed
physical file, including manifests, payloads and catalogs. Generation1
keeps the exact old v2 master, original windows,16 legacy catalogs and33 opaque backing files. Old
compressed evidence is never recursively activated. Public inventory shape stays stable.

Eight generations maximum, each bounded:

| Generation bound | Value |
| --- | --- |
| Decoded bytes / lines | 4,000,000 / 60,000 |
| Resident bytes | 4,000,000 |
| Windows / members per window / working Markdown | 16 / 256 / 64 |
| Record bytes / lines / line bytes | 160,000 / 2,000 / 400 |
| Compressed payload / decoded tar bytes | 1,048,576 / 8,388,608 |
| Control bytes / lines | 262,144 / 20,000 |

The index is at most 8,192 bytes. Derived totals are 32,000,000 decoded bytes/480,000 lines and 32,008,192
resident bytes. These bound storage/decode, not process memory.

The verify-retention working-Markdown readout is the aggregate across generations. Each generation
is independently limited to64; the version3 aggregate limit is192. Sealed catalogs remain charged to
their own generation, while raw records belong to the active generation. A total of64 therefore does
not itself require packing. Check actual per-generation pressure before scheduling capture or rollover.
The read-only diagnostic also checks counts against an independent physical/catalog census:

```bash
python3 -I -B docs/tasks/artifacts/history_archive/generation_pressure.py
```

Its active working-slot headroom does not waive the separate decoded,resident,line or window bounds.


All but the last generation are sealed. New raw drafts belong only to the active group and remain
unsealed until capture; all committed raw records still retain exact bytes. Orphans, shared ownership,
duplicate identities, links, malformed indices and reordered/removed seals refuse. Earlier v1/v2
schemas retain their original4MB/60000-line rules. Exhaustion requires a new retention decision. Append/seal/client/transaction checks pass. The full-decode fixture fits128MiB on Darwin/Python3.14.7;
this regression target is no runtime OS limit or other-platform proof. Scoped component checks pass.

## Sealed payload descriptors

The ledger probe verifies each materialized record's payload hash and declared line/byte counts.
Legacy seals require exactly one terminal newline; a blank final line refuses by name. Records with
structured whole-source interval headers retain their sealed raw bytes, including a blank EOF or no
final LF. An explicit base64 encoding declaration instead selects the decoded payload, with strict
fence, alphabet, width and decoded-size checks. No profile trims or adds newlines.

For example, raw part250 retains its blank EOF, while part639 has no final LF; encoded part658 hashes
its decoded source text. Descriptor integrity is separate from proof against the named historical Git
intervals and from changelog coverage. A false coverage declaration still refuses independently.

```bash
python3 -I -B scripts/local_environment.py -- bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh
```

## Independent controls

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_reader_contract.py --real
python3 -I -B docs/tasks/artifacts/history_archive/v3_reader_contract.py
python3 -I -B docs/tasks/artifacts/history_archive/v3_authoring_contract.py
python3 -I -B docs/tasks/artifacts/history_archive/v3_transaction_smoke.py
python3 -I -B docs/tasks/artifacts/history_archive/v3_resource_contract.py
```

V3 fixtures reconstruct the frozen cohort through logical bytes as history grows. Native tar/tree-hash
oracles and full-current retention checks remain separate. Transaction smoke uses a committed HEAD
archive/ledger copy; identity candidates cover commits without raw drafts.

## Refusal fixture maintenance

Refusal producers declare inputs/faults/fixtures/recovered outputs before effects, preserve source
populations/stores and reject overlap before activation. Creation/copy/write/unlink/child/removal
boundaries recheck authority. The known payload-symlink fixture has lexical removal authority and a
nonfollowing remover; other links/unknown entries refuse. Protected cleanup covers interrupted setup;
Git stays project-scoped.

```bash
python3 -I -B docs/tasks/artifacts/history_archive/legacy_refusals.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py --deep
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py --body
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py --physical
```

The native entry retains28 calibrated cases. Companion --deep verifies interrupted-copy teardown,
HEAD evidence and ten source-body omission controls with simulated children. Companion --body
compares complete original predicates,payload/resident builders,CLI arguments and public signatures
with frozen pre-guard source through explicit IO,extraction,setup and HEAD-evidence adapters.
Ten actual AST faults refuse,including redirected reader bindings;semantic reformatting passes.
This mode uses read-only Git and executes no fixture or build. Companion --physical runs the actual
unchanged entry in owned mirrors:19 input/profile/workspace/output/store refusals must precede any
effect. An independent child audit hook traps writes,environment changes and child dispatch;three
direct sentry controls and a valid-prefix activation control establish that boundary. All23 children
are reaped. Faulted mirror bytes,kinds,modes,links and identity/change metadata remain exact;read
access times are excluded. Exact lexical fault restoration precedes guarded cleanup;canonical
source/archive inputs stay exact. This finite entry proof does not execute the native28 cases.
Final calibration/watch integration remains .f.fixture-verification under the [checking policy](checking-policy.md).
Capture-pinned v1 traps substitute logical bytes only for deliberate raw-read faults, retaining the
original missing-public-read assertion. Legacy catalogs/non-changelog rows stay valid.

## Guarded migration tooling

Installation pins HEAD, formatter, installer, both readers and environment profile, checking exact
source/candidate hashes and byte/line counts. The flushed journal binds the complete audit before
renaming. Directory renames are individually atomic. Keep readers idle until installation/recovery
finishes; process interruption is proved, power loss is unproved.

Use a fresh workspace and match the source schema. Name an actual owning leaf (1–80 characters).
Append requires committed bounded raw records. Guards survive Python -O.

```bash
archive_tools=docs/tasks/artifacts/history_archive
archive_owner=G1-SLICE.6.4.5.capacity.apply.accept.authoring
# Choose ONE preparation:
python3 -I -B "$archive_tools/v2_migration.py" prepare target/archive # v1 to v2
python3 -I -B "$archive_tools/v2_migration.py" append target/archive --owner "$archive_owner" # v2/v3 capture
python3 -I -B "$archive_tools/v2_migration.py" generations target/archive --owner "$archive_owner" # v2 to v3
python3 -I -B "$archive_tools/v2_migration.py" seal target/archive --owner "$archive_owner" # v3 rollover
# Install; after interruption, recover before archive use:
python3 -I -B "$archive_tools/v2_install.py" install target/archive
python3 -I -B "$archive_tools/v2_install.py" recover target/archive
```

Recovery classifies both trees before effects: only exact original/staged/installed states qualify.
Source/candidate/code/audit/journal/HEAD drift, including later records/commits, refuses. Originals stay
backed up; rollback preserves the candidate. Incomplete staging leaves live storage unchanged.
V3 append captures all committed raw records in sorted bounded windows. It fills the active group,
then seals indexed bytes and opens another if any budget blocks the next window. Explicit seal opens
a group before retirement, carrying unsealed drafts byte-exact. Empty seals and slot9 refuse before
workspace creation. Source proof compares the whole same-revision cohort across groups/windows.
V3 accepts one record; v2 retains its one-window/two-record rules. Catalog headings navigate; manifests
give full identities. Packing grants no capacity: stop before an oversized retirement.

## Capture at the actual active-generation boundary

The0450 local capture used append after the active generation reached64 raw records. Its existing
window list was empty, so an empty seal would have refused. window22 captured all64 committed files
with no budget change. The source proof found64 byte-identical files and no missing/extra entries.
Independent before/after materializations matched all1055 logical records and3259088 bytes by size
and SHA-256. The installer retained original trees for explicit recovery; this is process-interruption
protection, with no new power-loss claim. A later raw record remains in the active generation as usual.

The0508 append addressed an actual full active group:63 committed raw records plus one catalog.
Window23 packs those63 full files;the public source proof reports no missing/extra member. Independent
native tar extraction matches all378790 captured bytes. Before/after installation materializations
match all1118 original logical names,sizes,SHA-256 hashes and3637878 bytes;the sealed generation1
index remains exact. Packing reduces active working files64→2 and aggregate85→23 without changing
any limit. Original transaction trees remain in target/history-capture-0508 for explicit recovery.
Live retrieval links route to catalog headings;sealed source bodies keep their original bytes.
The following completed inverse receipt665 is authored only after installation/cohort comparison.
This is verified retention and process-interruption protection,with no new power-loss claim.
