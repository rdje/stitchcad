# Logical inputs and original source membership

## Finite logical declaration records

`LogicalInputDeclaration` now owns an immutable authored finite domain,state and provenance.
Its `LogicalInputKind` is PocketOpening or NotionSize. Neither is a scalar formula operand kind;
a hardware member identifier supplies no inferred dimensions,and an opening identifier supplies
no physical construction or supported-envelope approval. Domain members are explicit validated
`MachineToken`s. Vendor codes and localized labels are not normalized into these identifiers;
a future source must preserve their own original records and explicit member association.

The editable `LogicalInputDeclarationDefinition` supplies the original declaration/source IDs,
kind,nonempty distinct domain vector and one state. No selection or domain member is supplied
implicitly. These state forms retain different authored meanings:

| State | Original fields |
| --- | --- |
| Known | Selected domain member and nonempty distinct scoped-evidence references |
| Assumed | Selected member and human-assumption reference |
| Unknown | Observation request;no selected value or fallback |
| Choice | Explicit selected member and design-decision provenance;no observed-fact claim |
| Preference | Selected member and preference provenance |
| Derived | Sole semantic operation reference;no authored or cached result |

Known,Assumed,Choice and Preference selections must belong to this actual domain. Unknown and
Derived still need their original possible domain. Evidence/assumption/observation/decision/
preference/operation IDs retain their source meaning;this constructor proves neither existence
nor truth,human approval,operation kind/dependencies or current Profile binding. Logical Derived
uses an actual later-bound semantic computation;it does not add enum-valued `let` or scalar strings.

Select `LogicalInputLimits::new(domain_members,evidence_records,text_bytes,validation_inspections)`
explicitly. There is no Default. The factory first derives complete actual domain count D,Known
evidence count E,all owned domain text plus separately owned selection text,and a conservative
`D*D + E*E + D` member/identity comparison bound. Every original repeated entry counts. Complete
checked arithmetic precedes domain,evidence,text,then comparison quotas. These are logical counts,
UTF-8 bytes and comparisons;they do not measure allocator capacity,string-comparison byte work,
CPU time or runtime execution. Caller-owned input buffers were allocated before this constructor.

Only after every quota passes does core check empty/duplicate domain,selection membership,then
missing/duplicate Known evidence,in that order. Validation uses borrowed prefix scans and no
workspace allocation. The successful record moves the original vectors without cloning,sorting,
normalizing or deduplicating them. `definition()` returns that immutable original data;`id()` and
`kind()` retain its original declaration fields. `limits()` retains constructor proof metadata and
`shape()` exposes the complete actual populations/bound. Authored equality compares definitions,
so changing constructor allowances cannot change the record's authored semantic content.

For example,neutral identifiers `option_a` and `option_b` describe no physical opening vocabulary:

```rust
use sc_core::{name::MachineToken,ontology::EntityId,value::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let record=LogicalInputDeclaration::new(LogicalInputDeclarationDefinition {
    id:EntityId::from_bits(1),source:EntityId::from_bits(2),
    kind:LogicalInputKind::PocketOpening,
    domain:vec![MachineToken::new("option_a")?,MachineToken::new("option_b")?],
    state:LogicalInputState::Choice {
        value:MachineToken::new("option_b")?,decision:EntityId::from_bits(3),
    },
},LogicalInputLimits::new(2,0,24,6))?;
assert_eq!(record.definition().state.tag(),LogicalInputStateTag::Choice);
# Ok(()) }
```

The two domain members own16 bytes and the separate selection8 bytes:E0,text24,comparisons6.
For domain `small,large` with Known selection `small` and two evidence references,the complete
requirements are D2/E2/text15/comparisons10. Exact quotas pass;D1,E1,text14 or comparisons9 refuses
the respective complete requirement before any domain/state validation. Unknown/Derived with
that same domain use text10/comparisons6 and no selected-value field.

`LogicalInputDeclarationError` retains the original declaration ID and typed refusal data in
its public `refusal` field. Domain/evidence duplication reports first and repeated authored
indices;duplicate evidence also retains its original identity. An outside-domain selection keeps
its original state label without emitting authored member text. Debug omits domain/selection text.
Private record fields prevent mutation,and no scalar conversion exists. Record construction is
structural admission. Canonical attachment and complete content accounting are described below;
Profile source binding,semantic operations,currentness and complete dependency proof remain separate.


Canonical Root attachment includes complete identity/metadata and resource-producer updates.
Numeric-only construction remains an explicitly empty logical scope throughout.


## Standalone logical inventory

`LogicalInputs` now owns the supplied `Vec<LogicalInputDeclaration>` as a standalone component.
It preserves original order, storage, definitions and provenance. Referenced source, evidence,
observation and operation IDs do not become owned declarations. All owned declaration IDs must
be distinct, including identical duplicates or records of different logical kinds.

Select `LogicalInputInventoryLimits::new(declarations,validation_inspections)` explicitly. Core
first derives the complete original population L and checked conservative L*L identity-comparison
bound, then checks the declaration quota followed by the comparison quota. Only then does it scan
for the first repeated identity, retaining first and repeated authored indices. Duplicate validation
allocates no workspace, reads no selected values and creates no accepted prefix on failure.
The caller's input buffers already exist; these are constructor count/comparison allowances,
not allocator, aggregate held-content, source-query, currentness or runtime budgets.

For L3, exact allowances3/9 pass; a declaration allowance2 refuses required3 before comparisons or
duplicates. Comparison allowance8 refuses required9 before duplicate scans. With three supplied
IDs `[40,50,40]`, allowances3/9 reach DuplicateIdentity with first0 and duplicate2. Identical
records still refuse. Empty input with explicit0/0 yields empty slices and Missing lookups;
it never invents an Unknown declaration, domain or default.

`declarations()` returns the complete original readonly slice. `limits()` and `shape()` retain
constructor policy and complete populations; authored equality compares declarations, independent
of constructor allowances. `declaration(id)` inspects an actual original member by owned ID.
`resolve(id,expected_kind)` additionally checks the exact logical role and returns a private
`LogicalInputBinding`. Its `inputs()`, `declaration()` and `index()` retain the actual owner,
original member and authored position. The binding is Copy without copying a definition, and
cannot outlive or replace its inventory/member. The index is a position witness, not identity.

For example, an Unknown original record stays Unknown after successful metadata binding:

```rust
use sc_core::{name::MachineToken,ontology::EntityId,value::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let record=LogicalInputDeclaration::new(LogicalInputDeclarationDefinition {
    id:EntityId::from_bits(1),source:EntityId::from_bits(2),
    kind:LogicalInputKind::PocketOpening,domain:vec![MachineToken::new("option_a")?],
    state:LogicalInputState::Unknown {observation:EntityId::from_bits(3)},
},LogicalInputLimits::new(1,0,8,2))?;
let inputs=LogicalInputs::new(vec![record],LogicalInputInventoryLimits::new(1,1))?;
let binding=inputs.resolve(EntityId::from_bits(1),LogicalInputKind::PocketOpening).map_err(|e|e.to_string())?;
assert!(std::ptr::eq(binding.inputs(),&inputs));
assert_eq!(binding.declaration().definition().state.tag(),LogicalInputStateTag::Unknown);
# Ok(()) }
```

`LogicalInputInventoryError` retains the selected limits and first whole-bound or duplicate cause.
`LogicalInputLookupError::Missing` retains the requested ID and kind. Kind retains the requested kind and
actual original member loan, rather than finding another record of that kind or returning a clone.
Kind errors borrow the original member. A caller returning an owned static error can explicitly
render it,as the example does;the native typed error remains available within the owner lifetime.
Diagnostics omit authored member text. Equal-data inventories can compare equal while their
bindings still hold different actual owner/member pointers. A later source controller must compare
that actual inventory with the original Root/Profile owner; equality or matching IDs is insufficient.

The inventory can now be attached to canonical Design inputs as described below. Its constructor
limits supply no aggregate content credit to the source gate. Profile attachment, source facets,
evidence truth,currentness,semantic recipes and complete programme acceptance retain separate owners.

## Attach to the original canonical Design

`CanonicalInputs::with_logicals(lengths,scalars,logicals)` moves the original logical owner beside
the original numeric vectors. It preserves the original logical declaration buffers, domains,
states, provenance and order. `logicals()` borrows that exact inventory; `resolve` then loans the
actual original member. A matching clone or equal ID supplies no replacement for these loans.

Owned IDs must be disjoint across lengths, then scalars, then logical declarations in authored
order. `CanonicalInputsError::DuplicateIdentity` retains both original `CanonicalInputPosition`
witnesses, including Logical. `contains(id)` checks ownership across all three families; source,
observation and operation references do not become owned declarations. The containing
DesignStructure census adds logical IDs after scalars and before Pieces. DesignInputs and higher
owners inherit that census, so a logical ID cannot also own a formula source, metadata, Piece or
other contained entity. Parameter, Material, metadata-host and measurement-inventory checks also
include logical ownership before their target/reference validation.

For example, a length ID70 with logical IDs `[80,70,90]` refuses with first Length0 and duplicate
Logical1. A containing Design ID70 likewise refuses at Design/Logical1. A formula source ID70
beside that logical cohort refuses at Structure(Logical1)/FormulaSource0. These checks involve
actual owned identities; a Derived operation reference901 may remain unresolved for later semantic
source binding without being mistaken for a separately owned scalar expression.

`CanonicalInputs::new(lengths,scalars)` remains an explicitly numeric-only constructor. Its logical
inventory has zero declarations and explicit zero count/comparison allowances. It creates no
Unknown state, domain or selection. Numeric Length/Scalar lookups never return logical records.
`CanonicalInputTarget` remains a numeric target representation, and scalar named parameters cannot
bind a logical declaration. Logical member binding uses `LogicalInputs::resolve` separately.
The numeric `FormulaSourceInputs` projection checks numeric declarations and their expression
sources; containing Design admission enforces the complete all-family ownership census first.
Logical Derived operation references never enter that numeric source graph or become formula `let`s.

```rust
use sc_core::{name::MachineToken,ontology::EntityId,value::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let record=LogicalInputDeclaration::new(LogicalInputDeclarationDefinition {
    id:EntityId::from_bits(70),source:EntityId::from_bits(900),
    kind:LogicalInputKind::PocketOpening,domain:vec![MachineToken::new("option_a")?],
    state:LogicalInputState::Unknown {observation:EntityId::from_bits(901)},
},LogicalInputLimits::new(1,0,8,2))?;
let logicals=LogicalInputs::new(vec![record],LogicalInputInventoryLimits::new(1,1))?;
let original_buffer=logicals.declarations().as_ptr();
let inputs=CanonicalInputs::with_logicals(vec![],vec![],logicals)?;
assert_eq!(inputs.logicals().declarations().as_ptr(),original_buffer);
assert!(inputs.contains(EntityId::from_bits(70)));
assert!(inputs.length(EntityId::from_bits(70)).is_none());
# Ok(()) }
```

`with_length_declaration` preserves the complete logical cohort while replacing only an existing
dedicated length in a detached candidate. Actual and prospective length-replacement accounting
both walk the original logical fields through the same producer below. This attachment grants no
logical mutation command, Profile binding, operation dependency proof or physical readiness.

`LogicalInputTarget {declaration,kind}` now supplies an explicit raw logical reference for
[typed logical recipe sources](semantic-recipes.md#standalone-logical-derivation-operation).
It grants no owner/member proof itself; use the actual inventory's `resolve` with those fields.
Domain, state and selected value remain singly owned by the actual canonical declaration.


## Account for actual logical content

`charge_logical_input_resources(&inputs,sink)` now walks the complete actual standalone inventory.
It charges the declaration vector before visiting members, then each original domain vector before
its tokens. Every domain member's original text is counted. Known, Assumed, Choice and Preference
also charge their separately owned selection text; equal content in a domain and selection still
counts twice. Known precharges its original evidence vector. Unknown and Derived have no selected
text to charge; their observation/operation references remain fixed identity payloads.

`charge_logical_declaration_resources(&record,sink)` exposes the same complete record walk for a
separately held snapshot. It does not invent an inventory index or charge a nonexistent container.
Exhaustive private owner views and definition/state patterns ensure new owned fields require an
accounting decision. Retained constructor limits and shapes are fixed Copy metadata, checked as
such; neither their allowances nor cached totals provide content credit.

Both functions use the caller's original cumulative `DesignResourceSink`. They create no meter,
result vector, lookup, cloned model or validation pass. They compute no selection and query no
source or currentness. Every original separately owned field counts, including unused records.
The caller finishes the meter and owns the complete source allowance. A previous sink refusal
remains the same refusal, including for an empty inventory and its zero collection charge.

```rust
use sc_core::{design::*,value::LogicalInputs};
fn logical_content(inputs:&LogicalInputs,limits:DesignResourceLimits)
    ->Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(limits);
    charge_logical_input_resources(inputs,&mut meter)?;
    meter.finish()
}
```

Locations use Structure scope and these original field tokens:

| Field | Owner and index |
| --- | --- |
| `canonical.logicals` | No declaration owner/index;complete standalone vector |
| `canonical.logicals.domain` | Original declaration ID and inventory position |
| `canonical.logicals.domain.member` | Original declaration ID and domain member position |
| `canonical.logicals.value` | Original declaration ID and inventory position |
| `canonical.logicals.evidence` | Original declaration ID and inventory position |

A directly supplied declaration has no invented inventory position:domain,selection and evidence
use index None. Domain-member text still retains its actual original member position. Field tokens
and declaration owner distinguish those positions; no ID sorting or registry reconstruction occurs.

For one declaration in each state, each with domain `small,large`, Known evidence2 and selected
`small` in the four selection-bearing states, the inventory requires20 collection entries and80
text bytes:6 declarations +12 domain entries +2 evidence entries;60 domain bytes +20 selection
bytes. Fixed IDs and constructor proof fields add no dynamic counter bits. Prior2 entries,3 text
bytes and1 counter bit remain in the same meter, producing22/83/1. One direct Known declaration
requires4 entries/15 bytes without an invented container entry.

Short allowances retain the original first field. For that six-record fixture, entry allowance5
refuses the complete declaration vector6 before traversal. Allowance7 reaches the first domain's
required2 after6 entries. Text allowance14 reaches its selected value5 after10 domain bytes;
entry allowance9 reaches Known evidence2 after8 entries. Later Choice/Preference and final Derived
domain refusals keep their actual original owner and field position. No aggregate cached charge
can replace those original causes.

Canonical accounting now composes this same walk after all length/scalar content, for both actual
Roots and prospective length replacements. A Root with one Known length/evidence2, one Known
Boolean scalar/evidence1, and logical Unknown/Derived/Known over `small,large` with Known evidence2
requires16 entries/35 text bytes in canonical content. With revision7, an empty recipe's retained
canonical identity8 bytes and otherwise empty components, the Inputs Root requires16 entries,
43 text bytes and3 counter bits.
Text allowance34 refuses
the final logical selection5 after30 domain bytes at owner90,index2; it retains that exact cause
before later source queries. Constructor policy and shape metadata supply no content credit.
Profile/source binding,currentness,semantic execution and complete programme acceptance remain
separate. Standalone accounting itself grants no containing Root membership or whole-source proof.

## Resolve original numeric target membership

`CanonicalInputs::resolve_numeric(CanonicalInputTarget)` now resolves an original numeric target
without requiring formula metadata. Semantic policy/logical-operation inputs already carry that
exact target representation;the caller does not invent a formula name,origin or metadata input ID.
This primitive is parallel to logical inventory membership above. It binds the actual supplied
inventory,not a containing Root/Profile source,currentness,Derived supplier or work allowance.

`CanonicalInputTarget::Length(id)` selects only a dedicated length. `Scalar { declaration,kind }`
selects only a general scalar of the exact declared kind. A Length-kind general scalar never
substitutes for a dedicated length,or conversely. A logical declaration with the same requested ID
is another representation,not a numeric operand. Missing targets have no fallback.

The private `CanonicalNumericInputBinding` retains `inputs()`,the exact requested `target()`,and
an actual original `member()`. Its `position()` is the genuine zero-based family position. The
member view is Length or Scalar with its original declaration loan and index;public member-view
construction alone is not a membership certificate. The binding is Copy without copying any state,
value or source body,and cannot replace or outlive its actual inventory. Equal-data inventories
still produce bindings to different original owner/member pointers.

```rust
use sc_core::{ontology::EntityId,value::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let identity=EntityId::from_bits(1);
let record=LengthDeclaration::new(LengthDeclarationDefinition {
    id:identity,source:EntityId::from_bits(2),
    state:LengthState::Unknown {observation:EntityId::from_bits(3)},
})?;
let inputs=CanonicalInputs::new(vec![record],vec![])?;
let binding=inputs.resolve_numeric(CanonicalInputTarget::Length(identity))
    .map_err(|e|e.to_string())?;
assert!(std::ptr::eq(binding.inputs(),&inputs));
assert_eq!(binding.position(),CanonicalInputPosition::Length {index:0});
# Ok(()) }
```

Unknown and Derived records remain valid membership results with their original state and sole
observation/formula source;no authored value is read or selected. Other state/provenance also stays
in its owner. The factory allocates no workspace,clones no body and performs no provider/source or
value query. A containing checker must freshly admit complete content and bound all these lookup
scans before calling it.

`CanonicalNumericInputError` retains the exact original inventory and requested target.
`MissingLength` and `MissingScalar` retain the actual other-family position when that identity
belongs to a scalar,length or logical record. `ScalarKind` retains expected kind,the original
scalar index and the actual mismatched declaration loan. The caller can explicitly inspect its
original state;Debug/Display omit authored values. The typed error borrows its witnesses. An
owning static error boundary can explicitly render it,as in the example,without discarding the
native lending API.

Focused cases cover dedicated versus general Length,original indices,Unknown/Derived/Preference
states,equal-data owner separation,logical-family collisions,missing targets,kind mismatch and
empty inventories. Original record-buffer addresses stay unchanged. Diagnostic checks use an
authored Count sentinel and confirm it does not appear in binding/member/error/refusal formatting.
Compiler/API guards prevent private owner replacement and lifetime escape. Root context/source
composition,Profile numeric sources and complete semantic supplier/order acceptance remain
separate;this membership primitive certifies none of them.

## Names used in this chapter

Every API name and example name this chapter uses in code style, with its meaning. API meanings come from the code's own documentation; example names are not defaults.

| Name | Meaning |
| --- | --- |
| `CanonicalNumericInputBinding` | Type: Actual original numeric target-to-inventory association. No formula name,origin,metadata input ID,selected value,Root/Profile source,Derived supplier or work allowance is manufactured. |
| `CanonicalNumericInputError` | Type: Original requested target,actual immutable inventory and typed lending refusal. |
| `DesignResourceSink` | Trait: Restricted charging capability: a producer receives this trait object, never replaceable meter state. Implementations are sealed; there is no policy setter, reset, model replacement or approval method. |
| `LogicalInputBinding` | Type: Privately bound original inventory member and exact logical kind, without a value read. Equal definitions in a different actual inventory cannot replace this owner or record. |
| `LogicalInputDeclaration` | Type: Immutable structurally validated finite logical declaration. This certifies no canonical Root membership, actual Profile pin, evidence truth, operation binding, physical scope or readiness. |
| `LogicalInputDeclarationDefinition` | Type: Editable logical declaration. Domain members and any selection are original machine identifiers, not localized labels, vendor-code normalization, scalar strings or generated geometry. |
| `LogicalInputDeclarationError` | Type: Original declaration identity and first exact refusal; no partial declaration is returned. |
| `LogicalInputInventoryError` | Type: Original explicit selected policy and first whole-population/ownership refusal. |
| `LogicalInputKind` | Enum: Logical source role; neither variant is a scalar formula kind or a physical specification. |
| `LogicalInputs` | Type: Immutable standalone original logical owner. Existing records keep their original domains, state/provenance and authored order. |
| `MissingLength` | Variant of `CanonicalInputsError`: Replacement names no existing dedicated length; general Length-kind scalars remain distinct. |
| `MissingScalar` | Refusal cause: the requested general scalar is absent; a dedicated length cannot substitute for it. |
| `option_a` | Neutral example domain value; it carries no physical meaning. |
| `option_b` | Neutral example domain value; it carries no physical meaning. |
| `refusal` | Public error field holding the original first typed cause. |
| `ScalarKind` | Variant of CanonicalNumericInputRefusal: Original actual scalar member has another declared kind. The original record is retained. |
| `small` | Neutral example domain value used in this byte-count example. |
| `with_length_declaration` | Method or function: Return a detached copy with one existing dedicated length replaced at its original position. The replacement's identity selects that original record. |
