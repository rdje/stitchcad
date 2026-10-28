# Self-contained history grows through a finite, versioned generation index

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — active v3 read/cutover authority; authoring available; scoped feature acceptance complete; full integration/platform proof remains separate.
- **Owner / source:** G1-SLICE.6.4.5.capacity.apply; director delegates the decision and requires
  signoff/production quality.0391 selects self-contained retrieval over conditional Git receipts.

answers: "how does retained history grow after the first aggregate budget is full?" · "why eight generations?" · "which archive guarantees remain unchanged?"

answers: "what pins source and recovery audit during generation cutover?"

answers: "how do append and explicit seal preserve a complete cohort across generations?"

answers: "how do existing refusal clients retain their predicates under generation storage?"

answers: "what full-capacity resource evidence supports the finite generation reader?"

## Authority and scope

This is separate authority for a version3 topology, not a v1/v2 constant increase. Existing v1/v2
readers retain4000000 decoded/resident bytes,60000 decoded lines and their original refusal rules.
All existing record, manifest, payload, catalog and opaque legacy backing bytes remain immutable.
The v2 master is preserved byte-exact as generation1's control. Ordinary retrieval remains entirely
local, self-contained and independent of historical Git objects. Git is used only for explicit
committed/source proofs. No archive evidence is replaced by a conditional Git locator.

The root v3 index has at most8 generation descriptors, at most8192 bytes and exact identities for
the generation controls and a domain-separated tree digest covering every indexed physical file.
Raw drafts remain unsealed; payload/manifest/catalog changes cannot preserve a sealed tree identity.
The tree stream starts with ASCII `StitchCAD-history-generation-tree-v1` followed by NUL. For each
indexed physical path in bytewise UTF-8 order, append path/NUL/decimal-byte-size/NUL/lowercase-file-
SHA256/LF, then SHA256 the complete stream. It includes the generation control, manifests, compressed
payloads and catalogs; it excludes the root index and uncaptured raw drafts. Paths contain no controls.
Each generation retains these independent limits:

| Dimension | Inclusive bound |
| --- | --- |
| Decoded bytes, including opaque evidence and catalogs | 4000000 |
| Decoded lines | 60000 |
| Actual resident bytes, including its controls | 4000000 |
| Windows / members per window | 16 / 256 |
| Control bytes / lines / files | 262144 / 20000 / 17 |
| Working Markdown / per-file bytes / lines / maxline | 64 / 160000 / 2000 / 400 |
| Compressed payload / decoded tar bytes | 1048576 / 8388608 |

Only the first generation retains the16 original v1 catalogs and33 opaque backing files. Later
ones contain new records, not recursively active archives. Sealed generations have no raw segments;
all new raw segments belong to the last, active generation. Window and logical identities are
unique across the complete index. Root metadata and physical orphan checks cover the whole tree.

## Budget derivation and limits of the estimate

Run python3 -I -B docs/tasks/artifacts/history_archive/generation_budget.py before cutover;
--self-test is registered in run_history_archive_probes.sh. Source6121f21 with uncommitted whole-task
record540 measures3999898 decoded bytes/53085 lines/994 records,2125362 physical bytes. OS wc agrees
with the physical byte census. The working source differs from HEAD only by explicitly reported data;
no committed-snapshot claim is made.25 boundary/oracle/refusal controls pass rc=0.

The planning window is the director's default100 slices. Its explicit assumption is at most one
full160000-byte retained segment per slice, plus a compact catalog allowance512+208 bytes per slice.
The allowance reserves a complete512-byte header per segment, although capture normally groups them;
208 covers each bounded200-byte logical name and heading syntax. Thus3999898+100*(160000+512+208)
=20071898 bytes. Ceiling division needs6 four-million-byte slots; the smallest binary fanout is8.
This is a workload envelope, not a guarantee for arbitrary future slices or an observed parser memory
peak. The loader and transaction must prove their own boundaries before installation.

Consequently total decoded history is at most32000000 bytes/480000 lines. Resident history is at
most32008192 bytes including the index. Control bytes are at most2105344; control files137 and
payload files128. Working Markdown is at most192: seven sealed groups'16 catalogs,16 original
catalogs, and64 in the active group. Shared files cannot be charged to multiple generations.
Per-record/payload/field/path limits remain unchanged. No automatic budget growth is permitted;
near capacity, an owned new retention decision is required before further retirement.

## Required implementation and acceptance

- Strict versioned dispatch: unsupported schemas/keys/types, duplicate identities, reordered or
  removed seals, changed old bytes, links, foreign volumes and unclassified physical files refuse.
- Preserve public logical addresses and inventory's return shape. A generation control records its
  exact windows and legacy catalogs; the root pins its bytes. All but the last descriptor are sealed.
  Appending windows changes only the active control/root. Opening another generation seals indexed bytes; remaining unsealed drafts retain exact bytes
  in the new active group. Initial transition may assign existing raw records to
  generation2 while generation1 preserves the original packed cohort and exact v2 master.
- Validate schemas, complete membership, per-file identities and each independent budget. Decode one
  flat tar level; never recursively expand retained compressed evidence or call filesystem extraction APIs.
- Plan every source/output/child before effects. Source HEAD, all producer dependencies, both original
  trees and candidate must be pinned. Keep exact old trees on the repository volume; refuse unknown
  recovery states. Readers remain idle across the multi-directory transaction.
- Test inclusive/+1 limits, ambiguous/truncated/corrupt populations, source/producer drift, real rename
  interruption, explicit recovery, offline public byte retrieval and materialization. Compare original
  contents with independent Git/OS tar oracles. No synthetic or client fixture may weaken its old predicate.
- Measure resource behavior; state process-interruption versus power-loss guarantees accurately.
  Full CI/platform integration remains separately owned; focused archive feature acceptance is required.

[Python's tar guidance](https://docs.python.org/3/library/tarfile.html#extraction-filters) motivates
explicit membership/path/resource checks beyond extraction filters. The reader continues not to extract.
[SQLite's atomic-commit description](https://sqlite.org/atomiccommit.html) distinguishes journal and
filesystem durability assumptions. The existing directory-swap protocol proves process interruption;
this decision does not confer SQLite's power-loss guarantee on that protocol.

## Cutover0392

Preparation pins HEAD, formatter, installer, reader, generation decoder and environment profile.
The capped journal binds the exact audit bytes, including both source/candidate identity lists.
Real interruptions, native byte oracles and self-hash/audit-binding omission controls pass; original
trees remain backed up. Runtime guards remain active under Python -O; owner length is bounded80
before effects. This proves process-interruption recovery, not hardware power-loss durability.
Authoring/sealing, current-client/permanent-smoke and full-budget resource acceptance remain immediate
G1-SLICE.6.4.5.capacity.apply.accept work before returning to garment intent.

## Authoring0393

Append greedily captures the largest sorted raw prefix fitting the actual compressed/tar and group
budgets; a full group seals before the remaining committed snapshot continues in a new group. All
prior indexed bytes and sealed descriptors remain exact. Same-revision source proof compares the
complete cohort across every new window/group. Explicit seal opens an empty active group before a
new retirement; uncaptured drafts remain unsealed and byte-exact. Empty seals and slot9 refuse before
workspace activation. Capture of one record is valid in v3; original v2 two-record rules remain fixed.
Git proof admits only exact generation1..8 control paths; committed-v3 HEAD fixtures exercise that
branch independently of the initial v2 transition. Client/resource feature acceptance remains open.

## Client acceptance0394

Legacy cohort discovery uses flat published manifests; the original v1 producer/population and28
assertions remain unchanged. V1 evidence readers do not require an unused generation decoder; v3
plans include it explicitly. Window title/prose fixtures re-sign only their synthetic tree metadata,
keeping inventory valid while independently checking title and committed immutability predicates.
Original CLI/guard comparison adapts only exact declared new validation nodes, requiring each node's
presence once. Output-hardlink fixtures keep archive sources valid; source-hardlink refusal is a
separate physical control. Permanent transaction smoke freezes current committed archive/ledger bytes,
uses real target-only swaps and preserves all live inputs; byte-identical candidates also exercise
actual installation/recovery when no raw records exist. Deeper authority/platform/resource proof is
separately owned; these focused checks do not claim its completion.

## Resource acceptance envelope (set before measurement0395)

The synthetic full-decoded fixture fills all8 groups to4000000 bytes each, preserving original
opaque bytes. An isolated observer launches the actual public verify CLI as its sole waited child;
it does not charge fixture-construction memory to the reader. The regression target is128MiB peak
RSS for this population, with a30-second deadlock timeout. Report elapsed time; this is no universal
latency SLA, OS memory enforcement or proof of every possible population/platform. Native tar byte
oracles and exact independent decoded totals verify the fixture before interpreting the measurement.

Normalize modern Darwin ru_maxrss as bytes and Linux as KiB; unsupported platforms refuse measurement.
[Linux man-pages](https://man7.org/linux/man-pages/man2/getrusage.2.html) specifies KiB and the largest
waited child. [CPython](https://github.com/python/cpython/blob/main/Modules/resource.c) returns the
kernel value directly. Apple's archived man page says kilobytes, but current
[XNU calcru](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_resource.c) assigns
resident_size_max without scaling; same-process native time -l and getrusage agree14680064 bytes on
this host. D270 owns the local unit mitigation; no external repository is modified.

## Resource result0395

v3_resource_contract.py independently reconstructs native tar bytes for1330 records/33 original
backings; all8 groups are exactly4000000 decoded bytes, with128 windows/192 working Markdown and
457 physical files. A real458th file refuses before decode; only the exclusively owned extra is removed. The actual public CLI runs as the fresh
observer's sole waited child. On Darwin arm64/Python3.14.7 it measured122470400 RSS bytes and0.270086s,
within the predeclared128MiB regression target. Native time separately measured121847808 bytes/0.22s;
the tracked --unit-oracle gave exact same-child time/getrusage agreement14729216 bytes. Unit/boundary
controls8 pass0. Earlier1167-record tracemalloc diagnostic found33324623 current/47814348 peak traced bytes and0 collected
cycles; traced Python allocations and whole-process RSS are different observations. No universal
population/latency/other-platform or hardware power-loss guarantee is inferred. Scoped component gate0396 passes; full integration remains separate.

answers: "does an aggregate working-Markdown count of64 require capture?"
D284 correction0429: no. Native generation inventory sums group counters for the CLI readout;
each group is bounded64 and the aggregate192. Source db57efb has21 sealed catalogs/43 active raw
records, totaling64; independent rg census also finds43 raw files. No actual pressure required packing.

## Scoped feature closure0396

The registered archive entry passes original legacy28/window1013, budget25/runtime4, reader46,
authoring10+4, transaction27+identity and resource8 controls. Original/public/source/ledger bytes,
process-interruption recovery and full real topology remain exact. Latest full-topology Darwin CLI
measurement is122716160 RSS bytes/0.320178s; this stays a scoped regression observation. The capacity
activity is complete; G1 garment input work resumes. No full repository CI, deep producer-authority,
other-platform or power-loss verdict follows this component gate.

## Active raw-boundary capture0450

answers: "when did the active group actually require capture after D284?"

After0449, native census found64 raw files in active generation2 and21 sealed catalogs:85 aggregate.
The active group's empty window list rules out empty seal. Published append/install captured all64
committed files into window22. Native source proof reports64 byte-identical files/zero missing-extra;
independent public materializations match1055 files/3259088 bytes by size/SHA-256. Retained backup98
files remains target/history-capture-0450/installation/backup. Post-capture working Markdown22, then23
with whole0449 record602; no capacity/policy or power-loss guarantee changed. Product resource work
resumes at .6.8.2.2.2.2.2.4.3; full integration acceptance stays separately owned.

## Per-generation pressure recurrence0490

answers: "how is active pressure distinguished reproducibly from the aggregate readout?"

D302 repeats D284's incorrect scope inference in0489 planning;no packing/deletion occurred.
The tracked generation_pressure.py validates the complete archive and each group,then independently
counts physical raw/catalog files. Before record644:21 sealed+43 active=64 aggregate;after:21+44=65.
Three named controls distinguish aggregate64/active43 from aggregate85/active64 and the next raw
record. Use this read-only diagnostic before capacity work;working slots do not waive other bounds.
Whole0489 retains the prior exact erroneous claim;current pointers are corrected without rewriting seals.
