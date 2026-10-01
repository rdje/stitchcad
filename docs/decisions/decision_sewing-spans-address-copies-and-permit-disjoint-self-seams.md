# Sewing spans address copies and permit disjoint self-seams

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.3c.2b.2`, D35/D57, ontology §4.2, reference skirt §8.

answers: "does a gather duplicate sewing ease or transfer to a replacement physical copy?" · "may a seam join one physical copy to itself?" · "how are source references and reflection separated?" · "what does declared seam ease mean before geometric walking?"

## Context

The supported construction vocabulary includes closing darts and folded-piece finishes. The reference
band's folded short-end closure joins material of one physical copy to itself; forbidding every
self-seam would require an artificial duplicate pattern/copy. D57's director ruling already gives
physical copies stable identities, so two copies of one Piece are distinct material domains.

## Decision

A side names one CutCopy id and one positive-length EdgeRange, in the source Piece's frame. An
explicit correspondence says whether the two stored lower endpoints meet each other, or the lower
endpoint of A meets B's upper endpoint. This is independent of later journal reversal and copy
reflection. G2/V1 consumes all three facts, never inferred from label names or drawing order.

Same-copy seams are permitted when the two resolved ranges have disjoint positive-length interiors;
shared endpoints are legal. Identical or overlapping material intervals on one copy cannot be sewn
to themselves and are a typed refusal. Different copies of one Piece can use the same source range.
The constructor checks current resolved fragments, so split/merge history cannot bypass disjointness.
Later queries preserve the held ranges and expose missing interiors/endpoint choices. Atomic commands
must revalidate replacements; an old validated graph is not a perpetual current-topology certificate.

Stop landmarks name semantic Notches or edge-anchored TurnPoints by stable id, attached to an explicit
span side. They must belong to the copy's source Piece and the resolved side interval. A TurnPoint
asserts sewing semantics; G2 checks actual turning geometry. Landmark identity is not a tessellation
index, and a profile's notch geometry is irrelevant to semantic stop resolution.

Declared ease is signed A-minus-B length, explicit or symbolic, rather than stretching material by
renderer convention. The distribution is explicit: uniform, positive weighted non-overlapping regions
of normalized span arc-length progress, or between two distinct named notch anchors on one side.
Weights are relative densities; omitted regions receive none. G2/G3 walking validates actual arc
length, direction and differential, and defines the sampling/resampling realization. G1 preserves
intent and checks structural domains; it never claims measured seam equality or a fulfilled ease budget.

## Consequences

- Partial spans and several spans sharing a source edge express one-to-many correspondence.
- The reference band can be represented without manufacturing a second physical copy. Its detailed
  range geometry remains the executable G2 fixture's obligation, not a invented numeric golden here.
- Unknown copies/landmarks, unresolved born references and overlapping self-seams are explicit failures.
- Global id, command revision, geometry, profile resolution and release checks remain separate gates.

## Gather binding (`G1-SLICE.3c.4a.2b`)

A Gather binds an explicit graph/span/side/physical-copy identity. Its attachment interval, signed
intake source and allocation come from the existing span; there is no independently authored intake
or hidden second distribution. A-minus-B is nonnegative for gathering A and nonpositive for gathering
B; explicit wrong-sign declarations are refused, while symbolic value/sign/conservation remains a
later validation obligation. Queries expose the raw signed declaration plus selected side, preserving its signed provenance. Direction and closing-operation identity remain
explicit intent. Missing targets or a changed side-copy binding are typed failures; a replacement
copy never inherits this gather. Current raw geometry evidence is retained without automatic rewriting.
