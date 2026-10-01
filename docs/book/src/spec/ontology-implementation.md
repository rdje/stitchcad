# Executable ontology at G1

The implementation companion to the [garment ontology](ontology.md). The normative object clauses
remain there; this chapter describes the running structural API, examples, repair evidence and
explicit deferred obligations. Geometry, realized ease and resolved profile values are not G1 claims.

`sc_core::ontology::piece` implements §4.1's **structural** content (`G1-SLICE.3c.1`).
`Piece::new(PieceDefinition, &IdentityLedger)` returns an immutable `Piece` or a typed `PieceError`
naming the failed invariant. `PieceDefinition` is editable input; it is not a validated domain object.
A command replaces a piece through this constructor rather than modifying its validated fields.

The boundary and each hole are cyclic sequences of `DirectedEdge { edge, direction }`. The last edge
implicitly precedes the first; **do not repeat the first edge at the end**. Empty loops, repeated cut
edges (including an edge shared by two cut loops), and references absent from the current ledger are
rejected. Construction lines must also name live edges. A cyclic list does not prove that geometric
endpoints coincide: even a one-edge curve loop still requires G2's closure check.

The cut quantity is the **total number of physical copies**. `Mirroring::Single` accepts any positive
quantity; `Mirroring::MirroredPairs` requires an even quantity (two means one L/R pair within one
piece definition). Separate cut-once L/R members use `Mirroring::PairMember { handedness, companion }`:
quantity counts this member's copies, the handedness is explicitly left or right, and the companion is
another Piece identity. This is the reference skirt §6's back-piece form. A member cannot name itself
as its companion; companion existence, reciprocity, opposite hands and equal quantities are checked
at design-collection validation (`G1-SLICE.6`), not guessed by the standalone constructor. Actual
geometric mirroring stays G2's obligation. A cut-on-fold
piece declares exactly one fold edge on its outer boundary; other pieces declare none. The fabric
side and assembly layer index are explicit content. Material assignment is either a material id or
`Unresolved(reason)` with a nonblank explanation: an unresolved assignment is not defaulted to fabric.
The design will validate material identities against its material collection when that collection lands.

Printed labels carry nonblank name, size, fabric and colorway text. Their cut quantity, pair L/R and
fold indicator derive from the cut plan rather than being separately editable inputs. For example:

- One mirrored-pair definition with quantity `2` prints cut two, pair L/R.
- Separate `skirt_back_right` and `skirt_back_left` definitions with quantity `1` each carry opposite
  handedness and reciprocal companion ids, and print their own cut-one R / cut-one L labels.
- A single front on fold with quantity `1` and one boundary fold edge prints cut one on fold.
- A material awaiting selection may carry the explicit reason "awaiting fabric selection" and print
  "unassigned woven fabric" / "undetermined" for fabric and colorway. This is inspectable content,
  not evidence that a production release is acceptable.

`definition()` exposes authored content by shared reference; editing a clone does not change the
validated piece. `edges()` includes boundary, hole and construction-line references. After a ledger
edit, `endpoint_resolutions()` resolves the held traversal endpoints and exposes `Resolution` values,
including repair tasks; it never rewrites the authored references. `endpoint_references()` supplies
an endpoint inventory for the command bus's future atomic registration step. Endpoint queries **do not
certify complete-edge integrity**: deleting an interior split fragment can leave both original endpoints
resolvable. Whole-interval evidence now answers that separate question, as described below. Full
recipe/geometry validation remains required before release.

Every piece reports `GeometricValidation::DeferredToG2`. This is deliberately **not** a valid-geometry
badge. Endpoint closure, CCW outer winding, opposite hole winding, simplicity, holes strictly inside
and non-intersecting, and intake conservation remain `G2-2D.1`'s obligations. G1 has no constructor
that marks these checks passed; this implements the boundary recorded in
`decision_ontology-invariants-structural-g1-geometric-g2.md`.

**Whole-interval queries (`G1-SLICE.3c.2a`).** `EdgeRange::new(edge, from, to)` requires exact
ascending bounds `from < to`; `EdgeRange::whole(edge)` covers `[0,1]`. A zero-length anchor is a
point reference. `IdentityLedger::resolve_range` partitions the entire interval through each journal
edit, with no grid or endpoint sampling. Its `RangeResolution` retains every `RangePortion`, in the
held edge's original forward traversal: live portions carry the current edge, local bounds and
`Direction`; lost portions carry a `RangeRepairTask` naming the original held range, affected local
interval and the orphaning operation/cause. Existing causes have no candidate interval mapping;
endpoint ambiguity carries its candidates in the separate point-resolution answer.

For example, split a boundary edge into three equal fragments and delete the middle. Resolving the
original endpoints still succeeds, but resolving the entire edge returns live first fragment, a
repair for the deleted middle fragment, live last fragment. `has_full_coverage()` is false. The same
query on a sub-range entirely inside the first surviving third has full coverage. An offset gap,
however narrow, likewise becomes an explicit uncovered interval; exact-arithmetic overflow becomes a
recomputation repair, never an approximation. An edge never created in the ledger is `UnknownSource`.

`start()` and `end()` on the range result are the original endpoint **point** resolutions. A bound
exactly at a split point can still offer both sides; a bound on a shared offset boundary can still be
ambiguous. Full positive-length coverage does not resolve those choices. It also does not prove
continuity, winding or geometric validity, and grants no approval. A repair remains visible after
later edits: a surviving endpoint or new unrelated edge cannot silently reattach lost interval content.

`Piece::range_resolutions()` supplies this full-edge evidence for boundary, holes and construction
lines, alongside each authored `DirectedEdge`. Range portions are ordered forward in the stored edge's
frame; a reversed authored traversal consumes that sequence backwards and reverses the fragment
directions. The future command bus must consider range repairs **as well as** point repairs. The
existing point-only registration verdict keeps its narrower meaning until design-level registration
and validation land. The interval contract and its limits are recorded in
`decision_range-resolution-preserves-entire-interval.md`.

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

For a single pattern Piece requesting two authored copies, supply ids A and B with the
same Piece id. Reordering A/B to B/A retains each identity and orientation.
Replacing B with a newly identified C leaves B absent; its seams must not silently
transfer to C. The sewing-graph slice next integrates this addressing contract and exposes
missing copy references; the command bus later owns atomic replacements and revision checks.

Piece ids and copy ids must be unique and disjoint, every named Piece must exist, and counts must
exactly equal each Piece's cut quantity. A `MirroredPairs` request has equal authored and reflected
populations. `Single` and separate L/R `PairMember` definitions allow authored copies only: the
separate members already have their own handed geometry. For the reference skirt, the cut plan has
five individually identified authored copies of its five cut-once Piece definitions; unfolding the
front's fabric-fold contour does not create a second physical copy.

Copy orientation is independent of topological traversal direction. Copy edges use the pattern's
source frame; G2/V1 applies the physical reflection transform. `CutPlan` exposes shared immutable
copies and always reports `GeometricValidation::DeferredToG2`; it certifies no contour, placement,
cutting or release result. Companion reciprocity and design-wide identity checks remain `.6` duties.
See `decision_physical-cut-copies-have-stable-identities.md` for the identity and replacement rule.

**Implementation vocabulary.** These are API names used in the executable examples above,
distinct from the garment concepts the glossary defines. The table declares their local meaning;
it supplies no new validation or physical-output claim.

| API token | Meaning here |
| --- | --- |
| `PieceDefinition` | Editable structural input, validated to construct a Piece |
| `PieceError` | Typed structural Piece refusal |
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
net-line geometry and assembly completeness remain later checks. An empty graph is structurally legal
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

**Directed grainlines (`G1-SLICE.3c.3b`).** `Grainline::new` takes immutable semantic input,
a named Piece and the identity ledger. `GrainlineDefinition` carries a directed arrow range,
explicit `GrainAlignment` and independent optional stripe/plaid ranges. Every reference must resolve
with complete owned positive-length coverage and unique endpoints; unknown edges, interval repairs,
ambiguous endpoints and foreign portions are typed `GrainlineError`s naming the affected field.
G1 does not prove that a held range is a straight line or satisfies its angular intent.

`ParallelTo` states codirection. `AtAngle` carries an explicit directed reference plus `GrainAngle`:
an authored Angle, formula declaration or profile declaration. A 45° angle expresses bias and 180°
expresses antiparallel intent without collapsing a directed arrow to an undirected axis. Formula and
profile identities supply no angle values here. Present profile bindings report `DeferredToG4`;
absence of such a binding means only that the definition has no profile angle field. G2 must consume
resolved values and prove the actual angular relation/straightness. Every grainline reports
`GeometricValidation::DeferredToG2`.

For example, an authored reversed arrow along a source edge starts at parameter 1 and ends at 0.
After that edge is split, `DirectedRangeResolution::portions()` traverses the second fragment before
the first and composes the authored reversal with each fragment's journal direction. Reversing the
source edge reflects point parameters and toggles that composed traversal, preserving the held arrow.
Deleting an interior fragment retains a repair in its proper traversal position; neither surviving
endpoint can hide it. `evidence()` retains the unmodified whole-range and separate point answers,
including split choices. Directed `start()`/`end()` select the authored endpoints, without picking a
split side. The definition is never automatically rewritten.

Stripe and plaid references are independent fields, each directed and validated. Omitting either is
explicit. Deliberately using coincident references is legal authored content, not evidence of angular
or print alignment. The constructor shares sewing's whole-interval ownership check, so a merged
foreign remainder is refused after reversal too. The structural/geometric grain boundary is recorded
in `decision_ontology-invariants-structural-g1-geometric-g2.md`.

| API token | Meaning in the grainline implementation |
| --- | --- |
| `DirectedRange` | Held positive interval plus authored traversal |
| `DirectedRangeResolution` | Raw range evidence plus directed traversal view |
| `GrainlineDefinition` | Editable arrow, alignment and optional print references |
| `GrainlineError` | Field-specific unresolved/foreign-reference refusal |
| `GrainAlignment` | Explicit codirection or angle intent |
| `GrainAngle` | Explicit, formula or profile angle source |
| `ParallelTo` | Explicit codirected alignment intent |
| `AtAngle` | Declared angle from a directed reference to the arrow |
| `DeferredToG4` | Profile declaration identity held without resolved value validation |

**Per-edge allowance descriptors (`G1-SLICE.3c.3c`).** `SeamAllowance::new` holds a source
Piece's whole edge, width origin, explicit corner and mandatory logical inclusion declaration. Unknown,
orphaned or ambiguous references and foreign current intervals are typed refusals. Whole-interval
coverage matters: after a merge, owned endpoints cannot conceal a foreign middle segment. Split,
merge, reversal and deletion queries expose the raw interval/endpoint evidence and never rewrite the
held descriptor or attach it to a replacement edge.

An explicit width carries both its authored parameter id and nonnegative Length. An authored zero is
legal explicit content, not an inferred absence or a fallback; the fixture's fold and fused interfacing
retain their absent allowance descriptors. Formula/profile width sources carry only declaration ids,
without evaluated values or copied uncertainty states. For example, distinct side-seam edges may share
one 10 000 µm width parameter while the hem names a 30 000 µm source and an envelope corner. Each
edge keeps its own descriptor identity and corner choice. Miter, slant, envelope, trim and step are
intent here; none constructs an offset until G2.

Inclusion carries a mandatory ProfileParameterRef, with possible resolved policy vocabulary
`IncludedInContour` and `GeneratedDownstream`. The design can retain one logical declaration while
two target profiles resolve it differently. G1 returns no resolved policy, cached flag, global boolean
or exported contour. Every descriptor reports `ProfileBindingValidation::DeferredToG4` and
`GeometricValidation::DeferredToG2`. `.5`/`.6` and G4 validate parameter existence, kinds, values and
states; G2-2D.2/.3 owns bounded offsets, corner geometry, logged topology repair and pathology tests.
The release matrix still gives unknown ownership/width a preview badge, draft sidecar and production
block; no unread binding produces a default. The fixture's five pieces and goldens remain unchanged.

| API token | Meaning in the allowance implementation |
| --- | --- |
| `AllowanceWidth` | Explicit authored parameter/value, formula declaration or profile declaration |
| `CornerTreatment` | Authored miter/slant/envelope/trim/step intent |
| `AllowanceInclusion` | Possible target-profile inclusion vocabulary, without a G1 selection |
| `IncludedInContour` | Resolved policy includes allowance in the contour |
| `GeneratedDownstream` | Resolved policy delegates allowance generation to the receiver |
| `SeamAllowanceDefinition` | Editable per-edge descriptor input |
| `SeamAllowanceError` | Typed negative-width or unresolved/foreign-edge refusal |
