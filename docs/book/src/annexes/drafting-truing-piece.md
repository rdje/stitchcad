# Native truing whole-Piece declarations

This companion continues the [selected-source and outcome contracts](drafting-truing-outputs.md).
It declares whole-Piece replacement inputs;actual geometry,currentness,fit and publication remain separate.

## Declare explicit outside-fragment roles

`TruingOutsideRoles` retains two explicit `TruingOutsideIntent` fields,prefix then suffix. Each
field is `Absent` or `Role(LocalTag)`;there is no default or generated tag. `admit` binds the complete
fixed proposal to actual `RequiredTruingEvidence` under `TruingOutsideRoleLimits::new(N,catalog)`.
N counts every selected-domain candidate. The catalogue policy bounds all definitions,active
occurrences and cached original key bytes,including inactive/unrelated declarations.

```rust
use sc_core::{construction::*,ontology::LocalTag};
fn admit<'a>(raw:&'a TruingOutsideRoles,evidence:&'a RequiredTruingEvidence<'a>)
    ->Result<AdmittedTruingOutsideRoles<'a>,TruingOutsideRoleError<'a>> {
    raw.admit(evidence,TruingOutsideRoleLimits::new(1,EdgeOutputRoleCatalogLimits::new(4,3,25)))
}
let raw=TruingOutsideRoles::new(TruingOutsideIntent::Role(LocalTag::new(2)),
    TruingOutsideIntent::Role(LocalTag::new(5)));
assert_eq!(*raw.intent(TruingOutsidePart::Prefix),TruingOutsideIntent::Role(LocalTag::new(2)));
```

Whole N quota precedes fresh D/A/key-byte/owner admission of the **same actual bound catalogue**.
The original binding kernel checks its cached counts without scanning keys or allocating. Only
then are prefix and suffix inspected,in that order. An unaccepted suffix returns the whole original
proposal and context,never an accepted prefix view. All prior storage/workspace obligations remain.

For `Refit`,a positive prefix is exactly original0 through the selected lower parameter;a positive
suffix is exactly the selected upper parameter through original1. The caller supplies neither a
replacement edge nor independently authored range bounds. Each positive outside portion requires a
role. A zero-length outside portion requires `Absent`;supplying a role instead returns the exact
canonical `SourceRange` cause. Original bounded rational parameters are copied without rescaling,
epsilon,rounding or geometry evaluation. This is the original held edge frame,not a current mapping.

For selected1/4–3/4 and selected candidate role9,prefix role2 declares0–1/4 and suffix role5
declares3/4–1. Selected0–3/4 requires absent prefix and a suffix role;selected1/4–1 requires a
prefix role and absent suffix. Selected0–1 requires both absent. `PreserveSource` also requires
both absent for **every** selected domain:the whole original source remains required unchanged,
and no outside output identity is introduced merely because the selected span is partial.

Every supplied role must have an actual definition and active membership. It must differ from
every selected-domain candidate tag and from the other outside role. A missing positive portion
refuses `Missing`;unknown/inactive tags refuse `Undefined`/`Inactive`;reusing selected role9 refuses
`SelectedRole`;prefix2/suffix2 refuses `Duplicate`. Source preservation with any supplied role
refuses `PreserveSource`,before tag lookup. Catalogue quotas still precede those field causes.

Two fixed association slots require constant space and no allocation;work is O(log(N+D+2)),for
D actual catalogue definitions. `proposal()`, `evidence()` and `limits()` retain the actual whole
original parents and policy. `roles()` lends present associations in prefix/suffix order and
reverse. Each authentic `TruingOutsideOutput` retains its whole `context()`,fixed `part()`,original
`intent()` field,exact derived `source()` range and actual active catalogue `role()`.
`logical_output()` declares the original operation/tag tuple;it mints nothing. Errors retain the
same whole proposal/evidence/policy and first quota or fixed-field cause. Original inputs stay unchanged.

These attributions do not choose output parameter orientation,whole-Piece placement/traversal,
source retirement or geometric correspondence. Those need explicit declarations and proof;an
original contour direction is distinct from physical-copy reflection and sewing correspondence.
Whole-Piece orientation/contour/lineage declarations remain required before native family closure.
Actual untouched outside geometry,current source resolution,identity issuance,fit,effects and
atomic publication retain their separate execution/evidence owners.

## Declare every output parameter direction

`TruingDirectionManifest` retains an explicit operation and ordered
`TruingDirectionAssignment` collection. Each record names a persistent output tag and a required
`TruingParameterDirection`. This dedicated type has no default and cannot be replaced by the
ontology's Piece-traversal `Direction`.

`SameAsSource` requests output start/end correspondence to the attributed source from/to bounds.
`ReverseSource` requests correspondence to source to/from. Neither choice is a numeric or affine
parameter map,an achieved position,Piece traversal,copy reflection or sewing correspondence.

```rust
use sc_core::{construction::*,ontology::{EntityId,LocalTag}};
let raw=TruingDirectionManifest::new(EntityId::from_bits(1000),vec![
    TruingDirectionAssignment::new(LocalTag::new(5),TruingParameterDirection::ReverseSource),
    TruingDirectionAssignment::new(LocalTag::new(9),TruingParameterDirection::SameAsSource),
    TruingDirectionAssignment::new(LocalTag::new(2),TruingParameterDirection::ReverseSource),
]);
assert_eq!(raw.assignments().len(),3);
fn admit<'a>(raw:&'a TruingDirectionManifest,outside:&'a AdmittedTruingOutsideRoles<'a>)
    ->Result<AdmittedTruingDirections<'a>,TruingDirectionError<'a>> {
    raw.admit(outside,TruingDirectionLimits::new(3,3,6))
}
```

`admit` uses the actual complete selected/outside population,not every active catalogue role.
Let T be selected candidates plus present outside roles,and M the complete raw assignment count.
Checked T addition and2M workspace multiplication precede whole T,M and workspace quotas. Those
precede operation equality,exact M=T cardinality,index allocation and original assignment scans.
Each tag must be unique and name an actual selected or outside output. Cardinality **together with**
unique actual-population membership proves complete attribution;equal counts alone prove nothing.

With selected9,prefix2 and suffix5,the actual population is T3. T2 refuses `Outputs` before M2
or W5. T3/M2 refuses `Assignments`;T3/M3/W5 refuses required workspace6. T3/M3/W6 admits
resource equality,then checks the original operation and fields. Missing assignments refuse `Count`.
Three assignments9/2/2 still refuse `Duplicate`;9/2/99 refuse `UnknownOutput` at original index2.
When a whole-domain refit declares only selected9,an otherwise active catalogue role2 is still an
unknown **output**. No missing direction is inferred from that role's presence or list position.

The additional index/association bound is2M logical entries,not allocator bytes or earlier owned
storage. Work is O(M log(M+T+2)). Every refusal retains the actual complete raw `manifest()`,whole
`outside()` parent,original `limits()` and first `refusal()`;no accepted prefix escapes.

`AdmittedTruingDirections` lends those same original parents/policy and reports actual
`workspace_entries()`. `assignments()` traverses original declaration order and reverse;
`assignment(tag)` queries in O(log(M+2)) work. Each authentic `TruingDirectionalOutput` retains
its whole `context()`,original `index()`/`assignment()`,actual selected/outside `source()` member
and explicit `parameter_direction()`. The source member lends its actual held `range()` and
unissued `logical_output()` tuple. No alternate source frame or caller-supplied replacement member
can enter admission. Original tags,directions,source fields and declaration order stay unchanged.

Explicit preservation admits an exactly empty new-output assignment set with T0/M0/W0 and the
actual operation. It supplies no direction defaults,retirement,no-op or fit certificate;original
active metadata remains present. Whole-Piece placement still needs original occurrence ordering
and composition with these explicit source-relative choices. Actual endpoint/stop positions,
interior parameter maps,current source/history,geometry,fit,effects and atomic publication remain
separate execution/evidence obligations.
