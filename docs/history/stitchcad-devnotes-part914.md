# Sealed engineering continuity — part914

- **Identity:** 27 lines, 2411 bytes, sha256:2cb02696aeb2d5e1e886531cc05df9c283999f5dde40cb39b0c6a3a6a1711ad8
- **Coverage:** whole0017 completed served-layer acceptance protocol from 7242d730803070b844f19150361d442f99603337.
- Whole source interval: docs/tasks/G1-SLICE-constructions.md bytes 23638–26049, 2411 bytes, sha256:2cb02696aeb2d5e1e886531cc05df9c283999f5dde40cb39b0c6a3a6a1711ad8
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

### `G1-SLICE.3c.4b.1` — served layers retain recipe and material intent with explicit lining scope

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires distinct served layers; feature-matrix rule 3
  separates modelled content from executable support. The prior commit has no layer descriptors.
- [x] **ROOT CAUSE (WHY + WHERE)** — a structurally valid Lining is still outside the v1 envelope.
  `cargo test -p sc-core --test layer_contract
  modelled_lining_refuses_v1_execution_with_named_diagnostic_piece_and_proving_gate` → `1 passed`,
  `rc=0`: inspection succeeds while execution returns env_lining, served Piece and G7.
  `foreign_middle_of_merged_offset_source_is_refused_despite_owned_ends_after_reversal` → `1 passed`,
  `rc=0`: live endpoints alone cannot establish whole-source ownership.
- [x] **FIX** — separate immutable Facing/Lining/Interfacing wrappers share owned-source and material
  validation. Offset operation/source intent stays authored; recipe owns dimensions and operation
  inputs. Explicit envelope checks distinguish inspection from execution; physical geometry is G2.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test layer_contract` → `9 passed`, `rc=0`:
  all kinds, materials, duplicates, wrong served Piece, split choices, hidden interiors, repair order
  and immutability. Three privacy doctests pass. Disabling lining scope and interval ownership
  separately makes each regression red, `rc=101`; restored checks pass. Moved fold checklist compares
  unchanged with `git show HEAD:docs/tasks/G1-SLICE.md`; existing Piece contracts remain green.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Fixture goldens stay unchanged.
- [x] **LOCKSTEP** — pre-code decision, `.6` envelope obligation, module/subsystem status, construction
  examples/API vocabulary, feature reasons, evidence relocation, frontier/logs/index and live memory
  are synchronized. The oldest interval lesson seals unchanged to devnotes-part16 with a verified
  digest. No geometric or release approval is implied. Next `.3c.4b.2` implements Hem.

