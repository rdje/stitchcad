# Sealed engineering continuity — part526

- **Identity:** 178 lines, 12535 bytes, sha256:0385cb8db82774e413495d5d4fe8b8acfd5282c3627d591045d5787452934e32
- **Coverage:** complete closed identity/sewing task scopes,0374 lesson/ledger and historical whole-recipe phase proof from 4d5a4a47bb8a3519528cbb2bd740e2ba1f7fbc95.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 4126–6436, 2310 bytes, sha256:b7acb6dd3a9d7e46e2ce9e05ad9ad2f45a94087f1618a66aa7cfc8fea78c7f8a
- Whole source interval: docs/tasks/G1-SLICE.md bytes 51253–53880, 2627 bytes, sha256:56f6422caf3fe0909857b9c47ff50c1efa658787eef58144fa634955763e2192
- Whole source interval: DEV_NOTES.md bytes 193–959, 766 bytes, sha256:c998a1b32dc5ec91419cdee64f97847b983964b14e45ad7f602298d00bba5725
- Whole source interval: CHANGELOG.md bytes 207–592, 385 bytes, sha256:b5f97d7eac7b2a433e3d0ff3fca482f83d3b00d4fc691fa4be70106e8b009db5
- Whole source interval: docs/book/src/annexes/formula-static-validation.md bytes 11134–17475, 6341 bytes, sha256:05b59a58eac2638ec30fa768e17b90c43b6288e2e3c6a9b1621f3df2db8dc63c
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.3a`
  Status: `done`
  Goal: the identity layer in `sc-core` (ontology §1) — `EntityId` (a dependency-free ULID), the
  injected `IdGenerator`, `EdgeRef`/`PointRef` (the entity id of the creating operation plus a
  persistent local tag), and the bounded exact rational parameter `t` in `[0, 1]`.
  Acceptance: an `EntityId` round-trips its 26-character Crockford form and orders lexicographically
  by creation; a deterministic `IdGenerator` reproduces ids byte-for-byte (the replay property);
  the parameter is exact under `+ − × ÷` with reduction, carries a total order and an
  `in_unit_interval` predicate, and reports overflow / division-by-zero as typed diagnostics; the
  crate still compiles for `wasm32-unknown-unknown`.
  Verification: recorded below — 40 tests green (31 unit, 9 property), `make wasm` cross-builds
  `sc-core`, every criterion re-derived in the `### G1-SLICE.3a` checklist.
  Commit: `STITCHCAD-G1-0004`
  Design: `decision_entity-identity-ulid-injected-generator.md`,
  `decision_edge-parameter-bounded-exact-rational.md`.

- ID: `G1-SLICE.3b`
  Status: `done`
  Goal: the persistent-identity contract (ontology §1.1) — reference resolution under split, merge,
  reverse, delete and offset-fragmentation, and the `RepairTask` an orphaned reference becomes. No
  silent reassignment.
  Acceptance: reference stability is a tested property under split/merge/reverse — a reference at the
  split point resolves to both fragments and the consumer states which it wants, merge recomputes the
  parameter by arc length, reverse maps `t` to `1 − t`; a deleted edge's references become visible
  `RepairTask`s naming the reference, the orphaning edit and the candidate resolutions; a design with
  unresolved references is savable and inspectable but cannot be released.
  Verification: recorded below — 62 unit + 8 contract-property + 9 identity-property tests green,
  `make wasm` cross-builds `sc-core`, every criterion re-derived in the `### G1-SLICE.3b` checklist.
  Commit: `STITCHCAD-G1-0005`
  Design: `decision_reference-resolution-journal-fold.md` (recorded before the code, per the `.3`
  decomposition's discipline), inheriting `decision_entity-identity-ulid-injected-generator.md` and
  `decision_edge-parameter-bounded-exact-rational.md`.

- ID: `G1-SLICE.6.4.4`
  Status: `done`
  Goal: current sewing copy/landmark targets and actual length-valued declaration sources.
  Children .1 current graph copy/range/stop revalidation; .2 actual formula length metadata and explicit
  G4 Profile-role boundary; .3 private owned sewing/root/landmark scope; .4 focused combined acceptance.
- ID: `G1-SLICE.6.4.4.1`
  Status: `done`
  Goal:0370–0373 sewing/input scope; [whole subtree/proof](../history/stitchcad-devnotes-part525.md).
- ID: `G1-SLICE.6.4.4.2`
  Status: `done`
  Goal:0370–0373 sewing/input scope; [whole subtree/proof](../history/stitchcad-devnotes-part525.md).
- ID: `G1-SLICE.6.4.4.3`
  Status: `done`
  Goal:0370–0373 sewing/input scope; [whole subtree/proof](../history/stitchcad-devnotes-part525.md).
- ID: `G1-SLICE.6.4.4.3.a`
  Status: `done`
  Goal:0370–0373 sewing/input scope; [whole subtree/proof](../history/stitchcad-devnotes-part525.md).
- ID: `G1-SLICE.6.4.4.3.b`
  Status: `done`
  Goal:0370–0373 sewing/input scope; [whole subtree/proof](../history/stitchcad-devnotes-part525.md).
- ID: `G1-SLICE.6.4.4.4`
  Status: `done`
  Goal: focused combined sewing/current amount and private ownership acceptance before garment families.
  Own three exact existing ownership/unused-source/borrowed-profile filters and unchanged-source
  privacy proof retention; whole closed sewing subtree/0373 continuity525. No broad integration gate.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-measure --test design_inputs_contract
    sewing_owner_refuses_graph_span_and_landmark_aliases_with_actual_root_positions -- --exact:1/0 rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): git diff --exit-code HEAD -- crates:empty rc=0;
    current metadata/source acceptance remains narrower than complete numeric/physical execution.
  - [x] FIX: cargo test -p sc-measure --test design_inputs_contract
    sewing_current_sources_validate_unused_landmark_anchors_and_keep_the_concrete_original_cause -- --exact:1/0 rc=0.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract
    sewing_current_sources_return_actual_borrowed_amounts_and_preserve_profile_deferral -- --exact:1/0 rc=0.
  - [x] NO REGRESSION: git diff --exit-code HEAD -- crates:rc=0; 0373 native5/privacy3/
    lint/fmt proofs retained unchanged. D154/D250 remain with original owners, no broad CI claim.
  - [x] LOCKSTEP: bash scripts/run_make.sh book:rc=0; scoped publication79/137 rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part525.md:3 exact spans rc=0.
    promotion: declined (focused closure of existing sewing decision). Completed STITCHCAD-G1-0374.
## _(2026-10-27 UTC)_ — focused complete sewing component acceptance

0374 closes implemented sewing/input scope with three exact ownership/unused-source/borrowed-profile
filters, each1/0 rc=0. Rust is unchanged from3bd83f9; its native5/privacy3/strict lint/fmt proofs remain
applicable. Current metadata/source checks preserve held data, original concrete causes and explicit
G4 Profile deferral. Structural topology and full numeric/physical/execution/release proof remain
separate. Book/scoped publication pass0;525 retains whole closed0370–0373 subtree and0373 lesson/
ledger in3 exact Git spans0. D154/D250 stay open as owned. Promotion declined: existing sewing
contract closure. Current garment families/declarations next; broad integration/CI deferred.

## STITCHCAD-G1-0374 — close focused sewing component acceptance

- Three exact ownership/current unused-source/borrowed-profile cases pass; Rust unchanged from0373.
- Close implemented sewing/source scope, retaining explicit G4 and full execution/physical deferrals.
- Book/publication pass; closed0370–0373 subtree/0373 retained whole525. Garment families next, broad CI later.

## Whole recipe before execution

A recipe must pass static checking in its entirety before its first statement computes a value.
Whole-source syntax/structure, literal inputs and ordered static inference are separate phases.
Each completes for the entire source before the next starts. For example,
`let bad:length=missing` followed by `assert late:eps_chord=1 mm==1 mm` refuses the later
syntax error, without looking up missing. A late invalid annotation, input literal or structural
limit likewise prevents earlier static checking. Syntax/structure also precedes literal conversion:
an earlier `let first:count=340282366920938463463374607431768211456` cannot hide a later
invalid assertion class, open parenthesis or structural bound. Both literal magnitude and structure
can raise formula_domain, so the token alone does not prove phase agreement. Source order remains
within each phase; whole-source lexical validation and raw annotation priority remain intact.

The reference parses each identified statement once into raw number/unit leaves, preserving original
spans. A separate iterative traversal normalizes every literal, including untaken branches, into
checked operand tuples. Static checks reuse those normalized owners. After every input succeeds,
it checks names/kinds in source order and publishes prior
let metadata. It returns a complete tuple plan after all checks; no accepted prefix escapes.
Detached static_statement still validates only its own statement. This reference tuple plan has
no canonical product identity and grants no numerical/geometry execution authority.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/whole_phase_contract.py --mutations
```

The [pure reference guards](formula-recipe-inputs.md#pure-input-review-and-phase-reference-guards)
cover imports, borrowed calls and the in-memory mutation mode.

D148's584 independent controls combine earlier missing-name, kind, callee, input-collision and
reserved-name errors with later malformed syntax, invalid annotations, literal width and node/
conditional/4096-statement bounds. Traces verify all input finishes before static inference or
prior-binding publication, original operand tuple identity, one parse per statement and real
spans/ordinals. Eleven actual compiled faults trigger body assertions; source stays unchanged.
Numerical/provider/state access is trapped. Grammar and accepted limits remain unchanged.
For example, with width declared as length:

```
let first: length = width / 0
let last: count = width
```

The last statement has a kind error. The reference refuses formula_dimension without executing
the first statement; its zero divisor therefore never reaches numerical evaluation. With the last
statement removed, the recipe is statically valid and its division refusal belongs to execution.
Static validity says that names and kinds fit; it does not promise that values can be computed.

The reference's preflight method accepts original recipe source and initial declaration pairs.
It locates top-level let/assert boundaries without treating newlines as statement terminators,
preserves the original source slices and literal spacing, and checks statements in authored order.
Each accepted let contributes only kind/origin metadata to the next statement's local namespace.
Forward/self references fail, both conditional branches are checked, and assertion labels introduce
no binding. Bare expressions remain available to the instrument's individual-expression tests;
the whole-recipe grammar admits only let/assert statements. Empty recipes are accepted.

The plan is returned as one tuple after every statement passes; a late error returns no accepted
prefix and never changes the caller's declarations. This reference plan is an instrument result,
not the future immutable typed product graph. Parsing still performs canonical literal-input
conversion. Preflight never reads input values, states, context availability or physical geometry,
and never calls numerical evaluation, binding storage or geometry resolution.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_recipe_contract.py --mutations
```

Complete196-case/consumer/fault receipts remain in `docs/history/stitchcad-devnotes-part491.md`.

The copied-book instrument plans the complete current book and every selected copy, log and
existing deletion candidate before it activates local stores. An unknown fixture file or directory,
a symlink, a hardlinked output, or an effective input that a copy would replace causes refusal
before creation, deletion or delegation. Empty selected parts directories are protected too.
Pure contracts() validates only its ten reference inputs and never activates stores; borrowed
consumer_contracts(verbose=False) produces only the two late-error copies, and measurement_contract()
produces only its ceiling copy. Each copy is rechecked immediately before replacement and before use.

The real shell replay retains the prepared store environment and the caller's effective
FORMULA_CONTRACT, FORMULA_GRAMMAR, FORMULA_EXAMPLES, FORMULA_FIXTURE and FORMULA_PARTS_DIR selectors.
Its FORMULA_BOOK remains this repository's book, as before. Source inspection and child dispatch use
the same selector snapshot. These locality controls leave the recipe grammar and196 cases unchanged;
intercepted child output is dispatch evidence. The separate real replay supplies numerical evidence.

Recipe size is measured from the accepted worked plan:21 statements. The13 separate refusal
candidates are not appended to that recipe. D123 formerly combined them into34, falsely refusing
a copied book with a25-statement ceiling. That independent control now passes, while restoring
the old aggregate formula in memory causes a named assertion failure. The normative ceiling
remains4096; the smaller value is used only to falsify the measurement in a copied book.

D119's reference ordering and D123's measurement defects are repaired at .5b.1b.2. Product sourced namespaces, kind signatures, scoped checks and immutable whole recipe proofs are implemented. Reference numeric provenance
is [verified separately](formula-runtime-validation.md#approximation-contributions-and-admissible-classes); reference origin/context routing is verified at .5e.1a. Static checking cannot settle
those execution obligations.

End of whole identity/sewing task scopes,0374 lesson/ledger and historical recipe-phase proof continuity.
