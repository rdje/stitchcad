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

Completed task navigation may keep each original ID and status beside a pointer to its whole
retained contract and proof. Part1077 retains the closed semantic-selection request subtree and
the complete lesson journal and changelog from commit9567e5b0, including their earlier navigation.
Part1078 retains the closed logical-source subtree from the same commit. The declared Git byte
intervals and decoded payload hashes preserve those original documents; live stubs are navigation.
Part1079 retains four completed0907–0910 task nodes and the complete committed lesson/ledger
snapshots from5480fb92. All original task IDs/statuses remain live;the complete original contracts,
check output and earlier journal navigation are retrievable from that immutable record.

```bash
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part1077.md
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part1078.md
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part1079.md
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

The receipt continuity check also exercises the staged400-byte task-line boundary with ASCII and
UTF-8, reconstructs the original whole D209 receipt after whitespace folding, and compares its sealed
defect report against the original repair commit. It rejects a missing, altered or duplicate report,
a new live D209 report, or an omitted closure pointer:

```bash
python3 -I -B scripts/local_environment.py -- python3 -I -B \
  docs/tasks/artifacts/ci_environment/receipt_retention_contract.py
```

This is a scoped retention check. Closed live navigation entries may point to sealed reports;
their presence does not make them open defects. The bounded status page publishes a summary rather
than global open/sealed totals. Complete automated lifecycle classification remains owned by
`PLANNING.5`; this check does not claim that broader census has been completed.

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

The pointer contract has16 cases, including valid retained or logical lesson links beside a
changelog link and refusal of an unknown lesson. Its plan controller preserves all13 original cases
and verifies the three additions against their introducing commit. Default and reordered native
runs each check all16 actual verdicts. Captures of child output prove dispatch and plan boundaries;
the separate native mutation run checks four real assertion failures and the restored16-case baseline.
Expected mutation evidence identifies the first affected case, its assertion site and verdict family.

The historical classifier control uses the helper and shell from the same original Git revision as
the old mutator in an isolated workspace. Injected unrelated assertion responses demonstrate the old
classifier's admission; they are separate from native verdicts. Current-body comparison permits only
the exact route-assignment adapter introduced with the public helper, then checks every remaining
original body node. Omitting the old cohort or changing that adapter is detected.

The retained-record client fixture preserves an already-packed authority byte-for-byte, including
version3 generation metadata. It attempts the original v1 window construction only if raw members
remain; that optional builder refuses other schemas before activation. No new window is needed for
the three fixed diagnostic records already retained in production packing.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_client_contract.py --real
```

This mode executes eight real read/Git/source-proof children and checks complete original provenance.
Companion controls cover seven physical refusals, seven body omissions and three content refusals;
their intercepted child responses are separate from native retrieval. Source/backing bytes remain
exact and the isolated workspace is removed.

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
python3 -I -B docs/tasks/artifacts/ci_environment/python_archive_fixture_contract.py --all
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
The existing environment probe suite invokes --all exactly once:body,physical and deep modes run
together,including actual watch argv/reflow and five omission/duplicate/dispatch controls. Native28
still run separately through legacy_refusals.py in the archive suite with real CLI/Git children.
Companion simulated children prove authority/lifecycle boundaries;they do not replace that replay
or pending producer-group/locality audits. Broad gates follow the [checking policy](publication-checks.md#checks).
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

The0569 append reached the same active working-file cap with62 raw records and two catalogs.
Window24 packs all62 exact committed files. Native tar extraction matches506939 bytes;complete
before/candidate/installed materializations match all1180 logical names,sizes,line counts,SHA-256
hashes and4144817 bytes. All102 original physical files remain exact in target/history-capture-0569;
sealed generation1 is unchanged. Installation reduces active64→3 and aggregate85→24 without raising
a bound. The next completed graph/Design receipt727 is authored only after these comparisons.
Keep original transaction trees for recovery evidence;do not rerun installation or recovery after
later edits. Live links route to window24 headings;sealed source bodies remain exact. This is
retention and process-interruption protection,with no new power-loss claim.

The0630 append reached active64 with61 raw records and three catalogs. Window25 captures all61
committed whole files. Independent before/candidate/installed materializations match all1241
logical names,sizes,line counts,SHA-256 hashes and4831870 bytes;native tar61/687053 bytes matches.
All104 original physical files remain exact in target/history-capture-0630;sealed generation1
is unchanged. Installation reduces active64→4 and aggregate85→25,without changing any limit.
Whole0628/0629 receipts788/789 follow the installed comparison,leaving active6/64 and aggregate27.
Live links route to window25 headings;sealed records stay exact. Protect the transaction/proof/
review workspaces;never rerun install/recovery after later edits. This verifies retained bytes and
the existing process-interruption protocol;it adds no hardware power-loss or platform guarantee.

The0684 append was required when two uncommitted drafts took active working files from63 to65.
Both drafts were copied/hash-verified in protected target/history-capture-0684-drafts before removal
from the archive;they were restored only after installation and complete cohort comparison.
Window26 captures all59 committed raw records. Before/candidate/installed native inventories match
all1300 original logical names,sizes,line counts,SHA-256 hashes and5239453 bytes. Independent tar
inspection matches59 records/407583 bytes;all44 unchanged physical backings/catalogs remain exact.
Live navigation routes69 links in three task files to window26 headings. Whole current retirements
stay raw. Transaction trees under target/history-capture-0684 and independent manifests under
target/history-proof-0684 remain protected;never rerun installation/recovery after later edits.
Packing changes no capacity,source proof or platform guarantee. Existing generation1 is unchanged;
active working files fall to5 before new raw drafts are restored.

The0734 append was required when one new draft took active working files64→65. Draft906 was
copied/hash-verified in protected target/history-capture-0734-drafts,then temporarily removed.
Window27 captures59 complete committed files from3a942a80. Before/candidate/installed native
inventories match1359 logical names,sizes,line counts,SHA-256 hashes and5482089 bytes. Native tar
inspection matches59 records/242636 bytes;47 unchanged physical inputs and generation1 stay exact.
The guarded installer retains original transaction trees in target/history-capture-0734;independent
manifests/materializations remain in protected proof/review directories. Draft906 was restored only
after complete readback. Live navigation routes71 links in seven task files to window27 headings.
Whole historical Tuck/Pleat0015 protocol907 remains a new exact record,keeping longer catalog links
within the task collection bound. With both new drafts retained,active working files are8 and
aggregate29. No limit,policy,source-proof or platform guarantee changes. This retains the existing
process-interruption protocol;no new power-loss claim. Never rerun install/recovery after later edits.

The0767 **early** append captures36 complete records906–941 fromcb1e7432 into window28.
Source generations had21/42 working files,aggregate63/192;active headroom was22,so this was
not a mandatory64-file boundary. D302's recurring aggregate/per-generation classification and
the older governance wording are corrected;the existing pressure diagnostic/control already
distinguishes them. Installed counts become21/7,aggregate28;continuity record942 then makes
active8/64 and aggregate29/192. No capacity or tool changes. Source proof matches all36 files;
all50 unchanged prior physical files stay byte-identical,including the sealed generation.
Latest public read941 matches3844 original bytes;the installer verifies complete logical/source
retention and keeps original trees in protected target/history_window28_capture for recovery.
Live links route55 destinations to window28 headings. Never rerun install/recovery after later edits.

### Active64 boundary and window30 —0883

Draft1055 expanded the active generation from64 to65 working Markdown files. A native pressure
census,after copy/hash-preserving the10967-byte draft,confirmed56 committed raw records plus
eight catalogs in generation2;aggregate85/192 was not the limiting scope. The published append
captured all56 exact whole files from a024533e into window30 without changing any budget.

Independent public before/candidate/installed materializations match1508 logical names and
6305703 bytes by size,line count and SHA-256. All114 original physical files remain exact in
the protected transaction backup;56 retained physical files,including older windows,stay
byte-identical. One guarded installation retired only the captured56 complete files. Draft1055
was restored byte-exact after the installed comparison;its four original Git source intervals
and public read match. Live navigation retargeted141 links in five maintained files.

Generation2 now has10/64 working files (nine catalogs plus the restored raw draft),leaving54
active slots;aggregate working31/192. Original transaction/draft storage remains under
`target/history-capture-0883` and `target/history-capture-0883-drafts`;independent manifests are
under `target/history-proof-0883`. Never rerun install/recovery after restoration or later edits.
This verifies retained bytes and existing process-interruption safeguards;it adds no power-loss
claim. Profile parameter product work stays separately owned/in flight while this prerequisite
is committed,and no product-code signoff is inferred from archive checks.

### Closed task goals and live navigation —0775

Whole0774 continuity and the original completed walk/mirror/slash goals/proof links remain
byte-exact in `docs/history/stitchcad-devnotes-part950.md`. All75 live node identities/statuses
remain present. The existing seam-edit task sibling now also owns the29-node constrained-truing
branch:all3767 source bytes move unchanged from execution. Its original scope/proof links remain
retrievable through that canonical sibling. Exact source-span/body and node-graph checks verify
this retention;the task-tree coverage census checks navigation. Native feature claims stay scoped.

```bash
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part950.md
```

### Bounded live changelog navigation —0779

Record954 retains the **complete**64804-byte prior `CHANGELOG.md`,including every older ledger,
identity and index route,plus whole0778 task/lesson continuity. This is early retirement:the source
was below its65536-byte ceiling with732 bytes remaining. The live changelog now points to that
complete sealed index;`list` continues to enumerate every logical record and `read` returns exact
source bytes. No history limit,reader API or existing record changes. Exact original Git spans,
body and public-reader readback verify retention for the committed shortened navigation.

```bash
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part954.md
```
