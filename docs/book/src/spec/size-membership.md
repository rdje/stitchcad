# Executable size membership foundation

G1-SLICE.4c.1 implements the identity, label, order, system and base-member foundation in sc-measure.
The normative contract is [size sets §2–§4](size-sets.md), with ownership recorded separately. This
foundation is not yet a complete SizeSet: axes, complete charts, breaks and resolved-profile
transformation records remain owned by .4c.2/.3/.4; .4c.5 and .4d review the completed families.

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
| `SizeMembershipError` | Label, empty/duplicate membership, base, lookup or revision refusal |

Identity and exact-label queries borrow the member from the authored list. The base query borrows the
explicit base member. Structured errors retain set, member, label or overflowing revision information;
no missing lookup silently selects a first member. Private representation prevents unchecked reordering
or blank-label mutation; definition inputs can be cloned and explicitly reconstructed.

## Verification and current boundary

Twelve contracts cover human labels, order/base separation, stable identity under reorder/removal,
unique identities/labels, missing lookups, distinct sets, all system choices, a custom member of one
and checked revision overflow. Compile-fail tests cover private label/membership mutation and an
attempt to add quantity data. Seven real production mutations must fail regression assertions,
including an introduced lexical sort and a wrapped revision fallback, then restore exact source.
The standard strict Rust gate runs these contracts; native/WASM integration is checked locally.

D70 records a conflict in the normative axes requirements: the field table says optional and only
multidimensional, while §7 says a one-dimensional set has one axis and no second representation.
G1-SLICE.4c.2 awaits the director's cardinality ruling. This foundation creates no implicit axis or
numeric default. Individual [garment chart observations](size-chart-observations.md) now borrow current metadata;
complete coverage, break/path readiness and profile transformation intent,
actual geometry, equivalence, evidence scope and release remain later owned proofs.
