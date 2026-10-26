# Sealed archive — whole0321 combined acceptance task and0298 member-point lesson

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.c.2.1` on `2026-10-26` (UTC).

- **Sealed identity:** 48 lines, 4048 bytes, sha256:1b7ff9d8de8882e15c27bd643237e7d7a0fe0ebe23f97006a16ae5da5b4ab953
- **Coverage:** complete intervals from 13b12b84de9e8f5dfff507f2b5e3fea1eaaef811.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 32130–35314, 3184 bytes, sha256:d199d8bcbc04280fd47ceff5e48c5f51fef3f2cc4509a2811602b184a18ebb5d
- Whole source interval: DEV_NOTES.md bytes 17194–18058, 864 bytes, sha256:1ba20103b839b223a1a7b2addb68e948b64f620702b3893cf47aa2931d366c82
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4d.2`
  Status: `done`
  Goal:0321 combined MeasurementTable/Ease/SizeSet structural acceptance; close .4 only after every child passes.
  Own measurement metadata/Ease/main SizeSet and availability/implementation-status book scope, live/task pointers,
  PLANNING and old .5 dependency exception. D235 overview still claims axes undecided/composition partial and
  trig/inverse unavailable; verify actual source/approved D70 and correct all encountered availability drift.
  Focus current measurement/table/Ease-set contracts: required landmarks/procedure and typed body/POM,
  per-POM query, canonical source/state/current-binding behavior. Reuse0320 model25 and unchanged composite31.
  Own book_publication/status.tsv registration of13 new principal model APIs under existing checker,
  then scoped publication/navigation; derive canonical18 top-level statuses before/after .4 closure.
  D236 check finds4 registered chapters absent from topic-index; own index repair and474 whole prior
  thirteenth-window governance receipt from140422e to keep the book fixed cap before adding links.
  Own472 whole0320 task/oldest0297 lesson/old scope paragraphs and .5 dependency exception;293 ledger
  and140 whole closed D233/D234 reports from140422e. Own475 whole closed .4a/.4a.2/.4b parent
  contracts/navigation from140422e to bound task aggregate; retain their live identity/status pointers.
  Also own473 whole original numeric-availability
  paragraph and prior .4d plan, supplementing context around472’s retained changed span,
  exact whole Git intervals/hash/public before retirement. No numeric certificate or mutation campaign rerun.
  Director feature-delivery ruling: focused structural acceptance now; full CI remains pre-push and G1 integration
  (.11/.12/.16), including current WASM/runtime proof. Closure claims no complete evaluator/bus/storage/app/release.
  - [x] ROOT CAUSE (WHY + WHERE): .4 acceptance versus actual APIs; D235 stale overview/D236 INDEX_COVERAGE reproduced; source check rc=0.
  - [x] ADDRESSED: cargo test -p sc-measure --test measurement_contract --test table_contract --test ease_set_contract gives46/0 rc=0.
  - [x] NO REGRESSION: source diff fb53a44 empty; prior25/31 proof retained; book/publication74 chapters/87 entries rc=0.
  Verification: book_publication.py check(ROOT) returns(74,87,1428,2315); topic/navigation/API-source/owner/roadmap checks rc=0.
  472/473/474/475/293/140 whole source intervals/hash/public byte equality rc=0; canonical18 census below gives6/18,
  only.4 active->done versus140422e. LIVE_STATUS changes5/18->6/18; full runtime/CI remains named later owners.
  Commit: STITCHCAD-G1-0321; next .6.2.2.c.2 remaining command-class schemas, no draft application/release claim.

  Progress census recipe (canonical18 roadmap leaves, excluding maintenance):
  ```python
  import re
  from pathlib import Path
  text = Path('docs/tasks/G1-SLICE.md').read_text()
  ids = ['1','2','3a','3b','3c', *map(str, range(4,17))]
  states = [re.search(r'^- ID: `G1-SLICE\.' + re.escape(k) + r'`\n  Status: `([^`]+)`', text, re.M).group(1) for k in ids]
  print(sum(s == 'done' for s in states), '/', len(states))
  ```
## _(2026-10-26 UTC)_ — complete explicit member points

0298 .4c.2.2 validates exact membership/axis pins, >=1 authored axis, distinct metadata/names and one
complete explicit point per actual member. Foreign/missing/duplicate coordinates and repeated tuples
refuse; sparse grids and unused values stay explicit. All authored vectors remain unchanged. Queries
revalidate sources/coverage and borrow original member/point/canonical records. Native aggregate5 plus
prior axis5 pass0; selected strict lint/fmt/book0.422/423/270 retain whole35cf8ac Git/hash/public bytes.
D70 axes contract closes; no full chart/break/resolved SizeSet or path-readiness claim. G1still5/18;
next .6.2.2.b.2.b paired split schema. Promotion: existing SizeSet decision extended, no new policy.
Broad integration/platform verification remains deferred to its owned milestone/pre-push.

