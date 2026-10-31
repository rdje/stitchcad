# Native slash/spread output contracts

Inputs remain in the [editing reference](drafting-edit-inputs.md). The shared edge-role
metadata also serves the [offset output contracts](drafting-output-contracts.md).

## Native slash/spread boundary and fullness intent

`SlashSpreadOutputIntent::new(input,boundary,fullness)` owns the actual
`SlashSpreadConstruction` and requires both output choices. Moving the input preserves its original
operation,Piece,directed cut,moving bank,pivot or movement axis,and embedded amount document. The
constructor performs no new allocation,evaluation,identity issuance or model edit. It declares
intent;an accepted output manifest and actual geometry remain separate stages.

`SlashSpreadBoundaryPolicy` has two explicit requests:

| Policy | Intent and remaining proof |
| --- | --- |
| `StraightBridges` | Straight connectors at corresponding opened cut mouths;actual endpoints/incidence and supported closed contour must be proved |
| `TrimOverlaps` | Trim overlapping banks;actual retained ranges/intersections and boundary selection must be explicit |

Neither policy supplies a default connector count,generated edge tag,intersection,loop choice or
geometric repair. A trim policy name cannot select an arbitrary loop. Unsupported or ambiguous
region/topology must refuse through the actual G2/G3 execution path;policy construction is not that
validation. The following manifest task owns explicit output roles,source/transform provenance and
trim-selection obligations before native family closure.

`SlashSpreadFullnessIntent::{Added,Removed,Unchanged}` records the intended effect separately from
boundary treatment. It stores no independently authored intake,Length/Area amount or solved fullness.
G2/G3 must derive the actual effect from the same original inputs and attribute it to this operation.
`Removed` neither closes a canonical Dart nor substitutes for its conserved intake. `Unchanged` is
an explicit request,not a conclusion inferred from a zero formula or a full Angle turn.

```rust
use sc_core::construction::*;
fn request_bridged_fullness(input:SlashSpreadConstruction)->SlashSpreadOutputIntent {
    SlashSpreadOutputIntent::new(input,SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added)
}
```

There is no Default or implicit input-to-output conversion. Every boundary/fullness combination
remains authored intent;its applicability depends on actual mode,amount,region and topology. A
negative rotation,zero distance or statically valid `1 mm/0` is preserved rather than normalized
into another policy or accepted geometric result. Original kind proofs remain attached to the
embedded opening document;they execute neither division nor a geometric transform.

`input()`,`boundary_policy()` and `fullness()` borrow actual owned fields. Input dependency views
continue to borrow that actual nested construction and its original cut/pivot/axis fields. No
output Piece,contour,reference retirement,canonical metadata reconciliation or command publication
is performed. Actual G2/G3 sources/results and the later `.5f.4.geometry` integration remain owned.

## Shared edge roles and slash operation binding

Edge-role catalogue and issuance-preserving transition implementation is shared by offset and
slash/spread declarations. The canonical names are `EdgeOutputRoleDefinition`,`EdgeOutputRoleCatalog`,
`AdmittedEdgeOutputRoleCatalog`,`ActiveEdgeOutputRole` and the corresponding catalogue/transition
policy,error and refusal types. These are edge-role metadata types;they introduce no PointRef role
namespace or geometric classifier. The previous `OffsetRole*`,`AdmittedOffsetRole*` and
`ActiveOffsetRole` names remain aliases of the same Rust types. Existing offset examples continue
to compile,without conversion,copied metadata or a second role-history implementation.

The implementation's algorithms and fields are preserved by the extraction. Debug type names and
Display prefixes now say edge-output roles rather than offset roles;typed variants and their
original field data are unchanged. Explicit count/key-byte admission,issued-definition preservation,
active membership and readonly evaluated-candidate presence retain their existing meanings.

`SlashSpreadOutputIntent::bind_edge_roles` requires the actual admitted shared catalogue and
explicit `EdgeOutputRoleCatalogLimits` for this association stage. Complete definitions,active ids
and original key bytes are checked in that order before operation identity. All definitions count,
including inactive ones. The association allocates no workspace;the catalogue's earlier admission
and owned storage retain their own policies. These entry/text allowances are not whole-execution
or allocation-byte guarantees.

```rust
use sc_core::construction::*;
fn bind_slash_roles<'a>(intent:&'a SlashSpreadOutputIntent,catalog:&'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits:EdgeOutputRoleCatalogLimits)->Result<BoundSlashSpreadEdgeRoles<'a>,SlashSpreadRoleBindingError<'a>> {
    intent.bind_edge_roles(catalog,limits)
}
```

Success and errors retain both actual complete owners and the selected policy. Operation mismatch
refuses;it cannot borrow a foreign operation's role catalogue. `BoundSlashSpreadEdgeRoles` exposes
its original `intent()`,`catalog()` and `limits()`. The catalogue's own active views still retain
actual active-list tag and definition fields in their original orders. Those views prove catalogue
membership;the complete binding supplies the checked slash-operation association.

For definitions `[9:moving,2:stationary,7:latent]` and active ids `[2,9]`,the original active order
remains2,9 while definitions remain9,2,7. Nothing renumbers tags or removes latent7. Empty initial
or fully inactive catalogues can also bind as metadata;they supply no executable contour or complete
output manifest. Changing membership still requires the existing issuance-preserving transition
checks and actual authoring/history authority.

A numeric operation match does not prove its actual ordered recipe kind,current Design membership,
global namespace,issuance or geometric continuity. Native source/transform/trim-selection manifest
admission remains the next component;actual G2/G3 contours,effects,mappings and bus publication
retain their separate owners. No catalogue alias or binding creates a Piece or retires an edge.

## Native edge candidate provenance and admission

`SlashSpreadEdgeManifest` owns the required operation id and all original
`SlashSpreadEdgeCandidate`s. Each candidate has an explicit persistent tag and a discriminated
`SlashSpreadEdgeOrigin`. Raw construction retains every field without accepting a result or
issuing an identity. Candidate order is authored order,not a generated edge-name scheme.

| Origin | Explicit fields | Meaning |
| --- | --- | --- |
| `Source` | Positive original DirectedRange and Stationary/Moving motion | Retain that source portion with its declared traversal and operation-specific transform |
| `StraightBridge` | Lower/Upper cut end and bank-to-bank direction | Request a straight connector between corresponding stationary and moving cut-end occurrences |

`Source` preserves its own original edge frame,bounds and traversal. It can represent an explicitly
retained portion after trimming. `Moving` refers to the actual input's rotation or directed
separation;`Stationary` retains its original geometry. Neither label proves the region was classified
correctly or that the range belongs to the current Piece.

`Lower` and `Upper` name the original cut range's parameter bounds. They do not swap when cut
traversal is Reversed. `StationaryToMoving` and `MovingToStationary` select the connector's output
traversal explicitly. These fields create no PointRef,coordinate or nonzero-length certificate.
A rotation pivot can make a proposed connector degenerate;actual geometry must diagnose it or
supply the correct explicit result rather than invent an edge. No fixed connector count is assumed.

`admit` requires the actual `BoundSlashSpreadEdgeRoles` and explicit
`SlashSpreadEdgeManifestLimits`. It first checks complete2N workspace,N candidates,D definitions,
A active ids and original key bytes. Only then does it check nonempty input and matching operation.
Each original candidate follows in order:duplicate tag,defined role,active role,then local source
or boundary-policy cause. A same-operation source is a definite dependency cycle. A straight bridge
under `TrimOverlaps` refuses rather than changing the authored policy.

The workspace count is one N-entry temporary tag index plus N retained candidate/active-role pairs.
The catalogue's prior allocation and whole storage/memory/geometry/query-batch resources retain their
own admission. Limits bound complete populations,never an accepted prefix. Empty input refuses;
a role catalogue's valid empty state does not supply a successful edge manifest.

```rust
use sc_core::construction::*;
fn admit_slash_edges<'a>(manifest:&'a SlashSpreadEdgeManifest,bound:&'a BoundSlashSpreadEdgeRoles<'a>,
    limits:SlashSpreadEdgeManifestLimits)->Result<AdmittedSlashSpreadEdges<'a>,SlashSpreadEdgeManifestError<'a>> {
    manifest.admit(bound,limits)
}
```

Errors retain the actual whole original manifest,bound intent/catalogue,selected policy and first
typed cause. Quota,empty and operation failures precede candidate inspection. Duplicate/undefined/
inactive/self-source/policy failures report actual original candidate positions. Nothing sorts,
renumbers,deduplicates,changes membership or returns a partially accepted list.

`AdmittedSlashSpreadEdges::outputs()` returns private views pairing original candidate fields with
actual catalogue active-list/definition fields in candidate order. Each retains its whole admitted
owner and original index. `logical_output()` uses the explicit operation/tag,not that index. For
candidate tags9,2 and catalogue active order2,9,both orders remain intact. Reverse iteration yields
2,9 without changing their identities.

Different explicit tags with equal source provenance remain distinct role requests. Such metadata
admission is not proof that coincident output geometry is appropriate. A manifest containing only
tag9 leaves active tag2 and inactive issued definitions intact;result absence never retires them.
Actual role matching remains G2/G3 work and must not infer continuity from equal keys or coordinates.

This component admits edge-role/provenance declarations only. It does not establish a complete
boundary/hole loop,trim intersection choice,source ownership/frame,current reference coverage or
retirement,metadata/copy/semantic reconciliation,measured fullness or publication authority. Those
native reconciliation contracts remain required before family closure. Actual transformed curves,
closure,reference maps and approximation/parameter budgets still require G2/G3 and `.5f.3`/`.5f.4.geometry`.
No ledger edit,mapped Param or accepted geometric prefix is supplied by this admission.
