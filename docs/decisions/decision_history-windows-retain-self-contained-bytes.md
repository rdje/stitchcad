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
