# Sealed engineering continuity — part522

- **Identity:** 98 lines, 7847 bytes, sha256:a65091852547c0dcc83daa90db1e26534b20837c715a472e5fecb54f9642c776
- **Coverage:** complete current sewing task/lesson/ledger and historical static-name proof from d4270c71a1bbdfdc6016411dc50e2104af76f3ae.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 51601–53336, 1735 bytes, sha256:80dc7a90b839d4b1cd51d5876ad59afb2db48175a549c0da1e6034930b967c8a
- Whole source interval: DEV_NOTES.md bytes 193–1161, 968 bytes, sha256:940fff1854f13633f97fe2b5e9bded7e23d558d2252112e2e25462ac17d0420f
- Whole source interval: CHANGELOG.md bytes 207–674, 467 bytes, sha256:eea3304057cc47c3e5fc117301426f8efca8cc19df36102001acdab7114bf263
- Whole source interval: docs/book/src/annexes/formula-static-validation.md bytes 8031–12631, 4600 bytes, sha256:b8e99a3a683a1ecc0afec3d0758d7759eb666dd0bac03f24113a1e10f1964f99
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.4.1`
  Status: `done`
  Goal: recheck actual immutable SewingGraph against supplied current plan/Pieces/landmarks/ledger.
  Share original constructor validator without cloning an alternate graph or widening its rules;
  retain first typed refusal/raw repair evidence and exact authored graph, focused checks/book.
  Own whole0369/topology continuity and historical header proof retirement521.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test sewing_contract:21/0 rc=0;
    prior graph success cannot authorize replacement copies/stops or lost interval interiors.
  - [x] ROOT CAUSE (WHY + WHERE): python3 -I -B Git-source token comparison: same original
    validator after receiver/iteration adaptation rc=0. Old missing-id queries cannot check source scope.
  - [x] FIX: cargo clippy -p sc-core --test sewing_contract -- -D warnings:rc=0;
    constructor/current query share original rules/order without rebuilding or mutating the graph.
  - [x] ADDRESSED: cargo test -p sc-core --test sewing_contract:21/0 rc=0; three new cases
    retain MissingCopy/RangeOutsidePiece/MissingLandmark/StopOutsideSide and full UnresolvedRange evidence.
  - [x] NO REGRESSION: cargo test -p sc-core --doc ontology::sewing::SewingGraph:1/0 rc=0;
    fmt pass0; glossary132 failures rc=1, exact D250 prior rows compare rc=0. Existing18 cases retained.
  - [x] LOCKSTEP: bash scripts/run_make.sh book:rc=0; scoped publication79/130/1454/2398 rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part521.md:4 exact spans rc=0.
    promotion: existing sewing decision answer extended. Completed STITCHCAD-G1-0370.
    .4.2 actual formula amount declarations/G4 boundary next; full CI integration/pre-push.
## _(2026-10-27 UTC)_ — current immutable sewing graph validation

0370 factors original SewingGraph constructor checks into validate_current; constructor delegates.
Git-source token comparison confirms identical validation after receiver/iteration adaptation. Shared
spans stay original; no replacement graph/value or geometry proof is generated. Actual current copies,
range ownership, stop sources and distribution rules recheck in the same order. Same-id retargets can
pass missing-id inventories while failing actual scope. Lost interiors retain full raw range evidence.
Affected target21/0 (18 original,3 new), privacy1/strict selected lint/fmt/book/publication79/130 pass0;
D250 remains exact132 rows.521 retains whole0369/topology/header proof in4 exact Git spans0.
Promotion: existing sewing decision answers replacement contexts. .4.2 actual symbolic declarations/
G4 role boundary next; global ownership, runtime/physical/integration/CI remain separate.

## STITCHCAD-G1-0370 — revalidate current immutable sewing sources

- Share original constructor rules with SewingGraph.validate_current; preserve graph and refusal order.
- Check actual same-id copy/stop source changes and full lost-interior evidence after prior success.
- Native21/privacy1/lint/fmt/book/publication pass0; D250132 unchanged; whole0369/header continuity521.
- Actual symbolic declarations/G4 boundary next; broader integration/CI remains owned.

## Names and single-statement static checking

The reference now separates three stages: syntax_statement parses a header and its operands;
static_statement checks declaration metadata, names and kinds; statement executes only after that
static stage succeeds. These are reference maintenance interfaces, not new sc-core APIs. The
product syntax parser still exposes unevaluated syntax and has not acquired static acceptance.

| Statement or declarations | Static result |
| --- | --- |
| let width:length=body_width, with body_width declared length but unknown | accepted without reading its state/value |
| let body_width:length=1 cm, with body_width already from measurement | formula_ambiguous_name, preserving measurement and recipe origins |
| a second let width:length=1 cm, after a prior recipe binding | formula_rebinding |
| let eps_num:length=1 cm | formula_rebinding; reserved context names cannot be authored bindings |
| assert closure:eps_geo=width==height, with both declared length | accepted without fetching the tolerance value |
| assert closure:eps_fmt=width==height outside export context | statically accepted; missing valid context value is a runtime check |
| assert closure:size_index=width==height | formula_parse; size_index is outside the closed TOLERANCE grammar role |
| assert closure:eps_num=flag==flag, with flag Boolean | formula_dimension; closure comparisons require arithmetic operands |
| let width:length=if(is_base_size, known_width, missing_width) | formula_unbound_name even if the missing branch would not execute |

Declarations are consumed as ordered pairs before a dictionary can discard a duplicate. The
reference checks machine spelling, declared kind/origin membership, reserved binding and duplicate
names; duplicate reports retain both source origins, including two declarations from the same
origin. A let collision with an earlier recipe binding is rebinding; one with an input origin is
ambiguity. Neither shadows silently. An assertion label names a check and introduces no value
binding; it does not make a later expression name visible.

The nine origin labels are measurement, ease, parameter, profile, material, geometry, recipe, size
and tolerance. All eight reserved names have statically known kinds, even where a context has not
provided their values. The five tolerance annotations are eps_num, eps_geo, eps_fmt, eps_imp and
eps_phys. A non-class annotation such as eps_chord is outside that grammar role; a valid class
with no supplied value has the separate runtime formula_tolerance_unbound rule.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_namespace_contract.py --mutations
```

The 1139 independent cases cover all nine-origin/eight-kind metadata combinations, all81 ordered
origin collision pairs, every reserved name against each origin and all six let kinds, spelling,
headers, kind mismatches, forward/self names, either conditional branch and both assertion operands.
Every arithmetic assertion kind pair is checked at all five tolerance classes. Environment entries
expose only kind/origin; any value, state, availability or geometry read fails. Evaluation, storage
and geometry callbacks fail if called, including through the runtime adapter for a static refusal.
Parsing retains its existing canonical literal-input conversion; this is not a claim that parsing
performs no numerical work.

Thirteen compiled in-memory changes to actual guards must fail body assertions. They cover spelling,
origin validation, pair collisions, reserved/input/recipe rebinding, let kind mismatches, assertion
class/kind roles, metadata value reads and execution before static acceptance. Source on disk remains
unchanged during those controls. Existing syntax-only fixtures now use syntax_statement directly,
so syntactically valid reserved-name bindings still have their original canonical bytes while later
static validation refuses them. No Rust syntax or serializer behavior changed.

This proves metadata namespace and single-statement checking in the reference. It does not validate
canonical input adapters, physical geometry or typed product diagnostic payloads; those retain
.5e.1, G2 and .5b.2–.4 owners. Whole-recipe checking is described below. D121 reference provenance at T1 is [verified separately](formula-runtime-validation.md#approximation-contributions-and-admissible-classes). D122 reference origin/context routing is
[separately verified](formula-runtime-validation.md#missing-values-by-origin); these static controls
make no runtime correctness claim. Product input adapters remain .5e.1.

End of whole0370 current sewing and historical static-name proof continuity.
