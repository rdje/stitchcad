# Sealed engineering continuity — part514

- **Identity:** 86 lines, 6912 bytes, sha256:8e9e8d7cd19f53e2ada0e585af41bf33c765e494a4e706a3b7da80f1bfb5b5d5
- **Coverage:** complete committed external inventory task/lesson/ledger and historical wanted-proof block from 0788c50575c74e615c50110fc7c1d56529193525.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 51208–52703, 1495 bytes, sha256:b324d61db16ecb43cb763890c6332e6b1ea43484e761d8e02b0cf869cf540ddc
- Whole source interval: DEV_NOTES.md bytes 193–1996, 1803 bytes, sha256:0810c5ef75a649353c363e058ad58397a16aa906192e9a51b34d9345feeb3255
- Whole source interval: CHANGELOG.md bytes 214–915, 701 bytes, sha256:cd562887d150cc739e7eafd137a27954e59f4c82513d706100815c68044f0b0f
- Whole source interval: docs/book/src/annexes/formula-wanted-signatures.md bytes 5188–8025, 2837 bytes, sha256:7714ed0edcb8caa7793c5e78eb480451eca694cca32db84d2be90d0d2a58855e
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.2.d.4.3.c.3.a`
  Status: `done`
  Goal: explicit owned SizeSet families/inventory with complete current source ids and exact pins.
  - [x] REPRODUCE / ISSUE: standalone valid sets can alias each other's owned ids; selected
    cargo test -p sc-measure --test size_set_inventory_contract passes6/0 rc=0, target/*native-0362.log.
  - [x] ROOT CAUSE (WHY + WHERE): per-set validators have no other set inventory; distinct set ids
    sharing axis80 refuse with exact both-owner positions, native6/0 rc=0 through local_environment.py.
  - [x] FIX: explicit owned family, complete private inventory/current checks and actual exact-ref
    borrows; native6/0 rc=0 retains singleton garment provenance and unselected-source refusal.
  - [x] ADDRESSED: selected inventory6+MTM5 pass0, target/size-inventory-native-0362.log;
    shared MTM fixture body equals committed original byte-for-byte, direct Git comparison rc=0.
  - [x] NO REGRESSION: privacy3/strict selected Clippy/fmt/book/publication79/121/1457/2396
    pass0; D250132 exact baseline rows/delta0, glossary rc=1; full host/Design/integration/CI later.
  - [x] LOCKSTEP: book/API/decision/defect/index/pointer aligned; transition_receipts.py513
    compares6 Git spans rc=0, whole0361/D256/call-lookup and complete historical handoff/journal;
    original unknown-capture boundary retained. Caps unchanged; G1still6/18.
    promotion: declined (existing external-set/context ownership). Commit0362; .3.b actual host next.
## _(2026-10-27 UTC)_ — explicit owned SizeSet inventory and exact current references

0362 SizeSet retains actual Garment/Mtm source ownership and delegates original current/path causes.
SizeSetInventory owns authored order, checking shared input and every owned set source id before
current semantics. Distinct set identities cannot reuse an axis/member/chart id. Reference queries
validate the entire current inventory, then require exact id/revision and return the actual object
with inventory/canonical/metadata borrows retained. Raw identity inspection supplies no current proof.
Empty explicit inventories mint nothing; canonical declarations can be shared through references.
Actual MTM and separately authored singleton garment sources use the same canonical inputs and
matching member labels; their explicit provenance remains distinct. Missing/wrong pins refuse;
removing a garment-only Count coordinate blocks even a selected still-valid MTM source. Native cases
also watch duplicate full sets and cross-set axis aliases with exact positions. Shared MTM fixture
body equals the committed original; its5 original contracts and6 new inventory cases pass0.
Privacy3/strict selected lint/fmt/book/publication pass0; D250132 exact baseline rows/delta0,
glossary rc=1. Continuity513 compares6 exact Git spans for whole0361/D256/call-lookup proofs and
complete historical .5e.1a.h/journal; original unknown-capture boundary remains explicit. Caps
unchanged; G1still6/18. Promotion declined: existing external-set/context ownership. Actual combined
source host .6.4.2.d.4.3.c.3.b next; full Design/execution/physical/profile/integration/CI unverified.


[Complete owned MTM source/D256 lesson](docs/history/stitchcad-devnotes-part513.md) retains
its original scope, checks and promotion decision.
## STITCHCAD-G1-0362 — own explicit SizeSet families and validate complete external inventories

- Add owned SizeSet family and private authored inventory with complete cross-set source identity checks.
- Exact reference queries revalidate all sources and borrow actual id/revision targets; singleton garment provenance stays explicit.
- Native6+5/privacy3/strict selected lint/fmt/book/publication pass0; moved MTM fixture body exact; D250132 rows baseline.
- Whole0361/D256/call-lookup/complete historical handoff continuity513 matches6 Git spans; original unknown boundary retained.
- Caps unchanged; G1still6/18. Actual combined host/full Design/execution/physical/integration/CI remains later.
## Verification and remaining proof

The catalogs are checked against independently authored kind matrices and the existing metadata
implementations: all 656 unary/binary kind cases and all 680702 built-in kind/class tuples at
arities zero through four. Matching alternatives must be unique. Complete typed built-in rows
are compared with all 24 normative rows, including symbolic tolerance roles, shared variables,
variadic requirements and exact result positions. Operator descriptors retain their closed
arities, 17 product alternatives and 14 directed quotients; wider variadic samples remain checked.

The two public integration suites are formula_operator_signature_contract and
formula_builtin_signature_contract. Nineteen actual compiled catalog faults must produce test-body
assertion failures and restore source exactly. The tracked wanted_signature_mutations.py producer
under docs/tasks/artifacts/formula_structure is exclusive; standing structural probes watch its
anchors. Its direct entry plans four inputs: the implementation, both current test sources and
their compile-time grammar chapter. Ordinary mode declares twenty diagnostic files and one mutable
implementation; classifier-only mode declares no files or mutable sources and invokes no native child.
Both modes validate directory/store/input plans before reads or activation:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/wanted_signature_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/wanted_signature_mutations.py
```

Run native mutations alone, without overlapping builds or probes. Fault and restoration calls
preserve both target arguments in their original order. Prepared child stores remain intact;
finally restores exact bytes before rebuilding both full five-test targets. A qualifying failed-body
thread panic must name a current assertion macro's guarded path, line and UTF-8 byte column.
Custom messages pass; nonthread notes, extraction, compiler and test-name noise refuse.

Two complete mode captures, seventeen independent omissions, a legacy-predicate red and ten
physical refusals check the plans and both test-source dispatch paths. Copied read-only mode accepts
three source hard links and unused log aliases without child, source or output writes.
The exclusive replay verifies all nineteen compiled failures, every executed target's failed-test
set and every failed body's current macro. Cargo stops after its first failed target in fourteen
cases; skipped targets receive no execution credit. Both restored five-test executables are selected
through current Cargo artifacts and run directly. All seventy-four frozen inputs, sixty-three
unchanged production Rust sources and twenty local diagnostic files remain exact. Dispatch controls
and native proof retain their separate scopes.

End of whole0362 external inventory and historical wanted-proof continuity.
