# Sewing spans address copies and permit disjoint self-seams

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.3c.2b.2`, D35/D57, ontology §4.2, reference skirt §8.

answers: "does a gather duplicate sewing ease or transfer to a replacement physical copy?" · "may a seam join one physical copy to itself?" · "how are source references and reflection separated?" · "what does declared seam ease mean before geometric walking?" · "does graph construction authorize replacement sewing sources?" · "can numeric canonical ids supply sewing formula or profile metadata?" · "can an owned sewing draft preserve damaged held references?" · "must unused sewing landmarks validate against current sources?"

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

SewingGraph.validate_current now shares the original constructor's structural validator. It checks
actual supplied plan/Pieces, source range ownership, semantic stops and distributions in original
order without rebuilding or rewriting the graph. Missing-id inventories alone cannot detect a
same-id copy reassigned to another Piece or a same-id landmark moved outside its side. Current
validation retains the original typed refusal and complete range repair evidence. It supplies no
formula/profile amount resolution, geometric walking, command or release authority.

DesignInputs.sewing_amount_declarations now borrows actual per-span amount metadata after complete
current root validation. Formula identities name actual Design parameters, then require current
Length-kind canonical targets; dedicated and general scalar families stay distinct. Explicit signed
amounts borrow original spans. Logical profile refs remain DeferredToG4 even when numeric canonical
ids coincide. No numeric value, Profile schema/content role, graph-source join or geometry follows
this metadata query; owned complete sewing composition remains separate.

DesignSewing now privately owns exact input/graph/landmark objects. Complete owned identities include
all original root positions, graph, landmarks and spans before current root validation. References
remain uses. Ownership admission preserves held sewing references after edits for inspection and
explicit repair, without rebuilding born-valid objects against later contexts. Current graph and
complete sewing-source readiness stay separate from ownership; a copy replacement does not reassign
an old seam. No numeric, Profile, geometric, execution or release proof follows construction.

DesignSewing current source queries now validate complete root/ownership, every semantic landmark's
actual current Piece/anchor (unused included), the original graph against owned current contexts,
and amount declarations. They retain existing raw anchor/graph choice/repair rules and concrete
original errors. Amount views borrow actual owner objects; a successful metadata-only query cannot
authorize stale graph sources. Profile deferral, whole Piece topology and full numeric/physical/
execution/release readiness remain explicit separate scopes.

## Retained ease source/portion primitive0491

answers: "how do repeated source cuts preserve symbolic ease without independently copying child declarations?"

Capture the actual span's original ease/copies/domains/direction once into private immutable
SeamEaseSource. SeamEasePortion shares that same root while retaining its own exact original-frame
intervals. Same/Opposite pairs follow A order;source/interior refusals retain actual portion evidence.
Native selected3/sewing36/docs3 pass:both cut orders share original source pointers,including partial
domains/reflection,explicit/formula/profile and all distribution kinds. Retained data outlives the
graph;replacement or topology deletion does not rebind or certify it. Capture clones weighted data
once,so command admission must precede capture. Canonical graph/source ownership,current readiness,
resource accounting and original-root quantity queries remain next .3.2/.3.3;D301 stays P1 before effects.
