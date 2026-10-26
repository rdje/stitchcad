# Typed evaluation requests

The sc-core command module provides immutable inputs for both
[instantiation paths](instantiation-paths.md). Lookup, authorization, numeric evaluation
and persistence remain with future handlers.

## An exact shared snapshot

`CommandSnapshotContext` names a Design, the caller's actor/principal claim and a mandatory
exact `DesignRevision`. `CommandQueryContext` remains an alias of that same type. Queries and
`CommandEvaluation` therefore preserve the same snapshot claims, including revision0 and counters
beyond128 bits. Execution must validate the requested revision and class-specific authority.
There is no mutation retry key or undo group on an evaluation request.

`SizeMemberTarget::new(size_set, revision, member)` names an explicit member of a particular set.
Its `Count` revision is separate from the Design counter.
Labels, array positions and a profile selection cannot silently choose a member or resolved set.
Execution checks membership, currentness and chart/axis/break bindings.

## Regenerate from measurements

`EvaluateInstance` carries a `SizeMemberTarget`, a `RegenerationChart` and an `EvaluationProfile`.
`RegenerationChart::Garment(chart)` selects an authored garment SizeChart;
`RegenerationChart::Mtm(chart)` selects an MtmChart. Execution resolves the selected member's
actual measurements and, for MTM, its body/Ease/garment correspondence through those chart bindings.
The request copies no numeric value, mapping or uncertainty state. Complete structural chart
coverage is a prerequisite for the path; it does not prove that every required value is available.

`EvaluationProfile::Logical` requests evaluation from the Design's logical inputs without a factory.
An operation that needs an unbound factory quantity still refuses it. This permits factory-free
headless evaluation without silently assigning physical defaults.
`FactoryResolved { selected, parameters }` instead names a resolved parameter-result identity and an
exact `SelectedProfilePin`: profile key, schema version and content version remain separate.
Execution must check that result's provenance against the selected labels, Design and SizeSet snapshot.
A claimed result identity or nonblank pin proves neither registry existence nor constraint satisfaction.

```rust
use sc_core::{command::{CommandActorClaim, CommandEvaluation, CommandSnapshotContext,
    CommandId, DesignRevision, EvaluateInstance, EvaluationPayload, EvaluationProfile, GradeSize, RegenerationChart,
    SizeMemberTarget}, ontology::EntityId};
use sc_units::Count;
let id = EntityId::from_bits;
let snapshot = CommandSnapshotContext::new(id(1), CommandActorClaim::Human(id(2)),
    DesignRevision::from_counter(7));
let member = SizeMemberTarget::new(id(3), Count::new(4), id(5));
let regeneration = CommandEvaluation::new(snapshot.clone(),
    EvaluationPayload::EvaluateInstance(EvaluateInstance::new(member,
        RegenerationChart::Garment(id(6)), EvaluationProfile::Logical)));
let graded = CommandEvaluation::new(snapshot,
    EvaluationPayload::GradeSize(GradeSize::new(id(8), id(9), member)));
assert_eq!(graded.command_id(), CommandId::GradeSize);
```

## Grade from a base and rules

`GradeSize::new(base_instance, rule_table, target)` retains an explicit base Instance and grade-rule
table. The target's exact SizeSet revision owns the authored breaks; no delta is inferred from chart
subtraction. Execution must resolve the actual base provenance, base member, rule-point references,
X/Y declarations and break coverage. Imported bases stay explicit; MTM singleton input retains
its grade-rule refusal.
Reconstruction and equivalence checks remain G3.

Payloads determine identity and evaluation/generate/discarded metadata, without design undo grouping.
Native contracts check input preservation. [Bus execution, artifact and release owners](command-layer.md)
remain separate.

## Resolve a selected profile

`EvaluationPayload::ResolveProfile(ResolveProfile::new(selected_pin))` requests parameter resolution
for the exact profile/schema/content pin against the shared Design snapshot. Actual measurement,
material and SizeSet bindings come from that Design revision; no input map or constraint string
substitutes for canonical records. The constructor preserves symbolic states and allocates no result.
Registry/grammar, typed constraints, evidence, precedence and composition checks remain G4.
