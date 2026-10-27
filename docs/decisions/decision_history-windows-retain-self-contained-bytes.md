# History windows retain self-contained bytes and stable logical addresses

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `SPINE.19.2`; D65 and the adopted archive/atomic-retirement contract.

answers: "how do I retrieve packed historical records?" · "does history retrieval require old Git objects?" · "what bounds archive storage and decompression?"
answers: "how are committed v2 raw records packed without changing old windows?"
answers: "what blocks further complete evidence retirement at the fixed aggregate history cap?"

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

## Transition guard support0341

Capacity.2.b adds read-backing for original control/payload bytes, legacy and initial-cohort source
proofs, and exact old logical/backing/catalog preservation before v1->v2 relocation is accepted.
Future v2 manifests/payloads/catalogs and legacy descriptor rows remain pinned; raw committed records
also retain exact Git bytes in both formats. Ordinary reads/backing retrieval remain independent of Git.
The shared metadata absence check remains fail-closed. D244 exposed fixture-relative Git pathspecs:
actual listing0/79 and master absent/present. Explicit --full-tree fixes the source/absence queries;
a controlled failed show and actual compiled omission reproduce/refuse the old behavior.
The guarded v2_migration.py prepare command creates a fresh source-pinned candidate only.45 transition
controls and33 public backing comparisons pass; the older946-record prototype correctly refuses
against the newer947-record HEAD. No live archive has moved; installation remains capacity.2.c.

## Installer and consumer support0342

Capacity.2.c.1 adds a journaled two-directory installation with explicit recovery. Each rename is
atomic; ordinary readers must be idle until the whole operation or recovery finishes. Exact original
source trees remain in repository-local target/ backups. Recovery preflights both trees and refuses
unknown bytes or changed HEAD/producer/candidate identities. This is process-interruption recovery,
not a machine power-loss guarantee. Ordinary producer output authority remains restricted to target/;
this maintenance entry alone owns the two fixed archive directory renames after exact-byte proof.
Ledger consumers resolve retained original catalogs and distinguish non-changelog receipts. V1 refusal
sources stay unchanged in a reconstructed fixture with explicit capture-pinned Git observations.
Tooling support does not install the candidate; capacity.2.c.2 owns current-HEAD preparation and use.

## Installed transition0343

Capacity.2.c.2.a installs source55ed18e as windows17–20, retaining947 prior records,33 exact original
backings and16 unchanged catalogs. Original directory trees remain in the same-volume transaction
backup. Live links use new catalog headings; whole0339–0342 receipts/task/lessons are retained494.
Ordinary read/list/materialize still need no Git or network. Source and committed-byte proofs remain
explicit Git operations. All independent caps are unchanged; subsequent raw records consume them.
Existing v1-only verification-client assumptions are owned by immediate capacity.2.c.2.b before
product work; focused migration proof does not claim full integration/CI. Later archive edits or HEAD
changes correctly refuse the source-pinned recovery command; the backup remains exact evidence.

## Verifier compatibility0344

Old-reader and original-refusal traps use exact retained v1 inputs. Current materialization and native
client checks use v2, while predecessor comparisons read original backing rows. Fixture Git comparisons
remain bound to the original v1 capture and prove complete path/byte membership independently.
D246 preserves the original missing-public-read assertion after packing; D247 publishes the legacy entry.
Focused native3.14 proof closes this retention continuation. Deep fixture/platform/full integration/CI
remain separately owned. Future aggregate exhaustion still requires a new decision; no cap was raised.

## Bounded v2 append0387

The explicit-owner append mode packs the complete committed raw snapshot into one new v2 window.
Preparation requires a valid source within existing bounds and a fresh target/ workspace. Every
physical archive input must match HEAD. Original windows, catalogs, payloads and legacy descriptors
stay byte-exact. Compact catalog headings preserve navigation; manifests retain full-file identities.
The existing source-pinned journaled installer owns installation/recovery and keeps original trees.
Source53bc624 packs44 records into window21; native tar and17 transaction controls pass. Restoring
draft538 leaves3992152/4000000 decoded bytes: packing frees working slots, not aggregate capacity.
G1-SLICE.6.4.5.capacity owns a new retention decision before further growth can exceed that cap.
The installer fixture's helper-source authority audit is separate; no complete producer proof is claimed.

## Retention decision required0390 — proposal, not adopted

Owner G1-SLICE.6.4.5.capacity; source007cdcb4adce4bb91704957cadb17b07da9b864c.
The existing archive remains valid:993 logical records,23 working Markdown,53051 decoded lines,
3997359/4000000 decoded bytes and2122823 resident bytes. Only2641 decoded bytes remain.
The next complete0388/0389 task/lesson/ledger bodies alone need3706B, before any descriptor:

| Source at that revision | Byte interval | Bytes | SHA-256 |
| --- | --- | --- | --- |
| docs/tasks/G1-SLICE.md | 49009–50999 | 1990 | d67d34b2b99c5641c7eb568690ea52c976dbf92797e578f9be825338646c7730 |
| DEV_NOTES.md | 193–1076 | 883 | aad79ec412a2bd847bf914a80f9e53b157262656e7f87ad9c91186ce35afbd00 |
| CHANGELOG.md | 207–1040 | 833 | 74dc51e698147966aac0c7e7da005ab2052c7a851fba26a9e17aae04066d8ccc |

Derivation: git show <revision>:<path>, slice each exact interval and hash with Python hashlib;
concatenate those byte slices and pipe to OS wc -c:3706 rc=0, independently matching Python lengths.
Existing history_archive.sh verify-retention checks the archive pressure rc=0. This is a source-bound
measurement, not a forecast. A dedicated watched retirement-feasibility producer is not yet added;
its omission is explicit. Existing source-span and archive validators remain the retained instruments.
Repacking/splitting cannot reduce this complete immutable body population below the aggregate cap.
Retiring only selected fields delays exhaustion and does not resolve the full retention obligation.

**A — preserve self-contained retrieval (recommended).** Authorize a separate finite-budget retention
redesign. Derive its total budget from the retained population, agreed future workload and measured
resource limits; verify it before adoption. Keep per-record/path/member/payload/control limits and
all existing bytes fixed. No proposed replacement budget or implementation is approved by this note.
This changes the aggregate-retention policy and needs an explicit director exception to the current
no-ceiling-increase-to-land-content rule; simply changing4000000 is rejected.

**B — freeze the existing archive; conditional future Git receipts.** Keep4000000 and every old byte.
Permit new completed-unit evidence to use exact Git object/interval descriptors. Before use, implement
the adopted version-object contract: finite descriptor bounds, named owner, ancestor reachability and
backup guarantee, whole-source/suffix hashes, executable retrieval, and actionable shallow/rewritten
object recovery. Future receipts would depend on those Git objects; unconditional shallow/offline
retrieval would continue only for existing archive records. No such new terminal is installed now.

Until the director chooses A or B, capacity.apply is blocked and product .6.4.5.4 stays pending.
No completed proof is discarded, no archive cap is raised and no new oversized segment is written.
At that source, the book had28B under its separate1200000B cap; scope-preserving containment precedes
new examples. That fact does not authorize an archive or book ceiling increase.
