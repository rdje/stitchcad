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
Private record fields prevent mutation,and no scalar conversion exists. This is standalone
structural admission:actual canonical inventory membership,Root/resource integration,Profile
source binding,semantic operations,currentness and complete dependency proof remain separate.


The standalone inventory and its exhaustive content producer come before actual Root attachment.
Canonical Root attachment must include complete identity/metadata and resource-producer updates
in the same change. An exposed Root with an uncharged logical cohort is never an intermediate
accepted state. Numeric-only construction remains an explicitly empty logical scope throughout.


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

This inventory is not yet attached to canonical Design/Profile inputs. Its constructor limits
supply no aggregate content credit to the later source gate. Exhaustive content accounting comes
next, then atomic Root/identity/resource integration. Source facets, evidence truth, currentness,
semantic recipes and complete programme acceptance retain their separate owners.


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

This is standalone content accounting. Canonical Root/identity integration, Profile/source binding,
currentness and semantic execution remain separate. Root attachment must wire this producer into
every actual/prospective canonical traversal atomically;the standalone function grants no uncharged
Root or whole-source acceptance.
