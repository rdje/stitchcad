# G1 enforces the ontology's structural invariants; the geometric ones are G2-2D.1's

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `G1-SLICE.3c` — the boundary between what the ontology proves
  at gate G1 and what `sc-geometry` proves at gate G2 (`docs/book/src/spec/ontology.md` §4.1, §9;
  `docs/tasks/G2-2D.md` leaf `.1`).

answers: "how do Pocket components retain explicit physical-copy and source-Piece references?" · "can G1 build an invalid piece?" · "are piece invariants enforced at G1?" · "who checks CCW winding and piece closure?" · "what does 'an invalid piece cannot be built' mean at G1?" ·
  "are geometric invariants deferred or dropped?" · "does G1 claim 2D correctness?" · "how are directed grainline and stripe/plaid references represented?" · "can a dart apex reference interior geometry?" ·
  "how do tucks and pleats retain distinct semantic kinds?" · "can lining be modelled while remaining outside v1 execution scope?" · "how does a faced hem bind a current Facing without duplicating it?"

## The fact / decision

At gate **G1**, `sc-core`'s ontology types enforce the invariants that need **no 2D predicate**: a `Piece`'s
boundary is a non-empty closed loop of distinct `EdgeRef`s, holes and construction lines reference edges that
exist, multiplicity ≥ 1, a cut-on-fold piece declares exactly one fold edge, label data is complete enough to
print, a `SeamSpan` references edges that exist, and every reference either resolves or becomes a visible
`RepairTask`. The **geometric** invariants — CCW winding, boundary simplicity, holes strictly inside and
non-intersecting, piece closure, dart-intake conservation against real boundary length — are enforced at gate
**G2** by `sc-geometry`'s robust predicates (`G2-2D.1` owns "winding and orientation rules, piece closure").
G1 makes **no 2D-correctness claim**; the geometric obligations are a typed, **visible deferral**, not a silent
omission.

## Why

1. **The predicates and the offset engine are G2 deliverables.** Roadmap §4.2 puts robust intersection
   predicates (Shewchuk-style) and the offset pathology corpus at G2; `G2-2D.1`'s goal is the "`sc-geometry` 2D
   kernel … winding and orientation rules, piece closure". G1 is the "executable architecture slice": the
   ontology types exist, carry identity and evaluate — the 2D correctness *proof* is explicitly G2's gate.
2. **Claiming otherwise at G1 would be over-claiming.** A piece with a self-intersecting boundary or a CW
   winding is *structurally* well-formed and only *geometrically* invalid. Without predicates, "an invalid
   piece cannot be built" can honestly mean only "a structurally invalid piece cannot be built".
3. **The deferral must be visible, not silent.** The same discipline the whole repository runs on: an unmet
   obligation is a named, typed state (a `GeometricValidation` that reads `DeferredToG2` until `sc-geometry`
   lands), so a reader sees exactly what G1 did and did not prove.

## How to apply

- `G1-SLICE.3c`'s object types enforce the **structural** invariants at construction and return typed
  diagnostics naming the invariant violated (ontology §9: "the diagnostic names the invariant").
- Geometric validity is carried as an unverified/deferred state at G1 and **discharged by `G2-2D.1`** when
  `sc-geometry` provides predicates; the G2 tests assert winding, simplicity, closure and intake conservation.
- Never mark a piece geometrically valid at G1, and never describe G1 as proving 2D correctness. The acceptance
  "an invalid piece cannot be built" is met at G1 for the **structural** class and explicitly handed to G2 for
  the geometric class.
- Related: [[decision_edge-parameter-bounded-exact-rational]] (the parameter the references carry),
  [[decision_entity-identity-ulid-injected-generator]] (the identity the references address).

## Directed grain references (`G1-SLICE.3c.3b`)

A grainline's semantic arrow, its alignment reference and independent optional stripe/plaid
references are directed positive EdgeRanges in their source Piece's frame. G1 validates complete
owned intervals and unique endpoints; G2 proves actual straightness, direction and angular relation.
Opposite authored direction is distinct; resolving composes it with journal reversal and reverses
fragment order where needed. Raw interval repairs and endpoint choices remain available unchanged.

Parallel declares codirection explicitly. AtAngle carries an authored Angle or formula/profile
parameter identity expected to resolve to an angle. Bias at 45° and opposite direction at 180° are
expressible without treating a grainline as an undirected axis. Symbolic inputs never supply values
or defaults; recipe/profile owners validate their declarations and G2 consumes resolved values.
There is no G1 geometric-equality or fabric-placement certificate.

## Dart intent (`G1-SLICE.3c.4a.1`)

A Dart's apex is an EdgeAnchor on Piece-owned internal construction geometry; it need not be a cut
boundary point. Two directed leg ranges and an explicit directed reference retain geometric intent.
G1 validates complete owned intervals, unique endpoints, the born-valid apex and distinct held leg
intervals. G2 proves actual leg/apex coincidence and physical straightness; G2/G3 proves intake
conservation under the closing operation. An explicit intake retains its parameter id and nonnegative
Length; formula/profile declarations retain identities without defaults or copied states. The closing
operation is a required identity, with existence/kind/dependency validation owned by `.5`/`.6`, not
an executed or certified closure. Post-edit queries preserve raw repairs/choices and held content.

## Tuck and pleat intent (`G1-SLICE.3c.4a.2a`)

Separate immutable Tuck/Pleat types share structural input/validation. Each holds a nonempty list
of directed fold ranges, intake provenance, an explicit directed folding reference and a closing
operation identity. Exact duplicate held intervals are refused regardless of traversal direction;
G1 validates full owned interval coverage and endpoint uniqueness. No physical pleat-count rule,
actual fold shape, coincidence or conserved intake is inferred from a reference list. G2/G3 executes
and verifies the typed closing operation; recipe/Design validates its identity, kind and dependencies.

## Served layer intent (`G1-SLICE.3c.4b.1`)

Facing/Lining/Interfacing are distinct immutable semantic types. Each names the served Piece, a
recipe offset operation and nonempty directed owned source ranges, plus explicit material assignment.
The recipe owns offset dimensions/input states; descriptors cache neither generated contours nor a
second output-Piece material state. G1 validates identity/scope, complete intervals, unique endpoints
and nonblank unresolved-material reasons; G2 proves actual offsets and shape/material relationships.

Feature matrix rule 3 separates modelled content from supported execution. Lining is structurally
representable, but `require_in_scope()` returns env_lining with served Piece and gate G7. Facing and
Interfacing pass that envelope check only, without geometry/release certification. G1-SLICE.6 must
apply the refusal before requested construction execution; inspection may retain modelled content.

## Hem intent (`G1-SLICE.3c.4b.2`)

Hem retains a whole owned finish edge, explicit depth provenance and a required logical fold-type
binding. Fold declarations/target-profile bindings own their domains/states; G1 does not invent a
physical fold vocabulary or cache solved values. Turned/Faced is a separate authored method; a faced
method names an existing Facing serving the same Piece. Birth and current-target queries validate
that stable identity, served owner and current layer sources. Missing, reassigned or invalid targets
are typed refusals rather than substitutes. Geometry and executed folds remain G2/G3 obligations;
recipe/Design still validates all declaration, operation and material registries.

## Pocket placement/composition (`G1-SLICE.3c.4d.1`)

PocketPieceRef explicitly names physical copy and source Piece for the served target and each
component. G1 requires nonempty distinct component copies, existing unambiguous current targets,
matching copy/source bindings, live owned birth anchoring and complete owned orientation. Current
validation follows historical anchors while retaining choices/repairs. The served target may also
be an explicit component; recipe dependency cycles and physical pocket shape belong `.5`/G3.
Opening type retains a logical recipe/Profile declaration with DeferredToG3 resolution/scope.
G1 invents no opening vocabulary or support approval. Component queries borrow current Piece
metadata; Design/G2 must inspect all component contour repairs and geometry before execution/release.
Known envelope diagnostics remain execution obligations, never silent approximation of an opening.
