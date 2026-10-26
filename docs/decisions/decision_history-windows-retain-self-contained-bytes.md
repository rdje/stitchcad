# History windows retain self-contained bytes and stable logical addresses

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `SPINE.19.2`; D65 and the adopted archive/atomic-retirement contract.

answers: "how do I retrieve packed historical records?" · "does history retrieval require old Git objects?" · "what bounds archive storage and decompression?"

A working archive at its 64-file ceiling blocks the next normal rollover. Consolidate immutable
records into a content-addressed tar.gz window, with complete per-file identity in a bounded manifest
and a Markdown catalog. Keep logical paths and the original bytes, including descriptors; redirect
live links to catalog anchors. Corrections remain superseding records. Compressed bytes travel with
the checkout, so shallow clones and disconnected readers can retrieve records without older Git
objects. Git supplies the capture snapshot, not the retrieval dependency.

The Python 3 standard-library maintenance reader verifies strict manifests, payload digest, exact
membership and each decoded file. Decompression is capped before parsing. Tar links and unexpected
paths are refused; materialization writes only validated logical files into a new same-volume target/
directory. It never invokes tar extraction or follows archive-authored paths. No third-party Python
package or package cache is introduced. Python is a documented maintenance prerequisite, separate
from Rust application runtime dependencies.

Preserve the existing logical per-record and combined decoded history bounds, plus the 64-file
working Markdown bound. Measure resident payloads, controls and catalogs separately and together.
Control and payload collections have finite additional bounds; packing never grants more aggregate
history capacity. SPINE.19.2 records the exact numeric protocol before implementation. Future
windows require the same proof and immutable descriptors; if aggregate capacity is exhausted, stop
and own a new retention decision rather than widening a ceiling to land content.

Retirement follows copy, verify, use, delete: the installed reader must reconstruct every source byte
in an isolated repository-volume fixture before the exact source copies are removed. Existing ledger
checks consume the reconstructed logical inventory and retain their mutation arms. Source identity,
content identity, coverage, navigation and size are distinct claims. A digest cannot establish semantic
truth, and this tooling does not settle D40's ledger-agnostic coverage/pointer obligations.

## Capacity feasibility0339

All16 v1 slots are occupied. The source-ad3f8fc prototype retains946 logical records and33 exact
old master/manifest/payload files;16 original catalogs stay physical. Four candidate windows contain
979 members. Native OS tar listing/content matches every original byte, independently of the writer.
Including opaque evidence, decoded pressure is3583027B/47932 lines; resident1966510B, active controls
221328B, maximum payload851673B. All existing independent caps fit this snapshot; growth remains bounded.
The installed v1 reader correctly refuses the prototype. No schema/guard/live backing changed.
G1-SLICE.RETENTION-CAPACITY.2 must implement a versioned transition, preserve logical addresses and
original backing/catalog bytes, maintain offline retrieval, and verify before exact source retirement.
The guarded producer is retention_capacity_probe.py under docs/tasks/artifacts/history_archive; it is
historical, source-HEAD-bound and must never be rerun against a later HEAD. No cap rise is adopted.

### Draft transition requirements for capacity.2

- The v2 master has exactly version/windows/legacy_catalogs. Active windows remain at most16;
  legacy_catalogs has the16 original catalog path/lines/bytes/sha256 rows, pinned at transition.
- Active manifests retain the v1 fields with version2. Each member adds kind=record or backing;
  at most256 members, bounded paths/integers and exact full-file digest remain mandatory.
- Records keep their original logical names and bytes. Backings retain the original v1 master,
  sixteen manifests and sixteen compressed payloads as33 exact byte members. Default decoding is
  one flat level; original payloads are evidence, not recursive active compression containers.
- Keep original catalogs physically present, byte-exact against pinned master rows. New catalogs
  cover the new active manifests. Their bytes and every opaque backing count in total decoded/
  resident pressure; active controls/payloads retain their separate existing limits.
- Keep public list/read/materialize and inventory(root)'s return shape. A separate read-backing
  operation may expose exact old files. No historical Git dependency is added to ordinary reads.
- v1 remains readable. Transition guard must reconstruct every committed v1 control/payload and
  compare exact catalogs before permitting physical relocation. Future guards preserve every
  committed v2 active manifest/payload/catalog and the original catalog descriptor rows unchanged.
- Optional source proofs retain original legacy-window comparisons. Initial repack windows share
  one committed v1 source revision: verify the complete cohort against that source's logical records,
  all33 original backing files and16 catalogs. Future ordinary captures compare committed raw input.
- Implement refusal controls for both schemas, paths, kinds, membership, digests, budgets and immutable
  relocation. Adapt current archive consumers to distinguish record/backing rows and legacy catalogs;
  frozen capture_window14/15/16 and this historical capacity probe must remain unchanged.
- In a repository-volume fixture, prove full public/offline retrieval and materialization first.
  Then install the verified candidate, verify again, and delete only source-pinned original paths.
  At64 raw files, pack the committed baseline before creating the migration's next retained record.

These are implementation requirements, not permission to bypass the current guard. The actual
migration must remeasure its newer source, including0339's retained record; prototype metrics apply
only to ad3f8fc. No broader or unbounded retention capacity is claimed.

## Reader support0340

Capacity.2.a adds strict flat-v2 decoding and public read/list/materialize for isolated candidates.
Original v1 functions remain byte-for-byte in AST after their explicit version-branch rename; current
v1 CLI, purity, metadata authority and materialization controls remain applicable. Live storage stays v1;
committed-byte guard/source proofs and installation remain capacity.2.b/.c. No old backing is relocated.
During this bounded transition, complete completed-unit receipts may stay in their existing changelog
sections, with original Git interval/hash, until494 is sealed after packing. This avoids a65th raw file.
Only duplicate DEV_NOTES navigation is curated; original archived lesson bodies remain retrievable.

The v2 reader validates retained JSON metadata and binds original logical identities; compressed
legacy payloads remain opaque and are never recursively decompressed during ordinary reading.
