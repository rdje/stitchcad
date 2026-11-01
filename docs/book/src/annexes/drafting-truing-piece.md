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
