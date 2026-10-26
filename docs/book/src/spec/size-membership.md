# Executable size membership foundation

G1-SLICE.4c.1 implements the identity, label, order, system and base-member foundation in sc-measure.
The normative contract is [size sets §2–§4](size-sets.md), with ownership recorded separately.
Membership alone does not certify axes/charts/breaks. Current input families and
[immutable profile intent](size-set-transformations.md) are implemented separately; .4d owns combined signoff.

## Labels, identities and authored order

A SizeMembership contains a pinned SizeSetReference, explicit SizeSystem, ordered SizeMember list and
one base-member identity. Members have stable identities and validated human SizeLabels. The list must
be nonempty; member identities and exact labels are unique within it. A member cannot alias its set.
The base must exist among the members; it is never inferred from position or label spelling.

Labels preserve exact Unicode, case and spacing after checking that they are not entirely whitespace.
They carry no machine-token, quantity or measurement semantics. Labels "12" and "0012" are distinct;
"M" and "m" are distinct; an authored spaced label remains spaced. A missing exact label is refused
rather than normalized or replaced by a peer. Labels may repeat in different size sets and do not
establish shared identity, chart values or body measurements.

For example, an authored range "2XL, S, M" retains that order. A numeric-looking range "42, 38, 40"
also retains that order. Choosing the second member as base selects its identity; it does not select
the median label or an inferred smallest size. Explicitly reversing the order in a validated replacement
preserves the existing member/base identities and leaves the original immutable object unchanged.
An inserted or replacement member receives its own identity; reusing the old label does not restore a
removed identity. Save, grading and export must later preserve this same distinction.

A custom single-member range has that member as base. It can describe the membership of an MTM range,
but does not yet establish the required body-chart provenance or instantiation-path readiness. No
quantities are present: those belong to order data, as the normative size-sets chapter specifies.

## Pinned revisions and system intent

The reference carries a SizeSet identity and a Count revision, including initial revision zero.
A revision-successor query retains the identity and increments exactly once. At the Count limit it
refuses instead of wrapping; neither query nor replacement mutates an earlier reference. This is
bounded revision arithmetic, not current Design/registry validation or permission to revise a set.
The command bus must later enforce currentness and monotonic authorized transitions.

The five designation-system choices are explicit: EN 13402, ASTM D5585, alphanumeric, numeric and
custom. Selecting a system does not parse labels, manufacture chart measurements or certify a
standard designation. Standards content/mapping status remains the [standards chapter's](standards.md)
separate obligation; unverified mapping data remains unknown when it is introduced.

## Public API vocabulary

| API | Contract |
| --- | --- |
| `SizeSetReference` | Stable set identity and exact authored Count revision |
| `SizeSystem` | Explicit designation-system intent without data defaults |
| `SizeLabel` | Nonblank exact human label without machine-token semantics |
| `SizeMember` | Stable member identity and label |
| `SizeMembershipDefinition` | Authored reference/system/order/base input |
| `SizeMembership` | Immutable validated membership foundation |
| `SizeFormulaContext` | Actual membership/member borrows and derived core size values |
| `SizeFormulaContextError` | Pinned-reference/member/ordinal/core-frame refusal |
| `SizeMembershipError` | Label, empty/duplicate membership, base, lookup or revision refusal |

Identity and exact-label queries borrow the member from the authored list. The base query borrows the
explicit base member. Structured errors retain set, member, label or overflowing revision information;
no missing lookup silently selects a first member. Private representation prevents unchecked reordering
or blank-label mutation; definition inputs can be cloned and explicitly reconstructed.

## Formula size context

SizeMembership::formula_context(expected_reference, selected_member_id) now derives the three
reserved size values from the actual immutable authored record. It compares the caller's explicit
expected set id/revision first, then finds the selected stable id in the original member vector.
The one-based ordinal is position+1; size_count is the member count; is_base_size compares the
selected id with the explicitly authored base. Labels and SizeSystem intent enter no computation.
No sort, numeric label parse, positional base selection or default member is permitted.

```rust
use sc_core::{ontology::EntityId, recipe::{FormulaRational, FormulaSizeName}};
use sc_measure::{SizeLabel, SizeMember, SizeMembership, SizeMembershipDefinition,
    SizeSetReference, SizeSystem};
use sc_units::Count;

let expected = SizeSetReference { id: EntityId::from_bits(1), revision: Count::new(0) };
let member = |id, label: &str| SizeMember { id: EntityId::from_bits(id),
    label: SizeLabel::new(label.to_owned()).expect("nonblank label") };
let membership = SizeMembership::new(SizeMembershipDefinition {
    reference: expected, system: SizeSystem::Custom,
    members: vec![member(11, "42"), member(12, "0012"), member(13, "2XL")],
    base: EntityId::from_bits(12),
}).expect("authored order/base");
let context = membership.formula_context(expected, EntityId::from_bits(11)).expect("exact pin/member");
assert_eq!(context.member().label.as_str(), "42");
assert_eq!(context.read(FormulaSizeName::Index).expect("ordinal").numeric_value()
    .expect("Count").rational(), FormulaRational::ONE);
assert_eq!(context.read(FormulaSizeName::IsBaseSize).expect("present false").boolean(), Some(false));
```

SizeFormulaContext retains the actual membership/member borrows and exposes its core-only
FormulaSizeContext through context(). Source retains actual set/revision/selected/base identities.
A reordered validated replacement changes the new snapshot's ordinal, with stable member/base ids;
the prior view retains its prior pin and values. Passing the old pin to the replacement refuses.
A singleton reads index/count1 and base true, including at initial or maximum Count revisions.
Human labels reused across sets never substitute for stable identity. Counts are widened before
one-based arithmetic; binding storage is checked later by the core's existing boundary.

SizeFormulaContextError returns ReferenceMismatch with both actual/requested pins and selected id
before any missing-member result. An absent selected id retains SizeMembershipError::MissingMember;
ordinal/core-frame failures propagate without arithmetic wrapping or a substituted member/value. Standard error
sources retain the actual underlying membership/core refusal. Debug omits human labels; explicit
member access retains them for presentation.

Matching the supplied pin is not global registry/currentness certification, complete SizeSet,
axes/charts/break/path readiness, profile transformation or release approval. The confirmed D70 axis rule
is implemented separately under .4c.2; [individual axes](size-sets.md#executable-individual-axes--g1-slice4c21)
now preserve explicit names/order and canonical source records. [Complete member coordinates](size-sets.md#complete-axismember-points--g1-slice4c22)
require at least one authored axis, including singleton sets.
The dependency remains sc-measure to sc-core; core imports no measurement implementation.
Five focused formula_size_membership_contract tests pass: all designation intents/actual pointer
borrows, pin-before-member priority, authored reorder/stable base, singleton revision boundaries and
same-label cross-set refusal. Selected strict Clippy passes; broad/platform proof remains deferred.

## Verification and current boundary

Twelve contracts cover human labels, order/base separation, stable identity under reorder/removal,
unique identities/labels, missing lookups, distinct sets, all system choices, a custom member of one
and checked revision overflow. Compile-fail tests cover private label/membership mutation and an
attempt to add quantity data. Seven real production mutations must fail regression assertions,
including an introduced lexical sort and a wrapped revision fallback, then restore exact source.
The verifier guards its mutable implementation and current read-only test source before effects,
and plans all eight log files. Each real failure must identify a current test assertion macro by
source, line and byte column; a panic message containing "assertion" supplies no proof by itself.
The restored public Cargo artifact runs the complete12-test target successfully.
The standard strict Rust gate runs these contracts; native/WASM integration is checked locally.

D70's director ruling2026-10-26 requires at least one authored axis for every complete SizeSet.
G1-SLICE.4c.2 implements explicit names, ordered values and complete member coordinates. This membership
foundation creates no implicit axis or numeric default. Individual [garment chart observations](size-chart-observations.md) now borrow current metadata;
complete chart/break coverage and [typed structural path inputs](size-sets.md#unified-current-input-family--g1-slice4c3d33)
are now available. Profile transformations, actual geometry, equivalence, global currentness, evidence
scope and release remain later owned proofs.
