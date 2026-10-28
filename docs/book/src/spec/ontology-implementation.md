# Executable ontology at G1

Structural APIs implement the [garment ontology](ontology.md). Geometry, realized ease and resolved
profile values remain deferred.

**Pieces and inventories.** `Piece::new(PieceDefinition, &IdentityLedger)` validates editable input
into immutable content or a typed `PieceError`. Boundaries and holes are cyclic `DirectedEdge` lists;
the last edge precedes the first, without repeating it. Empty loops, repeated cut edges across any
loops, and nonlive boundary/hole/construction references refuse. Even a one-edge loop still needs G2
endpoint closure, winding, simplicity, hole containment/nonintersection and intake conservation.
Every Piece reports `GeometricValidation::DeferredToG2`, never a geometry badge; see
`decision_ontology-invariants-structural-g1-geometric-g2.md`.

Quantity is total physical copies. `Mirroring::Single` accepts any positive quantity;
`MirroredPairs` requires an even quantity. Separate `PairMember { handedness, companion }` content
requires an explicit other Piece and left/right hand; quantity counts this member's copies.
`PieceInventory::new(pieces)` checks unique ids, companion existence/paired mode/reciprocity,
opposite hands and equal quantities. It retains authored order/fields, with shared pieces()/piece(id)
borrows and typed actual-value refusals. Empty inventories and equal odd member quantities are legal;
labels imply no hand or partner. A member cannot name itself.

A cut-on-fold Piece declares exactly one outer-boundary fold edge; other Pieces declare none.
Cutting side and layer index are explicit. MaterialAssignment is an id or nonblank Unresolved(reason);
assigned target validation belongs to the Design Material collection. Labels require nonblank name,
size, fabric and colorway. Quantity, L/R and fold indicators derive from the cut plan:

- One mirrored-pair definition, quantity2: cut two, pair L/R.
- Reference-skirt skirt_back_right/left, quantity1 each: cut-one R/L. Either inventory order accepts;
  unequal quantities refuse, equal odd member quantities accept.
- Single front, quantity1, one boundary fold edge: cut one on fold.
- Awaiting selection can retain "awaiting fabric selection", "unassigned woven fabric" and
  "undetermined". Inspectable unresolved content does not establish production readiness.

Shared definition() access and editing a clone cannot mutate a validated Piece. edges() includes
boundary, holes and construction lines; endpoint_references() inventories traversal endpoints for
later atomic bus registration. endpoint_resolutions() exposes current point evidence without
rewriting held references. Endpoint success alone cannot certify an entire edge.

**Detached Material replacement.** `Piece::with_material_assignment(assignment)` creates an immutable
replacement while leaving the original Piece unchanged. It changes only the Material assignment;
identity, boundary/hole/construction references, quantity, pairing, fold/cutting state, layer index
and every authored label field remain exact. No label or physical property is inferred from a new
assignment. Nonblank unresolved reason text is preserved, including surrounding whitespace; blank
text returns the same PieceError::UnexplainedMaterial as initial construction.

```rust
let candidate = piece.with_material_assignment(MaterialAssignment::Assigned(material_id))?;
assert_eq!(candidate.id(), piece.id());
```

This replacement preserves a previously born Piece's reference history. After a split, its original
edge may be retired while still resolving through the journal; rebuilding it with Piece::new against
the later ledger would incorrectly ask that historical edge to be live at a new birth. Material
replacement retains the original references and the same current point/range evidence, including
visible later losses. It repairs or retargets nothing and still reports geometry deferred to G2.
It supplies no registry/source/readiness certificate: the actual Design Material collection checks
assigned targets, and the command layer still owns authorization, current validation and atomic
publication. Constructing this detached value cannot replace private bus state. AssignMaterial metadata
inspect-previews and atomic group commits are implemented under fresh command-bus authority;
see [implemented execution](command-execution.md). Recipe/geometry/physical execution remains separately owned.

[Piece/cut-plan resource accounting](model-resource-accounting.md#actual-piece-and-cut-plan-accounting)
borrows original held reference vectors and authored text without rebirth, repair or registry/geometry
validation. Exact logical entries/UTF-8 bytes and owner/field/index refusals are distinct from readiness.

**Whole intervals.** `EdgeRange::new(edge, from, to)` requires exact ascending bounds; whole(edge)
covers [0,1]. A zero-length anchor is a point. `IdentityLedger::resolve_range` partitions the entire
interval through the journal, without sampling. `RangeResolution` retains ordered `RangePortion`s:
live portions carry current edge/local bounds/Direction; lost portions retain a `RangeRepairTask`
with held range, affected interval and orphaning operation/cause. Current repair causes have no
candidate interval mapping; point ambiguity retains its separate candidates.

Split an edge into thirds and delete the middle: both original endpoints still resolve, but the
whole range returns live/repair/live and has_full_coverage() is false. A sub-range inside the first
third retains full coverage. Arbitrarily narrow offset gaps remain uncovered; exact overflow creates
a recomputation repair. A never-created edge is UnknownSource. start()/end() retain original point
resolutions, including split/shared-offset-boundary choices. Positive-length coverage settles neither
those choices nor continuity, winding, geometry or approval. Later unrelated edges cannot reattach
lost content.

`Piece::range_resolutions()` returns every full held edge with its authored DirectedEdge. Portions
run forward in the stored frame; reversed traversal consumes them backwards with reversed fragment
directions. The bus must consider interval repairs and point repairs. The existing point-registration
verdict remains narrower; full recipe/geometry/release checks remain separate. See
`decision_range-resolution-preserves-entire-interval.md`.

**Current structural report.** DesignStructure.topology_readiness() returns a borrowed
DesignTopologyReadiness over the exact immutable structural owner. It includes every ledger
registration's original choice-aware answer, then every held Piece edge in Piece/authored order
(boundary, holes, construction lines). Each DesignPieceTopology retains Piece identity, edge index,
original DirectedEdge, whole raw RangeResolution and authored traversal start/end answers.

Only a matching owner/edge/parameter registration can supply a saved endpoint choice. Another
consumer's choice at the same point never selects for the Piece. Reversed traversal swaps authored
start/end while raw interval evidence stays in its original source frame. The query never mutates
registrations, repairs, Pieces, history or revision; its report cannot outlive or transfer to a new
structural owner. Registered lookup failures propagate the original LedgerError.

is_ready() requires unique registered points, unique Piece endpoints and full held-interval coverage.
A split with no stated registered choice blocks this scope. Split a held edge into thirds and delete
the middle: both endpoints and the ledger's point-only release_readiness may still resolve/allow,
but the new report blocks with live/repair/live portions. Unknown whole sources also block even if
raw point folding returns pass-through answers. No unrelated edge replaces a missing interval.

For a merge followed by a split exactly at a held edge's endpoint, positive coverage can remain full
while that endpoint is ambiguous. An explicit choice saved for that Piece resolves the authored
endpoint; range().start()/end() still retain the raw split evidence. A choice for a different owner
leaves the Piece ambiguous. An older blocked report stays tied to its unchanged older state after explicit repair.

| API | Meaning |
| --- | --- |
| `DesignTopologyReadiness` | Borrowed scoped current registered-point/Piece interval report |
| `DesignPieceTopology` | Actual held edge, raw interval and choice-aware traversal endpoint evidence |

This report covers structural references only. Explicitly empty drafts have no blocker in this scope;
that supplies no complete Design, sewing/garment source, geometry, construction, physical or release
proof. The existing ledger verdict retains its narrower contract. See
[the public source](../../../../crates/sc-core/src/design/topology.rs).

**Owned structural state (`G1-SLICE.6.4.2.b`).** `sc_core::design::DesignStructure::new(definition)`
owns the explicit Design identity, exact DesignRevision, CanonicalInputs, PieceInventory, CutPlan and
IdentityLedger. No record, revision, copy or registration is generated. definition() and the typed
component getters return shared canonical borrows; editing a detached definition does not change state.
An explicit empty draft accepts, including revision0; a counter beyond u128 remains exact.

Owned ids are checked in deterministic order: Design, lengths, scalars, Pieces, copies, journal
operations, with each family's original order retained. DuplicateIdentity reports both
DesignStructurePosition witnesses, including retired creating operations. Source/evidence/formula,
edge-creator and registration-owner references are legitimate uses rather than additional owners.
For example, a length id aliasing a Piece id refuses; a length's source naming that Piece is retained
without claiming the source exists in the correct registry or proves its value.

The original copy definitions are revalidated against this actual Piece inventory using CutPlan's
existing rules. A plan built for quantity1 refuses beside the same Piece's quantity2 state; a
previously allowed reflected copy refuses if its Piece now requires authored-only copies. The nested
CutPlan error retains actual ids and quantities. No copy is minted to repair the disagreement.

Pieces are not reconstructed against the current ledger. An old boundary whose interior or entire
edge was deleted stays inspectable through its held references and point/range repair evidence.
Existing journal/registration choices remain exact; construction neither reattaches nor registers
references. This structural component is not the complete Design: recipe/metadata/material/garment
registries, current source truth, combined readiness, command execution and release remain later work.

**Semantic notches (`G1-SLICE.3c.3a`).** `Notch::new(definition, &piece, &ledger)` stores an
immutable semantic matching mark with a piece identity and an exact `EdgeAnchor { edge, param }`.
The anchor must be live and uniquely resolved at construction, and its resolved position must lie
in a surviving interval of that piece's boundary, hole or construction edge. A foreign live edge
is refused separately from a missing edge. After a merge, the piece may own only part of the resulting
edge; an anchor in the foreign remainder is refused, including after reversal.

For example, a notch held at `t = 2/5` survives a split at `2/5` as a `SplitPoint` answer: first
fragment at `1`, second at `0`. The consumer states `SplitSide`; the notch never silently picks one.
A reversal reports `1 − t` and accumulated direction. Deletion returns the original held anchor and
orphaning operation in a repair task. `definition()` remains unchanged throughout these edits.
New marks cannot be born on a retired source; use an owned live fragment when authoring a new mark.

The definition's `NotchProfileBindings` contains logical `ProfileParameterRef` identities for style,
sample depth/width, production depth/width and encoding. These are declarations to resolve against
the eventual target Factory Profile, not copied values or a selected profile version. Sample and
production are separate fields; they may deliberately name the same declarations. `NotchStyle` and
`NotchEncoding` describe possible profile values from §4.5; this slice implements no notch geometry
or encoder and does not widen the release feature matrix's staged style set.

Every notch reports `ProfileBindingValidation::DeferredToG4`. Even an unknown declaration identity
can be retained as symbolic content; this is no claim that a profile contains or resolves it. G4 must
check each binding's declaration, expected type, uncertainty/evidence state and artifact policy.
G1 supplies no fallback dimensions, style or encoding. The normative release §8 matrix continues to
badge an unknown notch geometry in previews, carry a sidecar in drafts and block production; none of
those outcomes permits inventing geometry. The boundary is recorded in
`decision_profile-bindings-stay-symbolic-at-g1.md`.

**Physical copies (`G1-SLICE.3c.2b.1`).** The director's D57 ruling gives every physical cut
copy its own persistent identity, so two copies of one pattern Piece can have different seam
neighbours. `CutPlan::new(definitions, pieces)` validates an explicit complete plan; it never derives
ids from ordinals or generates missing copies. Each `CutCopyDefinition` supplies a copy id, pattern
Piece id and `CopyOrientation::Authored` or `Reflected`. `copy(id)` addresses a physical copy;
`copies_for_piece(piece_id)` finds its siblings without collapsing their identities.

Copies A/B of one Piece keep distinct ids when reordered. Replacing B with C leaves B missing;
seams cannot silently transfer. SewingGraph validates supplied copies at construction and exposes
missing targets; the bus owns atomic replacements and revision checks.

Piece ids and copy ids must be unique and disjoint, every named Piece must exist, and counts must
exactly equal each Piece's cut quantity. A `MirroredPairs` request has equal authored and reflected
populations. `Single` and separate L/R `PairMember` definitions allow authored copies only: the
separate members already have their own handed geometry. For the reference skirt, the cut plan has
five individually identified authored copies of its five cut-once Piece definitions; unfolding the
front's fabric-fold contour does not create a second physical copy.

Copy orientation is independent of topological traversal direction. Copy edges use the pattern's
source frame; G2/V1 applies the physical reflection transform. `CutPlan` exposes shared immutable
copies and always reports `GeometricValidation::DeferredToG2`; it certifies no contour, placement,
cutting or release result. PieceInventory checks pair metadata; DesignStructure checks owned ids/current copies; readiness follows.
See `decision_physical-cut-copies-have-stable-identities.md` for the identity and replacement rule.

**Implementation vocabulary.** API names supplement the garment glossary; declarations confer
no physical validation.

| API token | Meaning here |
| --- | --- |
| `PieceDefinition` | Editable structural input, validated to construct a Piece |
| `PieceError` | Typed structural Piece refusal |
| `PieceInventory` | Immutable ordered Pieces with complete companion metadata checks |
| `PieceInventoryError` | Actual duplicate or invalid companion identities/values |
| `DesignStructure` | Immutable owned structural component, without full Design/readiness claims |
| `DesignStructureDefinition` | Explicit identity/revision, inputs, Pieces, copies and topology |
| `DesignStructurePosition` | Original role/index of an owned structural entity |
| `DesignStructureError` | DuplicateIdentity witnesses or nested current CutPlan refusal |
| `DirectedEdge` | Held edge identity plus authored traversal |
| `Direction` | Original or accumulated reversed traversal |
| `Single` | Piece cut mode retaining its authored handedness |
| `MirroredPairs` | One Piece requesting equal authored/reflected copies |
| `PairMember` | Separately identified L/R member with companion metadata |
| `Resolution` | Unique point, explicit split choice or repair task |
| `SplitPoint` | Both split fragments offered for one held point |
| `SplitSide` | Consumer's explicit choice of split fragment |
| `RangeResolution` | Whole-range evidence plus separate endpoint answers |
| `RangePortion` | One live or unresolved positive-length portion |
| `RangeRepairTask` | Held range, affected interval and failure cause |
| `UnknownSource` | Edge never declared in the identity journal |
| `ProfileParameterRef` | Logical target-profile declaration, without a value |
| `NotchProfileBindings` | Style, dimensions and encoding declaration references |
| `NotchStyle` | Possible profile-selected style values; no G1 geometry |
| `NotchEncoding` | Possible export forms; no G1 encoder |
| `CutCopyDefinition` | Editable physical-copy identity, Piece and orientation |
| `Reflected` | Copy orientation requiring a later geometric transform |

**Copy-addressed sewing (`G1-SLICE.3c.2b.2`).** `SewingGraph::new` validates the graph against
its complete CutPlan, Piece collection, semantic landmark registry and topology ledger. Each
`SeamSpanDefinition` supplies its id, A/B `SeamSide`s (copy id plus positive EdgeRange), an explicit
`SeamDirection`, signed `DeclaredEase` and `StopLandmark`s on explicit sides. The immutable graph
exposes shared spans, not public mutators; commands will construct and validate atomic replacements.
Ids across this supplied scope must be unique. A missing copy, landmark, source interval or owned
material portion is a typed `SewingError`, with resolution evidence retained for repair/choice cases.
InvalidCutPlan retains the original CutPlanError through `Error::source()` (D273).

For example, two physical copies of a cut-two Piece can join different neighbours while using the
same pattern-frame edge interval. A one-to-many correspondence can use two spans: A's first half to
copy B, A's second half to copy C. A folded-piece self-seam can join `[0,1/2]` and `[1/2,1]` of one
edge on one copy, if the geometry supports that construction. The shared midpoint is legal; joining
`[0,1/2]` to `[1/3,1]` on that same copy is refused because their interiors overlap. Merging edges
does not bypass this rule: disjointness is checked on resolved fragments, rather than different held
edge names. G1 does not claim that the example's fold, net line or seam length has been constructed.

`SeamDirection::Same` means A's lower endpoint meets B's lower; `Opposite` means A's lower meets B's
upper. Later ledger reversal reports accumulated traversal through each side's range result; physical
copy reflection is the CutPlan's separate orientation. None rewrites the authored correspondence.
`resolve_side()` retains whole-interval and separate endpoint evidence: a lost middle fragment remains
a repair even if both endpoints resolve, and an endpoint at a split remains an explicit choice.
Born graph ranges must have full coverage and unique endpoints. Piece ownership covers every positive
portion, so an unowned middle interval between two owned endpoints is refused too.

`EaseAmount` is an explicit signed Length, a formula parameter identity, or a logical profile binding;
positive means A is longer than B, negative means B is longer. This is seam differential, separate
from the garment's body-to-garment ease policy. `EaseDistribution::Uniform` is explicit, not a default.
`Weighted` carries an explicit side and nonempty ordered regions of normalized span arc-length progress:
bounds ascend, relative densities are positive and regions do not overlap; omitted regions receive
none of the declared allocation. `BetweenNotches` requires two distinct, noncoincident semantic notch
stops on its named side. A turn point cannot silently substitute for a notch. No G1 method evaluates
a symbolic amount, measures geometry or realizes the distribution. Formula evaluation belongs to
`.5`; profile declaration/type/state resolution belongs to G4. The command bus must validate these
references as well as geometry before accepting a complete design.

`SewingLandmark` registers existing semantic Notches or born-valid edge-anchored `TurnPoint`s.
Every stop must belong to its side's source Piece and resolved interval; duplicate stops on one side,
missing landmarks, out-of-range stops and ambiguous/orphaned anchors have separate refusals. Physical
notch geometry is irrelevant to locating a semantic stop. TurnPoint asserts a sewing landmark; its
actual geometric turning is still unproved. The shared `AnchorError` vocabulary keeps `NotchError`
as a compatible alias for the existing notch construction API.

After an explicit plan/landmark replacement, `missing_copies()` and `missing_landmarks()` expose the
original targets still held by the graph. A new copy does not inherit an old copy's seams. Queries do
not mutate stored content or certify release. Every graph reports `GeometricValidation::DeferredToG2`
and `EaseValidation::DeferredToG2AndG3`; `walk`/`true`, actual length differential, source transforms,
net-line geometry and assembly completeness remain later checks.

[Sewing resource accounting](model-resource-accounting.md#actual-sewing-accounting) borrows held spans,
weighted regions, stop lists and semantic landmarks without resolving/rebirthing anchors or realizing ease.
Logical counts and original graph/span/index refusals remain separate from readiness.

SewingGraph.validate_current(plan, pieces, landmarks, ledger) applies the constructor's exact rules
again, in the same order, to its original immutable spans. Construction success and missing-id
inventories cannot authorize replacement contexts. Copy200 may still exist but now name another
Piece; the original held range then refuses with RangeOutsidePiece. Landmark400 may still exist
but have moved to the other side's Piece; its stop refuses with StopOutsideSide. A deleted middle
returns the original UnresolvedRange with live/repair/live evidence even when both endpoints resolve.
No graph, reference, registration or amount is rewritten/evaluated; geometry and symbolic declaration
resolution remain separate. The constructor and current query share one actual validator.

An empty graph is structurally legal
but makes no garment-assembly completeness claim. The contract is recorded in
`decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md`.

| API token | Meaning in the sewing implementation |
| --- | --- |
| `AnchorError` | Shared born-valid semantic-anchor refusal |
| `NotchError` | Compatible alias of the shared anchor refusal |
| `TurnPoint` | Immutable semantic edge-anchored sewing turn |
| `SewingLandmark` | Registry entry for a Notch or TurnPoint |
| `SeamSide` | Physical-copy id and source-frame EdgeRange |
| `SeamSideId` | Explicit A/B side selector |
| `SeamDirection` | Authored endpoint correspondence |
| `Opposite` | A lower endpoint meets B upper endpoint |
| `SeamSpanDefinition` | Editable oriented sewing-span input |
| `SewingGraphDefinition` | Editable graph id and span list |
| `SewingError` | Typed structural graph refusal |
| `DeclaredEase` | Signed amount source and allocation intent |
| `EaseAmount` | Explicit, formula or profile differential source |
| `EaseDistribution` | Uniform, weighted or between-notches allocation |
| `Uniform` | Explicit uniform allocation declaration |
| `Weighted` | Ordered positive-weight region declaration |
| `BetweenNotches` | Allocation between two named notch stops |
| `StopLandmark` | Semantic stop identity on an explicit side |

**Directed grainlines (`G1-SLICE.3c.3b`).** `Grainline::new` retains immutable arrow, explicit
alignment and independent optional stripe/plaid ranges against a named Piece/ledger. Complete owned
positive intervals/unique endpoints are required; unknown, repaired, ambiguous or foreign sources
return field-specific refusals. References prove no straightness or angular alignment.

`ParallelTo` states codirection; `AtAngle` carries directed reference/`GrainAngle`: authored Angle or
formula/profile declaration. 45° bias/180° antiparallel are intent; symbolic ids supply no value.
Present Profile angles report DeferredToG4; None means no such field. G2 consumes resolved values
and proves straightness/angle; every grain retains `GeometricValidation::DeferredToG2`.

A reversed arrow traverses1→0; after split, `portions()` visits second fragment first and composes
journal reversal. Reversal reflects point parameters/toggles traversal without rewriting the arrow.
Lost interiors retain ordered repairs despite live ends. `evidence()` keeps raw intervals/separate
points and split choices; directed `start()`/`end()` select authored endpoints, never a split side.

Stripe/plaid are independently directed/validated; omission is explicit and coincidence legal without
angular/print proof. Full ownership rejects reversed merged foreign remainders; see ontology decision.

**Current grain (`G1-SLICE.6.4.5.4.c`).** `validate_current(piece, ledger)` requires the held Piece,
then constructor rules in arrow/alignment/stripe/plaid order. Historical split/reversal never rewrites
or retargets. Later plaid loss retains role/raw evidence despite valid earlier sources/live ends.
Angles stay unread; success grants no angular, physical or whole Design readiness.

| API token | Grainline meaning |
| --- | --- |
| `DirectedRange` | Held positive interval plus authored traversal |
| `DirectedRangeResolution` | Raw range evidence plus directed traversal |
| `GrainlineDefinition` | Editable arrow, alignment and optional print references |
| `GrainlineError` | Held-owner or field-specific source refusal |
| `GrainAlignment` | Explicit codirection or angle intent |
| `GrainAngle` | Explicit, formula or profile angle source |
| `ParallelTo` | Explicit codirected alignment intent |
| `AtAngle` | Declared angle from directed reference to arrow |
| `DeferredToG4` | Profile identity without resolved value validation |

**Per-edge allowances (`G1-SLICE.3c.3c`).** `SeamAllowance::new` retains named Piece/whole edge,
width origin, explicit corner and required logical inclusion. Unknown/orphaned/ambiguous/foreign
intervals refuse; owned ends cannot conceal merged foreign middles. `edge_resolution()` retains raw
split/merge/reversal/deletion evidence; held descriptors never reattach to replacement edges.

Explicit width retains parameter id/nonnegative Length; zero implies neither absence nor default.
Fixture fold/fused interfacing allowances remain absent. Formula/profile widths retain declarations
without values/copied uncertainty. Distinct side seams may share10 000 µm while the hem names30 000 µm
and Envelope; ids/corners stay per edge. Miter/slant/envelope/trim/step remain intent; offsets are G2.

Required ProfileParameterRef inclusion permits `IncludedInContour`/`GeneratedDownstream`; target
profiles may resolve one declaration differently. G1 supplies no policy/cache/global boolean/contour.
Every allowance retains `ProfileBindingValidation::DeferredToG4`/`GeometricValidation::DeferredToG2`.
.5/.6/G4 checks parameter existence/kind/value/state; G2-2D.2/.3 owns bounded offsets/corners, logged
repairs/pathologies. Release retains preview badges, draft sidecars and production blocks for unknown
ownership/width. Unread bindings never default; five-piece fixture/goldens remain unchanged.

**Current allowances (`G1-SLICE.6.4.5.4.c`).** `validate_current(piece, ledger)` requires held Piece,
then original width/whole-edge rules. Lost middles retain complete repairs despite live ends.
Width/corner/inclusion stays held/unresolved; identical geometry on another Piece cannot substitute.

```rust,ignore
grain.validate_current(&piece, &ledger)?;
allowance.validate_current(&piece, &ledger)?;
```

| API token | Allowance meaning |
| --- | --- |
| `AllowanceWidth` | Explicit parameter/value, formula declaration or profile declaration |
| `CornerTreatment` | Authored miter/slant/envelope/trim/step intent |
| `AllowanceInclusion` | Target-profile vocabulary without a G1 selection |
| `IncludedInContour` | Resolved policy includes allowance in contour |
| `GeneratedDownstream` | Resolved policy delegates generation to receiver |
| `SeamAllowanceDefinition` | Editable per-edge descriptor input |
| `SeamAllowanceError` | Held-owner, negative-width or whole-source refusal |

[Structural object-family review](ontology-review.md) records the completed scope and remaining proofs.
