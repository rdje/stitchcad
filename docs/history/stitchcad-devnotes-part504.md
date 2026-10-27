# Sealed archive — whole0352, D228 and coupled-input proofs

Immutable historical segment, sealed by leaf `G1-SLICE.6.4.2.d.3.2` on `2026-10-27` (UTC).

- **Sealed identity:** 129 lines, 10712 bytes, sha256:b6281757d119573ea4f1b75ee07b59d50015049a49d0ee5bb16f4567e32371ca
- **Coverage:** complete committed target task/lesson/ledger, settled D228 report and old coupled-input book proof from e9753c457e0c384c3eda978cca2b82be69e39525.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 48678–50104, 1426 bytes, sha256:6162a6f63625fb72856277aa6924b34a5c00a474e8501debcfe595bd32189540
- Whole source interval: docs/tasks/PLANNING.md bytes 81941–83168, 1227 bytes, sha256:4e6a4589a76440d870152a460d607b06a388ce1f2633ec6d74738122b711a27f
- Whole source interval: DEV_NOTES.md bytes 193–1171, 978 bytes, sha256:23c678ae17b7ee6a08bc61d74c259f92cba5f4ce85e426e9aad26180899c2613
- Whole source interval: CHANGELOG.md bytes 207–749, 542 bytes, sha256:b91d329e677306f058b5133e5f7000153cce1d3dfd20211aa2f3146ce04a9667
- Whole source interval: docs/book/src/annexes/formula-recipe-inputs.md bytes 14100–20573, 6473 bytes, sha256:fb1bac43ad67b4079fdefcf036c6827a2f5ec4dfcf12a3585d7ac607a8a5aafe
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.2.d.3.1`
  Status: `done`
  Goal: shared value::CanonicalInputTarget with compatible DesignParameterTarget alias; exact existing
  variants, no conversion/lookup change. Own focused checks/docs and complete0351 continuity503.
  - [x] REPRODUCE / ISSUE: Material metadata needs the same pinned family/id/kind contract; source
    comparison proves exact original enum after name substitution, rc=0; no alternate representation.
  - [x] ROOT CAUSE (WHY + WHERE): target lived inside design/parameters.rs. cargo test -p sc-core
    --test design_parameters_contract gives9 passed/0 failed, rc=0; exact enum bytes confirm matching.
  - [x] FIX: common value export, original public DesignParameterTarget alias; selected native9/0,
    rc=0, target/canonical-target-native-0352.log. No new default, resolution or source-truth claim.
  - [x] ADDRESSED: cargo test -p sc-core --test design_parameters_contract passes9/0, rc=0.
  - [x] NO REGRESSION: privacy2/strict selected lint/fmt/book/publication77/111/1444/2361 pass0,
    target/canonical-target-*-0352.log; D250132 exact baseline rows/delta0, glossary rc=1.
  - [x] LOCKSTEP: API vocabulary/status/notes/index/pointer aligned; continuity503 matches3 Git spans0.
    Caps unchanged; G1still6/18. promotion: declined (existing canonical-source/identity contracts).
    Commit0352; Material .3.2 next; full host/Design/readiness/integration/CI remains later.
## D228 — size-break dimension identity resolved2026-10-26

Size-sets §6/field table defines per-dimension adjacent-member deltas, using waist as its example;
§7 says breaks are per axis and a one-dimensional system has one axis. The contract does not say
whether that axis owns one delta or multiple named POM/dimension deltas. Reproduce: rg the phrases
"per dimension", "waist jump", "per axis", "single axis" in docs/book/src/spec/size-sets.md;
reference-skirt.md:43–46/52–53 independently carries separate waist/hip/length inputs.
Witness: one authored "House size" axis, members S/M, waist+4cm and hip+5cm. Axis/pair alone
cannot distinguish both records unless an additional dimension key is explicit; no delta is inferred.
Impact: guessing the key can lose a required grade dimension or create incompatible SizeSet data.
Owner/schedule: G1-SLICE.4c.3d.0, P1 ruled0302 before .3d.1 implementation. Director approved
axis + explicit POM/dimension + adjacent member pair, retaining each canonical signed Length
state/source. Semantic conflict closed; declaration/coverage/composite implementation remains .3d.
Whole original ruling-review evidence is retained431. No break implementation is claimed by the ruling.

## _(2026-10-27 UTC)_ — shared canonical target representation

0352 moves the existing length/general-scalar target enum into value::CanonicalInputTarget for reuse
by Material metadata. DesignParameterTarget remains an identical public alias. A source comparison
matches the original complete enum after only its public-name substitution; no lookup, conversion,
state or source semantics change. Existing parameter9/privacy2/strict selected lint/fmt/book/publication
pass0. D250132 glossary rows remain exactly baseline, delta0. Whole0351 subtree/lesson/ledger503 compares3
Git spans, preserving all three live ids/statuses. Promotion: existing canonical-source/identity contracts.
Profile bindings remain symbolic under the canonical decision; G4 owns physical resolution and pins
remain instantiation context. G1still6/18; Material field/source contracts .6.4.2.d.3.2 follow, with full
host/Design/readiness/integration/CI pending. No new tests mirror the unchanged enum.

## STITCHCAD-G1-0352 — share exact canonical input targets

- Move the existing enum to value::CanonicalInputTarget; retain DesignParameterTarget as an identical public alias.
- Exact source comparison confirms unchanged variants; parameter9/privacy2/strict selected lint/fmt/book/publication pass0.
- D250132 glossary rows remain baseline; continuity503 matches3 Git spans and preserves all three task ids/statuses.
- Caps unchanged; G1still6/18. Material fields next; symbolic profile resolution remains G4, full host/Design/CI pending.

## Coupled input and identity review

G1-SLICE.5a.3f.2 checks the complete path from authored statements to normalized inputs and owned
identity. The [worked skirt examples](../spec/formula-language/examples.md) supply 17 bindings and
four assertions:21 statements with 25 expression operands. Independently authored complete statement
bytes match the actual published names/kinds/operands and the complete ordered recipe. The recursive
reference checks syntax with inference/evaluation trapped, plus complete ordered token coverage;
authored statement chunks do not supply an independent whole-recipe parser.

The 13 refusal examples also have a current stage. The missing conditional branch, text value and
unsupported cube refuse during syntax parsing. The other 10 still produce input identity: their
kind/name/division/unknown/tolerance/envelope checks belong to later static validation and evaluation.
Accepting their syntax and inputs does not accept their use in a garment. A bad literal in an
untaken branch or a call argument still aborts whole normalization, retaining the original span,
known statement ordinal 22 and operand role after the 21 valid worked statements.

| Obligation | Current evidence | Remaining implementation owner |
| --- | --- | --- |
| Whole let/assert syntax, order and fixed bounds | Statement/recipe contracts and coupled maximum/first excess tests | Syntax milestone G1-SLICE.5a.4 complete |
| All literal inputs, including untaken branches | Literal/normalized contracts and later whole-input refusal tests | Numeric binding G1-SLICE.5c |
| Names, annotations and original global metadata | 21 worked statements, source borrowing and Clone checks | Namespace/type acceptance G1-SLICE.5b |
| Exact typed identity, empty/order/alias behavior | Owned identity contracts, actual worked recipe and typed collision controls | Typed project fields/digests G1-SLICE.7 |
| Source/arena-independent identity, privacy and extraction | Ownership contracts and actual statement Debug controls | Command diagnostic envelope G1-SLICE.6 |
| All 13 worked refusals | Three syntax refusals; ten semantic checks explicitly deferred | Static validation/evaluation G1-SLICE.5b/.5e |
| Published values/assertion verdicts, lazy execution and replay | Independent reference only; inputs/identity compute no values | Product evaluator/DAG G1-SLICE.5e |
| Geometry selectors and construction | Syntax/identity retain ordered selector calls | G1-SLICE.5f and G2-2D |
| Storage, API/MCP and production acceptance | Distinct future workflow/approval requirements | G1-SLICE.7/.9 and G7-RELEASE |

Five new public controls check the actual worked/refusal populations, metadata and complete bytes,
whole later literal failures, alias/order invariants and combined limits of 4096 statements, 256 nodes and 16 nested conditionals
with first excesses. The bounded maximum serializes on a 64 KiB stack. Existing tests additionally
exercise all 4096 assertions with both 256-node operands; neither case is a performance/browser proof.
### Coupled input runner modes and evidence

Seven original production faults target metadata, missing later inputs, flattened refusal index,
recipe order, assertion operand identity, combined bound and normalized Debug privacy. Each compiles
and fails at a current guarded public assertion macro. The direct entry declares eight inputs before
activation or reads: all four implementations, current test source, complete worked-statement TSV,
the actual example chapter and refusal-stage TSV. Ordinary mode declares all seven numbered fault
logs plus restored.log and all four mutable implementations. Classifier-only declares no diagnostic
files, mutable source or native child; caller-provided valid local stores and unrelated values survive.

Complete mode captures verify original case tuples, source order/bindings, exact Cargo arguments,
filters, diagnostic bytes, per-case restoration and final byte checks before the full rebuild.
Twenty-eight actual omissions and the legacy nonthread predicate fail independently. Physical
controls refuse aliased endpoint logs, each hard-linked mutable source and missing late inputs
before reads or effects. An actual copied classifier-only entry accepts an input with three hard
links and two unused log aliases without source/output writes or child dispatch; all inputs, the
alias victim and environment remain exact. These captures establish dispatch; real compilation
supplies the native evidence separately.

Failed-body classification requires a thread panic at a current source/path/line/UTF-8 byte-column
assertion macro. Custom macro messages qualify; compiler-note locations, compiler/expect noise
and passing test names do not. D197 closes only after expression, reduction and input families
verify their original controls and current artifacts. D198 removes an obsolete statement-availability
phrase from the expression inventory; its expression fixtures and renderer remain unchanged.
Current inventory/reference and all four original interpreter renderer faults pass after repair.

The original seven input Cargo filters are retained, including their original substring semantics;
each currently selects exactly one public test. The four filtered tests per fault were not executed
and receive no coverage credit. Independent review inspects every actual failed body, including
small-stack worker evidence, against current quoted-thread macro locations. Eight local single-link
logs, seventy-five frozen inputs, sixty-three unchanged production Rust sources, all four mutable
implementations and the unchanged five-test source restore exactly. The full target passes after
restoration, and a newly Cargo-selected executable independently runs all five current tests.
The structural suite watches authored/reference populations and original actual anchors; this
proof supplies no numerical execution or browser/performance authority.

```bash
cargo test -p sc-core --test formula_recipe_input_review
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_mutations.py
```

The scoped input/identity review and syntax milestone .5a.4 are complete. Static proofs and numeric
foundations now exist separately; these historical input checks do not establish whole execution,
geometry, storage/recovery, command/API/MCP control, cross-platform results or production approval.

End of whole0352, D228 ruling and coupled-input proof continuity.
