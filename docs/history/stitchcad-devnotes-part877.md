# Sealed engineering continuity — part877

- **Identity:** 49 lines, 4830 bytes, sha256:887656f3a03e93c87ec9d3ba7c5855e03c6932f9fc8a42896c2190344e90ea94
- **Coverage:** whole0707 scope,lesson,changelog and whole0062 byte-protocol node from 9dfa514f484d772d2ed75f8cd2d9c45429d0abea.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 46274–48214, 1940 bytes, sha256:2baa48f6925c64314a2cc53813b8fa66a0248fdc63dfba71ce8a0b21d23fa119
- Whole source interval: DEV_NOTES.md bytes 193–1494, 1301 bytes, sha256:784ff3f4f48a1f398180fcfaf51dcd72747097948c7c2e4b0dd74fd2b1c55e70
- Whole source interval: CHANGELOG.md bytes 207–702, 495 bytes, sha256:e3253654337026ffd8d9acfabf77860ce4a1ae59c900305ae1d049b4344336ca
- Whole source interval: docs/tasks/G1-SLICE-canonical.md bytes 5133–6227, 1094 bytes, sha256:fda39c28404ea5f7fccfc8f09836fa88dc6a21d74ffb40adf045afb7f8c91a76
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5f.1.c.slash.outputs.manifest.declaration.reconciliation.references.disposition.recovery`
  Status: `done`
  Goal:recover global edge-edit versus per-owner registration repair and semantic source ownership
  before native disposition policy. Intended ledger behavior is a boundary,not a defect/permission.
  Scope:public actual two-owner delete/retire/repoint cases and retained-notch source membership;
  preserve original objects and expose resolved-but-outside-target distinction. No runtime algorithm
  change,automatic repair or geometry claim. Track explicit native plan next;retire whole0706.
  - [x] ISSUE:piece-local eligibility cannot authorize global edits or discharge registrations.
  - [x] ROOT CAUSE (disposition boundary):published ledger/anchor/Piece contracts read;actual
    two-owner delete/retire/repoint and resolved-but-outside-target cases pass,rc=0.
  - [x] FIX:public-interface evidence plus book/decision boundary;native plan separately owns
    coverage/global-source/owner-repair/trim intent,without automatic mutation or false permission.
  - [x] ADDRESSED (disposition boundary):local_environment.py -- cargo test --offline --locked
    -p sc-core --test slash_spread_disposition_boundary_contract →2 pass,rc=0;global edit affects
    both owners,repairs selected owner only;repoint leaves immutable notch definition unchanged.
  - [x] NO REGRESSION (disposition boundary):selected strict clippy/fmt/book/render/diff and
    history876 three exact Git spans pass,rc=0. git diff --exit-code -- crates/sc-core/src
    Cargo.toml Cargo.lock →0;previous maintained chapter/Rust fences are an exact unchanged prefix.
    Primitive ledger behavior is intentional;no source/geometry/currentness/authority certificate.
  - [x] LOCKSTEP:book/live/frontier sync;promotion: promoted (reference journal decision gains answers).
  Verification:native2;Commit:STITCHCAD-G1-0707. Next .disposition.plan.
## _(2026-10-31 UTC)_ — distinguish global topology from owner and semantic repair

0707 recovers global topology versus owner registration/semantic source scope. An actual delete derives repairs for both registered owners;explicit repoint/retire changes only the selected registration. It cannot infer piece-local authority or another holder's consent. No runtime algorithm changes.
Two actual Pieces may share a source edge. Replacing one contour can leave its old notch outside the target despite global liveness,resolved registrations and narrow ledger readiness. Repointing registration leaves the immutable notch unchanged. Birth validation is used only as a membership diagnostic.
Public boundary2/strict selected clippy/fmt/book/render/diff/history876 three exact Git spans pass. Production source/Cargo files unchanged;prior maintained chapter/Rust fences remain an exact prefix. No actual geometry,map,semantic repair or publication is supplied. The ledger boundary is intentional.
LIVE states/G1 6/18/G0 unapproved unchanged;Product note updated. promotion: promoted (reference journal decision gains answers). Whole0706 retained876. Native coverage/global-source/registration/trim intent follows;semantic/zero outcomes and actual G2/currentness/history/bus remain owned. D372 persists.

## STITCHCAD-G1-0707 — distinguish global topology from owner and semantic repair

- Verify actual global edge deletion versus explicit per-owner registration retire/repoint behavior.
- Demonstrate live/resolved source outside replacement Piece and unchanged held notch after repoint.
- Native2/lint/fmt/book/render/diff/history876 exact spans pass;runtime/prior book content unchanged.
- Native source/global/owner/trim intent and actual geometry/currentness/semantic publication stay owned.
- ID: `G1-SLICE.5a.3d.1`
  Status: `done`
  Goal: settle exact canonical expression byte contract before product serializer implementation.
  Protocol: [complete prior inventory/proposal plan](G1-SLICE-formulas.md#canonical-byte-contract-pre-change-protocol--completed-in-g1-0062).
  Verification: seven actual reference roles/ten binary symbols/two distinct call collisions/six
  authored canonical examples. Director D103 ruling: unary (- child), square (^2 child); complete
  contract adopted in grammar/expert annex/canonical decision. Scoped structural/language/book pass.
  Containment protocol: derive Knowledge Map after the new decision, compact existing curated
  labels while retaining canonical paths/owners; no budget or doctrine configuration change.
  Final falsification protocol: four actual inventory-renderer tag/branch/argument faults must fail
  authored-byte assertions; record interpreter refusals distinctly from product compiled proof,
  restore exact tracked producer before checks/commit.
  Commit: `STITCHCAD-G1-0062`; production serializer .3d.2 remains pending.


