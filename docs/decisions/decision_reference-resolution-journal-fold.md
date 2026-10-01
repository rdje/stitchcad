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
the identity layer decides mapping and ambiguity exactly while the geometry that produces the intervals is
`G2-2D`'s offset engine. Arc lengths enter only at merge, as caller-declared `sc-units::Length` evidence
recomputed with exact `Rational` arithmetic; the ledger stores no lengths and rounds nothing.

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
   G2 supplies real ones and inherits the contract unchanged.

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
