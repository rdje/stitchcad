# Profile bindings remain symbolic until a target profile resolves them

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.3c.3a` and its marks/allowances siblings; ontology §4.4–§4.6, §5.

answers: "does a G1 notch default its geometry?" · "where do notch style and encoding live?" · "can symbolic profile bindings generate physical geometry?"

## Context

Notch style, sample/production geometry and encoding are Factory Profile parameters, not authored
physical defaults on a design. G1 must represent the semantic object before G4 supplies the profile
schema, scoped evidence and artifact policy. Storing a convenient scalar here would invent a fact
and duplicate the parameter owner's value and uncertainty state.

## Decision

A `ProfileParameterRef` names a logical parameter declaration by stable identity. It contains no
scalar, enum selection, profile version or default value. The target Factory Profile resolves that
logical declaration at instantiation; the reference does not pin a design to one factory's content.
Physical-field roles on the object state the expected value kind: style, encoding, depth/width,
angle or allowance inclusion. G4 validates each binding's existence/type, resolves its value/state
and applies the artifact policy; G1 reports `ProfileBindingValidation::DeferredToG4` visibly.

The semantic anchor is independently born-valid against the topology ledger and named piece. It
resolves through the existing point/range contract after edits and is never rewritten automatically.
Neither a valid anchor nor a symbolic binding grants physical export readiness.

## Consequences

- No G1 API returns physical notch geometry from a reference or substitutes a fallback for an
  unread/unknown profile value. Sample and production references are carried separately, even when
  callers deliberately name the same declaration for both.
- Style/encoding vocabularies describe possible profile values; the notch does not select one by
  default. Parameter values and their uncertainty remain with the profile owner, without cached flags.
- G4 schema and value resolution (`G4-PROFILES.1` / `.7` / `.13`) must check the declarations and
  field kinds. This slice implements neither profile validation nor the artifact generator.
- The no-default contract uses the normative release §8 tuning. D58's contradictory future-task
  acceptance is corrected as dependency alignment, not treated as an alternate policy authority.
