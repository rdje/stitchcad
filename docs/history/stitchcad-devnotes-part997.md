# Sealed engineering continuity — part997

- **Identity:** 37 lines, 3415 bytes, sha256:fef69f74c384154b0fc00e9bae217181f70c6a9225a9531abddd9deebb7d1bb3
- **Coverage:** whole0821 task,lesson,ledger and prior ADR partition rationale from 707e5ad31c43c3b538b168d441a8ae0d4a513a0e.
- Whole source interval: docs/tasks/G1-SLICE-constructions.md bytes 70240–71622, 1382 bytes, sha256:53b3899f0b6ef7643a190922fed4c0f29778dabfc8f08d2e85b1396bb6720ef4
- Whole source interval: DEV_NOTES.md bytes 193–953, 760 bytes, sha256:aa7bc582890bee9100757166cef420b61c62bd8e7be6ae0922fb2d7108ee0bb5
- Whole source interval: CHANGELOG.md bytes 217–846, 629 bytes, sha256:af4fa7d0936c3a1e003cc734a1c6b688075b0f5f60d29883b72a1ff1c2e04ba0
- Whole source interval: docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md bytes 4897–5541, 644 bytes, sha256:968e7de206cb21ad2596826f0436e4e90e505476baabca30a47a9fd41d307874
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable;corrections belong in a new record.

---

### G1-SLICE.5f.2.inputs
- ID: `G1-SLICE.5f.2.inputs`
  Status: `done`
  Goal:borrowed closed13-family input sum;retain11 embedded operation IDs and explicit authored operation IDs beside CloseDart/AddNotch target intents;no context/proof/default or identity issuance.
  - [x] ROOT CAUSE: launcher `cargo test -p sc-core --test drafting_operation_input_contract`:
    initial fixture used nonexistent EdgeRange::Full;corrected to public EdgeRange::whole;4 tests pass,rc=0.
    First commit correctly refused missing in-file retirement receipts in execution sibling;fresh exact-source/archive receipts restored before retry.
  - [x] ADDRESSED: same target plus `cargo test -p sc-core --doc construction::operation_input`:
    all13 actual source loans/history IDs,semantic target distinction,privacy/repeated controls pass4;source lifetime/replacement compile-fail2 pass,rc=0.
  - [x] NO REGRESSION: launcher `cargo clippy -p sc-core --lib --test drafting_operation_input_contract -- -D warnings`,
    fmt and `cargo test -p sc-core --doc construction::operation_examples`:lint/5 book examples pass,rc=0;
    book/publication118/374/1806/3388/four old fences exact;996 three Git/public spans/archive1450/83,per-generation21/64 and62/64,rc=0.
  Commit:STITCHCAD-G1-0821. promotion: declined (existing raw source/identity boundary);whole integration verification remains .5f.2.verify.
## _(2026-11-02 UTC)_ — borrowed native drafting inputs

0821 adds a closed13-family raw input view and family tag;11 native operation IDs remain original,semantic intents retain explicit separate history IDs.
Actual payload borrows preserve documents,ordered/repeated controls and targets;Debug reveals only family. No value read,frame/currentness or program acceptance.
Focused input4/source-loan compile-fail2/book examples5/strict selected lint/fmt/build/publication118-374-1806-3388 pass0;four old Rust fences unchanged.
996 whole Git/body/public exact0;archive1450/83,per-generation21/64 and62/64. G1still6/18;coupled verification deferred. Promotion declined: existing raw source/identity contract;explicit Cartesian frame/current source ports next.

## STITCHCAD-G1-0821 — retain actual native drafting inputs

- Added borrowed DraftingOperationInput and closed family descriptor for all13 native families.
  CloseDart/AddNotch retain explicit authored history IDs beside original semantic target requests.
- Four source/identity/privacy tests,two compile-fail borrow guards,five actual book examples,
  strict selected lint/fmt/book/publication118/374/1806/3388 pass0;four original Rust fences exact.
- 996 three Git/body/public spans exact;archive1450/83,per-generation21/64 and62/64.
  G1still6/18;whole program/context/geometry acceptance remains separate and unverified.

The chapter is partitioned into a contract, a grammar part and an examples part because one file
landed at 599 lines / 37 317 B against a `book_collection` per-part health of 400 / 24 576 and a
ceiling of 700 / 40 960 — 87 % of the ceiling on the day it was born, with every table normative and
nothing to trim. The partition is the containment doctrine's own remedy for a `partitioned_canonical`
surface, and the glossary set the precedent. The three parts are 308 / 241 / 92 lines and
20 593 / 12 452 / 7 116 B, each inside health, and the contract's §7 parts table is censused in both
directions so a fourth file cannot appear unlisted.

