# Sealed engineering continuity — part948

- **Identity:** 73 lines, 5538 bytes, sha256:cbf14874ff0757cfc3afa2cee0e9a125b295544e0fa4a98d1f04e977030e1d11
- **Coverage:** whole0772 task/lesson/ledger and complete closed ADR diagnostic discussions from f154eff3be83aa1e762af22637b3f02df85dd87e.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 44383–46253, 1870 bytes, sha256:c0dc8c266da5817dc0aba27367e1cff6f82e1eed53b3aa1496dbf273bc48bdb7
- Whole source interval: DEV_NOTES.md bytes 193–717, 524 bytes, sha256:3156083370650732ddb69c4fa3646f3f96f85fd276b551c1f1e54ed023cec43d
- Whole source interval: CHANGELOG.md bytes 217–647, 430 bytes, sha256:c9b6b5386820653a7ba2374c42c0de697872c026e42c3aee7e2441a0afe1f0d7
- Whole source interval: docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md bytes 11419–14133, 2714 bytes, sha256:e2900c2de3a4057a6d79a9c43b0deb756075cde457ccd3e35eccdbef5790efe3
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable;corrections belong in a new record.

---

### G1-SLICE.5f.1.c.join.outputs.inventory
- ID: `G1-SLICE.5f.1.c.join.outputs.inventory`
  Status: `done`
  Goal:whole C/I/workspace admission against the same actual PieceInventory slice already bound
  by the source join;reuse canonical companion proof and CutPlan validation before output policy.
  Preserve original A/B pointers,plan order,ease/stops/orientations;no copied equal source accepted.
  Focused inventory/parent/compiler/book checks;retire whole0771 for bounded live continuity.
  - [x] ROOT CAUSE (WHY + WHERE):BoundJoinPiecesAtSeam preserves uniquely named A/B Pieces
    but whole companion/copy population remains separate. Prior parent join_input_contract passes
    `test result: ok. 4 passed; 0 failed`,rc=0;canonical CutPlan/PieceInventory interfaces inspected.
  - [x] ADDRESSED (verified):python3 -I -B scripts/local_environment.py -- cargo test -p sc-core
    --test join_copy_inventory_contract --test join_input_contract: `3 passed; 0 failed` and
    `4 passed; 0 failed`,rc=0. Whole C/I/I+3C before exact source-slice identity/canonical plan;
    actual both-side parents,unrelated/reflected copies,quota precedence,equal-clone refusal retained.
  - [x] NO REGRESSION:strict selected Clippy/lib/tests and cargo fmt --all -- --check rc=0;
    cargo test -p sc-core --doc construction::join::inventory gives `3 passed; 0 failed`,rc=0;
    construction::editing_examples gives `8 passed; 0 failed`,rc=0. Book build rc=0;D372 remains.
    First hook refused aggregate task size1500989>1500000;completed24-node goals retained947
    byte-exact,live IDs/statuses unchanged. Four947 Git spans/body verified;archive1401/34 rc=0. No copy consumed,
    metadata chosen,geometry solved,currentness/authority accepted or broad verification implied.
    STITCHCAD-G1-0772;promotion: declined (existing source/inventory doctrine);next .outputs.domains.
## _(2026-11-01 UTC)_ — complete held join inventory admission

0772 .join.outputs.inventory requires the exact source PieceInventory slice,never an equal cloned substitute.
Whole C/I/I+3C precede slice association/canonical CutPlan validation;actual A/B/copy/span metadata stay borrowed.
Native3/prior4/API3/book8/lint/fmt/book0;947 four exact spans/24 closed-node goals/archive1401/34 pass0.
No source fields reconciled,copies consumed or geometry/currentness/authority certified. Promotion:declined,existing doctrine.

## STITCHCAD-G1-0772 — admit the complete original join inventory

- Whole C/I/I+3C precede exact source-slice identity and canonical CutPlan validation.
- Actual A/B,unrelated/reflected copies and source metadata stay original;no clone substitution.
- Native3/prior4/API3/book8/lint/fmt/book pass0;947 whole0771 retained;G1still6/18.
- Output/placement/metadata/copy/lineage policy and actual geometry/execution remain owned.

## D124 diagnostic recognition proposal — resolved 2026-10-03 (UTC)

Reviewed2026-10-02 at G1-SLICE.5b.1c.1. Contract6 assigns formula_unsupported to loops and
function/macro definitions, but v1 defines no source forms for them. Actual reference returns
formula_unbound_name for loop(width), repeat(2,width) and while(width>0 um); fn helper(width)=width
and macro helper(width)=width return formula_parse. A declared loop identifier remains valid.
This is a diagnostic-contract gap, not permission to add executable loops or functions.

Recommended: retain the three reserved grammar keywords. Unknown calls stay formula_unbound_name;
malformed loop/definition syntax stays formula_parse. Recognized non-square exponents retain
formula_unsupported, and envelope calls retain their envelope tokens. Clarify the exclusions table
to separate unsupported capabilities from diagnostic recognition by the closed grammar.

Alternative: define an explicit closed set of recognizable excluded forms that raise
formula_unsupported. That requires exact source spellings and keyword/call-role rules, including
whether a previously valid parameter named loop, repeat, fn or macro remains valid. No such list
is inferred from these illustrative examples. Neither choice enables any excluded capability.
Director ruling2026-10-03: keep the current grammar, adopting the recommendation above.
G1-SLICE.5b.1c.2 implements the clarification and independently verifies diagnostic recognition,
ordinary identifier preservation and precedence before closing the full static reference review.
Product namespace/type/graph implementation remains .5b.2–.4; no executable excluded capability
or new reserved word is introduced.

## D131 delegated binding-source decision — 2026-10-03 (UTC)

Reserved-name and repeated-recipe formula_rebinding cases retain truthful sources/locations.
The delegated decision, executable evidence, approval boundary and reversal are in
[the binding-source record](decision_rebinding.md); canonical contract5.2.1 defines the schema.

## D121 tolerance-class diagnostic ruling — 2026-10-02 (UTC)

The director selected formula_domain for a T1 comparison whose operands carry an approximation
contribution. Contract4.2 already requires T2 or looser; the diagnostic table now explicitly covers
an inadmissible tolerance class. The reference records comparison (within or the actual assertion
name), requested tolerance_class and sorted contribution_sources. An equal numeric answer does not
erase its derivation. Product typed context and arbitrary-input transcendental accuracy remain
separate .5b–.5g obligations. D124 was pending when this ruling was made; its subsequent ruling is recorded above.

