# Sealed engineering continuity — part659

- **Identity:** 18 lines, 2454 bytes, sha256:3b89c29c9d89aa5d2af8090b44cfb851529df19ced346653ad92d0de818c7cdb
- **Coverage:** whole closed D302/D303/D304 ledgers before D305 logging from 6860292a88dd77464032a033debcb1bfc1896ba8.
- Whole source interval: docs/tasks/PLANNING.md bytes 92164–94618, 2454 bytes, sha256:3b89c29c9d89aa5d2af8090b44cfb851529df19ced346653ad92d0de818c7cdb
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

## D302 — aggregate count mistaken for active archive pressure

P2 fixed0490;owner G1-SLICE.6.5.3.2.retention. Recurrence of D284:0489 pointers/message inferred packing from64 aggregate workingMD.
Reproduction:validated generation decoder plus independent physical/catalog census →generation1=21,generation2=43 including42 raw/1 catalog,total64,rc=0. Contract limits each64/aggregate192;record644 fits active44.
Root:aggregate verify-retention readout was compared with a per-generation ceiling despite the public annex's explicit distinction. Impact:false blocker/unnecessary storage work;no archive packing/deletion occurred.
Fix:tracked generation_pressure.py validates groups and independently counts raw/catalog files;3 scope controls pass. After644:21+44=65 aggregate,20 active slots;retention1098 passes0. Current pointers corrected;whole0489 retains prior exact claim. No cap increase,sealed rewrite or packing;resume product.

## D303 — repeated retained-source interval claims

P1 fixed0492;owner G1-SLICE.6.5.3.2.3.2.3.2. Initial native duplicate_retained_interval_claims_cannot_spend_the_same_original_budget_twice →fails1/rc101 against first uncommitted0492 integration:two span IDs carry one identical source portion and the graph accepts it.
Root:source identity/content coherence alone does not enforce disjoint allocation claims. Impact:later quantity queries could spend the same original budget twice. Fixed with ordered per-source/per-side admission and original consumer/range evidence;native43 passes including four overlap arrangements/touching/separate budgets. Partial coverage stays legal. No released/committed
graph integration is affected.

## D304 — equal source content retained as multiple actual bodies

P1 fixed0492;owner G1-SLICE.6.5.3.2.3.2.3.2. Initial native equal_independent_source_captures_are_folded_into_one_actual_canonical_body →fails1/rc101 on first uncommitted0492 integration:equal roots have separate backing objects;one child does not point at the graph's charged canonical source.
Root:pool deduplicated identity/content but did not fold equal portion source pointers. Fixed after complete validation:fold equal same-id sources onto one actual shared body in first-consuming order;native pointer regression and logical meter7/limit6/snapshots14 pass within sewing43. Preserve original values/errors and charge each logical graph independently. No
committed/released integration is affected.
