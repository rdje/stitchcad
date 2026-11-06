# Native mirror semantic and sewing requirements

This companion follows [complete actual copy/companion association](drafting-mirror-copies.md).
`BoundMirrorPieceCopyPlan::require_semantics` binds an explicit result semantic intent and nine
mandatory original evidence-source fields to that same actual complete candidate context.
It requests evidence;it executes no semantic copy,repair,geometry,sewing or publication.

## Explicit result intent

Source canonical objects retain their original owners,ids,fields and held references in both modes.
Copying a contour cannot reassign a held Notch,Grainline,Dart or seam to the result Piece.

| `MirrorPieceSemanticIntent` | Required result policy |
| --- | --- |
| `SourceOnly` | Request no result semantic copies or new sewing relations from this operation;justify the complete result and reconcile any existing result consumers |
| `ExplicitResultMappings` | Require complete explicit result semantic/mark/reference/sewing mappings with actual canonical ownership and identity history |

There is no default intent. SourceOnly is an authored request,not an inference from an empty
registry,zero provider id or absent local mark list. It does not discharge required semantic marks,
existing result holders,retired-copy neighbours or other supported-envelope obligations. Execution
must refuse an unsupported or incomplete requested result rather than silently supply defaults.
ExplicitResultMappings supplies no automatic cloning permission. New result semantic objects need
distinct globally issued ids;retained result objects need explicit current canonical association.
Original source object ids cannot be reused,even if an output occupies coincident coordinates.

| API | Meaning |
| --- | --- |
| `MirrorPieceSemanticConsumerSources::new(whole,preserve,current,result,repairs)` | Five explicit complete consumer/owner/result/repair source fields |
| `MirrorPieceSemanticEffectSources::new(intakes,sewing,all_copies,authority)` | Four explicit complete semantic/sewing/effect/authority source fields |
| `MirrorPieceSemanticPolicy::new(intent,consumers,effects)` | Whole original intent and all nine requested sources |
| `RequiredMirrorPieceSemantics` | Actual complete plan,policy and prior correspondence context |
| `MirrorPieceSemanticEvidenceKind`, `MirrorPieceSemanticEvidenceRequirement` | Distinct mandatory scope and authentic original source-field view |

```rust
use sc_core::{construction::*,ontology::EntityId};
let id=|n|EntityId::from_bits(n);
let policy=MirrorPieceSemanticPolicy::new(MirrorPieceSemanticIntent::ExplicitResultMappings,
    MirrorPieceSemanticConsumerSources::new(id(700),id(701),id(702),id(703),id(704)),
    MirrorPieceSemanticEffectSources::new(id(705),id(706),id(707),id(708)));
assert_eq!(*policy.source(MirrorPieceSemanticEvidenceKind::Sewing),id(706));
fn require<'a>(plan:&'a BoundMirrorPieceCopyPlan<'a>,policy:&'a MirrorPieceSemanticPolicy)
    ->RequiredMirrorPieceSemantics<'a> {
    plan.require_semantics(policy)
}
```

## Nine obligations remain distinct

| Evidence kind | What execution must establish |
| --- | --- |
| `WholeSourceConsumers` | Complete current structural,semantic,mark and reference consumers of source and affected result objects |
| `SourcePreservation` | Original source holders,ids,fields,held references and canonical ownership stay unchanged |
| `CurrentOwnership` | Actual current Design/revision,Piece/frame/copy/Material ownership and global identity history |
| `ResultSemantics` | Complete result matches the explicit intent,including all explicit new/retained result object mappings |
| `VisibleRepairs` | Actual held point/interval correspondence,explicit ambiguous choices,gaps and visible repairs without retargeting |
| `CanonicalIntakes` | Canonical Dart/Tuck/Pleat/Gather intake and closing provenance,conservation and whole affected owners |
| `Sewing` | Actual copy-addressed spans,neighbours,directions,ease and stop landmarks reconcile explicitly |
| `AllCopyEffects` | Every retained/retired/proposed and affected companion copy's semantic and physical effects |
| `AuthoringAuthority` | Actual authority for global/canonical changes,selected-holder repairs and atomic publication |

These are requested sources,not registry-resolved providers,supplied evidence,human approval or
achieved effects. A shared provider id,including ordinary id0,does not merge obligations. There
is no optional source or empty-consumer shortcut. `requirements()` always lends all nine actual
original fields in the order above;reverse iteration reverses that order. Each item retains its
actual complete context,kind and source field. Equal values still occupy distinct original fields.

`plan()` lends the actual complete candidate context;`policy()` and `intent()` retain the original
request. `correspondence()` lends the same original geometric correspondence request already bound
through that plan:actual source frames,edge/point mappings,contours,accuracy,source preservation
and the axis-mode requirements remain mandatory. This stage introduces no duplicate geometric
mapping authority or separately entered intake,Length,Area,ease value or mark dimensions.

For source copies410/207/900/805 and result500,all four original copies retain their identities and
individual neighbours. A separate reciprocal pattern companion300 with physical copy333 does not
assign a physical pairing by ordinal. With explicit `[Retire551,Retain550]`,source-only intent still
requires all nine scopes,including actual retirement effects and any neighbour repair. No span is
moved from551 to550 merely because both were result copies. Reflection must preserve attributed
seam direction,ease,stops,notch representation/profile bindings,grain references and canonical
closing/intake sources through explicit supported mappings;this request executes none of them.

Association and fixed nine-item traversal take constant work/space,allocate nothing and inspect
no consumer registry. Earlier whole plan/inventory/resource bounds retain their own scope. No
whole-Design work budget or complete registry census is inferred. All original objects,plans and
held borrows remain unchanged. This completes native semantic/sewing requirement declaration;
actual supported mappings,provider evidence,currentness,global issuance,physical effects,geometry
and atomic authorized publication remain G2/G3/Design/bus obligations.

## Native declaration checkpoint

The scoped mirror-family checkpoint runs all19 mirror contract suites:80 native tests pass.
The actual `construction::mirror` API/book filter passes80 doctests,including the21 unchanged
examples across the input,output,physical-copy and semantic companions. Source commit
`d2126ec598a13922eb23dc580b350233e12642aa` and its core tree are pinned in the local
`target/history-proof-0739-mirror-checkpoint/receipt.json`. This checkpoint changes no Rust source,
test,Cargo input or prior Rust example. All32 native mirror task nodes are complete.

The native family now covers original inputs/source/cut/metadata,typed identity/output roles,
complete point/edge attribution and contours,correspondence requests,explicit Retain/Retire/Issue,
actual candidate inventory/CutPlan/companion association,and mandatory semantic/sewing requirements.
The tests include refusal,ownership/lifetime and immutable-parent cases;they establish the scoped
native contracts. They do not establish full CI,platform parity,current Design ownership,global
identity reservation/issuance,accepted provider evidence,actual reflection/physical-copy/semantic
or sewing effects,or authorized publication. Those execution owners remain pending.

The next native output family is walk,with explicit mark sharing/coalescing and physical-copy
identity policy. Ordered Design/registry/history integration remains .5f.2,selector/parameter/error
budget integration .5f.3,and actual geometry-backed replay .5f.4.geometry/G2/G3. Broader typed-v1
integration and the mandatory pre-push gate retain their own milestone;this result does not close G1.

## Names used in this chapter

Every API name and example name this chapter uses in code style, with its meaning. API meanings come from the code's own documentation; example names are not defaults.

| Name | Meaning |
| --- | --- |
| `d2126ec598a13922eb23dc580b350233e12642aa` | Git commit pinned as this chapter's evidence source. |
