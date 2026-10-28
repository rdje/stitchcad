# Sealed engineering continuity — part624

- **Identity:** 106 lines, 9365 bytes, sha256:cacf647ef0e1e671f327af7a69d4fff936545651176e0236760a87e506c517d6
- **Coverage:** whole0470 authoring scope/lesson/ledger and whole0467–0470 resource decisions from 71595c400486c84c9ca195d460cac251efcc2e76.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 52666–54601, 1935 bytes, sha256:560aa982dd70d08c1b8e7c585ed739cbaf4e97394d79d40a9529cb9071197c4e
- Whole source interval: DEV_NOTES.md bytes 193–1139, 946 bytes, sha256:1fb491e5208640972595986fae6343b38798086ce6dc2241c471131c3676e60b
- Whole source interval: CHANGELOG.md bytes 207–930, 723 bytes, sha256:39c69f0d229bf49f8f24f6859faaa14cda955ba86b03ff170abe2c3bab7720c3
- Whole source interval: docs/decisions/decision_command-layer-contract-and-undo-granularity.md bytes 8719–14480, 5761 bytes, sha256:0e7c1554fcb4e00acfa159f9da2528b72e2d0092a70b8f4112751b066c184144
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.2.2.4.3.4.1`
  Status: `done`
  Goal: exact authoring retained projection from actual bounded preparation before publication-counter/
  final-model/audit/history construction; include maximum-issued/sequence, evictions and redo discard.
  - [x] REPRODUCE / ISSUE: python3 -I -B native authoring-order/source check rc=0:actual bounded prep
    precedes borrowed final composition; all successors/final copies/fresh grant/effect follow admission.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part623.md:
    3 exact Git spans rc=0; whole0469 retained. Temporary prep/caller results are not final private owners.
  - [x] FIX: strict core-lib/affected-test Clippy and both package fmt pass rc=0; no clone/counter advance/
    census/authority callback in projection, whole append precharge/exact overflow chunks, no clipping.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus::retained_authoring:::
    5 passed rc=0:alias2M+13/2B+76/5, eviction2M+21/2B+60/30, branch2M+2K+29/2B+110/36(K512).
    Exact/one-short errors, consumed backing, no-history1029/1028(original900+129, actual entry642bits),
    quota/cancel Hcopy1 vs success2, smaller retry keys, post-admission revocation/fresh grant2 verified.
  - [x] NO REGRESSION: current3/runtime revision1/audit successor1/append-boundary1/bit-boundary1 pass
    rc=0; primitive forecasts match actual arithmetic. Native original handlers/current model walkers unchanged.
  - [x] LOCKSTEP: book build/native publication85 chapters/159 scoped API rows/1531 source links/
    2580 rendered links rc=0; bounded publication chapter/nav, indexed promotion and scope examples sync.
    retention1077 logical/44 working Markdown/60212 decoded lines/4557003 decoded bytes/2387815
    resident bytes rc=0. History .3.4.2/coupled/full deferred; no allocator/physical/release certificate.
## _(2026-10-28 UTC)_ — exact prospective retained authoring publication

0470 .6.8.2.2.2.2.4.3.4.1 borrows actual bounded preparation and admits final retained candidate/record/history before successor/final-model/audit construction; temporary prep is bounded separately.
Exact future counters, alias copies, eviction and redo IDs/models use current schema order/original witnesses. Authoring5/current3/revision1/auditforecast1/append-boundary1/bit-boundary1 and strict lint/fmt pass.
Real oracles:alias2M+13/2B+76/5, eviction2M+21/2B+60/30, branch2M+2K+29/2B+110/36(K512), beyond-u1281029/one-short1028 with actual entry642bits. No final publication copies on quota/cancel; smaller retry keys survive.
Fresh final authority still refuses revocation after admission. Full consumed reserve backing rejects early. History prospective .3.4.2/coupled/full pending; no allocator-peak certificate. Promotion:indexed decision; whole0469 retained623.
## STITCHCAD-G1-0470 — admit exact prospective retained authoring publication

- Project actual prepared candidate, counters, new/existing audit records and exact undo eviction/redo discard before publication construction.
- Preserve original retained witnesses, aliases, cancelled/refused keys, full reserve backing and fresh final authority; temporary preparation stays separately bounded.
- Authoring5/current3/revision1/audit forecast1/two count-boundary cases and strict lint/fmt pass against real model/record oracles, one-short limits and beyond-u128 carry.
- Whole0469 retained623; bounded publication chapter/nav explains exact scope. Prospective history admission and coupled/full verification remain pending.

## Exact actual accepted-record resources0467

answers: "does exact replay eliminate returned record data, and does a discarded branch own a vector?"

Actual authoring commit/record/result and history request/action/result readers exhaustively borrow all
owners. Schema-order counts include effect sequence, every represented revision and every original
request/report snapshot. Actual effective-command and discarded-redo vectors precharge their full lengths;
aliases remain in copied requests, omitted only from the actual effective-command vector. Grant/time,
replay/direction and oldest-undo identity are fixed, guarded fields. Returned exact replay copies own
complete records despite no new private journal/action entry. Nested original witnesses remain unchanged.
Real bus author39/124/2,Undo15/4/4,Redo15/4/3,prefix41/126/3,branch7/64/5 and eviction3/4/18
pass record4/report5/request4/strict lint/fmt. Original agent and authenticated human operator stay distinct;
source/authority callbacks0 during readers. Runtime and prior reader implementations remain byte-exact.
Private aggregate/prospective retained admission and coupled/full verification stay separately owned.

## Explicit immutable retained-policy configuration0468

answers: "does installing retained limits walk private state or certify its current resource demand?"

A third private positive-limits policy starts absent in every bus constructor and installs once without
model/reserve/journal/history traversal, validation, cloning or authority callbacks. Same or larger
replacement refuses. Costly metadata paths require operation/model/retained setup; cheap authorized
Piece reads require none. Configuration presence affects handler availability but conveys no authority,
validity or quota-room certificate. Setup checks operation demand then model presence then retained
presence before authority/private work. Native setup2 and prior model/operation setup cases plus strict
lint/fmt verify constructor variants, original error precedence and zero callbacks. Private aggregate/
current admission .3.3.2 and exact prospective admission .3.4 remain required before retained closure.

### Focused doctest invocation0468

answers: "how is bus privacy checked without unintentionally selecting unrelated doctests?"

The observed cargo test -p sc-core --doc filter containing command::bus::CommandBus plus a spaced
line suffix selected171 cases, including unrelated types. The owned expanded rustdoc/children were
terminated (cargo101, SIGTERM); this interrupted run is not full verification. The space-free filter
command::bus::CommandBus selects exactly11 bus compile-failure contracts, including retained-policy
reset. Use that bounded type filter; do not infer one-case scope from a line suffix containing spaces.

## Actual current private retained composition0469

answers: "does current retained admission count history snapshots, old audit records and consumed IDs?"

One cumulative private walk charges actual current model, full immutable ID backing (consumed prefix
included), initial/max-issued/audit counters, every accepted record, undo/redo models in stored order,
and chronological action records. All owners are exhaustive, vectors precharged, fixed fields guarded;
A is the external trusted authority service and excluded, while each model's actual owned source H counts.
Fresh authority/exact snapshot precede RetainedValidation0/1; no public private-state getter exists.
Cancellation/refusal preserves model/reserve/keys/capacities and original resource cause/source chain.
Exact authoring/history replays bypass the current walk. The real independent model/record oracle is
3M+29 entries/3B+74 bytes/36 bits; exact and one-short unit limits, three-source/no-copy/no-semantic
admission and full consumed reserve count pass current3/model4/strict lint/fmt. Extra retained source
walks are separate from per-model admission and do not add candidate copies. This is current-only:
prospective .3.4 must prevent growing accepted content before retained allocations/effects; coupled/
full verification stays deferred. The current-only Notch proof tests a later-over-bound state, not
prospective enforcement; replace that setup with early-refusal proof when .3.4 lands.

## Exact prospective retained authoring publication0470

answers: "can authoring fit by evicting undo/discarding redo, and when are publication copies allowed?"

Borrow actual bounded prepared candidate/report/request. Before issuing counters, building final model,
allocating retained records/history or cloning returned data, admit the exact future private artifact
state. Forecast actual maximum-issued and independent audit sequence, never restored base. Keep every
old audit/action; append the exact new request/report/counters/discard IDs, omit evicted undo and redo
model alternatives, and append the actual old current model only if history is configured. Schema order,
whole vector precharge, source H and full reserve backing remain; no supplied totals/token/authority.
RetainedPublication0/1 cancels before construction; fresh final Commit authority still follows all prep.
Temporary preparation and caller-delivered result ownership are distinct from the final retained budget;
no allocator peak is claimed. Actual alias2M+13/2B+76/5, eviction2M+21/2B+60/30 and branch
2M+2K+29/2B+110/36(K512) match independent real models/records. No-history beyond-u128 needs1029bits;
1028 refuses original records.commit.revision900+129 before final copies, actual returned entry642bits.
Authoring5/current3/revision1/audit forecast1/two count-boundary cases/strict lint/fmt pass. Smaller retry
keys, full consumed backing and post-admission revocation verified. Exact history .3.4.2/coupled/full pending.
