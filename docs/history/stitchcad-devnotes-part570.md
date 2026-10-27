# Sealed engineering continuity — part570

- **Identity:** 32 lines, 3135 bytes, sha256:9c4a810b3f7edddfef165847e0f2926d8bc00574c0bb04ef06f6973551368cf9
- **Coverage:** whole0420 Material replacement task, lesson, ledger and prior retirement from 7b3736fe043af621d7ce693c43e34e50f36e9918.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 47573–48780, 1207 bytes, sha256:2a9fe77f6e50e85f239eb5efc78a57edb507c4bc383fb170d0ad2c5c0e470e5d
- Whole source interval: DEV_NOTES.md bytes 193–1078, 885 bytes, sha256:aa8652de1038ba9a487d42166fc68ff481d74f6fa21df3f7eb59c62830f2b600
- Whole source interval: CHANGELOG.md bytes 207–694, 487 bytes, sha256:b1f028ce22f2e1ad21d8ef9911f8b428d469bafe52f8e15e7a8f08b8cbfde9d1
- Whole source interval: docs/tasks/PLANNING.md bytes 90009–90565, 556 bytes, sha256:3aef4c7e3928e2c45d7a7ebad894a4d81c6e91283b670899274e8d6ba429a6b4
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.2.b.1`
  Status: `done`
  Goal:0420 immutable detached Piece material replacement; exact held references, no publication.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test piece_contract material_replacement:3 passed rc=0; native rebirth rejects the retired split source.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --test piece_contract material_replacement:3 passed rc=0; Piece::new checks live birth edges, held sources retain history.
  - [x] FIX: cargo test -p sc-core --test piece_contract material_replacement:3 passed rc=0; shared reason predicate, only material changes; other fields/labels/ids exact.
  - [x] ADDRESSED: cargo test -p sc-core --test piece_contract material_replacement:3 passed rc=0; split/loss evidence, no-op and exact reason retained; original Piece/ledger unchanged.
  - [x] NO REGRESSION: cargo clippy -p sc-core --test piece_contract -- -D warnings and cargo fmt --package sc-core --check rc=0; no bus/registry/physical execution claim.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part569.md:4 exact spans rc=0; retention/book/publication/size/fmt0, decision/live sync; actual candidate remains .b.
## _(2026-10-27 UTC)_ — detached Material replacement retains birth history

0420 Piece::with_material_assignment clones original fields and changes only material intent.
Piece::new after a split refuses its retired birth edge, while held history still resolves.
Reuse has_required_reason, retaining exact nonblank text/ids. No label inference, reference rewrite,
registration or geometry certification. Native3 checks rebirth refusal vs replacement success,
all other fields/original owner unchanged, no-op equality, blank reason and exact split/deletion
range/endpoint evidence with the same ledger. Selected strict Clippy/fmt0. Whole0419 task/lesson/
ledger/retirement569:4 exact spans; book/publication/size/retention/fmt0. Actual registry/source/currentness/auth/publication
remain the candidate/command layer's obligations. Promotion: structural-vs-current ontology decision.

## STITCHCAD-G1-0420 — preserve held Piece references in detached Material replacements

- Immutable with_material_assignment changes only Material intent; original fields/labels/ids remain.
- Preserve split/deletion evidence; no rebirth, normalization, source certification or bus publication.
- Native3/selected strict Clippy/fmt pass0; whole0419 proof569 retains4 exact Git spans.
- Actual AssignMaterial candidate/handler/auth/current registry checks remain owned by .6.5.2.b/.3.

## D271–D280 — retained causes and verified corrections
[Whole retirement/proofs](../history/stitchcad-devnotes-part569.md); prior exact captures remain linked.
- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part569.md:4 exact Git spans rc=0; whole0419 proof/retirement retained.
- [x] ADDRESSED: same rc=0; completed bus node and original native5/privacy2/source boundaries remain linked.
- [x] NO REGRESSION: retention/native replacement3/strict selected Clippy/fmt rc=0; product integration/release remains owned.
