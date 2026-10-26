# SizeSet profile transformations

> **Status:** G1-SLICE.4c.4 is implementing immutable transformation intent in `sc-measure`.
> Selected profile labels and resolution-origin metadata are executable. Typed transformation payloads
> and their aggregate follow. G4 owns actual profile lookup, constraints, precedence and resolution;
> G3 owns numeric instantiation. This page extends [Size sets §9](size-sets.md#9-ownership-reference-and-override).

## Pin the selected profile explicitly

`SizeSetProfilePin` retains three distinct authored strings: `profile_id`, `schema_version` and
`content_version`. Roadmap §8's example selects profile `acme-cmt-pt`, schema `2.0` and content
`1.3.0`. Updating profile content does not silently change a saved pin or its schema version.

```rust
use sc_measure::{SizeSetProfilePin, SizeSetProfilePinDefinition};
let profile = SizeSetProfilePin::new(SizeSetProfilePinDefinition {
    profile_id: "acme-cmt-pt".to_owned(),
    schema_version: "2.0".to_owned(),
    content_version: "1.3.0".to_owned(),
}).expect("all three fields explicitly supplied");
assert_eq!(profile.definition().schema_version, "2.0");
assert_eq!(profile.definition().content_version, "1.3.0");
```

G1 checks presence only: an empty or whitespace-only field returns `SizeSetProfilePinError`, naming
that field and preserving its refused spelling. The order is profile key, schema version, content
version. Nonblank text retains Unicode, case and spacing exactly. Labels are opaque here; accepting
one proves neither a valid version grammar nor an existing or approved registry entry. G4 validates
those contracts. There is no normalization, version comparison, inferred default or latest-version
selection. `validation()` explicitly returns `ProfileBindingValidation::DeferredToG4`.

This selected pin belongs to a resolution operation. Existing authored `ProfileParameterRef` bindings
remain symbolic and factory/version agnostic until instantiation; this API does not rewrite them.

## Keep the source and resolved target distinct

`SizeSetResolutionOriginDefinition` records:

| Field | Authored meaning |
| --- | --- |
| `source` | Exact original SizeSet identity/revision named by the Design |
| `profile` | Explicit selected profile/schema/content pin |
| `resolved` | Separately allocated target identity/revision |
| `provenance` | Explicit correspondence/provenance record identity |

Pass that definition and an actual `SizeSetInputs::Garment` or `SizeSetInputs::Mtm` view to
`SizeSetResolutionOrigin::new`. It rechecks current supplied inputs, requires the exact source pin,
and refuses a target identity already present among supplied input records or equal to the origin's
provenance identity. Changing the revision does not permit reusing the source object's identity.
The checks cover membership, axes/coordinates, canonical numeric and measurement records, tables,
chart observations or MTM body/Ease records, the chart itself and available breaks.

For example, source `(set A, revision 3)` may record target `(set B, revision 0)` and the exact
selected profile. Target `(set A, revision 4)` refuses with `DuplicateIdentity`; a definition that
claims source `(set A, revision 2)` against revision-3 inputs returns `SourceMismatch` with both
references. The constructor never retargets a stale source or mutates the original membership/chart.
`definition()` borrows the immutable authored metadata; `validate_current(inputs)` rechecks it.

The caller or future command bus supplies the allocated target ID. Local nonaliasing is not proof
of global ID freshness, registry currentness or provenance truth. Those obligations remain with the
allocator, registry and evidence store. An origin can accompany draft chart/break inputs; recording
intent does not certify regeneration or grading readiness. Existing path checks remain separate,
including MTM's original grading refusal.

## What remains to be implemented

The next bounded slices add authored label mappings and base substitution, canonical break-scaling
and chart-replacement bindings, then an immutable aggregate retaining the original reference and
ordered transformations. Unknown or derived inputs must retain their original state and evidence.

Origin metadata alone is not a transformed SizeSet or transformed geometry. No profile is executed,
no chart or break is numerically altered, and no Design reference, saved revision, approval or release
artifact is changed by these constructors. Full precedence, hard restrictions, execution/equivalence,
serialization and release checks retain their G3/G4 and integration owners.
