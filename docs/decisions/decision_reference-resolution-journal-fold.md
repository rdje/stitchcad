# Reference resolution is a fold over an append-only edit journal; repair state is derived, never stored

- **Type:** `decision`
- **Date:** `2026-10-01` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `G1-SLICE.3b` — how the persistent-identity contract
  (`docs/book/src/spec/ontology.md` §1.1) is realized in `sc-core` (`ontology::topology`).

answers: "how do references survive edits?" · "does an edit rewrite stored references?" · "are repair tasks stored state or a derived view?" · "what does the offset contract consume before geometry exists?" · "how does undo relate to the identity journal?" · "who mints a fragment's identity?"

## The fact / decision

`sc-core` realizes ontology §1.1 as an **append-only journal of typed topology edits** (declare, split,
merge, reverse, delete, offset-fragment) held by an `IdentityLedger`. **A stored reference is never
rewritten by an edit**: an `EdgeRef` plus `Param` a consumer holds stays byte-identical for its lifetime,
and resolution is a **pure fold of the journal** — the same reference over the same journal always resolves
the same way. Fragments and merged edges are minted by the editing operation's own id through the injected
`IdGenerator` (creator + `LocalTag`, per §1), and validation precedes minting, so a rejected edit consumes
no identity. **Unresolved state is a derived view** (`resolve`, `open_repairs`, `release_readiness`), never
stored mutation state: the journal plus the registration set are the only truth, and a `RepairTask` is what
a fold *returns*, not a record an edit writes. The offset-fragmentation contract consumes **declared
source-parameter intervals** at G1 — which fragment covers which `[from, to]` of the source edge — because
interval membership and the primitive's declared affine parameter law are exact structural operations.
G2 must justify any geometric correspondence separately. Arc lengths enter merge as caller-declared
`sc-units::Length` evidence retained in the event and used by exact `Rational` arithmetic;the ledger
does not measure curves and rounds nothing.

## Why

1. **Rewriting stored references at edit time is the silent-reassignment risk §1.1 forbids.** Every consumer
   of an edge (notches, spans, grainlines, grade points — `.3c`'s objects) would have to be found and mutated
   atomically at every edit, and a missed consumer is silent corruption with no diagnostic. A fold cannot miss
   a consumer: resolution is computed *from the reference the consumer holds*, on demand.
2. **The journal is the topology half of what the recipe already is.** Ontology §3.1: the design is "the
   ordered operation list — the drafting history, replayable from scratch". With the injected generator
   (`decision_entity-identity-ulid-injected-generator.md`), replaying the journal reproduces every fragment
   identity byte-for-byte — `.3a`'s replay property, extended from ids to topology.
3. **Derived repair state cannot drift.** A stored "unresolved" flag is a hand-kept copy of what the journal
   already answers, and copies of derivable state are this repository's most-measured defect class (D34/D38:
   a number restated in prose whose producer is a human reading). `release_readiness()` is recomputed, never
   remembered, so "savable and inspectable, but not releasable" is a property of the data, not of a flag
   somebody forgot to update.
4. **Undo becomes journal algebra, not consumer archaeology.** `G1-SLICE.6`'s command bus inherits this: an
   undo is an inverse edit appended (or a truncation before persistence), and because no consumer's stored
   reference was ever rewritten, there is nothing to un-rewrite — undo restores semantics by construction.
5. **Intervals keep the offset contract testable at the G1/G2 boundary the invariants record already drew.**
   `decision_ontology-invariants-structural-g1-geometric-g2.md` splits structural (G1) from geometric (G2);
   "the mapping is unambiguous" is structural — exactly one interval contains `t` — while *which* intervals
   an offset produces is geometric. G1 tests the whole ambiguity/refusal machinery against declared intervals;
   G2 supplies real source domains and must justify the chosen parameter law against actual geometry.
   Interval endpoints alone do not establish general normal-offset arclength correspondence.

## How to apply

- Mutations enter only through `IdentityLedger`'s validated API; the journal is private and append-only
  (deserialization arrives with `sc-store` at `G1-SLICE.7`, behind the same validation — a journal is built,
  never pasted).
- Consumers (`.3c` objects) hold `(owner, EdgeRef, Param)` registrations and **never mutate them under an
  edit**; they resolve at read time and state a `SplitSide` explicitly where a split point offers both
  fragments. A reference is born resolved: registering one that does not resolve is a typed refusal
  (`BornUnresolved`), because §1.1 governs edits that orphan references, not creation.
- Any "helpful" automatic pick of a fragment for an ambiguous or orphaned reference violates §1.1 ("no silent
  reassignment"): ambiguity is a `Resolution` a consumer must choose through, and an orphan is a `RepairTask`
  naming the reference, the orphaning edit and the candidates — discharged only by an explicit `repoint` or
  `retire`.
- Exactness is non-negotiable: every recomputation (`t/s`, `(t−s)/(1−s)`, `t·L₁/(L₁+L₂)`, `1−t`,
  `(t−from)/(to−from)`) is `Rational` arithmetic; a result that does not fit is a typed diagnostic that
  itself becomes a visible `RepairTask` (`RecomputationFailed`), never a rounded fallback.
- Related: [[decision_entity-identity-ulid-injected-generator]] (the ids the journal mints),
  [[decision_edge-parameter-bounded-exact-rational]] (the parameter the fold recomputes),
  [[decision_ontology-invariants-structural-g1-geometric-g2]] (the G1/G2 boundary the intervals sit on).

## Scope/storage correction0696

answers: "does the identity journal store merge lengths?" · "does interval membership prove a normal-offset point map?"

D373 checked the original a6d465f9/0005 source:Merge already retained `first_length` and
`second_length`. The former no-length-storage sentence omitted the distinction between supplied
event evidence and computed geometry. The original affine primitive remains unchanged;the
[regular curved-offset counterexample](../book/src/annexes/drafting-output-contracts.md#offset-output-contract-recovery)
shows why it cannot be promoted to a general normal-offset certificate.

Destructive offset-fragmentation and nonretiring derived allowance declaration are different
operations. Canonical explicit-tag declarations can record an already identified operation without
minting a new EntityId or retiring net geometry. This clarification changes neither stored-reference
immutability nor the primitive's exact ambiguity/gap/repair semantics. Actual derived mapping and
role continuity remain separately owned contracts and G2/G3 evidence.

## Declared source eligibility0706

answers: "does slash source eligibility prove a current mapped reference?" · "can both bridge endpoints share an edge id?"

Native slash eligibility queries read exact declared source frames and retain all reference
alternatives. A retained identity contributes once with every original use available as evidence;
source candidates retain explicit tags. A bridge contributes distinct stationary/moving Start/End
occurrences at its original cut bound. Both share an EdgeRef but are different position alternatives;
an EdgeRef-only deduplication cannot establish unique point correspondence.

Unique/Ambiguous/Gap classify those declarations only. No output Param,journal fold,implicit alias,
physical point equivalence,canonical retirement or repair discharge follows. Later ledger changes
leave the original metadata inspectable while undeclared child aliases refuse. Actual mapping,
current frame/liveness,trim/source coverage and authorized repair/publication remain separate owners.

## Local plans and global/owner repair scopes0707

answers: "can a piece-local result authorize global source-edge retirement?" · "does a registration repair update the held semantic object?"

A topology edge edit is global to that ledger's reference fold. Registration retire/repoint instead
addresses an exact owner/edge/parameter key;neither operation supplies piece-local geometry authority.
Native public-interface cases show one deletion affecting two registered owners while explicit
registration repairs affect only their selected owner. Source absence in one Piece must not infer
global retirement or consent to withdraw registrations.

Global liveness and point resolution do not establish semantic Piece membership. A replacement
contour can exclude an old anchor while another Piece still uses its live source edge. Repointing
one registration does not mutate an immutable Notch holding the old anchor. Disposition must retain
whole source/consumer coverage,actual mapping and coordinated semantic repair/publication obligations.
The scoped ledger readiness verdict remains valid in its documented narrower domain;this is no
ledger algorithm defect or geometry/currentness certificate.
